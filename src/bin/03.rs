use std::collections::BTreeMap;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let text: String = input.lines().map(str::trim).collect();
    let width = input.lines().nth(1).unwrap().len();

    let mut nums = Number::parse(&text);
    let symbols = Symbol::parse(&text, width);

    for num in nums.iter_mut() {
        symbols.values().for_each(|c| {
            c.iter()
                .any(|p| p.iter().any(|x| (num.start_pos..=num.end_pos).contains(x)))
                .then(|| {
                    num.found = true;
                });
        });
    }

    Some(
        nums.iter()
            .filter(|num| num.found)
            .map(|num| num.num)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let text: String = input.lines().map(str::trim).collect();
    let width = input.lines().nth(1).unwrap().len();

    let nums = Number::parse(&text);
    let symbols = Symbol::parse(&text, width);

    let gear = symbols.get("*").expect("there is a gear symbol");

    let mut sum = 0;
    for row in gear.iter() {
        let o = nums
            .iter()
            .filter(|n| (n.start_pos..=n.end_pos).any(|pos| row.contains(&pos)));

        if o.clone().count() == 2 {
            sum += o.map(|n| n.num).product::<u32>();
        }
    }

    Some(sum)
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct Number {
    num: u32,
    start_pos: usize,
    end_pos: usize,
    found: bool,
}

impl Number {
    fn parse(input: &str) -> Vec<Self> {
        input
            .chars()
            .enumerate()
            .filter(|(_, ch)| ch.is_ascii_digit())
            .map(|(i, ch)| (i, ch))
            .fold(Vec::new(), |mut v: Vec<Number>, (i, ch)| {
                let num = ch.to_digit(10).unwrap();

                if !v.is_empty() && v.last().unwrap().end_pos == i - 1 {
                    let l = v.last_mut().unwrap();
                    l.num = l.num * 10 + num;
                    l.end_pos = i;
                } else {
                    v.push(Number {
                        num,
                        start_pos: i,
                        end_pos: i,
                        found: false,
                    })
                }
                v
            })
    }
}

struct Symbol {}

impl Symbol {
    fn parse(input: &str, width: usize) -> BTreeMap<&str, Vec<Vec<usize>>> {
        let mut symbols: BTreeMap<&str, Vec<Vec<usize>>> = BTreeMap::new();

        for (index, symbol) in input.match_indices(|c: char| !c.is_ascii_digit() && c != '.') {
            let mut positions: Vec<usize> = Vec::new();

            if index.checked_sub(width).is_some() {
                let top = index - width;
                let top_right = (index - width) + 1;
                let top_left = (index - width) - 1;

                positions.extend([top_left, top, top_right]);
            }

            let prev = index - 1;
            let next = index + 1;
            positions.extend([index, next, prev]);

            if index + width > 0 {
                let bottom = index + width;
                let bottom_right = (index + width) + 1;
                let bottom_left = (index + width) - 1;

                positions.extend([bottom_left, bottom, bottom_right]);
            }

            symbols
                .entry(symbol)
                .and_modify(|curr| curr.push(positions.clone()))
                .or_insert(vec![positions]);
        }

        symbols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6672));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(588643));
    }
}
