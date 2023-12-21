use std::{collections::HashMap, ops::Range};

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

type Part = [Range<usize>; 4];

fn process_input(input: &str) -> usize {
    let splitted = input.split("\n\n").collect::<Vec<_>>();
    let workflows = splitted[0]
        .lines()
        .map(Workflow::from)
        .map(|w| (w.id, w))
        .collect::<HashMap<_, _>>();

    let part: Part = [(1..4001), (1..4001), (1..4001), (1..4001)];
    process_part(&part, &workflows, "in")
}

fn process_part(part: &Part, workflows: &HashMap<&str, Workflow>, next_workflow: &str) -> usize {
    if part.iter().any(|r| r.is_empty()) || next_workflow == "R" {
        return 0;
    } else if next_workflow == "A" {
        return part[0].len() * part[1].len() * part[2].len() * part[3].len();
    }

    let next_workflow = workflows.get(next_workflow).unwrap();

    let mut count = 0;
    let mut tmp_part = part.clone();
    for rule in next_workflow.rules.iter() {
        let (part1, part2) = part_split_on_rule(&tmp_part, rule);
        count += process_part(&part1, workflows, rule.target);
        tmp_part = part2;
    }
    count += process_part(&tmp_part, workflows, next_workflow.default_rule);

    count
}

fn part_split_on_rule(part: &Part, rule: &Rule) -> (Part, Part) {
    let mut curr_part = part.clone();
    let curr_part_range = &mut curr_part[rule.part_category as usize];

    let mut next_part = part.clone();
    let next_part_range = &mut next_part[rule.part_category as usize];

    match rule.condition {
        core::cmp::Ordering::Greater => {
            let value = rule.value + 1;
            *curr_part_range = value..curr_part_range.end;
            *next_part_range = next_part_range.start..value;
        }
        core::cmp::Ordering::Less => {
            *curr_part_range = curr_part_range.start..rule.value;
            *next_part_range = rule.value..next_part_range.end;
        }
        _ => panic!("Invalid condition"),
    }

    (curr_part, next_part)
}

#[derive(Debug)]
struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
    default_rule: &'a str,
}

#[derive(Debug)]
struct Rule<'a> {
    part_category: PartCategory,
    condition: core::cmp::Ordering,
    value: usize,
    target: &'a str,
}

#[derive(Debug, Clone, Copy)]
enum PartCategory {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(s: &'a str) -> Self {
        let splitted = s.split('{').collect::<Vec<_>>();
        let id = splitted[0];
        let mut rules = splitted[1]
            .trim_end_matches('}')
            .split(',')
            .collect::<Vec<_>>();
        let default_rule = rules.pop().unwrap();
        let rules = rules.into_iter().map(Rule::from).collect::<Vec<_>>();
        Self {
            id,
            rules,
            default_rule,
        }
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        let splitted = value.split(':').collect::<Vec<_>>();
        let part_category = match splitted[0].chars().nth(0).unwrap() {
            'x' => PartCategory::X,
            'm' => PartCategory::M,
            'a' => PartCategory::A,
            's' => PartCategory::S,
            _ => panic!("Invalid category"),
        };
        let condition = match splitted[0].chars().nth(1).unwrap() {
            '>' => core::cmp::Ordering::Greater,
            '<' => core::cmp::Ordering::Less,
            _ => panic!("Invalid condition"),
        };
        let value = splitted[0][2..].parse::<usize>().unwrap();
        Self {
            part_category,
            condition,
            value,
            target: splitted[1],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 167409079868000);
    }
}
