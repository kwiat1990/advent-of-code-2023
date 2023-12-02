use std::collections::BTreeMap;

advent_of_code::solution!(2);

fn parse(line: &str) -> BTreeMap<&str, u32> {
    line.strip_prefix("Game")
        .unwrap()
        .split(": ")
        .skip(1)
        .flat_map(|cubes| {
            cubes
                .split(&[';', ','])
                .map(|cube| {
                    let (count, color) = cube.trim().split_once(' ').unwrap();
                    (color, count.parse::<u32>().unwrap())
                })
                .fold(BTreeMap::new(), |mut map, (key, value)| {
                    map.entry(key)
                        .and_modify(|curr| *curr = u32::max(*curr, value))
                        .or_insert(value);
                    map
                })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let limits: BTreeMap<&str, u32> = BTreeMap::from([("blue", 14), ("green", 13), ("red", 12)]);

    Some(
        input
            .lines()
            .map(parse)
            .enumerate()
            .filter(|(_, game)| !game.keys().any(|key| game.get(key) > limits.get(key)))
            .map(|(i, _)| i as u32 + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse)
            .map(|game| game.values().product::<u32>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2286));
    }
}
