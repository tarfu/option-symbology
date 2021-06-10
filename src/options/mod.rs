use fancy_regex::Regex;

use strum_macros::{Display, EnumString};

use std::{fmt, str::FromStr};

const OCC_OSI_REGEX: &str = r"^(?=.{16,21}$)(?P<symbol>[\w]{1,6})\s{0,5}(?P<year>\d{2})(?P<month>0\d|1[0-2])(?P<day>0[1-9]|[12]\d|3[01])(?P<contract>C|P|c|p)(?P<price>\d{8})$";
const IB_ACTIVITY_STATEMENT_TRADES: &str = r"^(?P<symbol>[\w]{1,6})\s(?P<day>0[1-9]|[12]\d|3[01])(?P<month>\w{3})(?P<year>\d{2})\s(?P<price>\d*[.]?\d+)\s(?P<contract>C|P|c|p)$"; //KO 28MAY21 32.01 C

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
enum Month3Letter {
    JAN = 1,
    FEB,
    MAR,
    APR,
    MAY,
    JUN,
    JUL,
    AUG,
    SEP,
    OCT,
    NOV,
    DEC,
}

/// Struct representing a complete option contract
#[derive(Debug, PartialEq)]
pub struct OptionData {
    /// ticker symbol
    pub symbol: String,
    /// 4 digit year -> e.g. 2021
    expiration_year: i32,
    /// expiration month 1->12
    expiration_month: i32,
    /// expiration day  1->31
    expiration_day: i32,
    pub strike_price: f64,
    pub contract_type: ContractType,
}

/// Enum if it is a Call or a Put
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

/// Error type which wraps [fancy_regex::Error]
#[derive(Debug, PartialEq)]
pub enum Error {
    NoResult,
    YearOutOfRange,
    MonthOutOfRange,
    DayOutOfRange,
    ChecksumError,
    RegexError(String),
}

impl ::std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We should make these more helpful, e.g. by including the parts of the regex that lead to
        // the error.
        match self {
            Error::NoResult => write!(f, "No Result for parsing String"),
            Error::RegexError(e) => write!(f, "RegexError: {}", e),
            Error::YearOutOfRange => write!(f, "Supplied year is out of range and not >2000"),
            Error::MonthOutOfRange => write!(
                f,
                "Supplied month is out of range and not between 1 and 12 "
            ),
            Error::DayOutOfRange => {
                write!(f, "Supplied Year is out of range and not between 1 and 31")
            }
            Error::ChecksumError => {
                write!(f, "Checksum could not be verified")
            }
        }
    }
}

impl OptionData {
    ///parse a string which is OSI compliant to [OptionData]
    pub fn parse_osi(osi: &str) -> Result<OptionData, Error> {
        let re = Regex::new(OCC_OSI_REGEX);
        let re = match re {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e.to_string())),
        };

        let result = re.captures(osi);
        let result = match result {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e.to_string())),
        };
        if result.is_none() {
            return Err(Error::NoResult);
        }
        let cap = result.unwrap();

        Ok(OptionData {
            expiration_year: 2000 + cap.name("year").unwrap().as_str().parse::<i32>().unwrap(),
            expiration_month: cap.name("month").unwrap().as_str().parse().unwrap(),
            expiration_day: cap.name("day").unwrap().as_str().parse().unwrap(),

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

    pub fn parse_ib_activity_statement_trades_symbol(osi: &str) -> Result<OptionData, Error> {
        let re = Regex::new(IB_ACTIVITY_STATEMENT_TRADES);
        let re = match re {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e.to_string())),
        };

        let result = re.captures(osi);
        let result = match result {
            Ok(r) => r,
            Err(e) => return Err(Error::RegexError(e.to_string())),
        };
        if result.is_none() {
            return Err(Error::NoResult);
        }
        let cap = result.unwrap();

        Ok(OptionData {
            expiration_year: 2000 + cap.name("year").unwrap().as_str().parse::<i32>().unwrap(),
            expiration_month: Month3Letter::from_str(cap.name("month").unwrap().as_str()).unwrap()
                as i32,
            expiration_day: cap.name("day").unwrap().as_str().parse().unwrap(),

            symbol: cap.name("symbol").unwrap().as_str().parse().unwrap(),
            contract_type: match cap.name("contract").unwrap().as_str() {
                "P" | "p" => ContractType::Put,
                "C" | "c" => ContractType::Call,
                _ => panic!(),
            },
            strike_price: cap.name("price").unwrap().as_str().parse::<f64>().unwrap(),
        })
    }

    /// serializes [OptionData] to a OSI compliant string like described here [https://ibkr.info/node/972]
    pub fn to_osi_string(&self) -> String {
        format!(
            "{symbol:<6}{year:0>2}{month:0>2}{day:0>2}{contract}{price:0>8}",
            symbol = self.symbol,
            day = self.expiration_day,
            month = self.expiration_month,
            year = self.expiration_year - 2000,
            contract = self.contract_type,
            price = self.strike_price * 1000 as f64
        )
        .to_string()
    }

    /// serializes [OptionData] to a OSI compliant string like described here [https://ibkr.info/node/972] but without padding of the symbol to 6 chars
    pub fn to_osi_string_no_symbol_padding(&self) -> String {
        format!(
            "{symbol}{year:0>2}{month:0>2}{day:0>2}{contract}{price:0>8}",
            symbol = self.symbol,
            day = self.expiration_day,
            month = self.expiration_month,
            year = self.expiration_year - 2000,
            contract = self.contract_type,
            price = self.strike_price * 1000 as f64
        )
        .to_string()
    }

    /// serializes [OptionData] to a Schwab compliant string like described here [http://www.schwabcontent.com/symbology/int_eng/key_details.html]
    pub fn to_schwab_string(&self) -> String {
        format!(
            "{symbol} {month:0>2}/{day:0>2}/{year:0>4} {price:.2} {contract}",
            symbol = self.symbol,
            day = self.expiration_day,
            month = self.expiration_month,
            year = self.expiration_year,
            contract = self.contract_type,
            price = self.strike_price as f64
        )
        .to_string()
    }

    pub fn get_expiration_year(&self) -> i32 {
        self.expiration_year
    }

    pub fn get_expiration_month(&self) -> i32 {
        self.expiration_month
    }

    pub fn get_expiration_day(&self) -> i32 {
        self.expiration_day
    }

    pub fn set_ymd(&self, year: i32, month: i32, day: i32) -> Result<(), Error> {
        if !year > 2000 {
            return Err(Error::YearOutOfRange);
        }
        if !(month >= 1 && month <= 12) {
            return Err(Error::MonthOutOfRange);
        }
        if !is_day_in_month_and_year(year, month, day) {
            return Err(Error::DayOutOfRange);
        }
        Ok(())
    }
}

/// leap year is every 4 years but not every 100 still every 400
fn is_leap_year(year: i32) -> bool {
    return (year % 4 == 0 && !(year % 100 == 0)) || year % 400 == 0;
}

const MONTH_WITH_31_DAYS: [i32; 7] = [1, 3, 5, 7, 8, 10, 12];
/// checks if the day of month fits the month and year
fn is_day_in_month_and_year(year: i32, month: i32, day: i32) -> bool {
    return day > 0
        && ((month == 2 && (day <= 28 || day == 29 && is_leap_year(year)))
            || (month != 2 && (day <= 30 || day == 31 && MONTH_WITH_31_DAYS.contains(&month))));
}

#[cfg(test)]
mod tests;
