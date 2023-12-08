use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, locations) = parse(input);

    let start = "AAA";
    let end = "ZZZ";

    let (mut destination, mut other) = locations.get_key_value(start).unwrap();

    for (dir, step_count) in directions.iter().cycle().zip(1..) {
        let new_destination = match dir {
            Direction::Left => other.left,
            Direction::Right => other.right,
        };

        (destination, other) = locations
            .get_key_value(new_destination)
            .expect("should find a location");

        if *destination == end {
            return Some(step_count);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, locations) = parse(input);

    let mut start_locations: Vec<_> = locations
        .keys()
        .filter(|l| l.ends_with('A'))
        .flat_map(|c| locations.get_key_value(*c))
        .collect();

    let end_locations: Vec<_> = locations
        .keys()
        .filter(|l| l.ends_with('Z'))
        .flat_map(|c| locations.get_key_value(*c))
        .collect();

    Some(
        start_locations
            .iter_mut()
            .map(|(mut loc, mut other)| {
                let mut step_count = 0;
                for dir in directions.iter().cycle() {
                    let new_location = match dir {
                        Direction::Left => other.left,
                        Direction::Right => other.right,
                    };

                    (loc, other) = locations
                        .get_key_value(new_location)
                        .expect("should find a location");

                    step_count += 1;

                    if end_locations.iter().any(|(end, _)| *end == loc) {
                        break;
                    }
                }
                step_count
            })
            .fold(1, lcm),
    )
}

/// Least Common Multiple https://rustp.org/number-theory/lcm/
fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

/// Greatest Common Divisor https://rustp.org/number-theory/gcd-or-hcf/
fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b),
    }
}

fn parse(input: &str) -> (Vec<Direction>, Location) {
    let mut lines = input.lines();

    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            dir => panic!("found unknown direction: {dir}"),
        })
        .collect();

    let locations: Location = lines
        .skip(1)
        .map(|line| {
            line.split(|c: char| !c.is_ascii_alphanumeric())
                .filter(|c| !c.is_empty())
                .collect::<Vec<&str>>()
        })
        .map(|c| {
            let mut c = c.iter().copied();
            (
                c.next().unwrap(),
                LocationEntry {
                    left: c.next().unwrap(),
                    right: c.next().unwrap(),
                },
            )
        })
        .collect();

    (directions, locations)
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialOrd, PartialEq)]
struct LocationEntry<'a> {
    left: &'a str,
    right: &'a str,
}
type Location<'a> = HashMap<&'a str, LocationEntry<'a>>;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
