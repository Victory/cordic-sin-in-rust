fn generate_table () {
    let pi: f32 = 3.1415926536897932384626;
    let k1: f32 = 0.6072529350088812561694; // 1/k
    let num_bits: uint = 32;
    let num_elms: uint = num_bits;
    let mul: f32 = 1073741824.000000; // 1<<(bits-2)

    println!("Cordic sin in rust");
    println!("num bits {}", num_bits);
    println!("mul {}", mul);
    println!("pi is {}", pi);
    println!("k1 is {}", k1);

    let shift: f32 = 2.0;
    for ii in range(0, num_bits) {
        let ipow: f32 = 1f32/shift.powi(ii as i32);
        let cur: f32 = ipow.atan() * mul;
        //println!("table values {:.10f}", cur);
        println!("table values 0x{}", std::f32::to_str_hex(cur));
    }
}

fn main() {
    generate_table();
}
