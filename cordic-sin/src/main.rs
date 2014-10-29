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

    println!("let cordic_tab = [");

    let shift: f64 = 2.0;
    for ii in range(0, num_bits) {
        let ipow: f64 = 1f64/shift.powi(ii as i32);
        let cur: f64 = ipow.atan() * (mul as f64);

        if ii % 4 == 0 && ii > 0 {
            println!("");
        }
        if ii == 0 {
            print!("{:0.8f}f64, ", cur);
        } else {
            print!("{:0.8f}, ", cur);
        }
    }

    println!("];");
}

fn sin(theta: f64) {

    let num_bits: uint = 32;

    let k1: f64 = 0.6072529350088812561694; // 1/k
    let mul: uint = 1<<(num_bits-2);
    let mut x: f64 = (mul as f64) * k1;
    let mut y: f64 = 0.0;
    let mut z: f64 = theta;
    


    let mut d: int;

    
    let z: int = theta as int;
    let cordic_tab = [
        843314856.53262615f64, 497837829.38176435, 263043836.58692065, 133525158.66814968, 
        67021686.89696868, 33543515.72887244, 16775850.86663180, 8388437.33958306, 
        4194282.66686198, 2097149.33333944, 1048575.66666686, 524287.95833334, 
        262143.99479167, 131071.99934896, 65535.99991862, 32767.99998983, 
        16383.99999873, 8191.99999984, 4095.99999998, 2048.00000000, 
        1024.00000000, 512.00000000, 256.00000000, 128.00000000, 
        64.00000000, 32.00000000, 16.00000000, 8.00000000, 
        4.00000000, 2.00000000, 1.00000000, 0.50000000];

    for ii in range(0, num_bits) {
        if z >= 0 {
            d = 0;
        } else {
            d = -1;
        }

        /*
        tx = x - (((y>>k) ^ d) - d);
        ty = y + (((x>>k) ^ d) - d);
        tz = z - ((cordic_ctab[k] ^ d) - d);
        x = tx; y = ty; z = tz;
        */

        println!("{}", d);
    }

}
fn main() {
    generate_table();

    sin(0.5);
}
