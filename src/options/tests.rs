use crate::options::{is_day_in_month_and_year, is_leap_year, ContractType, OptionData};

#[test]
fn osi_well_formated() {
    let apple_01nov13_call_470 = OptionData {
        strike_price: 470 as f64,
        contract_type: ContractType::Call,
        symbol: "AAPL".to_string(),
        expiration_year: 2013,
        expiration_month: 11,
        expiration_day: 1,
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
        expiration_year: 2013,
        expiration_month: 11,
        expiration_day: 1,
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
        expiration_year: 2013,
        expiration_month: 11,
        expiration_day: 1,
    };

    assert_eq!(
        OptionData::parse_osi("AAPL  131101c00470000").unwrap(),
        apple_01nov13_call_470
    );
}

#[test]
fn osi_formatting() {
    let apple_01nov13_call_470 = OptionData {
        strike_price: 470 as f64,
        contract_type: ContractType::Call,
        symbol: "AAPL".to_string(),
        expiration_year: 2013,
        expiration_month: 11,
        expiration_day: 1,
    };

    assert_eq!(
        "AAPL  131101C00470000",
        apple_01nov13_call_470.to_osi_string()
    );
}

#[test]
fn parse_ib_activity_statement_trades_symbol(){
    let apple_01nov13_call_470 = OptionData {
        strike_price: 470 as f64,
        contract_type: ContractType::Call,
        symbol: "AAPL".to_string(),
        expiration_year: 2013,
        expiration_month: 11,
        expiration_day: 1,
    };

    let apple_01nov13_call_470_parsed = OptionData::parse_ib_activity_statement_trades_symbol("AAPL 01NOV13 470.0 C");

    assert_eq!(
        apple_01nov13_call_470,
        apple_01nov13_call_470_parsed.unwrap()
    );
}

#[test]
fn schwab_formatting() {
    let apple_01nov13_call_470 = OptionData {
        strike_price: 470 as f64,
        contract_type: ContractType::Call,
        symbol: "AAPL".to_string(),
        expiration_year: 2013,
        expiration_month: 11,
        expiration_day: 1,
    };

    assert_eq!(
        "AAPL 11/01/2013 470.00 C",
        apple_01nov13_call_470.to_schwab_string()
    );
}

#[test]
fn test_is_leap_year() {
    assert_eq!(true, is_leap_year(2000));
    assert_eq!(true, is_leap_year(2004));
    assert_eq!(false, is_leap_year(2100));
    assert_eq!(false, is_leap_year(2021));
}
#[test]
fn test_day_in_month() {
    assert_eq!(true, is_day_in_month_and_year(2000, 2, 29));
    assert_eq!(false, is_day_in_month_and_year(2001, 2, 29));
    assert_eq!(false, is_day_in_month_and_year(2000, 2, 30));
    assert_eq!(false, is_day_in_month_and_year(2000, 4, 31));
    assert_eq!(true, is_day_in_month_and_year(2001, 8, 31));
}