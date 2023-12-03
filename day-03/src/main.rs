use std::{
    error,
    io::{self, BufRead},
    ops,
};

struct Schematic(Vec<String>);

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut lines = Vec::new();
    for line in io::stdin().lock().lines() {
        lines.push(line?);
    }

    println!("{}", Schematic(lines).sum());

    Ok(())
}

impl Schematic {
    fn sum(&self) -> usize {
        let mut sum = 0;
        for (line_nr, line) in self.0.iter().enumerate() {
            let mut chars = line.chars().enumerate().peekable();
            while chars.peek().is_some() {
                let mut start = None;
                let mut end = 0;
                let num = (&mut chars)
                    .take_while(|(i, c)| {
                        end = *i;
                        if c.is_ascii_digit() {
                            if start.is_none() {
                                start = Some(*i);
                            }
                            true
                        } else {
                            false
                        }
                    })
                    .map(|(_, c)| c)
                    .collect::<String>()
                    .parse::<usize>();

                if let (Some(start), Ok(num)) = (start, num) {
                    if self.is_part(line_nr, start, end) {
                        sum += num;
                    }
                }
            }
        }

        sum
    }

    fn range_contains_part_symbol(&self, line: &str, range: ops::Range<usize>) -> bool {
        for pos in range {
            if let Some(c) = line.chars().nth(pos) {
                if !c.is_ascii_digit() && c != '.' {
                    return true;
                }
            }
        }

        false
    }

    fn is_part(&self, line_nr: usize, start: usize, end: usize) -> bool {
        let range = if start > 0 {
            (start - 1)..(end + 1)
        } else {
            (start)..(end + 1)
        };

        if line_nr > 0 {
            if let Some(line) = self.0.get(line_nr - 1) {
                if self.range_contains_part_symbol(line, range.clone()) {
                    return true;
                }
            }
        }

        if let Some(line) = self.0.get(line_nr) {
            if self.range_contains_part_symbol(line, range.clone()) {
                return true;
            }
        }

        if let Some(line) = self.0.get(line_nr + 1) {
            if self.range_contains_part_symbol(line, range) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        let schematic = Schematic(vec![
            "467..114..".into(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ]);

        assert_eq!(schematic.sum(), 4361);
    }
}
