use std::collections::HashSet;

fn main() {
    let input = include_str!("../test_input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let map = map_with_border(input);
    let beam = Beam::new((1, 1), Direction::Right);

    let mut cache = HashSet::new();
    process_beam(beam, &map, &mut cache);

    cache
        .iter()
        .map(|beam| beam.pos)
        .collect::<HashSet<_>>()
        .len()
}

fn map_with_border(input: &str) -> Vec<Vec<char>> {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert!(map.iter().all(|line| line.len() == map[0].len()));

    map.insert(0, vec![' '; map[0].len()]);
    map.push(vec![' '; map[0].len()]);

    map.iter_mut().for_each(|line| line.insert(0, ' '));
    map.iter_mut().for_each(|line| line.push(' '));

    map
}

fn process_beam(mut beam: Beam, map: &Vec<Vec<char>>, cache: &mut HashSet<Beam>) {
    let tile = map[beam.pos.1][beam.pos.0];

    if tile == ' ' || !cache.insert(beam) {
        return;
    }

    match beam.dir {
        Direction::Up | Direction::Down => match tile {
            '-' => process_beam(beam.split(), map, cache),
            '/' => beam.go_right(),
            '\\' => beam.go_left(),
            _ => beam.go_forward(),
        },
        Direction::Left | Direction::Right => match tile {
            '|' => process_beam(beam.split(), map, cache),
            '/' => beam.go_left(),
            '\\' => beam.go_right(),
            _ => beam.go_forward(),
        },
    }

    process_beam(beam, map, cache);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    pos: (usize, usize),
    dir: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Beam {
    fn new(pos: (usize, usize), dir: Direction) -> Self {
        Self { pos, dir }
    }

    #[inline]
    fn go_left(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };
        self.go_forward();
    }

    #[inline]
    fn go_right(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
        self.go_forward();
    }

    #[inline]
    fn split(&mut self) -> Self {
        let mut new_beam = *self;
        new_beam.go_left();
        self.go_right();
        new_beam
    }

    #[inline]
    fn go_forward(&mut self) {
        match self.dir {
            Direction::Up => self.pos.1 -= 1,
            Direction::Down => self.pos.1 += 1,
            Direction::Left => self.pos.0 -= 1,
            Direction::Right => self.pos.0 += 1,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 46);
    }
}
