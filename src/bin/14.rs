use std::collections::HashMap;
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    tilt_grid(&mut grid, Direction::North);

    Some(
        grid.iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|x| **x == 'O').count() * (row.len() - i))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen_patterns: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    for cycle in 0.. {
        let seen_cycle = *seen_patterns.entry(grid.clone()).or_insert(cycle);
        if seen_cycle < cycle {
            let modulo = cycle - seen_cycle;
            let remaining_cycles = (1_000_000_000 - cycle) % modulo;
            for _ in 0..remaining_cycles {
                tilt_full_cycle(&mut grid);
            }
            break;
        }
        tilt_full_cycle(&mut grid);
    }

    Some(
        grid.iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|x| **x == 'O').count() * (row.len() - i))
            .sum(),
    )
}

enum Direction {
    North,
    West,
    South,
    East,
}

fn tilt_grid(grid: &mut Vec<Vec<char>>, direction: Direction) {
    for _ in 0..grid.len() {
        match direction {
            Direction::North => {
                for y1 in 0..grid.len() - 1 {
                    let y2 = y1 + 1;
                    for x in 0..grid[0].len() {
                        if grid[y1][x] == '.' && grid[y2][x] == 'O' {
                            grid[y2][x] = '.';
                            grid[y1][x] = 'O';
                        }
                    }
                }
            }
            Direction::West => {
                for y in 0..grid[0].len() {
                    for x1 in 0..grid.len() - 1 {
                        let x2 = x1 + 1;
                        if grid[y][x1] == '.' && grid[y][x2] == 'O' {
                            grid[y][x2] = '.';
                            grid[y][x1] = 'O';
                        }
                    }
                }
            }
            Direction::South => {
                for y1 in (1..grid.len()).rev() {
                    let y2 = y1 - 1;
                    for x in 0..grid[0].len() {
                        if grid[y1][x] == '.' && grid[y2][x] == 'O' {
                            grid[y2][x] = '.';
                            grid[y1][x] = 'O';
                        }
                    }
                }
            }
            Direction::East => {
                for y in 0..grid[0].len() {
                    for x1 in (1..grid.len()).rev() {
                        let x2 = x1 - 1;
                        if grid[y][x1] == '.' && grid[y][x2] == 'O' {
                            grid[y][x2] = '.';
                            grid[y][x1] = 'O';
                        }
                    }
                }
            }
        }
    }
}

fn tilt_full_cycle(grid: &mut Vec<Vec<char>>) {
    tilt_grid(grid, Direction::North);
    tilt_grid(grid, Direction::West);
    tilt_grid(grid, Direction::South);
    tilt_grid(grid, Direction::East);
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
        assert_eq!(result, Some(64));
    }
}
