extern crate filedes;
extern crate nix;

pub mod ring;

use filedes;
use filedes::ring;
use nix;
use std::os::unix::io::RawFd;

#[test]
fn it_can_create_a_ringbuffer() {
    let ring = filedes_ring::new().unwrap();
    println!("Got a ring: {}", ring);
}

#[test]
fn adding_to_ring_works() {
    let mut ring = filedes_ring::new().unwrap();
    let (one, two) = filedes::unix_socket_pair().unwrap();
    ring.add(one).unwrap();
    assert_eq!(1, ring.count);
    ring.add(two).unwrap();
    assert_eq!(2, ring.count);
}

#[test]
fn adding_many_to_a_ring_works() {
    let mut ring = filedes_ring::new().unwrap();

    loop {
        let (one, two) = filedes::unix_socket_pair().unwrap();
        match ring.add(one) {
            Ok(()) => {
                nix::unistd::close(one).unwrap();
            }
            Err(filedes_ring::Error::Limit(e)) => {
                println!("I hit {}", e);
                nix::unistd::close(one).unwrap();
                break;
            }
            Err(e) => {
                panic!(e);
            }
        }
        match ring.add(two) {
            Ok(()) => {
                nix::unistd::close(two).unwrap();
            },
            Err(filedes_ring::Error::Limit(e)) => {
                println!("I hit {}", e);
                nix::unistd::close(two).unwrap();
                break;
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
    let mut additional_fds: Vec<RawFd> = vec!();
    loop {
        match filedes::unix_socket_pair() {
            Ok((one, two)) => {
                additional_fds.push(one);
                additional_fds.push(two);
            }
            Err(e) => {
                println!("Hit {}, aborting", e);
                break;
            }
        }

    }
    println!("I managed to store a bunch of FDs in {}", ring);
    println!("...and I opened {} FDs", additional_fds.len());
    assert!(additional_fds.len() > 0);
    for fd in additional_fds {
        nix::unistd::close(fd).unwrap();
    }
}