advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').fold(0, |mut sum, s| {
        sum += calc_sum(s);
        sum
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: Vec<Vec<Label>> = vec![vec![]; 256];

    for label in input.trim().split(',') {
        let (key, num) = label.split_once(['=', '-']).unwrap();
        let curr_box = calc_sum(key) as usize;

        match num.parse::<u32>() {
            Ok(v) => {
                match boxes[curr_box].iter().position(|x| x.name == key) {
                    Some(pos) => boxes[curr_box][pos].focal = v,
                    None => boxes[curr_box].push(Label {
                        name: key,
                        focal: v,
                    }),
                };
            }
            Err(_) => {
                if let Some(pos) = boxes[curr_box].iter().position(|x| x.name == key) {
                    boxes[curr_box].remove(pos);
                }
            }
        };
    }

    Some(calc_boxes(&boxes))
}

fn calc_sum(s: &str) -> u32 {
    s.as_bytes()
        .iter()
        .fold(0, |sum, c| (*c as u32 + sum) * 17 % 256)
}

fn calc_boxes(boxes: &[Vec<Label>]) -> u32 {
    boxes.iter().zip(1..).fold(0, |mut sum, (b, i)| {
        sum += b
            .iter()
            .zip(1..)
            .map(|(label, j)| i * j * label.focal)
            .sum::<u32>();
        sum
    })
}

#[derive(Debug, Clone, Copy)]
struct Label<'a> {
    name: &'a str,
    focal: u32,
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
        assert_eq!(result, Some(145));
    }
}
