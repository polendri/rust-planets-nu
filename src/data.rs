/*!
Contains common data structures used by the library.
*/

use std::char;
use std::fmt;

use error;

// Public

/// Represents a 3-byte RGB colour value.
#[deriving(Eq, PartialEq)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Show for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

/// Converts a string of the form "#XXXXXX" (where X is a hex digit) to an RGB object.
///
/// Also supports some text strings like "green" or "blue".
pub fn to_rgb(rgb_str: &str) -> Result<RGB, error::Error> {
    match rgb_str {
        "red"     => return Ok(RGB { red: 0xffu8, green: 0x00u8, blue: 0x00u8 }),
        "orange"  => return Ok(RGB { red: 0xffu8, green: 0x80u8, blue: 0x00u8 }),
        "yellow"  => return Ok(RGB { red: 0xffu8, green: 0xffu8, blue: 0x00u8 }),
        "green"   => return Ok(RGB { red: 0x00u8, green: 0xffu8, blue: 0x00u8 }),
        "cyan"    => return Ok(RGB { red: 0x00u8, green: 0xffu8, blue: 0xffu8 }),
        "blue"    => return Ok(RGB { red: 0x00u8, green: 0x00u8, blue: 0xffu8 }),
        "purple"  => return Ok(RGB { red: 0x80u8, green: 0x00u8, blue: 0xffu8 }),
        "magenta" => return Ok(RGB { red: 0xffu8, green: 0x00u8, blue: 0xffu8 }),
        "pink"    => return Ok(RGB { red: 0xffu8, green: 0x00u8, blue: 0x80u8 }),
        _         => (),
    }

    let str_to_err = |string| { error::Error::new(error::LibError, string) };
    let short_err = str_to_err(format!("RGB string is too short. ({})", rgb_str));
    let long_err = str_to_err("RGB string is too long.".to_string());
    let digit_err = str_to_err("Encountered a non-hex digit.".to_string());
    let len = rgb_str.len();
    if len < 7 {
        return Err(short_err);
    } else if len > 7 {
        return Err(long_err);
    }
    let mut iter = rgb_str.chars();
    match iter.next().unwrap() {
        '#' => (),
        _ => return Err(str_to_err("Could not find leading '#'.".to_string())),
    };
    let r = (expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8 << 4) +
        expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8;
    let g = (expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8 << 4) +
        expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8;
    let b = (expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8 << 4) +
        expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8;
    Ok(RGB { red: r, green: g, blue: b })
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use error;

    #[test]
    fn test_to_rgb_errors() {
        assert_eq!(error::LibError, to_rgb("#af320").unwrap_err().kind);   // too short
        assert_eq!(error::LibError, to_rgb("#af32021").unwrap_err().kind); // too long
        assert_eq!(error::LibError, to_rgb("#ag03a3").unwrap_err().kind);  // non-hex digit
        assert_eq!(error::LibError, to_rgb("1234567").unwrap_err().kind);  // no leading '#'
    }

    #[test]
    fn test_to_rgb() {
        assert_eq!(RGB { red: 175u8, green: 50u8, blue: 8u8 }, to_rgb("#af3208").unwrap());
    }

    #[test]
    fn test_to_rgb_show() {
        assert_eq!("#af3208", format!("{}", RGB { red: 175u8, green: 50u8, blue: 8u8 }).as_slice());
    }
}
