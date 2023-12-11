use std::{error, io};

#[cfg(feature = "part1")]
const VOID: usize = 1;

#[cfg(not(feature = "part1"))]
const VOID: usize = 1_000_000;

type Error = Box<dyn error::Error>;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Cell {
    /// Empty space, equal to a million empty cells.
    Abyss,
    Empty,
    Galaxy,
}

#[derive(Debug)]
struct Universe(Vec<Vec<Cell>>);

fn main() -> Result<(), Error> {
    let result = Universe::try_from_lines(io::stdin().lines().map_while(Result::ok))?
        .expand()
        .pairs(VOID)
        .into_iter()
        .map(distance)
        .sum::<usize>();

    eprintln!("{result}");

    Ok(())
}

fn distance(points: (Pos, Pos)) -> usize {
    let mut distance = 0;

    let ((ax, ay), (bx, by)) = points;
    distance += if ax < bx { bx - ax } else { ax - bx };
    distance += if ay < by { by - ay } else { ay - by };

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
                    row.insert(x + x_offset, Cell::Abyss);
                }

                x_offset += 1;
            }
        }

        let mut y_offset = 0;
        for (y, is_empty) in empty_rows.iter().enumerate() {
            if *is_empty {
                sky.insert(y + y_offset, vec![Cell::Abyss; x_len + x_offset]);

                y_offset += 1;
            }
        }

        self
    }

    fn pairs(&self, void: usize) -> Vec<(Pos, Pos)> {
        let Universe(sky) = self;

        let mut galaxies: Vec<Pos> = Vec::new();
        for (y, row) in sky.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Galaxy = cell {
                    galaxies.push((x, y));
                }
            }
        }

        if void < 1 {
            panic!("void smaller than one");
        }

        // Invlate the coordinates by a multiple of abyss cells in their x/y path.
        if void > 1 {
            for galaxy in galaxies.iter_mut() {
                for row in sky.iter().take(galaxy.1) {
                    if let Some(Cell::Abyss) = row.first() {
                        galaxy.1 += void - 2;
                    }
                }

                if let Some(row) = sky.first() {
                    for col in row.iter().take(galaxy.0) {
                        if let Cell::Abyss = col {
                            galaxy.0 += void - 2;
                        }
                    }
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
                .pairs(1)
                .into_iter()
                .map(distance)
                .sum::<usize>()
        );
    }

    #[test]
    fn example_part_2() {
        let universe =
            Universe::try_from_lines(include_str!("../../examples/11.txt").lines()).unwrap();

        assert_eq!(
            1030,
            universe
                .expand()
                .pairs(10)
                .into_iter()
                .map(distance)
                .sum::<usize>()
        );
    }
}
