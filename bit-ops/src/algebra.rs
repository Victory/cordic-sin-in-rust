
#[allow(unused_assignments)]

pub fn add (a: i8, b: i8) -> i8 {   
    let mut i: i8 = 0;
    let mut c: i8 = 0;
    i = a ^ b;
    c = (a & b) << 1;
    
    if c & i != 0i8 {
        return add(i, c);
    }
    return i | c;
}


pub fn subtract (a: i8, b: i8) -> i8 {
    let not_b_plus_1 = add(!b, 1i8);
    return add(a, not_b_plus_1);
}

// return a > b
pub fn gt(a: i8, b: i8) -> bool {
    let a = a ^ 0b1000_0000;
    let b = b ^ 0b1000_0000;

    let mut ltb = (!a & b) as u8;
    let mut gtb = (a & !b) as u8;

    ltb = ltb | (ltb >> 1);
    ltb = ltb | (ltb >> 2);
    ltb = ltb | (ltb >> 4);

    return (gtb & !ltb) != 0;
}
