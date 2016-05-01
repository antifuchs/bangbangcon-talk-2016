extern crate filedes;
extern crate nix;

use filedes::ring;
use filedes::{add_two_sockets_to_ring,add_tmpfile_to_ring};

// In Linux, this works! We can send rings down rings, and the system
// will get very very slow, but sockets containing FDs can be sent
// down sockets, and can be read off them.
//
// In OS X, I'm getting NoFDReceived messages, which indicates to me
// that the OS is closing those. Ugh!
#[cfg(not(target_os="macos"))]
#[test]
fn adding_rings_to_rings_works() {
    let mut outer_ring = ring::new().unwrap();
    let mut total = 0;
    let mut outer_entries = 0;
    'outer: loop {
        let mut inner_ring = ring::new().unwrap();
        'inner: loop {
            match add_tmpfile_to_ring(&mut inner_ring) {
                Ok(n) => { total += n; }
                Err(ring::Error::Limit(nix::Error::Sys(nix::errno::Errno::EAGAIN))) => {
                    if inner_ring.count > 1 {
                        println!("The inner ring is at {}, outer at {}, proceeding to the next stage", inner_ring.count, outer_ring.count);
                        match outer_ring.add(&ring::StashableThing::from(&inner_ring)) {
                            Ok(()) => {}
                            Err(_) => { break 'outer }
                        }
                        break 'inner;
                    } else {
                        break 'outer;
                    }
                }
                Err(ring::Error::Limit(e)) => {
                    println!("I hit {} - this means something global is full, probably", e);
                    outer_ring.add(&ring::StashableThing::from(&inner_ring)).unwrap();
                    break 'outer;
                }
                Err(e) => { panic!(e); }
            }
        }
        outer_entries += 1;
        if outer_entries > 50 {
            break;
        }
    }
    println!("Assembled an outer ring of {} and a total of {} FDs", outer_ring, total);
    assert!(outer_ring.count > 1);

    let mut closed = 0;
    for inner_thing in outer_ring.iter() {
        match inner_thing {
            ring::StashedThing::Pair(mut inner_ring) => {
                while inner_ring.count > 0 {
                    let thing = inner_ring.pop().unwrap();
                    match thing {
                        ring::StashedThing::Pair(_) => {
                            panic!("I don't know how I could get to a ring in inner");
                        }
                        ring::StashedThing::One(fd) => {
                            nix::unistd::close(fd).unwrap();
                            closed += 1;
                        }
                    }
                }
            }
            _ => { panic!("Don't know how I could get to a non-ring in outer"); }
        }
    }
    assert_eq!(total, closed);

}
