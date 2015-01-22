extern crate core;
use core::num::FromPrimitive;



#[derive(Copy)]
#[derive(Show)]
#[derive(FromPrimitive)]
enum Direction {
    SpinUp = 0x1,
    SpinDown = 0x2
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
        let d1 = FromPrimitive::from_u8(0x1).expect("unknown direction");
        let d2 = FromPrimitive::from_u8(0x2).expect("unknown direction");
        let p1 = Particle{spin: d1};
        let p2 = Particle{spin: d2};

        return Pair{lhs:p1, rhs: p2};
    }
}

fn main () {
}
