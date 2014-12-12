
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
