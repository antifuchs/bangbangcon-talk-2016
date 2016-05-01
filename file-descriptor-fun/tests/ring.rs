extern crate filedes;
extern crate nix;

use filedes::ring;
use filedes::add_two_sockets_to_ring;
use std::os::unix::io::RawFd;

#[test]
fn it_can_create_a_ringbuffer() {
    let ring = ring::new().unwrap();
    println!("Got a ring: {}", ring);
}

#[test]
fn adding_to_ring_works() {
    let mut ring = ring::new().unwrap();
    let (one, two) = filedes::unix_socket_pair().unwrap();
    ring.add(&ring::StashableThing::from(one)).unwrap();
    assert_eq!(1, ring.count);
    ring.add(&ring::StashableThing::from(two)).unwrap();
    assert_eq!(2, ring.count);

    let other_ring = ring::new().unwrap();
    ring.add(&ring::StashableThing::from(&other_ring)).unwrap();
    assert_eq!(3, ring.count);

    let received = ring.pop().unwrap();
    match received {
        ring::StashedThing::One(_) => {
            println!("Yay!");
        }
        _ => {
            panic!("Huh!");
        }
    }
}

#[test]
fn adding_many_to_a_ring_works() {
    let mut ring = ring::new().unwrap();

    loop {
        match add_two_sockets_to_ring(&mut ring) {
            Ok(_) => {}
            Err(ring::Error::Limit(e)) => {
                println!("I hit {}", e);
                break;
            }
            Err(e) => { panic!(e); }
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

    // println!("Waiting 60s");
    // std::thread::sleep(std::time::Duration::from_secs(60));

    println!("Closing the additional FDs now...");
    for fd in additional_fds {
        nix::unistd::close(fd).unwrap();
    }

    let should_close = ring.count;
    let mut closed = 0;

    println!("Closing the stashed FDs now...");
    for thing in ring.iter() {
        closed += 1;
        match thing {
            ring::StashedThing::One(fd) => {
                nix::unistd::close(fd).unwrap();
            }
            ring::StashedThing::Pair(_) => {}
        }
    }
    assert_eq!(should_close, closed);

    println!("Closing the stashed FDs a second time, properly...");
    while ring.count > 0 {
        let thing = ring.pop().unwrap();
        match thing {
            ring::StashedThing::One(fd) => {
                nix::unistd::close(fd).unwrap();
            }
            ring::StashedThing::Pair(_) => {
            }
        }
    }
}
