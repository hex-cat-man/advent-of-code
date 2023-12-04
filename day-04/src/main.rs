use std::{
    error,
    io::{self, BufRead},
    str,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    winners: Vec<usize>,
    numbers: Vec<usize>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut score = 0;
    for line in io::stdin().lock().lines() {
        score += line?.parse::<Card>()?.score();
    }

    println!("Score: {score}");

    Ok(())
}

impl Card {
    fn score(&self) -> usize {
        let mut score = 0;
        for num in &self.numbers {
            if self.winners.contains(num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2
                }
            }
        }

        score
    }
}

impl str::FromStr for Card {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().skip_while(|c| *c != ':').peekable();

        let mut winners = Vec::with_capacity(5);
        while chars.peek().is_some_and(|c| *c != '|') {
            while chars.peek().is_some_and(|c| !c.is_ascii_digit()) {
                chars.next();
            }
            winners.push(
                (&mut chars)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse()?,
            );
        }

        let mut numbers = Vec::with_capacity(8);
        while chars.peek().is_some() {
            while chars.peek().is_some_and(|c| !c.is_ascii_digit()) {
                chars.next();
            }
            numbers.push(
                (&mut chars)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse()?,
            );
        }

        Ok(Card { winners, numbers })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1_parse() {
        for (input, card) in [
            (
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
                Card {
                    winners: vec![41, 48, 83, 86, 17],
                    numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                },
            ),
            (
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
                Card {
                    winners: vec![13, 32, 20, 16, 61],
                    numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
                },
            ),
            (
                "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
                Card {
                    winners: vec![1, 21, 53, 59, 44],
                    numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
                },
            ),
            (
                "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
                Card {
                    winners: vec![41, 92, 73, 84, 69],
                    numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
                },
            ),
            (
                "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
                Card {
                    winners: vec![87, 83, 26, 28, 32],
                    numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
                },
            ),
            (
                "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
                Card {
                    winners: vec![31, 18, 13, 56, 72],
                    numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
                },
            ),
        ] {
            assert_eq!(input.parse::<Card>().unwrap(), card);
        }
    }

    #[test]
    fn example_part1_score() {
        for (input, score) in [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
        ] {
            assert_eq!(input.parse::<Card>().unwrap().score(), score);
        }
    }
}
