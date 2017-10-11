extern crate sindra;

use sindra::float::*;

#[test]
fn test_float() {
    assert_eq!(float("0.0"), Ok(0.0));
    assert_eq!(float("4.3"), Ok(4.3));

    assert_eq!(float("4.3e2"), Ok(430.0));
    assert_eq!(float("4.3e-2"), Ok(0.043));
    assert_eq!(float("4.3E2"), Ok(430.0));
    assert_eq!(float("4.3E-2"), Ok(0.043));

    assert_eq!(float("43e2"), Ok(4300.0));
    assert_eq!(float("43e-2"), Ok(0.43));
}
