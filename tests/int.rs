extern crate sindra;

use sindra::int::*;

#[test]
fn test_num() {
    assert_eq!(int("0"), Ok(0));

    assert_eq!(int("4"), Ok(4));
}
