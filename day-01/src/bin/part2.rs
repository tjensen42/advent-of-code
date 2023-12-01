static VALUES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("../input.txt");
    let sum: usize = input
        .lines()
        .map(|line| calibration_value_for_line(line))
        .sum();
    println!("Sum: {sum}");
}

fn calibration_value_for_line(line: &str) -> usize {
    let first = first_number(line);
    let last = last_number(line);
    first * 10 + last
}

fn first_number(line: &str) -> usize {
    let value = VALUES
        .iter()
        .enumerate()
        .filter_map(|(v, str)| line.find(*str).and_then(|pos| Some((pos, v + 1))))
        .min_by(|a, b| a.cmp(b));
    let digit = line
        .char_indices()
        .find(|(_, c)| c.is_numeric())
        .and_then(|(i, c)| Some((i, c.to_digit(10).unwrap() as usize)));
    
    match (value, digit) {
        (Some((pos1, x1)), Some((pos2, x2))) => {
            if pos1 < pos2 {
                x1
            } else {
                x2
            }
        }
        (None, Some((_, x))) | (Some((_, x)), None) => x,
        (None, None) => panic!("No numbers found in line"),
    }
}

fn last_number(line: &str) -> usize {
    let value = VALUES
        .iter()
        .enumerate()
        .filter_map(|(v, str)| line.rfind(*str).and_then(|pos| Some((pos, v + 1))))
        .max_by(|a, b| a.cmp(b));
    let digit = line
        .char_indices()
        .rfind(|(_, c)| c.is_numeric())
        .and_then(|(i, c)| Some((i, c.to_digit(10).unwrap() as usize)));

    match (value, digit) {
        (Some((pos1, x1)), Some((pos2, x2))) => {
            if pos1 > pos2 {
                x1
            } else {
                x2
            }
        }
        (None, Some((_, x))) | (Some((_, x)), None) => x,
        (None, None) => panic!("No numbers found in line"),
    }
}

#[cfg(test)]
mod test {
    use crate::calibration_value_for_line;

    #[test]
    fn test_input() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let sum: usize = input
            .lines()
            .map(|line| calibration_value_for_line(line))
            .sum();
        assert_eq!(sum, 281);
    }
}
