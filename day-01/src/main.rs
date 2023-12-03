use std::{
    error,
    io::{self, BufRead},
};

/// Damn elfs.
const ELVISH: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

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

    let s = s.as_ref();

    for (i, mut c) in s.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            let elvish = ELVISH
                .iter()
                .find(|(num_str, _)| s[..=i].ends_with(*num_str))
                .map(|(_, cr)| cr);

            if let Some(replace_char) = elvish {
                c = *replace_char;
            }
        }

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

    #[test]
    fn test_parse_calibration_value_part_two() {
        for (s, n) in [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            // Ahhhhhhhhhh!
            ("2fiveshtds4oneightsjg", 28),
        ] {
            assert_eq!(parse_calibration_value(s).unwrap(), n)
        }
    }
}
