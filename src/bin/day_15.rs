use std::{error, io};

#[cfg(any(test, not(feature = "part1")))]
use std::{array, str::FromStr};

type Error = Box<dyn error::Error>;

#[cfg(any(test, not(feature = "part1")))]
#[derive(Debug)]
enum Operation {
    RemoveLens,
    AddLens(u8),
}

#[cfg(any(test, not(feature = "part1")))]
#[derive(Debug)]
struct Step {
    label: String,
    operation: Operation,
    hash: u8,
}

#[cfg(any(test, not(feature = "part1")))]
#[derive(Debug)]
struct Lense {
    label: String,
    focal_length: u8,
}

#[cfg(any(test, not(feature = "part1")))]
#[derive(Debug)]
struct Boxes([Vec<Lense>; 256]);

fn main() -> Result<(), Error> {
    #[cfg(feature = "part1")]
    for line in io::stdin().lines() {
        let line = line?;
        println!("{}", init_seq_sum(line));
    }

    #[cfg(not(feature = "part1"))]
    for line in io::stdin().lines() {
        let line = line?;
        let labels = line
            .replace('\n', "")
            .split(',')
            .map(Step::from_str)
            .collect::<Vec<_>>();

        let mut boxes = Boxes::default();
        for label in labels {
            let label = label?;
            boxes.apply(label)?;
        }

        println!("{}", boxes.focusing_power());
    }

    Ok(())
}

#[cfg(any(test, feature = "part1"))]
fn init_seq_sum(s: impl AsRef<str>) -> usize {
    let mut sum = 0;

    for step in s.as_ref().replace('\n', "").split(',') {
        sum += hash(step) as usize;
    }

    sum
}

fn hash(s: impl AsRef<str>) -> u8 {
    let mut val = 0;

    for c in s.as_ref().chars() {
        val += c as u16;
        val *= 17;
        val %= 256;
    }

    val as u8
}

#[cfg(any(test, not(feature = "part1")))]
impl Boxes {
    fn apply(&mut self, step: Step) -> Result<(), Error> {
        let lense_box = self.0.get_mut(step.hash as usize).ok_or("no such box")?;

        match step.operation {
            Operation::RemoveLens => {
                if let Some((i, _)) = lense_box
                    .iter()
                    .enumerate()
                    .find(|(_, lense)| lense.label == step.label)
                {
                    lense_box.remove(i);
                }
            }
            Operation::AddLens(focal_length) => {
                let lense = Lense {
                    label: step.label,
                    focal_length,
                };

                let label = &lense.label;

                if let Some((i, _)) = lense_box
                    .iter()
                    .enumerate()
                    .find(|(_, lense)| &lense.label == label)
                {
                    lense_box[i] = lense;
                } else {
                    lense_box.push(lense);
                }
            }
        }

        Ok(())
    }

    fn focusing_power(&self) -> usize {
        let mut sum = 0;
        for (i, lense_box) in self.0.iter().enumerate() {
            let box_slot = i + 1;
            sum += lense_box.iter().enumerate().fold(0, |fs, (i, lense)| {
                let slot = i + 1;
                fs + box_slot * slot * lense.focal_length as usize
            })
        }

        sum
    }
}

#[cfg(any(test, not(feature = "part1")))]
impl Default for Boxes {
    fn default() -> Self {
        Self(array::from_fn(|_| Vec::new()))
    }
}

#[cfg(any(test, not(feature = "part1")))]
impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut label = String::new();
        let mut operation = None;
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                '-' => operation = Some(Operation::RemoveLens),
                '=' => {
                    operation = Some(Operation::AddLens(chars.as_str().parse::<u8>()?));
                    break;
                }
                c => label.push(c),
            }
        }

        let hash = hash(&label);

        Ok(Step {
            label,
            operation: operation.ok_or("label does not contain operation")?,
            hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_hash() {
        assert_eq!(52, hash("HASH"));
    }

    #[test]
    fn example_sequence() {
        assert_eq!(1320, init_seq_sum(include_str!("../../examples/15.txt")));
    }

    #[test]
    fn example_steps() {
        let labels = include_str!("../../examples/15.txt")
            .replace('\n', "")
            .split(',')
            .map(Step::from_str)
            .collect::<Vec<_>>();

        let mut boxes = Boxes::default();
        for label in labels.into_iter() {
            boxes.apply(label.unwrap()).unwrap();
        }

        assert_eq!(145, boxes.focusing_power());
    }
}
