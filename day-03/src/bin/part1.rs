fn main() {
    let input = include_str!("../input.txt").trim();
    let part_nums = find_part_numbers(input);
    println!("Sum: {}", part_nums.iter().sum::<usize>());
}

fn find_part_numbers(input: &str) -> Vec<usize> {
    let schematic: Vec<Vec<_>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let filter_matrix = create_filter_matrix(&schematic);

    let mut part_nums: Vec<usize> = Vec::new();
    let mut tmp = Vec::new();
    for (i, row) in schematic.iter().enumerate() {
        tmp.clear();
        let mut is_valid_part_num = false;
        for (j, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                tmp.push(*col);
                if filter_matrix[i][j] {
                    is_valid_part_num = true;
                }
            } else if !tmp.is_empty() {
                if is_valid_part_num {
                    part_nums.push(String::from_utf8(tmp.clone()).unwrap().parse().unwrap());
                }
                tmp.clear();
                is_valid_part_num = false;
            }
        }
        if !tmp.is_empty() && is_valid_part_num {
            part_nums.push(String::from_utf8(tmp.clone()).unwrap().parse().unwrap());
        }
    }
    part_nums
}

fn create_filter_matrix(schematic: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut filter_matrix = vec![vec![false; schematic[0].len()]; schematic.len()];
    for (i, row) in schematic.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            for x in -1..=1 {
                for y in -1..=1 {
                    if (x == 0 && y == 0) || (i == 0 && x == -1) || (j == 0 && y == -1) {
                        continue;
                    }
                    let tmp_i = i.saturating_add_signed(x);
                    let tmp_j = j.saturating_add_signed(y);
                    if tmp_i < schematic.len()
                        && tmp_j < schematic[i].len()
                        && schematic[tmp_i][tmp_j] != b'.'
                        && !schematic[tmp_i][tmp_j].is_ascii_digit()
                    {
                        filter_matrix[i][j] = true;
                    }
                }
            }
        }
    }
    filter_matrix
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        let part_nums = find_part_numbers(input);
        assert_eq!(part_nums.iter().sum::<usize>(), 4361);
    }
}
