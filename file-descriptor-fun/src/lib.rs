#![allow(dead_code)]
#![deny(warnings)]
#![crate_type = "lib"]

extern crate nix;

pub mod ring;

use nix::sys::socket;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::os::unix::io::RawFd;

const BASE_PATH: &'static str = "/tmp/filedes_fun/";
const MAX_BACKLOG_QUEUE: usize = 265;

const SOCKET_TYPE: socket::SockType = socket::SockType::Stream;
const SOCKET_PROTO: nix::c_int = 0;

// Setup the directory they'll live in.
pub fn setup() -> io::Result<()> {
    fs::create_dir_all(Path::new(self::BASE_PATH))
}

pub fn teardown() -> io::Result<()> {
    fs::remove_dir_all(Path::new(self::BASE_PATH))
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
                              socket::SOCK_NONBLOCK);
}
