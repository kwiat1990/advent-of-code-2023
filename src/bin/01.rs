use std::collections::BTreeMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for n in input.lines() {
        let nums: Vec<char> = n.chars().filter(|c: &char| c.is_numeric()).collect();

        sum += format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
            .parse::<u32>()
            .unwrap();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mapping: BTreeMap<&str, &str> = BTreeMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    for line in input.lines() {
        let mut tree: BTreeMap<usize, u32> = BTreeMap::new();

        for word in mapping.keys().chain(mapping.values()) {
            line.match_indices(word).for_each(|(x, _)| {
                let num = mapping.get(word).unwrap_or(word).parse::<u32>().unwrap();
                tree.insert(x, num);
            });
        }

        let values: Vec<u32> = tree.values().copied().collect();

        sum += values.first().unwrap() * 10 + values.last().unwrap();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
