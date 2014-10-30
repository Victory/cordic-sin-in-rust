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
            print!("0x{}, ", std::f64::to_str_hex(cur.floor()));
        } else {
            print!("0x{}, ", std::f64::to_str_hex(cur.floor()));
        }
    }

    println!("];");
}

fn int_to_f64 (lhs: int) -> f64 {
    let rhs: f64 = unsafe {
        std::mem::transmute(lhs)
    };
    return rhs;
}


fn f64_to_int (lhs: f64) -> int {
    let rhs: int = unsafe {
        std::mem::transmute(lhs)
    };
    return rhs;
}


fn sin(theta: f64) {

    let num_bits: uint = 32;

    let k1: int = f64_to_int(0.6072529350088812561694); // 1/k
    let mul: int = 1<<(num_bits-2);
    let mut x: int = mul * k1;
    let mut y: int = 0;
    let mut z: int = f64_to_int(theta);
    let mut tx: int;
    let mut ty: int ;
    let mut tz: int;

    let mut d: int;

    println!("x {}", x);

    let cordic_tab = [
        0x3243f6a8, 0x1dac6705, 0xfadbafc, 0x7f56ea6, 
        0x3feab76, 0x1ffd55b, 0xfffaaa, 0x7fff55, 
        0x3fffea, 0x1ffffd, 0xfffff, 0x7ffff, 
        0x3ffff, 0x1ffff, 0xffff, 0x7fff, 
        0x3fff, 0x1fff, 0xfff, 0x7ff, 
        0x3ff, 0x1ff, 0xff, 0x7f, 
        0x3f, 0x1f, 0xf, 0x8, 
        0x4, 0x2, 0x1, 0x0, ];
    
    for k in range(0, num_bits) {
        if z >= 0 {
            d = 0;
        } else {
            d = -1;
        }
        tx = x - (((y>>k) ^ d) - d);
        ty = y + (((x>>k) ^ d) - d);
        tz = z - ((cordic_tab[k] ^ d) - d);
        x = tx; 
        y = ty; 
        z = tz;

        println!("{}", k);
        println!("{}", int_to_f64(tx));
    }
}


fn main() {
    let i: int = 42882;
    let f: f64 = 3498043.4824;
    assert!(int_to_f64(f64_to_int(f)) == f);
    assert!(f64_to_int(int_to_f64(i)) == i);

    generate_table();
    sin(0.5);
}
