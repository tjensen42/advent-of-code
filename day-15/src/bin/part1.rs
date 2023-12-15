fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |acc, v| (acc + v) * 17 % 256)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "HASH";
        assert_eq!(process_input(input), 52);
    }

    #[test]
    fn test_input_2() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 1320);
    }
}
