#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase)
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase)
    }
    let mut output = vec![];
    let mut has_error: (bool, u32) = (false, 0);
    let mut sum = number.iter().fold(0, |acc, x| {
        if *x>=from_base {
            has_error=(true, *x);
        }
        acc * from_base + x
    });

    if has_error.0 {
        return Err(Error::InvalidDigit(has_error.1))
    }

    while sum > 0 {
        output.insert(0, sum%to_base);
        sum /= to_base;
    }

    if output.is_empty() {
        return Ok(vec![0])
    }

    Ok(output)
}
