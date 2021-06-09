use fancy_regex::Regex;

use crate::options::Error;



const ISIN_REGEX: &str = r"^(?P<country>[A-Z]{2})(?P<identifier>[A-Z0-9]{9})(?P<checksum>[0-9]{1})$"; 

pub struct ISIN {
    pub county_code: String,
    pub identifier: String,
    pub checksum: i32
}

impl ISIN {
    pub fn parse_isin(isin: &str) -> Result<ISIN, Error> {
        
        let re = Regex::new(ISIN_REGEX);
        let re = match re {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e)),
        };

        let result = re.captures(isin);
        let result = match result {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e)),
        };
        if result.is_none() {
            return Err(Error::NoResult);
        }

        let cap = result.unwrap();

        // TODO: verify checksum
        Ok(ISIN {
            county_code: cap.name("country").unwrap().as_str().parse().unwrap(),
            identifier: cap.name("identifier").unwrap().as_str().parse().unwrap(),
            checksum: cap.name("year").unwrap().as_str().parse::<i32>().unwrap() as i32
        })

    }
}