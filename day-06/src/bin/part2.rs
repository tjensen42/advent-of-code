fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    lines[0].retain(|c| c.is_ascii_digit());
    lines[1].retain(|c| c.is_ascii_digit());

    let time: usize = lines[0].parse().unwrap();
    let distance: usize = lines[1].parse().unwrap();

    (1..time).filter(|i| i * (time - i) > distance).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 71503);
    }
}
