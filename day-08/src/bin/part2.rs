use std::collections::BTreeMap;

use num::Integer;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let lines = input.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();
    let steps = lines[0];
    let network = network_from_input(&lines[1..]);

    network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|node| step_count_to_end_node(&network, node, steps))
        .reduce(|acc, e| acc.lcm(&e))
        .unwrap()
}

fn step_count_to_end_node(
    network: &BTreeMap<&str, (&str, &str)>,
    start_node: &str,
    steps: &str,
) -> usize {
    let mut tmp = start_node;
    let mut step_count = 0;
    for step in steps.chars().cycle() {
        tmp = next_node(network, tmp, step);
        step_count += 1;
        if tmp.ends_with('Z') {
            break;
        }
    }
    step_count
}

fn next_node<'a>(
    network: &BTreeMap<&str, (&'a str, &'a str)>,
    current_node: &str,
    step: char,
) -> &'a str {
    let nodes = network.get(current_node).unwrap();
    match step {
        'L' => nodes.0,
        'R' => nodes.1,
        _ => unreachable!(),
    }
}

fn network_from_input<'a>(lines: &'a [&'a str]) -> BTreeMap<&str, (&str, &str)> {
    let mut network = BTreeMap::new();
    for line in lines {
        let split = line
            .split(&['=', ' ', '(', ',', ')'])
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        network.insert(split[0], (split[1], split[2]));
    }
    network
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input_2_1.txt").trim();
        assert_eq!(process_input(input), 6);
    }
}
