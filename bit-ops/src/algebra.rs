
#[allow(unused_assignments)]

pub fn add (a: u8, b: u8) -> u8 {   
    let mut i: u8 = 0;
    let mut c: u8 = 0;
    i = a ^ b;
    c = (a & b) << 1;
    
    if c & i != 0u8 {
        return add(i, c);
    }
    return i | c;
}

