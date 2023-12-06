use std::io;

fn main() {
    let input = split_pairs(
        io::stdin()
            .lines()
            .map_while(Result::ok)
            .fold(String::new(), |s, l| s + &l + "\n"),
    );

    println!("{}", solve_part_1(input));
}

fn solve_part_1(pairs: Vec<(usize, usize)>) -> usize {
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

fn split_pairs(input: impl AsRef<str>) -> Vec<(usize, usize)> {
    let mut lines = input.as_ref().lines();

    let times = lines
        .next()
        .expect("missing time line")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().expect("invalid distance"));

    let mut distance = lines
        .next()
        .expect("missing distance line")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().expect("invalid distance"));

    times
        .map(|time| (time, distance.next().expect("missing distance")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        // Time:      7  15   30
        // Distance:  9  40  200
        let pairs = split_pairs(include_str!("../sample.txt"));

        assert_eq!(pairs, vec![(7, 9), (15, 40), (30, 200)]);
        assert_eq!(solve_part_1(pairs), 288);
    }
}
