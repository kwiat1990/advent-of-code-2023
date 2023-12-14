advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let patterns = parse(input);
    let mut row = 0;

    for pattern in patterns {
        for (i, pattern) in pattern.windows(2).enumerate() {
            println!("{i} {:?} {:?}", pattern[0], pattern[1]);
            if pattern[0] == pattern[1] {
                row = i + 1;
                break;
            }
        }
    }

    println!("{row}");

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    let mut patterns: Vec<Vec<&str>> = Vec::new();
    let mut curr = 0;

    for line in input.lines() {
        if line.is_empty() {
            curr += 1;
            continue;
        }

        match patterns.get_mut(curr) {
            Some(v) => v.push(line),
            None => patterns.push(vec![line]),
        }
    }

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
