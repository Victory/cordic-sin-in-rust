fn generate_table () {
    let pi: f32 = 3.1415926536897932384626;
    let k1: f32 = 0.6072529350088812561694; // 1/k
    let num_bits: uint = 32;
    let num_elms: uint = num_bits;
    let mul: uint = 1<<(num_bits-2);

    println!("Cordic sin in rust");
    println!("num bits {}", num_bits);
    println!("pi is {}", pi);
    println!("k1 is {}", k1);

    let shift: f32 = 2.0;
    for ii in range(0, num_bits) {
        let ipow: f32 = 1.0/shift.powi(ii as i32);
        let cur: f32 = ipow.atan();
        println!("table values {:.31f}", cur);
    }
}


fn main() {
    generate_table();
}
