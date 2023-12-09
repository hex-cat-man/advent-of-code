use std::{error, io, mem, str};

type Error = Box<dyn error::Error>;

#[derive(Debug, PartialEq)]
struct History(Vec<i32>);

fn main() -> Result<(), Error> {
    let result = io::stdin()
        .lines()
        .map_while(Result::ok)
        .map(|s| s.parse::<History>())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(History::extrapolate)
        .sum::<i32>();

    println!("{result}");

    Ok(())
}

impl History {
    fn extrapolate(&self) -> i32 {
        let mut values: Vec<i32> = Vec::new();
        let mut seq = self.0.clone();

        while !seq.iter().all(|n| *n == 0) {
            let seqx = mem::take(&mut seq);
            for i in 1..seqx.len() {
                seq.push(seqx[i] - seqx[i - 1]);
            }

            values.push(seqx.last().cloned().unwrap_or_default());
        }

        values.iter().sum()
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
    fn example() {
        let histories = include_str!("../example.txt")
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

        assert_eq!(histories.iter().map(History::extrapolate).sum::<i32>(), 114);
    }
}
