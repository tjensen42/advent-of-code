use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let splitted = input.split("\n\n").collect::<Vec<_>>();
    let workflows = splitted[0]
        .lines()
        .map(Workflow::from)
        .map(|w| (w.id, w))
        .collect::<HashMap<_, _>>();
    let parts = splitted[1].lines().map(Part::from).collect::<Vec<_>>();

    let results = parts
        .iter()
        .map(|p| {
            let mut result = "in";
            while result != "A" && result != "R" {
                let tmp_workflow = workflows.get(result).unwrap();
                result = tmp_workflow
                    .rules
                    .iter()
                    .filter_map(|r| r.part_target(p))
                    .next()
                    .unwrap_or(tmp_workflow.default_rule);
            }
            result
        })
        .collect::<Vec<_>>();

    parts
        .iter()
        .zip(results.iter())
        .filter(|(_, r)| **r == "A")
        .map(|(p, _)| p.x + p.m + p.a + p.s)
        .sum()
}

#[derive(Debug)]
struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
    default_rule: &'a str,
}

#[derive(Debug)]
struct Rule<'a> {
    part_category: char,
    condition: core::cmp::Ordering,
    value: usize,
    target: &'a str,
}

impl Rule<'_> {
    fn part_target(&self, part: &Part) -> Option<&str> {
        let part_value = match self.part_category {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("Invalid part category"),
        };

        match part_value.cmp(&self.value) == self.condition {
            true => Some(self.target),
            false => None,
        }
    }
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
        let part_category = splitted[0].chars().nth(0).unwrap();
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

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let mut ratings = s
            .split(|c| !char::is_ascii_digit(&c))
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap());
        Self {
            x: ratings.next().unwrap(),
            m: ratings.next().unwrap(),
            a: ratings.next().unwrap(),
            s: ratings.next().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 19114);
    }
}
