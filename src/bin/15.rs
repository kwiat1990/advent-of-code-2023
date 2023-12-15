advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').fold(0, |mut sum, s| {
        sum += calc_sum(s);
        sum
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn calc_sum(s: &str) -> u32 {
    s.as_bytes()
        .iter()
        .fold(0, |sum, c| (*c as u32 + sum) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
