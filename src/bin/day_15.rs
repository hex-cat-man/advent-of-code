use std::{error, io, str};

type Error = Box<dyn error::Error>;

fn main() -> Result<(), Error> {
    for line in io::stdin().lines() {
        let line = line?;
        println!("{}", init_seq_sum(line));
    }

    Ok(())
}

fn init_seq_sum(s: impl AsRef<str>) -> usize {
    let mut sum = 0;

    for step in s.as_ref().replace('\n', "").split(',') {
        sum += hash(step) as usize;
    }

    sum
}

fn hash(s: impl AsRef<str>) -> u8 {
    let mut val = 0;

    for c in s.as_ref().chars() {
        val += c as u16;
        val *= 17;
        val %= 256;
    }

    val as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_hash() {
        assert_eq!(52, hash("HASH"));
    }

    #[test]
    fn example_sequence() {
        assert_eq!(
            1320,
            init_seq_sum(include_str!("../../examples/15.txt"))
        );
    }
}
