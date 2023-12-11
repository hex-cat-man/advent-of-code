use std::{error, io};

type Error = Box<dyn error::Error>;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Galaxy,
}

#[derive(Debug)]
struct Universe(Vec<Vec<Cell>>);

fn main() -> Result<(), Error> {
    let result = Universe::try_from_lines(io::stdin().lines().map_while(Result::ok))?
        .expand()
        .pairs()
        .into_iter()
        .map(distance)
        .sum::<usize>();

    eprintln!("{result}");

    Ok(())
}

fn distance(points: (Pos, Pos)) -> usize {
    let ((mut ax, mut ay), (bx, by)) = points;
    let mut distance = 0;

    while ax != bx {
        if ax < bx {
            ax += 1;
        } else {
            ax -= 1;
        }
        distance += 1;
    }

    while ay != by {
        if ay < by {
            ay += 1;
        } else {
            ay -= 1;
        }
        distance += 1;
    }

    distance
}

impl Universe {
    fn try_from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, Error> {
        Ok(Self(
            lines
                .map(|line| {
                    line.as_ref()
                        .chars()
                        .map(Cell::try_from)
                        .collect::<Result<Vec<Cell>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    fn expand(mut self) -> Self {
        let Universe(sky) = &mut self;

        let y_len = sky.len();
        let x_len = sky.first().map(Vec::len).unwrap_or_default();

        let mut empty_rows = vec![true; y_len];
        let mut empty_cols = vec![true; x_len];
        let mut cells = Vec::with_capacity(y_len * x_len);

        for (y, row) in sky.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                cells.push((x, y));
                if let Cell::Galaxy = cell {
                    empty_rows[y] = false;
                    empty_cols[x] = false;
                }
            }
        }

        let mut x_offset = 0;
        for (x, is_empty) in empty_cols.iter().enumerate() {
            if *is_empty {
                for row in sky.iter_mut() {
                    row.insert(x + x_offset, Cell::Empty);
                }

                x_offset += 1;
            }
        }

        let mut y_offset = 0;
        for (y, is_empty) in empty_rows.iter().enumerate() {
            if *is_empty {
                sky.insert(y + y_offset, vec![Cell::Empty; x_len + x_offset]);

                y_offset += 1;
            }
        }

        self
    }

    fn pairs(&self) -> Vec<(Pos, Pos)> {
        let Universe(sky) = self;

        let mut galaxies: Vec<Pos> = Vec::new();
        for (y, row) in sky.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Galaxy = cell {
                    galaxies.push((x, y));
                }
            }
        }

        // Reverse the order here and in the inner loop below (easier to debug).
        galaxies.reverse();

        let mut pairs = Vec::new();
        while let Some(a) = galaxies.pop() {
            for galaxy in galaxies.iter().rev() {
                pairs.push((a, *galaxy));
            }
        }

        pairs
    }

    fn print(&self) -> String {
        let Universe(sky) = self;

        let y_len = sky.len();
        let x_len = sky.first().map(Vec::len).unwrap_or_default();

        let mut s = String::with_capacity(y_len * x_len * y_len);

        for row in sky {
            for cell in row {
                s.push(match cell {
                    Cell::Empty => '.',
                    Cell::Galaxy => '#',
                })
            }

            s.push('\n');
        }

        s
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Cell::Empty,
            '#' => Cell::Galaxy,
            invalid => return Err(format!("'{invalid}': invalid cell char").into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let universe =
            Universe::try_from_lines(include_str!("../../examples/11.txt").lines()).unwrap();

        assert_eq!(
            374,
            universe
                .expand()
                .pairs()
                .into_iter()
                .map(distance)
                .sum::<usize>()
        );
    }
}
