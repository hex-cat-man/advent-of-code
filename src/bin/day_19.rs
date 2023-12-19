use std::{collections::HashMap, error, io, str::FromStr};

type Error = Box<dyn error::Error>;

#[derive(Debug)]
enum Category {
    ExtremelyCoolLooking,
    /// It makes a noise when you hit it.
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    Workflow(String),
}

#[derive(Debug)]
enum Rule {
    Action(Action),
    Lesser {
        left: Category,
        right: usize,
        action: Action,
    },
    Greater {
        left: Category,
        right: usize,
        action: Action,
    },
}

#[derive(Debug)]
struct Workflow {
    label: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn main() -> Result<(), Error> {
    let (workflows, machine_parts) = init(io::stdin().lines().map_while(Result::ok))?;

    println!("{}", sum_accepted_parts(workflows, machine_parts)?);

    Ok(())
}

fn sum_accepted_parts(
    workflows: HashMap<String, Workflow>,
    machine_parts: Vec<MachinePart>,
) -> Result<usize, Error> {
    let mut sum = 0;

    'outer: for machine_part in machine_parts {
        let mut workflow = workflows.get("in").ok_or("no `in` rule")?;

        eprintln!("{machine_part:?}");

        loop {
            eprintln!("\tWorkflow '{}'", &workflow.label);

            match workflow.process(&machine_part) {
                Action::Accept => {
                    eprintln!("\tAccepted");
                    break;
                }
                Action::Reject => {
                    eprintln!("\tRejected");
                    continue 'outer;
                }
                Action::Workflow(label) => {
                    workflow = workflows.get(label).ok_or("unknown workflow")?;
                    continue;
                }
            }
        }

        sum += machine_part.sum();
    }

    Ok(sum)
}

fn init(
    mut lines: impl Iterator<Item = impl AsRef<str>>,
) -> Result<(HashMap<String, Workflow>, Vec<MachinePart>), Error> {
    let workflows: Vec<Workflow> = parse_from_lines(&mut lines)?;
    let machine_parts: Vec<MachinePart> = parse_from_lines(&mut lines)?;

    let mut map = HashMap::new();
    for workflow in workflows {
        map.insert(workflow.label.clone(), workflow);
    }

    Ok((map, machine_parts))
}

fn parse_from_lines<T>(lines: &mut impl Iterator<Item = impl AsRef<str>>) -> Result<Vec<T>, Error>
where
    T: FromStr<Err = Error>,
{
    let mut v = Vec::new();

    for line in lines {
        let line = line.as_ref();

        v.push(line.parse()?);

        if line.is_empty() {
            break;
        }
    }

    Ok(v)
}

impl Workflow {
    fn process(&self, machine_part: &MachinePart) -> &Action {
        for rule in &self.rules {
            match rule {
                Rule::Action(action) => return action,
                Rule::Lesser {
                    left,
                    right,
                    action,
                } => {
                    if machine_part.get(left) < *right {
                        return action;
                    }
                }
                Rule::Greater {
                    left,
                    right,
                    action,
                } => {
                    if machine_part.get(left) > *right {
                        return action;
                    }
                }
            }
        }

        todo!()
    }
}

impl MachinePart {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn get(&self, category: &Category) -> usize {
        match category {
            Category::ExtremelyCoolLooking => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        }
    }
}

impl TryFrom<char> for Category {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'x' => Category::ExtremelyCoolLooking,
            'm' => Category::Musical,
            'a' => Category::Aerodynamic,
            's' => Category::Shiny,
            invalid => return Err(format!("'{invalid}': invalid category").into()),
        })
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Action::Accept,
            "R" => Action::Reject,
            workflow => Action::Workflow(workflow.to_string()),
        })
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':').rev();

        let action = parts.next().ok_or("no action")?.parse::<Action>()?;

        if let Some(condition) = parts.next() {
            let mut chars = condition.chars();
            let category = chars.next().ok_or("no categroy")?.try_into()?;
            let op = chars.next().ok_or("no operator")?;
            let n = chars.as_str().parse::<usize>()?;

            Ok(match op {
                '<' => Rule::Lesser {
                    left: category,
                    right: n,
                    action,
                },
                '>' => Rule::Greater {
                    left: category,
                    right: n,
                    action,
                },
                invalid => return Err(format!("'{invalid}': invalid operator").into()),
            })
        } else {
            Ok(Rule::Action(action))
        }
    }
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let label = (&mut chars).take_while(|c| *c != '{').collect::<String>();

        let comparisons = chars.take_while(|c| *c != '}').collect::<String>();
        let comparisons = comparisons.split(',');

        let mut rules = Vec::new();
        for cmp in comparisons {
            rules.push(cmp.parse()?);
        }

        Ok(Self { label, rules })
    }
}

impl FromStr for MachinePart {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let s = &s[1..(s.len() - 1)]; // Ignore '{' and '}'

        let mut iter = s.split(',');

        let x = iter
            .next()
            .ok_or("no x value pair")?
            .split('=')
            .nth(1)
            .ok_or("no x value")?
            .parse::<usize>()?;

        let m = iter
            .next()
            .ok_or("no m value pair")?
            .split('=')
            .nth(1)
            .ok_or("no m value")?
            .parse::<usize>()?;

        let a = iter
            .next()
            .ok_or("no a value pair")?
            .split('=')
            .nth(1)
            .ok_or("no a value")?
            .parse::<usize>()?;

        let s = iter
            .next()
            .ok_or("no s value pair")?
            .split('=')
            .nth(1)
            .ok_or("no s value")?
            .parse::<usize>()?;

        Ok(Self { x, m, a, s })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (workflows, machine_parts) =
            init(include_str!("../../examples/19.txt").lines()).unwrap();

        assert_eq!(19114, sum_accepted_parts(workflows, machine_parts).unwrap());
    }
}
