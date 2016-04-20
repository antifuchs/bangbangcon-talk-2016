extern crate nix;

mod filedes {
    use nix::sys::socket;
    use std::path::{Path,PathBuf};
    use std::fs;
    use std::io;
    use nix;
    use std::os::unix::io::RawFd;

    const BASE_PATH: &'static str = "/tmp/filedes_fun/";
    const MAX_BACKLOG_QUEUE: usize = 265;

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
        let socket = try!(socket::socket(socket::AddressFamily::Unix, socket::SockType::Stream, socket::SockFlag::empty(), 0));
        let sockaddr = try!(make_socket_addr(path));
        try!(socket::bind(socket, &sockaddr));
        try!(socket::listen(socket, MAX_BACKLOG_QUEUE));
        Ok(socket)
    }

    pub fn connect_to_socket(path: &str) -> Result<RawFd, nix::Error> {
        let socket = try!(socket::socket(socket::AddressFamily::Unix, socket::SockType::Stream, socket::SockFlag::empty(), 0));
        let sockaddr = try!(make_socket_addr(path));
        try!(socket::connect(socket, &sockaddr));
        Ok(socket)
    }
}

#[cfg(test)]
mod tests {
    use filedes;
    use std::str;
    use std::thread;
    use nix::sys::socket;
    use nix::unistd;

    #[test]
    fn it_does_very_simple_things() {
        filedes::setup().unwrap();
        filedes::make_socket_addr("mysock").unwrap();
        filedes::server_socket("mysock").unwrap();
    }

    #[test]
    fn it_roundtrips() {
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
}
