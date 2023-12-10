advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| extrapolate_history(parse(line)))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut row = parse(line);
                row.reverse();
                extrapolate_history(row)
            })
            .sum(),
    )
}

fn extrapolate_history(row: Vec<i64>) -> i64 {
    let mut sum = 0;
    let mut current_row = row.clone();

    while current_row.iter().min() != current_row.iter().max() {
        current_row = current_row.windows(2).map(|arr| arr[1] - arr[0]).collect();
        sum += current_row.last().unwrap();
    }

    sum + row.iter().last().unwrap()
}

fn parse(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .filter_map(|ch| ch.parse::<i64>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
