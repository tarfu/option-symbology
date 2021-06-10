use fancy_regex::Regex;

use crate::options::Error;

const ISIN_REGEX: &str =
    r"^(?P<country>[A-Z]{2})(?P<identifier>[A-Z0-9]{9})(?P<checksum>[0-9]{1})$";

#[derive(Debug, PartialEq)]
pub struct ISIN {
    isin: String,
}

impl ISIN {
    pub fn parse_isin(isin: &str) -> Result<ISIN, Error> {
        let re = Regex::new(ISIN_REGEX);
        let re = match re {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e.to_string())),
        };

        let result = re.captures(isin);
        let result = match result {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e.to_string())),
        };
        if result.is_none() {
            return Err(Error::NoResult);
        }

        if verify_isin(isin) {
            Ok(ISIN {
                isin: isin.to_string(),
            })
        } else {
            Err(Error::ChecksumError)
        }
    }

    pub fn get_county_code(&self) -> &str {
        &self.isin[0..2]
    }

    pub fn get_identifier(&self) -> &str {
        &self.isin[2..11]
    }

    pub fn get_checksum(&self) -> &str {
        &self.isin[11..]
    }

    pub fn get_isin(&self) -> &str {
        &self.isin
    }
}

fn verify_isin(isin: &str) -> bool {
    let last_char = isin.as_bytes().last().copied().unwrap();
    let checksum_char = compute_checksum(isin) + b'0';

    last_char == checksum_char
}

/// As described on:
/// https://en.wikipedia.org/wiki/International_Securities_Identification_Number
/// https://en.wikipedia.org/wiki/Luhn_algorithm
fn compute_checksum(isin: &str) -> u8 {
    let digits = replace_chars_to_numbers(isin);
    let sum_odd: i32 = digits
        .iter()
        .rev()
        .skip(1)
        .step_by(2)
        .map(|f| (*f as i32) * 2)
        .flat_map(|f| if f > 9 { vec![f / 10, f % 10] } else { vec![f] })
        .sum();
    let sum_even: i32 = digits
        .iter()
        .rev()
        .skip(2)
        .step_by(2)
        .map(|f| (*f as i32))
        .sum();

    let checksum = sum_even + sum_odd;

    ((10 - (checksum % 10)) % 10) as u8
}

fn replace_chars_to_numbers(isin: &str) -> Vec<u8> {
    isin.as_bytes()
        .into_iter()
        .flat_map(|f| convert_char_as_byte_to_numbers(f))
        .collect::<Vec<u8>>()
}

fn convert_char_as_byte_to_numbers(c: &u8) -> Vec<u8> {
    match c {
        b'0'..=b'9' => vec![c - b'0'],
        b'A'..=b'Z' => {
            let index = c - b'A' + 10;
            vec![index / 10, index % 10]
        }
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_some_good_isins() {
        assert!(verify_isin("US0378331005")); // Apple
        assert!(verify_isin("US5949181045")); // Microsoft
        assert!(verify_isin("US38259P5089")); // Google
    }

    #[test]
    fn fail_some_bad_isins() {
        assert!(!verify_isin("US5949181040")); // Microsoft (checksum zeroed)
        assert!(!verify_isin("US38259P5080")); // Google (checksum zeroed)
        assert!(!verify_isin("US0378331000")); // Apple (checksum zeroed)

        assert!(!verify_isin("SU5941981045")); // Microsoft (two chars transposed)
        assert!(!verify_isin("US3825P95089")); // Google (two chars transposed)
        assert!(!verify_isin("US0378313005")); // Apple (two chars transposed)
    }

    #[test]
    fn parse_isin() {
        let parsed = ISIN::parse_isin("US0378331005").unwrap();
        assert_eq!("US", parsed.get_county_code());
        assert_eq!("037833100", parsed.get_identifier());
        assert_eq!("5", parsed.get_checksum());
    }

    #[test]
    fn parse_isin_errors() {
        assert_eq!(ISIN::parse_isin("US0378331000"), Err(Error::ChecksumError)); // checksum wrong
        assert_eq!(ISIN::parse_isin("US037833100"), Err(Error::NoResult)) // no checksum
    }
}
