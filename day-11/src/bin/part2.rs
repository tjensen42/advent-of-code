use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input, 1000000));
}

fn process_input(input: &str, expansion_rate: usize) -> usize {
    let galaxies = galaxies_from_str(input, expansion_rate);

    galaxies
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(*x2) + y1.abs_diff(*y2))
        .sum()
}

fn galaxies_from_str(s: &str, expansion_rate: usize) -> Vec<(usize, usize)> {
    let expansion_rate = expansion_rate.saturating_sub(1);
    let mut expansion_count = 0;
    let mut galaxies = Vec::new();
    for (x, line) in s.lines().enumerate() {
        if line.chars().any(|c| c == '#') {
            for (y, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
                galaxies.push((x + expansion_count * expansion_rate, y));
            }
        } else {
            expansion_count += 1;
        }
    }

    galaxies.sort_by_key(|(_, y)| *y);
    let y_diffs = galaxies
        .windows(2)
        .map(|g| g[0].1.abs_diff(g[1].1))
        .collect::<Vec<_>>();

    let mut expansion_count = 0;
    for (galaxy, y_diff) in galaxies.iter_mut().skip(1).zip(y_diffs.iter()) {
        if *y_diff > 1 {
            expansion_count += y_diff - 1;
        }
        galaxy.1 += expansion_count * expansion_rate;
    }

    galaxies
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_2() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input, 2), 374);
    }

    #[test]
    fn test_input_10() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input, 10), 1030);
    }

    #[test]
    fn test_input_100() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input, 100), 8410);
    }
}
