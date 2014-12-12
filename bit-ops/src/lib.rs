#![crate_type = "lib"]
#![crate_name = "bit-ops"]

pub mod algebra;

#[test]
fn test_adding () {
    let a = 3u8;
    let b = 6u8;
    assert!(a + b == algebra::add(a, b));
}


#[test]
fn test_subtracting () {
    let a = 9u8;
    let b = 3u8;
    assert!(a - b == algebra::subtract(a, b));

    let minuend = 8;

    for subtrahend in range(0u8, 128) {
        assert!(minuend - subtrahend == algebra::subtract(minuend, subtrahend));
    }

}
