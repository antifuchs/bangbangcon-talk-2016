#![allow(dead_code)]
#![deny(warnings)]

extern crate nix;

mod filedes {
    use nix::sys::socket;
    use std::path::{Path, PathBuf};
    use std::fs;
    use std::io;
    use nix;
    use std::os::unix::io::RawFd;

    const BASE_PATH: &'static str = "/tmp/filedes_fun/";
    const MAX_BACKLOG_QUEUE: usize = 265;

    const SOCKET_TYPE: socket::SockType = socket::SockType::Stream;
    const SOCKET_PROTO: nix::c_int = 0;

    // Setup the directory they'll live in.
    pub fn setup() -> io::Result<()> {
        fs::create_dir_all(Path::new(self::BASE_PATH))
    }

    fn sockpath(path: &str) -> PathBuf {
        Path::new(BASE_PATH).join(Path::new(path))
    }

    pub fn make_socket_addr(path: &str) -> Result<socket::SockAddr, nix::Error> {
        socket::SockAddr::new_unix(sockpath(path).as_path())
    }

    pub fn server_socket(path: &str) -> Result<RawFd, nix::Error> {
        let socket = try!(socket::socket(socket::AddressFamily::Unix,
                                         SOCKET_TYPE,
                                         socket::SockFlag::empty(),
                                         SOCKET_PROTO));
        let sockaddr = try!(make_socket_addr(path));
        try!(socket::bind(socket, &sockaddr));
        try!(socket::listen(socket, MAX_BACKLOG_QUEUE));
        Ok(socket)
    }

    pub fn connect_to_socket(path: &str) -> Result<RawFd, nix::Error> {
        let socket = try!(socket::socket(socket::AddressFamily::Unix,
                                         SOCKET_TYPE,
                                         socket::SockFlag::empty(),
                                         SOCKET_PROTO));
        let sockaddr = try!(make_socket_addr(path));
        try!(socket::connect(socket, &sockaddr));
        Ok(socket)
    }

    // Creates a socketpair in the UNIX domain and returns it
    pub fn unix_socket_pair() -> Result<(RawFd, RawFd), nix::Error> {
        return socket::socketpair(socket::AddressFamily::Unix,
                                  SOCKET_TYPE,
                                  SOCKET_PROTO,
                                  socket::SockFlag::empty());
    }
}

mod filedes_ring {
    use filedes;
    use nix;
    use std::fmt;
    use std::os::unix::io::RawFd;

    pub struct Ring {
        read: RawFd,
        write: RawFd,
        count: u64,
    }

    impl fmt::Display for Ring {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "#<Ring containing {} fds>", self.count)
        }
    }

    #[derive(Debug)]
    pub enum Error {
        // A real error that prevents the Ring buffer from working
        Bad(nix::Error),

        // An error that indicates some limit being reached. This is sometimes expected and realistic!
        Limit(nix::Error),
    }

    impl From<nix::Error> for Error {
        fn from(err: nix::Error) -> Error {
            match err {
                nix::Error::Sys(nix::errno::Errno::EMFILE) => Error::Limit(err),
                nix::Error::Sys(_) => Error::Bad(err),
                nix::Error::InvalidPath => Error::Bad(err),
            }
        }
    }

    // Create a new Ring with a UNIX domain socket pair.
    pub fn new() -> Result<Ring, Error> {
        let (read, write) = try!(filedes::unix_socket_pair());
        return Ok(Ring {
            read: read,
            write: write,
            count: 0,
        });
    }
}

#[cfg(test)]
mod filedes_tests {
    use filedes;
    use std::str;
    use std::thread;
    use nix::sys::socket;
    use nix::sys::uio::IoVec;
    use std::os::unix::io::RawFd;
    use nix::unistd;

    #[test]
    fn it_does_very_simple_things() {
        filedes::setup().unwrap();
        filedes::make_socket_addr("mysock").unwrap();
        filedes::server_socket("mysock").unwrap();
    }

    #[test]
    fn it_sends_data() {
        filedes::setup().unwrap();
        let s_sock = filedes::server_socket("mysock2").unwrap();
        thread::spawn(move || {
            let conn = socket::accept(s_sock).unwrap();
            socket::send(conn, "flub".as_bytes(), socket::MsgFlags::empty()).unwrap();
            unistd::close(s_sock).unwrap();
        });
        let sock = filedes::connect_to_socket("mysock2").unwrap();

        let mut buf: [u8; 5] = [0, 0, 0, 0, 0];
        let received_bytes = socket::recv(sock, &mut buf, socket::MsgFlags::empty()).unwrap();
        assert_eq!(received_bytes, 4);
        assert_eq!(str::from_utf8(&buf[0..received_bytes]).unwrap(), "flub");
    }

    #[test]
    fn it_sends_fds() {
        filedes::setup().unwrap();
        let s_sock = filedes::server_socket("mysock3").unwrap();
        thread::spawn(move || {
            let conn = socket::accept(s_sock).unwrap();

            let buf = vec![IoVec::from_slice("!".as_bytes())];

            let fds = vec![conn];
            let cmsgs = vec![socket::ControlMessage::ScmRights(fds.as_slice())];
            socket::sendmsg(conn,
                            &buf.as_slice(),
                            cmsgs.as_slice(),
                            socket::MsgFlags::empty(),
                            None)
                .unwrap();
            unistd::close(s_sock).unwrap();
        });
        let sock = filedes::connect_to_socket("mysock3").unwrap();

        let mut backing_buf = vec![0];
        let mut buf = vec![IoVec::from_mut_slice(&mut backing_buf)];
        let mut cmsg: socket::CmsgSpace<([RawFd; 15])> = socket::CmsgSpace::new();
        let msg = socket::recvmsg(sock,
                                  &mut buf.as_mut_slice(),
                                  Some(&mut cmsg),
                                  socket::MsgFlags::empty())
                      .unwrap();
        assert_eq!(1, msg.cmsgs().count());

    }
}

#[cfg(test)]
mod ring_tests {
    use filedes_ring;

    #[test]
    fn it_can_create_a_ringbuffer() {
        let ring = filedes_ring::new().unwrap();
        println!("Got a ring: {}", ring);
    }

    #[test]
    fn it_can_create_many_ringbuffers() {
        let mut limit_reached = false;
        let mut count = 0;

        while !limit_reached {
            match filedes_ring::new() {
                Ok(_) => {},
                Err(filedes_ring::Error::Bad(err)) => { panic!(err); }

                // This ends up matching just shortly before the
                // $`ulimit -n` / 2$ mark: We're creating socket
                // pairs, after all!
                Err(filedes_ring::Error::Limit(_)) => { limit_reached = true; }
            }
            count += 1;
        }
        println!("Reached the limit at {} ring buffers", count);
        assert!(limit_reached);
    }
}
