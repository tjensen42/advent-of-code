use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let lines = input.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();
    let mut steps = lines[0].chars().cycle();
    let network = network_from_input(&lines[1..]);

    let mut tmp = "AAA";
    let mut step_count = 0;
    while tmp != "ZZZ" {
        let node = network.get(tmp).unwrap();
        match steps.next() {
            Some('L') => tmp = node.0,
            Some('R') => tmp = node.1,
            _ => unreachable!(),
        }
        step_count += 1;
    }

    step_count
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
    fn test_input_1() {
        let input = include_str!("../test_input_1_1.txt").trim();
        assert_eq!(process_input(input), 2);
    }

    #[test]
    fn test_input_2() {
        let input = include_str!("../test_input_1_2.txt").trim();
        assert_eq!(process_input(input), 6);
    }
}
