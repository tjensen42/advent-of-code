fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    lines[0].retain(|c| c.is_ascii_digit() || c.is_whitespace());
    lines[1].retain(|c| c.is_ascii_digit() || c.is_whitespace());

    let get_numbers = |s: &str| {
        s.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>()
    };
    let times = get_numbers(&lines[0]);
    let distances = get_numbers(&lines[1]);

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| (1..time).filter(|i| i * (time - i) > distance).count())
        .filter(|&x| x > 0)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 288);
    }
}
