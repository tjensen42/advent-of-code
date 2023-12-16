fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|line| line.chars().collect())
                .collect::<Vec<_>>()
        })
        .map(|p| horizontal_reflection(&p).max(vertical_reflection(&p)))
        .sum()
}

fn horizontal_reflection(pattern: &Vec<Vec<char>>) -> usize {
    for row in 0..pattern.len() - 1 {
        let mut diff = row_diff(&pattern[row], &pattern[row + 1]);
        if diff == 0 || diff == 1 {
            let range_top = (0..row).rev();
            let range_bottom = (row + 2)..pattern.len();
            for (top, bottom) in range_top.zip(range_bottom) {
                diff += row_diff(&pattern[top], &pattern[bottom]);
            }
            if diff == 1 {
                return (row + 1) * 100;
            }
        }
    }
    0
}

fn vertical_reflection(pattern: &Vec<Vec<char>>) -> usize {
    let transposed = transpose(pattern);
    horizontal_reflection(&transposed) / 100
}

fn row_diff(a: &[char], b: &[char]) -> usize {
    a.iter()
        .zip(b.iter())
        .fold(0, |acc, (a, b)| if a == b { acc } else { acc + 1 })
}

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty(), "Cannot transpose an empty vector");
    let len = v[0].len();
    (0..len)
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 400);
    }
}
