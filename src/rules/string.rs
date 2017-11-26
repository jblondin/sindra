//! Rules and methods for parsing strings with escapes.

use regex::{Regex, Captures};
use std::char;

type Result<T> = ::std::result::Result<T, &'static str>;

/// Generates a regex pattern 1-6 hexadecimal digit unicode character specification.
macro_rules! unicode_char_pattern {
    () => { r#"\\u\{([[:xdigit:]]{1,6})\}"# };
}
lazy_static! {
    /// Regular expression for unicode character specification.
    pub static ref UNICODE_REGEX: Regex = { Regex::new(unicode_char_pattern!()).unwrap() };
}

/// Generates a regex pattern for escape sequences: single-caracter, one-byte, or unicode.
macro_rules! escape_pattern {
    () => { concat!(
        r#"(?xs)
        \\                                   # opening backslash
        [nrt\\0'"]                           # single-character escapes
        |                                    # or
        (?:\\x[[:xdigit:]]{2})               # one-byte escapes
        |                                    # or
        (?:"#, unicode_char_pattern!(), r")" // unicode escapes
    )};
}
lazy_static! {
    /// Regular expression for string escapes.
    pub static ref ESCAPE_REGEX: Regex = { Regex::new(escape_pattern!()).unwrap() };
}

/// Regex patten for a string literal.
pub const PTN_STRING: &str = concat!(
    r#"(?xs)
    "                               # opening quote
    (?P<s>                          # main string capture group start

    (?:                             # repeatable character (or escape sequence) group
    (?:"#,                          // escape sequence non-capturing group start
    escape_pattern!(),              // escape sequences
    r#")                            # escape sequence non-capturing group end
    |                               # or
    [^"\\]                          # anything but a backslash or double quote
    )*                              # any number of characters / escape sequences

    )                               # main string capture group end
    "                               # closing quote
    "#
);
lazy_static! {
    /// Regular expression for a string literal.
    pub static ref STRING_REGEX: Regex = { Regex::new(PTN_STRING).unwrap() };
}
lazy_static! {
    /// Regular expression for a front-anchored string literal.
    pub static ref STRING_REGEX_ANC: Regex = {
        let anchored_str = "^(?:".to_string() + PTN_STRING + ")";
        Regex::new(&anchored_str).unwrap()
    };
}

/// Interface method for `rustpeg` files to use the string regex.
pub fn match_str_ext(input: &str, pos: usize) -> Option<(usize, Captures)> {
    match STRING_REGEX_ANC.captures(&input[pos..]) {
        Some(caps) => Some((pos + caps.get(0).unwrap().as_str().len(), caps)),
        None => None
    }
}

/// Processes regular expression captures from `STRING_REGEX` or `STRING_REGEX_ANC` into `String`,
/// handling all escapes as necessary.
pub fn convert_string(captures: Captures) -> Result<String> {
    if let Some(string_match) = captures.name("s") {
        let s = string_match.as_str();

        // 'result' will store final string (with all escapes replaced)
        let mut result = String::with_capacity(s.len());
        // 'offset' keeps track of position in matched text
        let mut offset = 0;
        // loop over rest of captures
        // re-run escape regex on string to find each escape sequence
        for cap in ESCAPE_REGEX.captures_iter(s) {
            // captures_iter will only yield matches, so this unwrap is ok
            let mat = cap.get(0).unwrap();
            // add everything up until this match to the result
            result.push_str(&s[offset..mat.start()]);
            // process the escape and add it to the result
            result.push_str(escape(mat.as_str())?.as_str());
            // update our position
            offset = mat.end();
        }
        // add everything after last match (or after string_match.start() if no matches) to result
        result.push_str(&s[offset..]);
        Ok(result)
    } else {
        Err("failed to parse string")
    }
}

fn escape(code: &str) -> Result<String> {
    let code_len = code.len();
    if code_len < 2 {
        return Err("invalid empty escape");
    }
    match &code[..2] {
        "\\n"     => Ok("\n".to_string()),
        "\\r"     => Ok("\r".to_string()),
        "\\t"     => Ok("\t".to_string()),
        "\\\\"    => Ok("\\".to_string()),
        "\\0"     => Ok("\0".to_string()),
        "\\'"     => Ok("'".to_string()),
        "\\\""    => Ok("\"".to_string()),
        "\\x"     => Ok(two_digit_escape(&code[2..])?),
        "\\u"     => {
            if let Some(caps) = UNICODE_REGEX.captures(code) {
                if let Some(mat) = caps.get(1) {
                    return Ok(unicode_escape(mat.as_str())?);
                }
            }
            Err("improperly formatted unicode character code")
        },
        _         => Err("unknown escape sequence"),
    }
}

fn two_digit_escape(code: &str) -> Result<String> {
    // two-digit character code escapes
    if code.len() != 2 {
        return Err("two-digit character code expected");
    }
    let byte_code = u8::from_str_radix(code, 16).map_err(|_| "invalid two-digit escape")?;
    String::from_utf8(vec![byte_code]).map_err(|_| "invalid two-digit escape")
}

fn unicode_escape(code: &str) -> Result<String> {
    let unicode_value = u32::from_str_radix(code, 16).map_err(|_| "invalid unicode escape")?;
    let c = char::from_u32(unicode_value).ok_or("no character found for unicode value")?;
    let mut s = String::new();
    s.push(c);
    Ok(s)
}

