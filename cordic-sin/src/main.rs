fn generate_table () {
    let pi: f64 = 3.1415926536897932384626;
    let k1: f64 = 0.6072529350088812561694; // 1/k
    let num_bits: uint = 32;
    //let num_elms: uint = num_bits;
    let mul: uint = 1<<(num_bits-2);

    println!("Cordic sin in rust");
    println!("num bits {}", num_bits);
    println!("mul {}", mul);
    println!("1<<(num_bits-2) {}", ((1i << (num_bits-2)) as f64));
    println!("pi is {}", pi);
    println!("k1 is {}", k1);

    let shift: f64 = 2.0;
    for ii in range(0, num_bits) {
        let ipow: f64 = 1f64/shift.powi(ii as i32);
        let cur: f64 = ipow.atan() * (mul as f64);
        //println!("table values {:.10f}", cur);
        println!("table values 0x{}", std::f64::to_str_hex(cur));
    }
}

fn main() {
    generate_table();
}
