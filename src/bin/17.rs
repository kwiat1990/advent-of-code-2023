advent_of_code::solution!(17);

use pathfinding::prelude::{dijkstra, Matrix};

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 1, 3))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 4, 10))
}

fn solve(input: &str, min: u32, max: u32) -> u32 {
    let grid = Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(|x| x.to_digit(10).unwrap())),
    )
    .unwrap();

    let goal = (grid.rows - 1, grid.columns - 1);

    dijkstra(
        &((0, 0), (0, 0), 0),
        |&(pos, dir, count)| neighbours(&grid, pos, dir, count, max, min),
        |&(pos, _, count)| pos == goal && count >= min,
    )
    .unwrap()
    .1
}

type Moves = Vec<(((usize, usize), (isize, isize), u32), u32)>;

fn neighbours(
    grid: &Matrix<u32>,
    pos: (usize, usize),
    dir: (isize, isize),
    count: u32,
    max_count: u32,
    min_count: u32,
) -> Moves {
    let mut moves = Vec::with_capacity(3);
    let mut update_path = |dir, new_count| {
        moves.extend(
            &grid
                .move_in_direction(pos, dir)
                .map(|new_pos| ((new_pos, dir, new_count), grid[new_pos])),
        );
    };
    if count < max_count {
        update_path((dir.0, dir.1), count + 1);
    }
    if count >= min_count {
        update_path((-dir.1, -dir.0), 1);
        update_path((dir.1, dir.0), 1);
    } else if count == 0 {
        update_path((1, 0), 1);
        update_path((0, 1), 1);
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
