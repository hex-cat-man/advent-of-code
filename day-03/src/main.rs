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

    let schematic = Schematic(lines);
    println!("Part number sum: {}", schematic.part_num_sum());
    println!("Gear ratio sum: {}", schematic.gear_ratio_sum());

    Ok(())
}

impl Schematic {
    fn part_num_sum(&self) -> usize {
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

    fn gear_ratio_sum(&self) -> usize {
        let mut sum = 0;
        for (line_nr, line) in self.0.iter().enumerate() {
            let mut chars = line.chars().enumerate().filter(|(_, c)| *c == '*');

            // Check each "gear" position for EXACTLY two numbers.
            while let Some((i, '*')) = chars.next() {
                let mut nums = vec![];

                if line_nr > 0 {
                    if let Some(line) = self.0.get(line_nr - 1) {
                        let (n1, n2) = self.get_number_at(line, i);

                        if let Some(n) = n1 {
                            nums.push(n);
                        }

                        if let Some(n) = n2 {
                            nums.push(n);
                        }
                    }
                }

                // Look before and after the "gear" in the same line.
                if let Some(line) = self.0.get(line_nr) {
                    let (n1, n2) = self.get_number_at(line, i);

                    if let Some(n) = n1 {
                        nums.push(n);
                    }

                    if let Some(n) = n2 {
                        nums.push(n);
                    }
                }

                if let Some(line) = self.0.get(line_nr + 1) {
                    let (n1, n2) = self.get_number_at(line, i);

                    if let Some(n) = n1 {
                        nums.push(n);
                    }

                    if let Some(n) = n2 {
                        nums.push(n);
                    }
                }

                if nums.len() == 2 {
                    sum += nums[0] * nums[1];
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

    /// Get up to two numbers at the given "gear" position.
    fn get_number_at(&self, line: &str, pos: usize) -> (Option<usize>, Option<usize>) {
        let mut chars = line.chars().peekable();

        if chars.nth(pos).is_some_and(|c| c.is_ascii_digit()) {
            (
                // Only find the start of the number to the left and follow it to the end.
                {
                    let mut start = pos;
                    while start > 0
                        && line
                            .chars()
                            .nth(start - 1)
                            .is_some_and(|c| c.is_ascii_digit())
                    {
                        start -= 1;
                    }

                    line.chars()
                        .skip(start)
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<usize>()
                        .ok()
                },
                None,
            )
        } else {
            (
                // Check to the left of pos. If it is a digit, find the start of the number.
                {
                    let mut start = pos;
                    while start > 0
                        && line
                            .chars()
                            .nth(start - 1)
                            .is_some_and(|c| c.is_ascii_digit())
                    {
                        start -= 1;
                    }

                    line.chars()
                        .skip(start)
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<usize>()
                        .ok()
                },
                // Check to the right of pos. If it is a digit, parse it and all following digits.
                {
                    line.chars()
                        .skip(pos + 1)
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<usize>()
                        .ok()
                },
            )
        }
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

        assert_eq!(schematic.part_num_sum(), 4361);
    }

    #[test]
    fn example_part_two() {
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

        assert_eq!(schematic.gear_ratio_sum(), 467835);
    }
}
