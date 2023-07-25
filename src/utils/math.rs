use bigdecimal::ParseBigDecimalError;
use pad::PadStr;
use std::ops::Div;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    ParseBigDecimal(#[from] bigdecimal::ParseBigDecimalError),
}
#[allow(dead_code)]
pub fn safe_div(amount0: &BigDecimal, amount1: &BigDecimal) -> BigDecimal {
    let big_decimal_zero: &BigDecimal = &BigDecimal::zero();
    if amount1.eq(big_decimal_zero) {
        BigDecimal::zero()
    } else {
        amount0.clone().div(amount1.clone())
    }
}

pub fn divide_by_decimals(big_float_amount: BigDecimal, decimals: u64) -> BigDecimal {
    let bd = BigDecimal::from_str(
        "1".pad_to_width_with_char((decimals + 1) as usize, '0')
            .as_str(),
    )
    .unwrap()
    .with_prec(10);

    big_float_amount.div(bd).with_prec(10)
}

pub fn to_big_decimal(price_str: &str) -> Result<BigDecimal, ParseBigDecimalError> {
    let val = BigDecimal::from_str(price_str);
    if val.is_ok() {
        let new_val = divide_by_decimals(val.unwrap(), 18);
        Ok(new_val)
    } else {
        val
    }
}
