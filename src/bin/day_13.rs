use std::{error, io, str, usize};

type Error = Box<dyn error::Error>;

#[derive(Debug)]
struct Pattern {
    ground: Vec<Vec<char>>,
    flipped: bool,
}

enum Reflection {
    /// Reflection rows above the horizontal line of the reflection.
    Horizontal(usize),
    /// Reflection rows left the vertical line of the reflection.
    Vertical(usize),
}

fn main() -> Result<(), Error> {
    let mut patterns = parse_patterns(io::stdin().lines().map_while(Result::ok))?;

    let result = patterns
        .iter_mut()
        .map(Pattern::summarize)
        .sum::<Option<usize>>()
        .ok_or("did not find all reflections")?;

    eprintln!("{result}");

    Ok(())
}

fn parse_patterns(lines: impl Iterator<Item = impl Into<String>>) -> Result<Vec<Pattern>, Error> {
    let mut pattern: Option<String> = None;
    let mut patterns = Vec::new();

    for line in lines {
        let mut line: String = line.into();
        if line.is_empty() {
            if let Some(pattern) = pattern.take() {
                patterns.push(pattern.parse::<Pattern>()?);
            }
        } else if let Some(ref mut pattern) = pattern {
            line.insert(0, '\n');
            pattern.push_str(&line);
        } else {
            pattern = Some(line);
        }
    }

    if let Some(pattern) = pattern {
        patterns.push(pattern.parse::<Pattern>()?);
    }

    Ok(patterns)
}

impl Pattern {
    fn flip(&mut self) {
        let mut pattern = vec![
            self.ground.iter().map(|_| '.').collect::<Vec<_>>();
            self.ground.first().map(Vec::len).unwrap_or_default()
        ];

        for (y, row) in self.ground.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                pattern[x][y] = *cell;
            }
        }

        self.ground = pattern;
        self.flipped = !self.flipped;
    }

    /// The index of the inner reflected rows.
    fn reflection(&self) -> Option<Reflection> {
        let len = self.ground.len();

        'rows: for (i, row) in self.ground.iter().enumerate().skip(1) {
            let j = i - 1;
            if self.ground[j] != *row {
                continue;
            }

            let mut mj = j;
            let mut mi = i;

            loop {
                if mj == 0 || mi == (len - 1) {
                    break;
                }

                mj -= 1;
                mi += 1;

                if self.ground[mj] != self.ground[mi] {
                    continue 'rows;
                }
            }

            return Some(if self.flipped {
                Reflection::Vertical(j + 1)
            } else {
                Reflection::Horizontal(i)
            });
        }

        None
    }

    fn summarize(&mut self) -> Option<usize> {
        self.reflection()
            .or_else(|| {
                self.flip();
                self.reflection()
            })
            .map(|reflection| match reflection {
                Reflection::Horizontal(n) => n * 100,
                Reflection::Vertical(n) => n,
            })
    }
}

impl str::FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ground = s
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Self {
            ground,
            flipped: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut patterns = parse_patterns(include_str!("../../examples/13.txt").lines()).unwrap();
        assert_eq!(
            405,
            patterns
                .iter_mut()
                .map(Pattern::summarize)
                .sum::<Option<usize>>()
                .unwrap()
        );
    }
}
