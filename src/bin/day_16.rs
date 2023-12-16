use std::{error, io};

#[cfg(not(feature = "part1"))]
use std::thread;

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
    #[cfg(feature = "part1")]
    let result = {
        let mut tiles = Contraption::try_from_lines(io::stdin().lines().map_while(Result::ok))?;
        tiles.illuminate((0, 0), Direction::Right);
        tiles.energized_count()
    };

    #[cfg(not(feature = "part1"))]
    let result = {
        let tiles = Contraption::try_from_lines(io::stdin().lines().map_while(Result::ok))?;

        // I am terribly ashamed of my actions (but its not to bad when compiled in release, so...).
        tiles
            .edge()
            .chunks(16) // Number of max concurrent threads.
            .flat_map(|chunk| {
                let jobs = chunk
                    .iter()
                    .map(|(pos, direction)| {
                        let mut tiles = tiles.clone();

                        let pos = *pos;
                        let direction = *direction;

                        thread::spawn(move || {
                            tiles.illuminate(pos, direction);
                            tiles.energized_count()
                        })
                    })
                    .collect::<Vec<_>>();

                jobs.into_iter().map(|handle| handle.join())
            })
            .try_fold(0, |max, result| match result {
                Ok(n) => Ok(n.max(max)),
                Err(_) => Err("error in thread"),
            })?
    };

    println!("{}", result);

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

    #[cfg(any(test, not(feature = "part1")))]
    fn edge(&self) -> Vec<(Pos, Direction)> {
        let mut edge = Vec::new();

        if let Some(top) = self
            .grid
            .first()
            .map(|row| row.iter().enumerate().map(|(x, _)| x).collect::<Vec<_>>())
        {
            let left = self
                .grid
                .iter()
                .enumerate()
                .map(|(y, _)| y)
                .collect::<Vec<_>>();

            for x in &top {
                edge.push(((*x as isize, 0), Direction::Down));
            }

            for y in &left {
                edge.push(((0, *y as isize), Direction::Right));
            }

            let bottom = left.len() - 1;
            for x in &top {
                edge.push(((*x as isize, bottom as isize), Direction::Up));
            }

            let right = top.len() - 1;
            for y in left.iter().rev() {
                edge.push(((right as isize, *y as isize), Direction::Left));
            }
        }

        edge
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

    #[test]
    fn example_part_2() {
        let tiles =
            Contraption::try_from_lines(include_str!("../../examples/16.txt").lines()).unwrap();

        let mut max = 0;

        for (pos, direction) in tiles.edge() {
            let mut tiles = tiles.clone();
            tiles.illuminate(pos, direction);
            max = tiles.energized_count().max(max);
        }

        assert_eq!(51, max);
    }
}
