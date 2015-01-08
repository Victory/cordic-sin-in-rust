#![crate_type = "lib"]
#![crate_name = "bit-ops"]

pub mod algebra;

#[test]
fn test_adding () {
    let a = 3i8;
    let b = 6i8;
    assert!(a + b == algebra::add(a, b));
}


#[test]
fn test_subtracting () {
    let a = 9i8;
    let b = 3i8;

    let result = a - b == algebra::subtract(a, b);
    assert!(result);

    for minuend in range(0i8, 128) {
        for subtrahend in range(0i8, 128) {
            let result = minuend - subtrahend == algebra::subtract(minuend, subtrahend);
            assert!(result);
        }
    }

}


#[test]
fn test_gt () {
     for a in range(-10i8, 20i8) {
        for b in range (-10i8, 20i8) {
            assert!(a > b == algebra::gt(a, b), format!("{} > {} == a > b", a, b));
        }
    }
}

#[test]
fn test_lt () {
     for a in range(-10i8, 20i8) {
        for b in range (-10i8, 20i8) {
            assert!(a < b == algebra::lt(a, b), format!("{} < {} == {} ", a, b, a < b));
        }
    }
}


#[test]
fn test_hemming () {
    use algebra::hemming;

    let result = hemming(1i8);
    assert!(result == 1i8);

    let result = hemming(2i8);
    assert!(result == 1i8);

    let result = hemming(3i8);
    assert!(result == 2i8);
}
