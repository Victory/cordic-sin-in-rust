extern crate "bit-ops" as bit_ops;

use bit_ops::algebra::{add, subtract};

fn main () {
    let a = 3u8;
    let b = 6u8;
    println!("{:0>8b} + {:0>8b} = {:0>8b}", a, b, add(a, b));

    let a = 8u8;
    let b = 28u8;
    println!("{:0>8b} - {:0>8b} = {:0>8b} = {:0>8b}", a, b, subtract(a, b), a - b);
    println!("{} - {} = {}", a, b, subtract(a, b));
}
