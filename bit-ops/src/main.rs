extern crate "bit-ops" as bit_ops;

use bit_ops::algebra::{add, subtract};

fn main () {
    let a = 3i8;
    let b = 6i8;
    println!("{:0>8b} + {:0>8b} = {:0>8b}", a, b, add(a, b));

    let a = 8i8;
    let b = 28i8;
    println!("{:0>8b} - {:0>8b} = {:0>8b} = {:0>8b}", a, b, subtract(a, b), a - b);
    println!("{} - {} = {}", a, b, subtract(a, b));
}
