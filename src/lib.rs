use chrono::NaiveDate;
use fancy_regex::Regex;
use std::fmt;

const OCC_OSI_REGEX: &str = r"^(?=.{16,21}$)(?P<symbol>[\w]{1,6})\s{0,5}(?P<year>\d{2})(?P<month>0\d|1[0-2])(?P<day>0[1-9]|[12]\d|3[01])(?P<contract>C|P|c|p)(?P<price>\d{8})$";

#[derive(Debug, PartialEq)]
pub struct OptionData {
    pub symbol: String,
    pub expiration_date: NaiveDate,
    pub strike_price: f64,
    pub contract_type: ContractType,
}

// impl std::cmp::PartialEq for OptionData {
//     fn eq(&self, other: &Self) -> bool {
//         return self.symbol == other.symbol && self.expiration_date == other.expiration_date && self.strike_price == other.strike_price && self.contract_type == other.contract_type;
//     }
// }

#[derive(Debug, PartialEq)]
pub enum ContractType {
    Call,
    Put,
}

impl fmt::Display for ContractType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContractType::Call => write!(f, "C"),
            ContractType::Put => write!(f, "P"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    NoResult,
    RegexError(fancy_regex::Error),
}

impl ::std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We should make these more helpful, e.g. by including the parts of the regex that lead to
        // the error.
        match self {
            Error::NoResult => write!(f, "No Result for parsing String"),
            Error::RegexError(e) => write!(f, "RegexError: {}", e),
        }
    }
}

impl OptionData {
    pub fn parse_osi(osi: &str) -> Result<OptionData, Error> {
        let re = Regex::new(OCC_OSI_REGEX);
        let re = match re {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e)),
        };

        let result = re.captures(osi);
        let result = match result {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e)),
        };
        if result.is_none() {
            return Err(Error::NoResult);
        }
        let cap = result.unwrap();

        Ok(OptionData {
            expiration_date: NaiveDate::from_ymd(
                2000 + cap.name("year").unwrap().as_str().parse::<i32>().unwrap(),
                cap.name("month").unwrap().as_str().parse().unwrap(),
                cap.name("day").unwrap().as_str().parse().unwrap(),
            ),
            symbol: cap.name("symbol").unwrap().as_str().parse().unwrap(),
            contract_type: match cap.name("contract").unwrap().as_str() {
                "P" | "p" => ContractType::Put,
                "C" | "c" => ContractType::Call,
                _ => panic!(),
            },
            strike_price: cap.name("price").unwrap().as_str().parse::<i32>().unwrap() as f64
                / (1000 as f64),
        })
    }

    pub fn to_osi_string(&self) -> String {
        format!("{symbol:<6}{date}{contract}{price:0>8}", symbol=self.symbol, date=self.expiration_date.format("%y%m%d"), contract=self.contract_type, price=self.strike_price*1000 as f64).to_string()
    }

    pub fn to_schwab_string(&self) -> String {
        format!("{symbol} {date} {price:.2} {contract}", symbol=self.symbol, date=self.expiration_date.format("%m/%d/%Y"), contract=self.contract_type, price=self.strike_price as f64).to_string()
    }

}

#[cfg(test)]
mod tests {
    use crate::{ContractType, OptionData};
    use chrono::NaiveDate;

    #[test]
    fn osi_well_formated() {
        let apple_01nov13_call_470 = OptionData {
            strike_price: 470 as f64,
            contract_type: ContractType::Call,
            symbol: "AAPL".to_string(),
            expiration_date: NaiveDate::from_ymd(2013, 11, 01),
        };

        assert_eq!(
            OptionData::parse_osi("AAPL  131101C00470000").unwrap(),
            apple_01nov13_call_470
        );
    }

    #[test]
    fn osi_symbol_padding_wrong() {
        let apple_01nov13_call_470 = OptionData {
            strike_price: 470 as f64,
            contract_type: ContractType::Call,
            symbol: "AAPL".to_string(),
            expiration_date: NaiveDate::from_ymd(2013, 11, 01),
        };

        assert_eq!(
            OptionData::parse_osi("AAPL 131101C00470000").unwrap(),
            apple_01nov13_call_470
        );
        assert_eq!(
            OptionData::parse_osi("AAPL131101C00470000").unwrap(),
            apple_01nov13_call_470
        );
    }

    #[test]
    fn osi_contract_type_small() {
        let apple_01nov13_call_470 = OptionData {
            strike_price: 470 as f64,
            contract_type: ContractType::Call,
            symbol: "AAPL".to_string(),
            expiration_date: NaiveDate::from_ymd(2013, 11, 01),
        };

        assert_eq!(
            OptionData::parse_osi("AAPL  131101c00470000").unwrap(),
            apple_01nov13_call_470
        );
    }

    #[test]
    fn osi_formatting(){
        let apple_01nov13_call_470 = OptionData {
            strike_price: 470 as f64,
            contract_type: ContractType::Call,
            symbol: "AAPL".to_string(),
            expiration_date: NaiveDate::from_ymd(2013, 11, 01),
        };

        assert_eq!(
            "AAPL  131101C00470000",
            apple_01nov13_call_470.to_osi_string()
        );
    }

    #[test]
    fn schwab_formatting(){
        let apple_01nov13_call_470 = OptionData {
            strike_price: 470 as f64,
            contract_type: ContractType::Call,
            symbol: "AAPL".to_string(),
            expiration_date: NaiveDate::from_ymd(2013, 11, 01),
        };

        assert_eq!(
            "AAPL 11/01/2013 470.00 C",
            apple_01nov13_call_470.to_schwab_string()
        );
    }
}
