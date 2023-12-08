use std::{collections::HashMap, error, io, str};

struct Node {
    name: String,
    left: String,
    right: String,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut input = io::stdin().lines().map_while(Result::ok);

    println!(
        "{}",
        walk_to_zzz(consume_directions(&mut input)?, make_node_map(input)?)?
    );

    Ok(())
}

fn consume_directions<S: AsRef<str>>(
    lines: &mut impl Iterator<Item = S>,
) -> Result<String, Box<dyn error::Error>> {
    let directions = lines
        .next()
        .ok_or("no directions")?
        .as_ref()
        .trim()
        .to_string();
    lines.next(); // Skip empty line.

    Ok(directions)
}

fn make_node_map<S: AsRef<str>>(
    lines: impl Iterator<Item = S>,
) -> Result<HashMap<String, Node>, Box<dyn error::Error>> {
    let mut map = HashMap::new();
    for line in lines.into_iter() {
        let node = line.as_ref().parse::<Node>()?;
        map.insert(node.name.clone(), node);
    }

    Ok(map)
}

fn walk_to_zzz(
    directions: String,
    map: HashMap<String, Node>,
) -> Result<usize, Box<dyn error::Error>> {
    let mut direction_iter = directions.chars();
    let mut steps = 0;
    let mut current_node = map.get("AAA").ok_or("no starting node")?;

    while current_node.name != "ZZZ" {
        let direction = if let Some(direction) = direction_iter.next() {
            direction
        } else {
            direction_iter = directions.chars();
            direction_iter.next().ok_or("did not reach ZZZ")?
        };

        eprintln!("{} go {direction}", current_node.name);
        current_node = match direction {
            'L' => map.get(&current_node.left).ok_or("node is missing left")?,
            'R' => map.get(&current_node.right).ok_or("node is missing left")?,
            invalid => return Err(format!("invalid direction: '{invalid}'").into()),
        };

        steps += 1;
    }

    Ok(steps)
}

impl str::FromStr for Node {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('=');
        let name = parts.next().ok_or("missing node name")?.trim().to_string();

        let mut directions = parts.next().ok_or("missing node leafs")?.trim().split(',');

        let left = directions
            .next()
            .ok_or("missing left")?
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        let right = directions
            .next()
            .ok_or("missing right")?
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        Ok(Node { name, left, right })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut input = include_str!("../example1.txt").lines();

        assert_eq!(
            walk_to_zzz(
                consume_directions(&mut input).unwrap(),
                make_node_map(input).unwrap()
            )
            .unwrap(),
            2
        );
    }

    #[test]
    fn example2() {
        let mut input = include_str!("../example2.txt").lines();

        assert_eq!(
            walk_to_zzz(
                consume_directions(&mut input).unwrap(),
                make_node_map(input).unwrap()
            )
            .unwrap(),
            6
        );
    }
}
