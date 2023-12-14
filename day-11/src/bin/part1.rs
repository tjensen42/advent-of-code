use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let universe = universe_from_str(input);
    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(y, galaxy)| if *galaxy > 0 { Some((x, y)) } else { None })
        })
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(*x2) + y1.abs_diff(*y2))
        .sum()
}

fn universe_from_str(s: &str) -> Vec<Vec<usize>> {
    let mut counter = 1;
    let mut universe = Vec::new();
    for (x, line) in s.lines().enumerate() {
        universe.push(vec![0; line.chars().count()]);
        for (y, char) in line.chars().enumerate() {
            if char == '#' {
                universe[x][y] = counter;
                counter += 1;
            }
        }
        assert_eq!(universe[x].len(), universe[0].len());
    }

    let mut inserted = 0;
    for y in 0..universe.len() {
        let new_y = y + inserted;
        if universe[new_y].iter().all(|&e| e == 0) {
            universe.insert(new_y + 1, universe[new_y].clone());
            inserted += 1;
        }
    }

    let mut inserted = 0;
    for x in 0..universe[0].len() {
        let new_x = x + inserted;
        if universe.iter().all(|row| row[new_x] == 0) {
            universe.iter_mut().for_each(|row| row.insert(new_x + 1, 0));
            inserted += 1;
        }
    }

    universe
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 374);
    }
}
