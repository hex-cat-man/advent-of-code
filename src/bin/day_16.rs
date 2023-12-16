use std::{error, io};

type Error = Box<dyn error::Error>;

type Pos = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Empty,
    MirrorLeftRight,
    MirrorRightLeft,
    SplitterHorizontal,
    SplitterVertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    kind: TileKind,
    energized: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Contraption {
    grid: Vec<Vec<Tile>>,
    visited: Vec<(Direction, Pos)>,
}

fn main() -> Result<(), Error> {
    let mut tiles = Contraption::try_from_lines(io::stdin().lines().map_while(Result::ok))?;

    tiles.illuminate((0, 0), Direction::Right);

    println!("{}", tiles.energized_count());

    Ok(())
}

impl Direction {
    fn next(&self, pos: Pos) -> Pos {
        let (x, y) = pos;
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}

impl Contraption {
    fn try_from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, Error> {
        Ok(Self {
            grid: lines
                .map(|line| {
                    line.as_ref()
                        .chars()
                        .map(Tile::try_from)
                        .collect::<Result<Vec<Tile>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
            visited: vec![],
        })
    }

    fn illuminate(&mut self, pos: Pos, direction: Direction) {
        let (x, y) = pos;

        if x < 0 || y < 0 {
            return;
        }

        if self
            .visited
            .iter()
            .any(|(vdir, vpos)| vdir == &direction && vpos == &pos)
        {
            return;
        }

        self.visited.push((direction, pos));

        if let Some(tile) = self
            .grid
            .get_mut(y as usize)
            .and_then(|row| row.get_mut(x as usize))
        {
            tile.energized += 1;
            match tile.kind {
                TileKind::Empty => self.illuminate(direction.next((x, y)), direction),
                TileKind::MirrorLeftRight => {
                    let direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Up,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                    };

                    self.illuminate(direction.next((x, y)), direction);
                }
                TileKind::MirrorRightLeft => {
                    let direction = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                    };

                    self.illuminate(direction.next((x, y)), direction);
                }
                TileKind::SplitterHorizontal => match direction {
                    Direction::Right | Direction::Left => {
                        self.illuminate(direction.next((x, y)), direction);
                    }
                    Direction::Up | Direction::Down => {
                        self.illuminate(Direction::Left.next((x, y)), Direction::Left);
                        self.illuminate(Direction::Right.next((x, y)), Direction::Right);
                    }
                },
                TileKind::SplitterVertical => match direction {
                    Direction::Right | Direction::Left => {
                        self.illuminate(Direction::Up.next((x, y)), Direction::Up);
                        self.illuminate(Direction::Down.next((x, y)), Direction::Down);
                    }
                    Direction::Up | Direction::Down => {
                        self.illuminate(direction.next((x, y)), direction);
                    }
                },
            }
        }
    }

    fn energized_count(&mut self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|tile| tile.energized > 0).count())
            .sum::<usize>()
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile {
                kind: TileKind::Empty,
                energized: 0,
            },
            '/' => Tile {
                kind: TileKind::MirrorLeftRight,
                energized: 0,
            },
            '\\' => Tile {
                kind: TileKind::MirrorRightLeft,
                energized: 0,
            },
            '-' => Tile {
                kind: TileKind::SplitterHorizontal,
                energized: 0,
            },
            '|' => Tile {
                kind: TileKind::SplitterVertical,
                energized: 0,
            },
            invalid => return Err(format!("'{invalid}': invalid tile char").into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut tiles =
            Contraption::try_from_lines(include_str!("../../examples/16.txt").lines()).unwrap();

        tiles.illuminate((0, 0), Direction::Right);

        assert_eq!(46, tiles.energized_count());
    }
}
