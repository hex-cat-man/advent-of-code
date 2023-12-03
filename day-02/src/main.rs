use std::{
    collections::HashMap,
    error,
    io::{self, BufRead},
    str,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cube {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: usize,
    sets: Vec<HashMap<Cube, usize>>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut sum = 0;
    let mut pow_sum = 0;
    for line in io::stdin().lock().lines() {
        let game: Game = line?.parse()?;

        if game.is_possible(12, 13, 14) {
            sum += game.id;
        }

        pow_sum += game.required_pow();
    }

    println!("Sum of possible game ids: {sum}");
    println!("Sum of minimal cube power: {pow_sum}");

    Ok(())
}

impl Game {
    fn max(&self, cube: &Cube) -> usize {
        self.sets
            .iter()
            .filter_map(|set| set.get(cube))
            .max()
            .cloned()
            .unwrap_or_default()
    }

    fn required_pow(&self) -> usize {
        self.max(&Cube::Red) * self.max(&Cube::Green) * self.max(&Cube::Blue)
    }

    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        if red < self.max(&Cube::Red) {
            return false;
        }

        if green < self.max(&Cube::Green) {
            return false;
        }

        if blue < self.max(&Cube::Blue) {
            return false;
        }

        true
    }
}

impl str::FromStr for Game {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().skip(5).enumerate().peekable(); // Skip "Game "

        let id: usize = (&mut chars)
            .take_while(|(_, c)| c.is_ascii_digit())
            .map(|(_, c)| c)
            .collect::<String>()
            .parse()?;

        let mut sets = Vec::new();
        let mut set = HashMap::new();

        while chars.peek().is_some() {
            chars.next(); // Skip ' ' (':', ',', ';', is consumed by take_while).

            let amount: usize = (&mut chars)
                .take_while(|(_, c)| c.is_ascii_digit())
                .map(|(_, c)| c)
                .collect::<String>()
                .parse()?;

            // Assume true, since the last semicolon is omitted.
            let mut is_semicolon = true;
            let color: Cube = (&mut chars)
                .take_while(|(_, c)| match c {
                    ';' => false,
                    ',' => {
                        is_semicolon = false;
                        false
                    }
                    _ => true,
                })
                .map(|(_, c)| c)
                .collect::<String>()
                .parse()?;

            set.insert(color, amount);

            if is_semicolon {
                sets.push(set);
                set = HashMap::new()
            }
        }

        Ok(Game { id, sets })
    }
}

impl str::FromStr for Cube {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Cube::Red,
            "green" => Cube::Green,
            "blue" => Cube::Blue,
            _ => return Err(format!("'{s}' is not a color").into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        for (s, game) in [
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                Game {
                    id: 1,
                    sets: vec![
                        [(Cube::Blue, 3), (Cube::Red, 4)].into(),
                        [(Cube::Red, 1), (Cube::Green, 2), (Cube::Blue, 6)].into(),
                        [(Cube::Green, 2)].into(),
                    ],
                },
            ),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                Game {
                    id: 2,
                    sets: vec![
                        [(Cube::Blue, 1), (Cube::Green, 2)].into(),
                        [(Cube::Green, 3), (Cube::Blue, 4), (Cube::Red, 1)].into(),
                        [(Cube::Green, 1), (Cube::Blue, 1)].into(),
                    ],
                },
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                Game {
                    id: 3,
                    sets: vec![
                        [(Cube::Green, 8), (Cube::Blue, 6), (Cube::Red, 20)].into(),
                        [(Cube::Blue, 5), (Cube::Red, 4), (Cube::Green, 13)].into(),
                        [(Cube::Green, 5), (Cube::Red, 1)].into(),
                    ],
                },
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                Game {
                    id: 4,
                    sets: vec![
                        [(Cube::Green, 1), (Cube::Red, 3), (Cube::Blue, 6)].into(),
                        [(Cube::Green, 3), (Cube::Red, 6)].into(),
                        [(Cube::Green, 3), (Cube::Blue, 15), (Cube::Red, 14)].into(),
                    ],
                },
            ),
            (
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                Game {
                    id: 5,
                    sets: vec![
                        [(Cube::Red, 6), (Cube::Blue, 1), (Cube::Green, 3)].into(),
                        [(Cube::Blue, 2), (Cube::Red, 1), (Cube::Green, 2)].into(),
                    ],
                },
            ),
        ] {
            assert_eq!(s.parse::<Game>().unwrap(), game)
        }
    }

    #[test]
    fn possible_game() {
        assert!("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse::<Game>()
            .unwrap()
            .is_possible(12, 13, 14));

        assert!(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
                .parse::<Game>()
                .unwrap()
                .is_possible(12, 13, 14)
        );

        assert!(
            !"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                .parse::<Game>()
                .unwrap()
                .is_possible(12, 13, 14)
        );

        assert!(
            !"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                .parse::<Game>()
                .unwrap()
                .is_possible(12, 13, 14)
        );

        assert!("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .parse::<Game>()
            .unwrap()
            .is_possible(12, 13, 14));
    }

    #[test]
    fn required_power() {
        let mut sum = 0;
        for game in [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ] {
            sum += game.parse::<Game>().unwrap().required_pow();
        }

        assert_eq!(sum, 2286)
    }
}
