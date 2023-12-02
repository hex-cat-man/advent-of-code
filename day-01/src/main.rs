use std::{
    error,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut sum = 0;

    // Get each line of the "calibration document" from stdin.
    for line in io::stdin().lock().lines() {
        sum += parse_calibration_value(line?)?;
    }

    println!("{sum}");

    Ok(())
}

/// Get the two digit integer from the first and last digit in the string.
fn parse_calibration_value(s: impl AsRef<str>) -> io::Result<usize> {
    let mut first = None;
    let mut last = None;

    for c in s.as_ref().chars() {
        if c.is_ascii_digit() {
            if first.is_none() {
                first = Some(c);
            }

            last = Some(c);
        }
    }

    match (first, last) {
        (Some(first), Some(last)) => [first, last]
            .into_iter()
            .collect::<String>()
            .parse()
            .map_err(io::Error::other),
        _ => Err(io::Error::other("string does not contain ascii digits")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_calibration_value() {
        for (s, n) in [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ] {
            assert_eq!(parse_calibration_value(s).unwrap(), n)
        }
    }
}
