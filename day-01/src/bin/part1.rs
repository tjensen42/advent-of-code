fn main() {
    let input = include_str!("../input.txt");
    let sum: u32 = input.lines().map(|line| calibration_value(line)).sum();
    println!("Sum: {sum}");
}

fn calibration_value(line: &str) -> u32 {
    let first_digit = line
        .chars()
        .find(|x| char::is_numeric(*x))
        .and_then(|x| char::to_digit(x, 10));
    let last_digit = line
        .chars()
        .rfind(|x| char::is_numeric(*x))
        .and_then(|x| char::to_digit(x, 10));

    match (first_digit, last_digit) {
        (Some(a), Some(b)) => a * 10 + b,
        (_, _) => panic!("No numbers found in line"),
    }
}

#[cfg(test)]
mod test {
    use crate::calibration_value;

    #[test]
    fn test_input() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let sum: u32 = input.lines().map(|line| calibration_value(line)).sum();
        assert_eq!(sum, 142);
    }
}
