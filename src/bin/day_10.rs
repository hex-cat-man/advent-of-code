//! # Part 2
//!
//! - [Area of a polygon (Coordinate Geometry)]
//! - [Irregular Polygon]
//!
//! Thankfully the loop should have no intersections.
//!
//! [Area of a polygon (Coordinate Geometry)]: <https://www.mathopenref.com/coordpolygonarea.html>
//! [Irregular Polygon]: <https://www.mathopenref.com/polygonirregular.html>

use std::{error, io};

type Error = Box<dyn error::Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    kind: TileKind,
    pos: Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum TileKind {
    /// A vertical pipe connecting north and south.
    VerticalPipe = b'|',
    /// A horizontal pipe connecting east and west.
    HorizontalPipe = b'-',
    /// A 90-degree bend connecting north and east.
    NorthWestPipe = b'J',
    /// A 90-degree bend connecting north and west.
    NorthEastPipe = b'L',
    /// A 90-degree bend connecting south and west.
    SouthWestPipe = b'7',
    /// A 90-degree bend connecting south and east.
    SouthEastPipe = b'F',
    /// The starting position of the animal; there is a pipe on this tile, but your sketch doesn't
    /// show what shape the pipe has.
    Ground = b'.',
    /// Ground; there is no pipe in this tile.
    Start = b'S',
}

#[derive(Debug)]
struct Field(Vec<Vec<TileKind>>);

#[derive(Debug)]
struct LoopPath<'a> {
    field: &'a Field,
    start: Tile,
    tile: Tile,
    path: Vec<Tile>,
}

fn main() -> Result<(), Error> {
    let field = Field::try_from_lines(io::stdin().lines().map_while(Result::ok))?;
    println!("{}", field.farthest_pos_steps()?);
    Ok(())
}

impl Field {
    fn try_from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, Error> {
        Ok(Self(
            lines
                .map(|line| {
                    line.as_ref()
                        .chars()
                        .map(TileKind::try_from)
                        .collect::<Result<Vec<TileKind>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    fn get(&self, pos: Pos) -> Option<TileKind> {
        let Pos(x, y) = pos;
        self.0.get(y).and_then(|row| row.get(x)).cloned()
    }

    fn start_tile(&self) -> Result<Tile, Error> {
        let mut start = None;
        for (y, row) in self.0.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let TileKind::Start = tile {
                    start = Some(Pos(x, y));
                    break;
                }
            }
        }

        if let Some(pos) = start {
            let up = self
                .get(pos.up())
                .is_some_and(|tile| pos.up() != pos && tile.connects_down());
            let right = self
                .get(pos.right())
                .is_some_and(|tile| tile.connects_left());
            let down = self.get(pos.down()).is_some_and(|tile| tile.connects_up());
            let left = self
                .get(pos.left())
                .is_some_and(|tile| pos.left() != pos && tile.connects_right());

            // Get the underlining tile (probably more complicated than it needs to be, but
            // eh...).
            let tile = match (up, right, down, left) {
                (true, true, false, false) => TileKind::NorthEastPipe,
                (true, false, true, false) => TileKind::VerticalPipe,
                (true, false, false, true) => TileKind::NorthWestPipe,
                (false, true, true, false) => TileKind::SouthEastPipe,
                (false, true, false, true) => TileKind::HorizontalPipe,
                (false, false, true, true) => TileKind::SouthWestPipe,
                _ => return Err("the start tile must be connected to two pipes".into()),
            };

            Ok(Tile { kind: tile, pos })
        } else {
            Err("field has no start tile".into())
        }
    }

    fn loop_path(&self) -> Result<LoopPath<'_>, Error> {
        let start = self.start_tile()?;

        Ok(LoopPath {
            field: self,
            start,
            tile: start,
            path: vec![start],
        })
    }

    fn farthest_pos_steps(&self) -> Result<usize, Error> {
        Ok(self
            .loop_path()?
            .map_while(Result::ok)
            .enumerate()
            .last()
            .map(|(i, _)| (i + 1) / 2)
            .unwrap_or_default())
    }
}

impl Iterator for LoopPath<'_> {
    type Item = Result<Tile, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let tile = self.tile;

        // End the iterator if we reached start again (do not emit the start position twice!).
        if self.path.len() > 1 && self.start.pos == tile.pos {
            return None;
        }

        macro_rules! directions {
            ($a:ident, $b:ident) => {{
                let next = tile.pos.$a();

                if Some(next) == self.path.iter().nth_back(1).map(|n| n.pos) {
                    tile.pos.$b()
                } else {
                    next
                }
            }};
        }

        // Advance along the pipe loop.
        let next = match self.tile.kind {
            TileKind::VerticalPipe => directions!(down, up),
            TileKind::HorizontalPipe => directions!(left, right),
            TileKind::NorthWestPipe => directions!(left, up),
            TileKind::NorthEastPipe => directions!(right, up),
            TileKind::SouthWestPipe => directions!(down, left),
            TileKind::SouthEastPipe => directions!(down, right),
            invalid => unimplemented!("cannot move from start '{invalid:?}'"),
        };

        if let Some(tile) = self.field.get(next) {
            self.tile = Tile {
                kind: tile,
                pos: next,
            };
        } else {
            return Some(Err("next position is out of bounds".into()));
        }

        self.path.push(self.tile);

        Some(Ok(tile))
    }
}

impl Pos {
    fn up(&self) -> Pos {
        let Pos(x, y) = *self;
        Pos(x, if y > 0 { y - 1 } else { y })
    }

    fn right(&self) -> Pos {
        let Pos(x, y) = *self;
        Pos(x + 1, y)
    }

    fn left(&self) -> Pos {
        let Pos(x, y) = *self;
        Pos(if x > 0 { x - 1 } else { x }, y)
    }

    fn down(&self) -> Pos {
        let Pos(x, y) = *self;
        Pos(x, y + 1)
    }
}

impl TileKind {
    fn connects_up(&self) -> bool {
        matches!(
            self,
            TileKind::VerticalPipe | TileKind::NorthEastPipe | TileKind::NorthWestPipe
        )
    }

    fn connects_down(&self) -> bool {
        matches!(
            self,
            TileKind::VerticalPipe | TileKind::SouthEastPipe | TileKind::SouthWestPipe
        )
    }

    fn connects_left(&self) -> bool {
        matches!(
            self,
            TileKind::HorizontalPipe | TileKind::NorthWestPipe | TileKind::SouthWestPipe
        )
    }

    fn connects_right(&self) -> bool {
        matches!(
            self,
            TileKind::HorizontalPipe | TileKind::NorthEastPipe | TileKind::SouthEastPipe
        )
    }
}

impl TryFrom<char> for TileKind {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => TileKind::VerticalPipe,
            '-' => TileKind::HorizontalPipe,
            'J' => TileKind::NorthWestPipe,
            'L' => TileKind::NorthEastPipe,
            '7' => TileKind::SouthWestPipe,
            'F' => TileKind::SouthEastPipe,
            '.' => TileKind::Ground,
            'S' => TileKind::Start,
            invalid => return Err(format!("'{invalid}': invalid tile").into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_start() {
        let field = Field::try_from_lines(include_str!("../../examples/10-1.txt").lines()).unwrap();
        assert_eq!(
            Tile {
                kind: TileKind::SouthEastPipe,
                pos: Pos(1, 1)
            },
            field.start_tile().unwrap()
        );

        let field = Field::try_from_lines(include_str!("../../examples/10-2.txt").lines()).unwrap();
        assert_eq!(
            Tile {
                kind: TileKind::SouthEastPipe,
                pos: Pos(0, 2)
            },
            field.start_tile().unwrap()
        );
    }

    #[test]
    fn example_1() {
        let field = Field::try_from_lines(include_str!("../../examples/10-1.txt").lines()).unwrap();
        assert_eq!(4, field.farthest_pos_steps().unwrap());
    }

    #[test]
    fn example_2() {
        let field = Field::try_from_lines(include_str!("../../examples/10-2.txt").lines()).unwrap();
        assert_eq!(8, field.farthest_pos_steps().unwrap());
    }
}
