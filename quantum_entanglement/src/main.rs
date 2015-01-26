extern crate core;

use core::num::FromPrimitive;
use core::fmt;

use Direction::{SpinUp, SpinDown, SpinSuper};
use Plan::{Switch, OddBall};

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

enum Plan {
    Switch, // up-up-up -> down-down-down
    OddBall // 
}

enum Detector {
    D12, // 12 o'clock
    D3, // 3 o'clock
    D9, // 9 o' clock
}

impl Particle {
    fn new_pair () -> Pair<Particle> {
        let d1 = SpinSuper;
        let d2 = SpinSuper;
        let p1 = Particle{spin: d1};
        let p2 = Particle{spin: d2};

        return Pair{lhs: p1, rhs: p2};
    }

    pub fn measure (&mut self, theta: int) {
        // TODO theta = 60degrees use 3/4th and 1/4th
        // TODO theta = 0 SpinUp
        // TODO theta = 180 SpinDown
    }

    // NOTE on spooky and premeditated only if measured in the same
    // direction must the spins be opposite

    // measure with with a message
    pub fn spooky (&mut self, friend: &mut Particle) -> Pair<Direction> {

        let spin = match self.spin {
            SpinUp => SpinDown,
            SpinDown => SpinUp,
            SpinSuper => SpinUp
        };
        friend.spin = spin;

        let spin = match friend.spin {
            SpinUp => SpinDown,
            SpinDown => SpinUp,
            _ => panic!("broke the universe")
        };
        self.spin = spin;

        return Pair{lhs: self.spin, rhs: friend.spin};
    }

    pub fn premeditated (&mut self, friend: &mut Particle, theta: int, plan: Plan) -> Pair<Direction> {

        // TODO: Use plan
        friend.spin = SpinUp;
        
        let spin = match friend.spin {
            SpinUp => SpinDown,
            SpinDown => SpinUp,
            _ => panic!("broke the universe")
        };
        self.spin = spin;

        return Pair{lhs: self.spin, rhs: friend.spin};
    }
}


fn main () {

    let particles = Particle::new_pair();

    let mut lhs = particles.lhs;
    let mut rhs = particles.rhs;

    lhs.spooky(&mut rhs);

    println!("lhs.spin {}, rhs.spin {}", lhs.spin, rhs.spin);

    // premeditated should give +55.6% difference, spooky would give 50%
}
