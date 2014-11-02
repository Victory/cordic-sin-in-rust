/**
use the taylor series to caculate tan^-1 (x) where |x| <= 1, x >= 0
**/
fn taylor_atan (x: f64) -> f64 {
    let mut a: f64 = 0.0;
    let mut s: f64 = 1.0;
    let mut xpow: f64;

    // by definition
    if x == 0.0 {
        return 0.0;
    }

    // this is well known, and the taylor series we are using explictly assumes |x| < 1
    if x == 1.0 {
        return std::f64::consts::FRAC_PI_4;
    }

    for ii in range(1i, 60) {
        if ii % 2 == 0 {
            continue;
        }
        xpow = 1.0;
        for _ in range(0i, ii) {
            xpow *= x;
        }
        a += s * xpow / (ii as f64);
        s = -1.0*s;
    }

    return a;
}

fn generate_table () {

    let pi: f64 = std::f64::consts::PI;
    let k1: f64 = 0.6072529350088812561694; // 1/k
    let num_bits: uint = 32;
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
        let cur: f64 = taylor_atan(ipow) * (mul as f64);

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

fn sin(theta: f64) -> f64 {

    let num_bits: uint = 32;

    let k1: f64 = 0.6072529350088812561694; // 1/k
    let mul: f64 = (1i << (num_bits-2)) as f64;
    let cordic_k1: f64 = k1 * mul;
    let mut x: int = cordic_k1.floor() as int; //0x26DD3B6A; 
    let mut y: int = 0;
    let mut z: int = (mul * theta).floor() as int;
    let mut tx: int;
    let mut ty: int;
    let mut tz: int;

    let mut d: int;
    
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
        x = tx; y = ty; z = tz;
    }

    let s: f64 = (y as f64)/(mul as f64);

    return s;

}


fn main() {
    generate_table();

    let frac_pi_2: f64 = std::f64::consts::FRAC_PI_2; //pi/2.0;

    for ii in range(0i, 50) {
        let cur: f64 = ii as f64;
        let theta: f64 = cur/50.0 * frac_pi_2;
        let s: f64 = sin(theta);
            println!("sin({}) ~= {}", theta, s);
    }
}
