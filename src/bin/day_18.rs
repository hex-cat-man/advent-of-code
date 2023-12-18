//! # Part 1
//!
//! Oh hey, its [Pick's theorem] and the [Shoelace formula] again!
//!
//! [Pick's theorem]: <https://en.wikipedia.org/wiki/Pick's_theorem>
//! [Shoelace formula]: <https://en.wikipedia.org/wiki/Shoelace_formula>

use std::{error, io};

type Error = Box<dyn error::Error>;

type Pos = (i64, i64);

fn main() -> Result<(), Error> {
    #[cfg(feature = "part1")]
    let (coordinates, boundary) =
        coordinates_from_lines_part_1(io::stdin().lines().map_while(Result::ok))?;
    #[cfg(not(feature = "part1"))]
    let (coordinates, boundary) =
        coordinates_from_lines(io::stdin().lines().map_while(Result::ok))?;

    let result = area(coordinates, boundary);
    println!("{result}");

    Ok(())
}

#[cfg(any(test, feature = "part1"))]
fn coordinates_from_lines_part_1(
    lines: impl Iterator<Item = impl AsRef<str>>,
) -> Result<(Vec<Pos>, i64), Error> {
    let mut coordinates = Vec::new();
    let mut pos: (i64, i64) = (0, 0);
    let mut boundary = 0;
    for line in lines {
        coordinates.push(pos);
        let mut split = line.as_ref().split_whitespace();
        let direction = split.next().ok_or("no direction")?;
        let len = split.next().ok_or("no len")?.parse::<i64>()?;
        boundary += len;

        let (x, y) = pos;
        pos = match direction {
            "U" => (x, y - len),
            "R" => (x + len, y),
            "D" => (x, y + len),
            "L" => (x - len, y),
            invalid => return Err(format!("'{invalid}': invalid direction").into()),
        };
    }

    Ok((coordinates, boundary))
}

#[cfg(any(test, not(feature = "part1")))]
fn coordinates_from_lines(
    lines: impl Iterator<Item = impl AsRef<str>>,
) -> Result<(Vec<Pos>, i64), Error> {
    let mut coordinates = Vec::new();
    let mut pos: (i64, i64) = (0, 0);
    let mut boundary = 0;
    for line in lines {
        coordinates.push(pos);
        let hex = line
            .as_ref()
            .split_whitespace()
            .nth(2)
            .ok_or("missing instruction")?;

        if hex.len() != 9 {
            return Err("instruction to short".into());
        }

        let direction = hex.chars().nth(7).ok_or("missing direction")?;
        let len = i64::from_str_radix(&hex[2..7], 16)?;

        boundary += len;

        let (x, y) = pos;
        pos = match direction {
            '3' => (x, y - len),
            '0' => (x + len, y),
            '1' => (x, y + len),
            '2' => (x - len, y),
            invalid => return Err(format!("'{invalid}': invalid direction").into()),
        };
    }

    Ok((coordinates, boundary))
}

fn area(mut coordinates: Vec<Pos>, boundary: i64) -> i64 {
    if let Some(pos) = coordinates.first().cloned() {
        coordinates.push(pos);
    }

    let mut sum: i64 = 0;
    for (i, pos_b) in coordinates.iter().enumerate().skip(1) {
        let pos_a = coordinates.get(i - 1).expect("I did an of by one");
        sum += (pos_a.0 * pos_b.1) - (pos_a.1 * pos_b.0);
    }

    if sum < 0 {
        sum *= -1;
    }

    let area = sum / 2;

    // Use Pick's theorem to get the size of the interior + boundary.
    // A = i + (b / 2) - 1     | - (b / 2) + 1
    // i = A - (b / 2) + 1     | + b
    // i + b = A + (b / 2) + 1
    area + (boundary / 2) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let (coordinates, boundary) =
            coordinates_from_lines_part_1(include_str!("../../examples/18.txt").lines()).unwrap();
        assert_eq!(62, area(coordinates, boundary));
    }

    #[test]
    fn example_part_2() {
        let (coordinates, boundary) =
            coordinates_from_lines(include_str!("../../examples/18.txt").lines()).unwrap();
        assert_eq!(952408144115, area(coordinates, boundary));
    }
}
