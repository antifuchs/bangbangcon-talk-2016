use nix;
use nix::sys::socket;
use nix::sys::uio::IoVec;
use nix::unistd;
use std::fmt;
use std::os::unix::io::RawFd;

// OS X doesn't let us go beyond 256kB for the buffer size, so this is the max:
const SEND_BUF_SIZE: usize = (512-64) * 1024;

pub struct Ring {
    read: RawFd,
    write: RawFd,
    pub count: u64,
}

impl fmt::Display for Ring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#<Ring containing {} fds>", self.count)
    }
}

impl Drop for Ring {
    fn drop(&mut self) {
        //println!("Dropping sockets holding {} fds", self.count);
        unistd::close(self.write).unwrap();
        unistd::close(self.read).unwrap();
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
            nix::Error::Sys(nix::errno::Errno::EAGAIN) => Error::Limit(err),
            nix::Error::Sys(nix::errno::Errno::ENFILE) => Error::Limit(err),
            nix::Error::Sys(_) => Error::Bad(err),
            nix::Error::InvalidPath => Error::Bad(err),
        }
    }
}

// Create a new Ring with a UNIX domain socket pair.
pub fn new() -> Result<Ring, Error> {
    use super::unix_socket_pair;

    let (read, write) = try!(unix_socket_pair());
    // Adjust limits:
    let buf_size: usize = SEND_BUF_SIZE;
    try!(socket::setsockopt(write, socket::sockopt::SndBuf, &buf_size));
    return Ok(Ring {
        read: read,
        write: write,
        count: 0,
    });
}

impl Ring {
    // Adds an FD to a Ring. Closing the FD to free up resources is left to the caller.
    pub fn add(&mut self, fd: RawFd) -> Result<(), Error> {
        let buf = vec![IoVec::from_slice("!".as_bytes())];

        let fds = vec![fd];
        let cmsgs = vec![socket::ControlMessage::ScmRights(fds.as_slice())];
        match socket::sendmsg(self.write,
                              &buf.as_slice(),
                              cmsgs.as_slice(),
                              socket::MsgFlags::empty(),
                              None) {
            Ok(_) => {
                self.count += 1;
                Ok(())
            }
            Err(e) => Err(From::from(e)),
        }
    }
}
