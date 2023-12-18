advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let matrices = parse(input);

    Some(sum_reflections(&matrices, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrices = parse(input);

    Some(sum_reflections(&matrices, true))
}

fn sum_reflections(matrices: &[Vec<Vec<char>>], fix_smudge: bool) -> usize {
    matrices
        .iter()
        .map(|matrix| {
            find_reflections(matrix, fix_smudge) * 100
                + find_reflections(&transpose_grid(matrix), fix_smudge)
        })
        .sum()
}

fn find_reflections(matrix: &Vec<Vec<char>>, fix_smudge: bool) -> usize {
    for i in 1..matrix.len() {
        let (left, right) = &matrix.split_at(i);

        if !fix_smudge && left.iter().rev().zip(right.iter()).all(|(l, r)| l == r) {
            return i;
        }

        if fix_smudge
            && left
                .iter()
                .rev()
                .zip(right.iter())
                .flat_map(|(l, r)| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(a, b)| if a == b { 0 } else { 1 })
                })
                .sum::<usize>()
                == 1
        {
            return i;
        }
    }

    0
}

fn transpose_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| grid[row][col]).collect())
        .collect()
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|board| board.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
