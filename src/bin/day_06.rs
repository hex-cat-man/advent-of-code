use std::{env, io};

fn main() {
    let part1_flag = env::args().skip(1).any(|flag| flag == "--part1");

    let input = io::stdin()
        .lines()
        .map_while(Result::ok)
        .fold(String::new(), |s, l| s + &l + "\n");
    let input = if part1_flag {
        split_pairs(input)
    } else {
        vec![parse_pair(input)]
    };

    println!("{}", solve(input));
}

fn solve(pairs: Vec<(usize, usize)>) -> usize {
    let mut product = 1;

    for (time, distance) in pairs {
        let mut ways = 0;
        let mut t = 1;

        while t < time {
            if (time - t) * t > distance {
                ways += 1;
            }

            t += 1;
        }

        product *= ways;
    }

    product
}

fn parse_pair(input: impl AsRef<str>) -> (usize, usize) {
    let mut lines = input.as_ref().lines();

    let time = lines
        .next()
        .expect("missing time line")
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .expect("invalid time");

    let distance = lines
        .next()
        .expect("missing distance line")
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .expect("invalid distance");

    (time, distance)
}

fn split_pairs(input: impl AsRef<str>) -> Vec<(usize, usize)> {
    let mut lines = input.as_ref().lines();

    let times = lines
        .next()
        .expect("missing time line")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().expect("invalid distance"));

    let mut distances = lines
        .next()
        .expect("missing distance line")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().expect("invalid distance"));

    times
        .map(|time| (time, distances.next().expect("missing distance")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        // Time:      7  15   30
        // Distance:  9  40  200
        let pairs = split_pairs(include_str!("../../examples/06.txt"));

        assert_eq!(pairs, vec![(7, 9), (15, 40), (30, 200)]);
        assert_eq!(solve(pairs), 288);
    }

    #[test]
    fn example_part_2() {
        // Time:      7  15   30
        // Distance:  9  40  200
        let pair = parse_pair(include_str!("../../examples/06.txt"));

        assert_eq!(pair, (71530, 940200));
        assert_eq!(solve(vec![pair]), 71503);
    }
}
