fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        let values: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        sum += next_value(&values);
    }
    sum
}

fn next_value(values: &[isize]) -> isize {
    if values.iter().all(|&v| v == 0) {
        0
    } else {
        let diffs = values.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
        values.last().unwrap() + next_value(&diffs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 114);
    }
}
