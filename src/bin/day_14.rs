use std::{error, fmt, io, str};

type Error = Box<dyn error::Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
    Empty,
}

#[derive(Debug)]
struct Pattern {
    rocks: Vec<Vec<Rock>>,
}

fn main() -> Result<(), Error> {
    let mut pattern: Pattern = io::stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n")
        .parse()?;

    pattern.tilt_north();

    let result = pattern.total_load();

    println!("{result}");

    Ok(())
}

impl Pattern {
    fn tilt_north(&mut self) {
        for (y, row) in self.rocks.clone().iter().enumerate() {
            for (x, rock) in row.iter().enumerate() {
                if let Rock::Round = rock {
                    let mut offset = 1;
                    while let Some(Rock::Empty) = y
                        .checked_sub(offset)
                        .and_then(|y| self.rocks.get(y))
                        .and_then(|row| row.get(x))
                    {
                        self.rocks[y - offset + 1][x] = Rock::Empty;
                        self.rocks[y - offset][x] = Rock::Round;
                        offset += 1;
                    }
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        let len = self.rocks.len();
        self.rocks
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .map(|rock| match rock {
                        Rock::Round => len - i,
                        Rock::Cube | Rock::Empty => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl str::FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks = s
            .lines()
            .map(|l| l.chars().map(Rock::try_from).collect::<Result<Vec<_>, _>>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { rocks })
    }
}

impl TryFrom<char> for Rock {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Rock::Empty,
            '#' => Rock::Cube,
            'O' => Rock::Round,
            invalid => return Err(format!("'{invalid}': invalid rock").into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut pattern: Pattern = include_str!("../../examples/14.txt")
            .lines()
            .collect::<Vec<&str>>()
            .join("\n")
            .parse()
            .unwrap();

        pattern.tilt_north();

        assert_eq!(136, pattern.total_load());
    }
}
