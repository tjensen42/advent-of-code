fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut platform: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    tilt_platform_north(&mut platform);
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| (i, row.iter().filter(|c| **c == 'O').count()))
        .map(|(i, count)| (platform.len() - i) * count)
        .sum()
}

fn tilt_platform_north(platform: &mut Vec<Vec<char>>) {
    for row in 0..platform.len() {
        for column in 0..platform[0].len() {
            if platform[row][column] == 'O' {
                let mut tmp_row = row;
                while tmp_row > 0 && platform[tmp_row - 1][column] == '.' {
                    tmp_row -= 1;
                }
                if tmp_row != row {
                    platform[tmp_row][column] = 'O';
                    platform[row][column] = '.';
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 136);
    }
}
