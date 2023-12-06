advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse(input);
    let mut output: Vec<u32> = Vec::new();

    for race in 0..races.first().unwrap().len() {
        let time = races[0][race];
        let record = races[1][race];
        let win_count = (1..=time).filter(|i| (time - i) * i > record).count() as u32;
        output.push(win_count);
    }

    Some(output.iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_2(input);
    let time = race[0];
    let record = race[1];

    Some((1..=time).filter(|i| (time - i) * i > record).count() as u64)
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|c| c.parse().ok())
                .collect()
        })
        .collect()
}

fn parse_2(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            line.matches(char::is_numeric).fold(0, |mut n, c| {
                let num = c.parse::<u64>().unwrap_or_default();
                if n == 0 {
                    n += num;
                } else {
                    n = n * 10 + num;
                }
                n
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
