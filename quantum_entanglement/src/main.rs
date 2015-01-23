extern crate core;

use core::num::FromPrimitive;
use core::fmt;

#[derive(Copy)]
#[derive(Show)]
#[derive(FromPrimitive)]
enum Direction {
    SpinUp = 0x1,
    SpinDown = 0x2,
    SpinSuper = 0x3,
}


impl fmt::String for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = match self {
            &Direction::SpinUp => "SpinUp",
            &Direction::SpinDown => "SpinDown",
            &Direction::SpinSuper => "SpinSuper"
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
        let d1 = FromPrimitive::from_u8(0x3).expect("unknown direction");
        let d2 = FromPrimitive::from_u8(0x3).expect("unknown direction");
        let p1 = Particle{spin: d1};
        let p2 = Particle{spin: d2};

        return Pair{lhs:p1, rhs: p2};
    }
}

fn main () {

    let particles = Particle::new_pair();


    println!("lhs.spin {}", particles.lhs.spin);
}
