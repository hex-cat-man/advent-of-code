use std::{cmp, collections::HashMap, error, io, str, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    #[cfg(feature = "part1")]
    Joker,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    #[cfg(not(feature = "part1"))]
    /// A joker can be used for the strongest possible hand, but is the weakest card.
    Joker,
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

    let mut sum = 0;
    for (rank, hand) in hands.into_iter().enumerate() {
        sum += (rank + 1) * hand.bid;
    }

    sum
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
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

        #[cfg(not(feature = "part1"))]
        let mut jokers = 0;
        let mut partitions = HashMap::new();

        for card in cards {
            #[cfg(not(feature = "part1"))]
            if card == Card::Joker {
                jokers += 1;
                continue;
            } 

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

        #[cfg(feature = "part1")]
        let jokers = 0;

        match (jokers, partitions.as_slice()) {
            (0, [_, _, _, _, 5]) => HandType::FiveOfKind,
            (0, [_, _, _, _, 4]) => HandType::FourOfKind,
            (_, [_, _, _, _, 4]) => HandType::FiveOfKind,
            (0, [_, _, _, 2, 3]) => HandType::FullHouse,
            (0, [_, _, _, 1, 3]) => HandType::ThreeOfKind,
            (_, [_, _, _, 1, 3]) => HandType::FourOfKind,
            (0, [_, _, _, 2, 2]) => HandType::TwoPair,
            (_, [_, _, _, 2, 2]) => HandType::FullHouse,
            (0, [_, 1, 1, 1, 2]) => HandType::OnePair,
            (_, [_, 1, 1, 1, 2]) => HandType::TwoPair,
            (0, [1, 1, 1, 1, 1]) => HandType::HighCard,
            (5 | 4, _) => HandType::FiveOfKind,
            (3, [_, _, _, _, 2]) => HandType::FiveOfKind,
            (3, [_, _, _, _, 1]) => HandType::FourOfKind,
            (2, [_, _, _, _, 3]) => HandType::FiveOfKind,
            (2, [_, _, _, _, 1]) => HandType::ThreeOfKind,
            (1, [_, _, _, _, 2]) => HandType::ThreeOfKind,
            (2, [_, _, _, _, 2]) => HandType::FourOfKind,
            (1, [_, _, _, _, 1]) => HandType::OnePair,
            invalid => unreachable!("{invalid:?}"),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = Box<dyn error::Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Joker,
            'T' => Card::Ten,
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

    #[cfg(feature = "part1")]
    #[test]
    fn example_part1() {
        let hands = include_str!("../sample.txt")
            .lines()
            .map(Hand::from_str)
            .map_while(Result::ok)
            .collect::<Vec<_>>();

        #[rustfmt::skip]
        assert_ne!(hands, vec![
            Hand { bid: 765, cards: [Card::Three, Card::Two, Card::T, Card::Three, Card::King], hand_type: HandType::OnePair },
            Hand { bid: 684, cards: [Card::T, Card::Five, Card::Five, Card::J, Card::Five], hand_type: HandType::TwoPair },
            Hand { bid: 28, cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven], hand_type: HandType::TwoPair },
            Hand { bid: 220, cards: [Card::King, Card::T, Card::J, Card::J, Card::T], hand_type: HandType::ThreeOfKind },
            Hand { bid: 483, cards: [Card::Queen, Card::Queen, Card::Queen, Card::J, Card::Ace], hand_type: HandType::ThreeOfKind },
        ]);

        assert_eq!(rank(hands), 6440);
    }

    #[test]
    fn example_part2() {
        let hands = include_str!("../sample.txt")
            .lines()
            .map(Hand::from_str)
            .map_while(Result::ok)
            .collect::<Vec<_>>();

        #[rustfmt::skip]
        assert_ne!(hands, vec![
            Hand { bid: 765, cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King], hand_type: HandType::OnePair },
            Hand { bid: 684, cards: [Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five], hand_type: HandType::TwoPair },
            Hand { bid: 28, cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven], hand_type: HandType::TwoPair },
            Hand { bid: 220, cards: [Card::King, Card::Ten, Card::Joker, Card::Joker, Card::Ten], hand_type: HandType::ThreeOfKind },
            Hand { bid: 483, cards: [Card::Queen, Card::Queen, Card::Queen, Card::Joker, Card::Ace], hand_type: HandType::ThreeOfKind },
        ]);

        assert_eq!(rank(hands), 5905);
    }
}
