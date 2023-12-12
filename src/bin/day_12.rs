use std::{error, io, str};

type Error = Box<dyn error::Error>;

#[derive(Debug, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

struct SpringGroup {
    record: Vec<Spring>,
    groups: Vec<u8>,
}

fn main() -> Result<(), Error> {
    let result = io::stdin()
        .lines()
        .map_while(Result::ok)
        .map(|s| s.parse::<SpringGroup>())
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter()
        .map(|s| s.arrangements())
        .sum::<usize>();

    println!("{result}");

    Ok(())
}

/// How delightfully devilish!
fn possibilites(
    springs: &[Spring],
    with: Option<Spring>,
    mut group: Option<u8>,
    mut groups: &[u8],
) -> usize {

    let next = if let Some(ref with) = with {
        Some((with, springs))
    } else {
        springs.split_first()
    };

    if let Some((spring, springs)) = next {
        match spring {
            Spring::Operational => {
                if group.is_some_and(|group| group > 0) {
                    // Undershot group: invalid arrangement.
                    return 0;
                }

                possibilites(springs, None, None, groups)
            }
            Spring::Damaged => {
                if let Some(n) = group {
                    if let Some(n) = n.checked_sub(1) {
                        group = Some(n);
                    } else {
                        // Overshot group: invalid arrangement.
                        return 0;
                    }
                } else if let Some(next) = groups.split_first() {
                    let n;
                    (n, groups) = next;
                    group = Some(*n - 1);
                } else {
                    // No further groups: invalid arrangement.
                    return 0;
                }

                possibilites(springs, None, group, groups)
            }
            Spring::Unknown => {
                possibilites(springs, Some(Spring::Operational), group, groups)
                    + possibilites(springs, Some(Spring::Damaged), group, groups)
            }
        }
    } else if groups.iter().sum::<u8>() + group.unwrap_or_default() > 0 {
        // Reached the end of springs, but not the end of all groups: invalid arrangement.
        0
    } else {
        // Reached the end of springs: valid arrangement.
        1
    }
}

impl SpringGroup {
    fn arrangements(&self) -> usize {
        possibilites(&self.record, None, None, &self.groups)
    }
}

impl str::FromStr for SpringGroup {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        let record = split
            .next()
            .ok_or("no spring state")?
            .chars()
            .map(Spring::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let groups = split
            .next()
            .ok_or("no groups")?
            .split(',')
            .map(u8::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SpringGroup { record, groups })
    }
}

impl TryFrom<char> for Spring {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            invalid => return Err(format!("'{invalid}': invalid spring kind").into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            21,
            include_str!("../../examples/12.txt")
                .lines()
                .map(SpringGroup::from_str)
                .map_while(Result::ok)
                .map(|s| s.arrangements())
                .sum::<usize>()
        );
    }
}
