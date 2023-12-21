use std::cmp::Ordering;
use std::collections::HashMap;
advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (workflow, parts) = parse(input);
    let mut sum = 0;

    for part in &parts {
        let mut state = Action::Workflow("in");

        while let Action::Workflow(key) = state {
            for work in workflow.get(key).unwrap() {
                state = work.action.clone();

                if let Some((name, ordering, value)) = work.condition {
                    if part.get(name).unwrap().cmp(&value) == ordering {
                        break;
                    }
                }
            }
        }

        if state == Action::Accepted {
            sum += part.values().sum::<u32>();
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> (Workflow, Vec<HashMap<&str, u32>>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, workflow) = line.split_once('{').unwrap();
            let workflow: Vec<_> = workflow.split_terminator(&['{', '}', ',']).collect();
            let chain: Vec<_> = workflow
                .iter()
                .flat_map(|w| {
                    w.split_terminator(',').map(|x| {
                        let mut split = x.rsplit(':');
                        let action = match split.next().unwrap() {
                            "A" => Action::Accepted,
                            "R" => Action::Rejected,
                            name => Action::Workflow(name),
                        };

                        let condition = match split.next() {
                            Some(c) => {
                                let mut condition = c.split_inclusive(&['>', '<']);
                                let line = condition.next().unwrap();
                                let ordering = match line.chars().last().unwrap() {
                                    '>' => Ordering::Greater,
                                    '<' => Ordering::Less,
                                    _ => Ordering::Equal,
                                };
                                let name = &line[..line.len() - 1];
                                let num = condition.next().unwrap().parse::<u32>().unwrap();

                                Some((name, ordering, num))
                            }
                            None => None,
                        };
                        WorkflowChain { action, condition }
                    })
                })
                .collect();
            (name, chain)
        })
        .collect();

    let parts = parts
        .lines()
        .map(|line| {
            line.split_terminator(&['{', '}', ','])
                .filter(|c| !c.is_empty())
                .map(|c| {
                    let (key, val) = c.split_once('=').unwrap();
                    (key, val.parse::<u32>().unwrap())
                })
                .collect()
        })
        .collect();

    (workflows, parts)
}

type Workflow<'a> = HashMap<&'a str, Vec<WorkflowChain<'a>>>;

#[derive(Debug, Clone)]
struct WorkflowChain<'a> {
    condition: Option<(&'a str, Ordering, u32)>,
    action: Action<'a>,
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
enum Action<'a> {
    Rejected,
    Accepted,
    Workflow(&'a str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
