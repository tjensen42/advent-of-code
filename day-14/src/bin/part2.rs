use std::hash::{Hash, Hasher};

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut platform: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut cache = Vec::new();
    for i in 0..1_000_000_000 {
        platform_spin_cycle(&mut platform);
        let platform_hash = hash(&platform);
        if cache.contains(&platform_hash) {
            let start_of_cycle = cache.iter().position(|p| *p == platform_hash).unwrap();
            let cycle_len = i - start_of_cycle;
            let remaining = (1_000_000_000 - i - 1) % cycle_len;
            for _ in 0..remaining {
                platform_spin_cycle(&mut platform);
            }
            break;
        } else {
            cache.push(platform_hash);
        }
    }

    platform
        .iter()
        .enumerate()
        .map(|(i, row)| (i, row.iter().filter(|c| **c == 'O').count()))
        .map(|(i, count)| (platform.len() - i) * count)
        .sum()
}

fn hash(platform: &Vec<Vec<char>>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    platform.hash(&mut hasher);
    hasher.finish()
}

fn platform_spin_cycle(platform: &mut [Vec<char>]) {
    tilt_platform_north(platform);
    tilt_platform_west(platform);
    tilt_platform_south(platform);
    tilt_platform_east(platform);
}

#[inline(always)]
fn tilt_platform_north(platform: &mut [Vec<char>]) {
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

#[inline(always)]
fn tilt_platform_south(platform: &mut [Vec<char>]) {
    for row in (0..platform.len()).rev() {
        for column in 0..platform[0].len() {
            if platform[row][column] == 'O' {
                let mut tmp_row = row;
                while tmp_row < platform.len() - 1 && platform[tmp_row + 1][column] == '.' {
                    tmp_row += 1;
                }
                if tmp_row != row {
                    platform[tmp_row][column] = 'O';
                    platform[row][column] = '.';
                }
            }
        }
    }
}

#[inline(always)]
fn tilt_platform_west(platform: &mut [Vec<char>]) {
    platform.iter_mut().for_each(|row| {
        for column in 0..row.len() {
            if row[column] == 'O' {
                let mut tmp_column = column;
                while tmp_column > 0 && row[tmp_column - 1] == '.' {
                    tmp_column -= 1;
                }
                if tmp_column != column {
                    row[tmp_column] = 'O';
                    row[column] = '.';
                }
            }
        }
    })
}

#[inline(always)]
fn tilt_platform_east(platform: &mut [Vec<char>]) {
    platform.iter_mut().for_each(|row| {
        for column in (0..row.len()).rev() {
            if row[column] == 'O' {
                let mut tmp_column = column;
                while tmp_column < row.len() - 1 && row[tmp_column + 1] == '.' {
                    tmp_column += 1;
                }
                if tmp_column != column {
                    row[tmp_column] = 'O';
                    row[column] = '.';
                }
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 64);
    }
}
