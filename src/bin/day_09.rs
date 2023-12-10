use std::{error, io, mem, str};

type Error = Box<dyn error::Error>;

#[derive(Debug, PartialEq)]
struct History(Vec<i32>);

fn main() -> Result<(), Error> {
    #[cfg(feature = "part1")]
    let func = History::extrapolate_next;
    #[cfg(not(feature = "part1"))]
    let func = History::extrapolate_prev;

    let result = io::stdin()
        .lines()
        .map_while(Result::ok)
        .map(|s| s.parse::<History>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(func)
        .sum::<i32>();

    println!("{result}");

    Ok(())
}

impl History {
    fn extrapolate(self, next: bool) -> i32 {
        let mut values: Vec<i32> = Vec::new();
        let mut seq = self.0;

        while !seq.iter().all(|n| *n == 0) {
            let seqx = mem::take(&mut seq);
            for i in 1..seqx.len() {
                seq.push(seqx[i] - seqx[i - 1]);
            }

            if next {
                values.push(seqx.last().cloned().unwrap_or_default());
            } else {
                values.push(seqx.first().cloned().unwrap_or_default());
            }
        }

        if next {
            values.into_iter().sum()
        } else {
            values.into_iter().rev().fold(0, |n, l| l - n)
        }
    }

    #[cfg(any(test, feature = "part1"))]
    fn extrapolate_next(self) -> i32 {
        self.extrapolate(true)
    }

    #[cfg(any(test, not(feature = "part1")))]
    fn extrapolate_prev(self) -> i32 {
        self.extrapolate(false)
    }
}

impl str::FromStr for History {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let history = s
            .split_whitespace()
            .map(i32::from_str)
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(History(history))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_parse() {
        let histories = include_str!("../../examples/09.txt")
            .lines()
            .map(|s| s.parse::<History>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(
            histories,
            vec![
                History(vec![0, 3, 6, 9, 12, 15]),
                History(vec![1, 3, 6, 10, 15, 21]),
                History(vec![10, 13, 16, 21, 30, 45]),
            ]
        );
    }

    #[test]
    fn example_part1() {
        let result = include_str!("../../examples/09.txt")
            .lines()
            .map(|s| s.parse::<History>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(History::extrapolate_next)
            .sum::<i32>();

        assert_eq!(result, 114);
    }

    #[test]
    fn example_part2() {
        let result = include_str!("../../examples/09.txt")
            .lines()
            .map(|s| s.parse::<History>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(History::extrapolate_prev)
            .sum::<i32>();

        assert_eq!(result, 2);
    }
}
