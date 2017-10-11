extern crate sindra;

use sindra::string::*;


#[test]
fn test_string() {
    fn assert_string(input: &str, expected: &str) {
        let mat = match_str_ext(input, 0);
        assert!(mat.is_some());
        let mat = mat.unwrap();
        assert_eq!(mat.0, input.len());
        let converted = convert_string(mat.1);
        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), expected);
    };
    assert_string(r#""foo""#, "foo");
    assert_string(r#""\x41\x2D\x5A""#, "A-Z");
    assert_string(r#""\u{263A}\u{2639}""#, "☺☹");
    assert_string(r#""☺☹""#, "☺☹");
}
