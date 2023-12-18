//! # Part 1
//!
//! Oh hey, its [Pick's theorem] and the [Shoelace formula] again!
//!
//! [Pick's theorem]: <https://en.wikipedia.org/wiki/Pick's_theorem>
//! [Shoelace formula]: <https://en.wikipedia.org/wiki/Shoelace_formula>

use std::{error, io};

type Error = Box<dyn error::Error>;

type Pos = (isize, isize);

fn main() -> Result<(), Error> {
    let (coordinates, boundary) = coordinates_from_lines(io::stdin().lines().map_while(Result::ok))?;
    let result = area(coordinates, boundary as isize);
    println!("{result}");

    Ok(())
}

fn coordinates_from_lines(
    lines: impl Iterator<Item = impl AsRef<str>>,
) -> Result<(Vec<Pos>, usize), Error> {
    let mut coordinates = Vec::new();
    let mut pos: (isize, isize) = (0, 0);
    let mut boundary = 0;
    for line in lines {
        coordinates.push(pos);
        let mut split = line.as_ref().split_whitespace();
        let direction = split.next().ok_or("no direction")?;
        let len = split.next().ok_or("no len")?.parse::<isize>()?;
        boundary += len as usize;

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

fn area(mut coordinates: Vec<Pos>, boundary: isize) -> isize {
    eprintln!("{coordinates:?}");

    if let Some(pos) = coordinates.first().cloned() {
        coordinates.push(pos);
    }

    let mut sum: isize = 0;
    for (i, pos_b) in coordinates.iter().enumerate().skip(1) {
        let pos_a = coordinates.get(i - 1).expect("I did an of by one");
        sum += (pos_a.0 * pos_b.1) - (pos_a.1 * pos_b.0);
    }

    if sum < 0 {
        sum *= -1;
    }

    // Use Pick's theorem to get the interior size of the boundary.
    // A = i + (b / 2) - 1
    // i = -((b / 2) - 1 - A)
    let area = sum / 2;
    let interior = -((boundary / 2) - 1 - area);

    // This time the boundary counts to the area we want (the ditch).
    interior + boundary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (coordinates, boundary) =
            coordinates_from_lines(include_str!("../../examples/18.txt").lines()).unwrap();
        assert_eq!(62, area(coordinates, boundary as isize));
    }
}
