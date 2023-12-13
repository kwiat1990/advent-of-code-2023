advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let (points, empty_rows, empty_cols) = parse(input);

    Some(points.iter().enumerate().fold(0, |mut sum, (i, first)| {
        for second in &points[i..] {
            sum += first.distance(second)
                + first.add_empty_cols(second, &empty_cols)
                + first.add_empty_rows(second, &empty_rows);
        }
        sum
    }))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (points, empty_rows, empty_cols) = parse(input);

    Some(points.iter().enumerate().fold(0, |mut sum, (i, first)| {
        for second in &points[i..] {
            let expansion = 1_000_000 - 1;
            let added_expansion = (first.add_empty_cols(second, &empty_cols)
                + first.add_empty_rows(second, &empty_rows))
                * expansion;

            sum += first.distance(second) + added_expansion;
        }
        sum
    }))
}

fn parse(input: &str) -> (Vec<Point>, Vec<usize>, Vec<usize>) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let empty_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_, line)| !line.contains(&'#'))
        .map(|(i, _)| i)
        .collect();

    let empty_cols: Vec<usize> = (0..grid.len())
        .map(|y| (0..grid.len()).map(|x| grid[x][y]).collect::<Vec<char>>())
        .enumerate()
        .filter(|(_, line)| !line.contains(&'#'))
        .map(|(i, _)| i)
        .collect();

    let points: Vec<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| Point { y, x })
        })
        .collect();

    (points, empty_rows, empty_cols)
}

#[derive(Debug)]
struct Point {
    y: usize,
    x: usize,
}

impl Point {
    fn distance(&self, other: &Self) -> usize {
        let x = (self.x).abs_diff(other.x);
        let y = (self.y).abs_diff(other.y);
        x + y
    }

    fn add_empty_rows(&self, other: &Self, rows: &[usize]) -> usize {
        let min = usize::min(self.y, other.y);
        let max = usize::max(self.y, other.y);

        rows.iter().filter(|x| (min..=max).contains(x)).count()
    }

    fn add_empty_cols(&self, other: &Self, cols: &[usize]) -> usize {
        let min = usize::min(self.x, other.x);
        let max = usize::max(self.x, other.x);

        cols.iter().filter(|x| (min..=max).contains(x)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
