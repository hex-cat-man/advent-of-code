use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    error, io, str, usize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    bid: usize,
    cards: [Card; 5],
    hand_type: HandType,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let hands = io::stdin()
        .lines()
        .map_while(Result::ok)
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;

    println!("{}", rank(hands));

    Ok(())
}

fn rank(mut hands: Vec<Hand>) -> usize {
    hands.sort();

    for hand in &hands {
        eprintln!("{hand:?}");
    }

    let mut sum = 0;
    for (rank, hand) in hands.into_iter().enumerate() {
        sum += (rank + 1) * hand.bid;
    }

    sum
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            cmp::Ordering::Equal => {
                for (i, card) in self.cards.iter().enumerate() {
                    match card.cmp(&other.cards[i]) {
                        cmp::Ordering::Greater => return cmp::Ordering::Less,
                        cmp::Ordering::Less => return cmp::Ordering::Greater,
                        _ => {}
                    }
                }

                cmp::Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

impl From<[Card; 5]> for HandType {
    fn from(mut cards: [Card; 5]) -> Self {
        cards.sort();

        let mut partitions = HashMap::new();

        for card in cards {
            if let Some(n) = partitions.get(&card) {
                partitions.insert(card, n + 1);
            } else {
                partitions.insert(card, 1);
            }
        }

        let mut partitions = partitions.values().cloned().collect::<Vec<u8>>();

        while partitions.len() < 5 {
            partitions.push(0);
        }

        partitions.sort();

        match partitions.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [0, 1, 1, 1, 2] => HandType::OnePair,
            [_, _, _, 2, 2] => HandType::TwoPair,
            [_, _, _, 1, 3] => HandType::ThreeOfKind,
            [_, _, _, 2, 3] => HandType::FullHouse,
            [_, _, _, _, 4] => HandType::FourOfKind,
            [_, _, _, _, 5] => HandType::FiveOfKind,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = Box<dyn error::Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            c => return Err(format!("'{c}': invalid card").into()),
        })
    }
}

impl str::FromStr for Hand {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split_whitespace();
        let mut hand = line.next().ok_or("empty hand")?.chars();
        let bid = line.next().ok_or("no bid")?.parse::<usize>()?;

        let err = "missing card";
        let cards = [
            hand.next().ok_or(err)?.try_into()?,
            hand.next().ok_or(err)?.try_into()?,
            hand.next().ok_or(err)?.try_into()?,
            hand.next().ok_or(err)?.try_into()?,
            hand.next().ok_or(err)?.try_into()?,
        ];

        let hand_type = cards.into();

        Ok(Hand {
            bid,
            cards,
            hand_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn example_part1() {
        let hands = include_str!("../sample.txt")
            .lines()
            .map(Hand::from_str)
            .map_while(Result::ok)
            .collect::<Vec<_>>();

        #[rustfmt::skip]
        assert_ne!(hands, vec![
            Hand { bid: 765, cards: [Card::Three, Card::Two, Card::T, Card::Three, Card::K], hand_type: HandType::OnePair },
            Hand { bid: 684, cards: [Card::T, Card::Five, Card::Five, Card::J, Card::Five], hand_type: HandType::TwoPair },
            Hand { bid: 28, cards: [Card::K, Card::K, Card::Six, Card::Seven, Card::Seven], hand_type: HandType::TwoPair },
            Hand { bid: 220, cards: [Card::K, Card::T, Card::J, Card::J, Card::T], hand_type: HandType::ThreeOfKind },
            Hand { bid: 483, cards: [Card::Q, Card::Q, Card::Q, Card::J, Card::A], hand_type: HandType::ThreeOfKind },
        ]);

        assert_eq!(rank(hands), 6440);
    }
}
