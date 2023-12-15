fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let splitted = line.split_whitespace().collect::<Vec<_>>();
            let springs = splitted[0];
            let conditions = splitted[1]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();
            count(springs, &conditions)
        })
        .sum()
}

fn count(springs: &str, conditions: &[usize]) -> usize {
    if springs.is_empty() {
        return if conditions.is_empty() { 1 } else { 0 };
    }

    if conditions.is_empty() {
        return if springs.contains('#') { 0 } else { 1 };
    }

    let mut result = 0;
    if springs.starts_with(['.', '?']) {
        result += count(&springs[1..], conditions);
    }

    if springs.starts_with(['#', '?']) {
        let cond = conditions[0];
        if cond <= springs.len()
            && !springs[..cond].contains('.')
            && (cond == springs.len() || springs.chars().nth(cond) != Some('#'))
        {
            if cond == springs.len() {
                result += count(&springs[cond..], &conditions[1..]);
            } else {
                result += count(&springs[cond + 1..], &conditions[1..]);
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 21);
    }
}
