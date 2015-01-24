extern crate core;

use core::num::FromPrimitive;
use core::fmt;

use Direction::{SpinUp, SpinDown, SpinSuper};

#[derive(Copy)]
#[derive(Show)]
enum Direction {
    SpinUp,
    SpinDown,
    SpinSuper,
}


impl fmt::String for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = match self {
            &SpinUp => "SpinUp",
            &SpinDown => "SpinDown",
            &SpinSuper => "SpinSuper"
        };
        write!(f, "{}", r)
    }
}


struct Pair<T> {
    lhs: T,
    rhs: T
}

struct Particle {
    spin: Direction
}

impl Particle {
    fn new_pair () -> Pair<Particle> {
        let d1 = SpinSuper;
        let d2 = SpinSuper;
        let p1 = Particle{spin: d1};
        let p2 = Particle{spin: d2};

        return Pair{lhs:p1, rhs: p2};
    }
}

fn main () {

    let particles = Particle::new_pair();


    println!("lhs.spin {}", particles.lhs.spin);
}
