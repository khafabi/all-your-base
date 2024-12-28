use crate::Error::InvalidOutputBase;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(InvalidOutputBase);
    }

    if let Some(&invalid_digit) = number.iter().find(|&&digit| digit >= from_base) {
        return Err(Error::InvalidDigit(invalid_digit));
    }

    if number.is_empty() {
        return Ok(vec![0]);
    }

    let decimal_value = to_decimal(number, from_base);

    let result = from_decimal(decimal_value, to_base);

    Ok(result)
}

fn to_decimal(number: &[u32], base: u32) -> u32 {
    number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, &digit)| acc + digit * base.pow(idx as u32))
}

fn from_decimal(decimal: u32, base: u32) -> Vec<u32> {
    fn helper(decimal: u32, base: u32, acc: Vec<u32>) -> Vec<u32> {
        match decimal {
            0 => acc,
            _ => helper(decimal / base, base, [&[decimal % base], &acc[..]].concat()),
        }
    }

    let result = helper(decimal, base, vec![]);
    match result.is_empty() {
        true => vec![0],
        false => result,
    }
}
