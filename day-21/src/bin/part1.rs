use std::collections::HashSet;

use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input, 64));
}

fn process_input(input: &str, steps: usize) -> usize {
    let tiles = input.chars().filter(|c| *c != '\n').collect::<Vec<_>>();

    let columns = input.find('\n').unwrap();
    let mut grid = Grid::from_vec(tiles, columns);

    grid.insert_row(0, vec!['#'; grid.cols()]);
    grid.insert_row(grid.rows(), vec!['#'; grid.cols()]);
    grid.insert_col(0, vec!['#'; grid.rows()]);
    grid.insert_col(grid.cols(), vec!['#'; grid.rows()]);

    let start_pos = grid
        .indexed_iter()
        .find_map(|(p, t)| if *t == 'S' { Some(p) } else { None })
        .unwrap();

    let mut positions = HashSet::new();
    positions.insert(start_pos);
    for _ in 0..steps {
        positions = do_step(&grid, &positions);
    }

    positions.len()
}

fn do_step(grid: &Grid<char>, positions: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut new_positions = HashSet::new();
    for p in positions {
        for new_pos in [
            (p.0 + 1, p.1),
            (p.0 - 1, p.1),
            (p.0, p.1 + 1),
            (p.0, p.1 - 1),
        ] {
            if grid[new_pos] != '#' {
                new_positions.insert(new_pos);
            }
        }
    }
    new_positions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input, 6), 16);
    }
}
