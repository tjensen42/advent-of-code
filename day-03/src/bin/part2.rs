#![allow(dead_code)]

fn main() {
    let input = include_str!("../input.txt").trim();
    let gear_ratios = gear_ratios(input);
    println!("Sum: {}", gear_ratios.iter().sum::<usize>());
}

fn gear_ratios(input: &str) -> Vec<usize> {
    let schematic: Vec<Vec<_>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut part_nums = Vec::new();
    let mut tmp = Vec::new();
    let mut tmp_next_gear = None;
    for row in 0..schematic.len() {
        for column in 0..schematic[row].len() {
            if schematic[row][column].is_ascii_digit() {
                tmp.push(schematic[row][column]);
                if tmp_next_gear.is_none() {
                    tmp_next_gear = next_to_gear(&schematic, row, column);
                }
            } else if !tmp.is_empty() {
                if let Some(gear_index) = tmp_next_gear {
                    part_nums.push((gear_index, num_from_bytes_unchecked(&tmp)));
                }
                tmp_next_gear = None;
                tmp.clear();
            }
        }
        if !tmp.is_empty() {
            if let Some(gear_index) = tmp_next_gear {
                part_nums.push((gear_index, num_from_bytes_unchecked(&tmp)));
            }
            tmp_next_gear = None;
            tmp.clear();
        }
    }

    part_nums.sort_by(|a, b| a.0.cmp(&b.0));
    let ratios = part_nums
        .windows(2)
        .filter_map(|x| {
            if x[0].0 == x[1].0 {
                Some(x[0].1 * x[1].1)
            } else {
                None
            }
        })
        .collect();

    ratios
}

fn next_to_gear(schematic: &Vec<Vec<u8>>, row: usize, column: usize) -> Option<(usize, usize)> {
    for x in -1..=1 {
        for y in -1..=1 {
            if (x == 0 && y == 0) || (row == 0 && x == -1) || (column == 0 && y == -1) {
                continue;
            }
            let tmp_row = row.saturating_add_signed(x);
            let tmp_column = column.saturating_add_signed(y);
            if tmp_row < schematic.len()
                && tmp_column < schematic[0].len()
                && schematic[tmp_row][tmp_column] == b'*'
            {
                return Some((tmp_row, tmp_column));
            }
        }
    }
    None
}

fn num_from_bytes_unchecked(bytes: &[u8]) -> usize {
    let mut num = 0;
    for byte in bytes {
        num = num * 10 + (*byte - b'0') as usize;
    }
    num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        let gear_ratios = gear_ratios(input);
        assert_eq!(gear_ratios.iter().sum::<usize>(), 467835);
    }
}
