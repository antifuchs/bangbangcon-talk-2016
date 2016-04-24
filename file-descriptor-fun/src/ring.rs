use nix;
use nix::sys::socket;
use nix::sys::uio::IoVec;
use nix::unistd;
use std::result;
use std::fmt;
use std::os::unix::io::RawFd;

// OS X doesn't let us go beyond 256kB for the buffer size, so this is the max:
const SEND_BUF_SIZE: usize = 256 * 1024;

/// A ring buffer containing file descriptors.
///
/// You can stuff FDs in with the [`add`](#method.add) method, and
/// iterate over them one by one using the iterator structure returned
/// by [`iter`](#method.iter).
pub struct Ring {
    read: RawFd,
    write: RawFd,

    /// The number of file descriptors contained in the ring buffer.
    pub count: u64,
}

impl fmt::Display for Ring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#<Ring containing {} fds>", self.count)
    }
}

impl Drop for Ring {
    fn drop(&mut self) {
        // println!("Dropping sockets holding {} fds", self.count);
        unistd::close(self.write).unwrap();
        unistd::close(self.read).unwrap();
    }
}

/// Any sort of error that can occur while trying to speak the ring
/// buffer protocol
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum ProtocolError {
    /// Expected to receive an FD, but did not get one
    NoFDReceived,

    /// Expected one FD, got more
    TooManyFDsReceived,
}

#[derive(Debug)]
pub enum Error {
    /// A real error that prevents the Ring buffer from working
    Bad(nix::Error),

    /// An error that indicates some limit being reached. This is
    /// sometimes expected and realistic!
    Limit(nix::Error),

    /// A protocol error (e.g., messages on the socket didn't have the
    /// right format)
    Protocol(ProtocolError),
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

/// A specialized Result type for fd Ring buffer operations.
pub type Result<T> = result::Result<T, Error>;

// Create a new Ring with a UNIX domain socket pair.
pub fn new() -> Result<Ring> {
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
    /// Adds an FD to a Ring. Closing the FD to free up resources is left to the caller.
    pub fn add(&mut self, fd: RawFd) -> Result<()> {
        try!(self.insert(fd));
        self.count += 1;
        Ok(())
    }

    fn insert(&self, fd: RawFd) -> Result<()> {
        let buf = vec![IoVec::from_slice("!".as_bytes())];

        let fds = vec![fd];
        let cmsgs = vec![socket::ControlMessage::ScmRights(fds.as_slice())];
        try!(socket::sendmsg(self.write,
                              &buf.as_slice(),
                              cmsgs.as_slice(),
                              socket::MsgFlags::empty(),
                             None));
        Ok(())
    }

    fn next(&self) -> Result<RawFd> {
        let mut backing_buf = vec![0];
        let mut buf = vec![IoVec::from_mut_slice(&mut backing_buf)];

        // TODO: deal with the constant 15 here.
        let mut cmsg: socket::CmsgSpace<([RawFd; 15])> = socket::CmsgSpace::new();
        let msg = try!(socket::recvmsg(self.read,
                                       &mut buf.as_mut_slice(),
                                       Some(&mut cmsg),
                                       socket::MsgFlags::empty()));
        match msg.cmsgs().next() {
            Some(socket::ControlMessage::ScmRights(fd)) => {
                // TODO: this could probably handle the case of multiple FDs via buffers
                match fd.len() {
                    1 => {
                        let the_fd = fd[0];
                        try!(self.insert(the_fd));
                        Ok(the_fd)
                    }
                    0 => Err(Error::Protocol(ProtocolError::NoFDReceived)),
                    _ => Err(Error::Protocol(ProtocolError::TooManyFDsReceived)),
                }
            }
            _ => Err(Error::Protocol(ProtocolError::NoFDReceived)),
        }
    }

    /// Returns an iterator on the FDs contained in the ring buffer
    pub fn iter(&self) -> RingIter {
        RingIter {
            ring: &self,
            offset: 0,
        }
    }
}

/// An iterator over the File descriptors contained in an FD ring buffer
pub struct RingIter<'a> {
    ring: &'a Ring,
    offset: u64,
}

impl<'a> Iterator for RingIter<'a> {
    type Item = RawFd;

    fn next(&mut self) -> Option<RawFd> {
        self.offset += 1;
        if self.offset > self.ring.count {
            return None;
        }
        match self.ring.next() {
            Ok(next_fd) => Some(next_fd),
            Err(_) => None,
        }
    }
}
