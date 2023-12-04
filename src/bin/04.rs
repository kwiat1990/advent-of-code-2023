use std::collections::BTreeSet;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(Card::parse).map(|n| n.score).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let cards: Vec<Card> = input.lines().map(Card::parse).collect();
    let mut copies = vec![1; cards.len()];

    for card in cards {
        let i = card.id as usize - 1;

        for j in i + 1..=i + card.count {
            copies[j] += copies[i]
        }
    }

    Some(copies.into_iter().sum())
}

#[derive(Clone, Copy, Debug)]
struct Card {
    id: u32,
    count: usize,
    score: u32,
}

impl Card {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(&[':', '|']).map(str::trim);

        let id = parts
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut get_nums = || {
            parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<BTreeSet<u32>>()
        };

        let winning_nums = get_nums();
        let numbers = get_nums();
        let count = numbers.intersection(&winning_nums).count();

        Self {
            id,
            count,
            score: if count > 0 {
                u32::pow(2, count as u32 - 1)
            } else {
                count as u32
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
