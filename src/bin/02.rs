use std::collections::BTreeMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let limits: BTreeMap<&str, u32> = BTreeMap::from([("blue", 14), ("green", 13), ("red", 12)]);

    Some(
        input
            .lines()
            .map(Game::parse)
            .filter(|game| {
                game.blue <= *limits.get("blue").unwrap()
                    && game.red <= *limits.get("red").unwrap()
                    && game.green <= *limits.get("green").unwrap()
            })
            .map(|game| game.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Game::parse)
            .map(|game| game.red * game.green * game.blue)
            .sum(),
    )
}

#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
    id: u32,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (game, sets) = line.split_once(':').unwrap();

        let id = game
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut game = Self {
            id,
            red: 0,
            blue: 0,
            green: 0,
        };

        for set in sets.split(';').map(str::trim) {
            for cubes in set.split(',').map(str::trim) {
                let (count, color) = cubes.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();

                match color {
                    "blue" => game.blue = u32::max(game.blue, count),
                    "green" => game.green = u32::max(game.green, count),
                    "red" => game.red = u32::max(game.red, count),
                    _ => panic!("unexpected color"),
                }
            }
        }

        game
    }
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
