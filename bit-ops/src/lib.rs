#![crate_type = "lib"]
#![crate_name = "bit-ops"]

pub mod algebra;

#[test]
fn test_adding () {
    let a = 3u8;
    let b = 6u8;
    let expected = a + b;

    assert!(a + b == algebra::add(a, b));

}
