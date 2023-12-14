advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for _ in 0..rows.len() {
        for (y1, y2) in (0..rows.len()).zip(1..rows[0].len()) {
            if y1 == y2 {
                break;
            }
            for x in 0..rows[0].len() {
                if rows[y1][x] == '.' && rows[y2][x] == 'O' {
                    rows[y2][x] = '.';
                    rows[y1][x] = 'O';
                }
            }
        }
    }

    Some(rows.iter().rev().zip(1..).fold(0, |mut sum, (row, i)| {
        sum += row.iter().filter(|x| **x == 'O').count() as u32 * i;
        sum
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
