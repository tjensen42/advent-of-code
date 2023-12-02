use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt").trim();
    let sum: usize = input
        .lines()
        .map(|line| Game::from_str(line).expect("Invalid input!"))
        .map(|game| game.minimum_set_of_cubes().power_of_set())
        .sum();
    println!("Sum: {}", sum);
}

#[derive(Debug, Default)]
struct Game {
    index: usize,
    sets: Vec<Set>,
}

impl Game {
    pub fn minimum_set_of_cubes(&self) -> Set {
        let r = self.sets.iter().map(|s| s.red).max();
        let g = self.sets.iter().map(|s| s.green).max();
        let b = self.sets.iter().map(|s| s.blue).max();
        Set::new(r.unwrap_or(0), g.unwrap_or(0), b.unwrap_or(0))
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(':');

        let mut game = Game::default();
        game.index = s
            .next()
            .and_then(|game| {
                game.split_whitespace()
                    .last()
                    .and_then(|i| i.parse::<usize>().ok())
            })
            .expect("Invalid input!");

        let sets = s.next().map(|s| s.split(';')).expect("Invalid input!");
        game.sets = sets
            .map(|set| Set::from_str(set).expect("Invalid input!"))
            .collect();
        Ok(game)
    }
}

#[derive(Debug, Default)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    pub fn new(red: usize, green: usize, blue: usize) -> Set {
        Set { red, green, blue }
    }

    pub fn power_of_set(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();
        let cubes = s.split(',');
        for cube in cubes {
            let mut split = cube.split_ascii_whitespace();
            let count: usize = split
                .next()
                .and_then(|x| x.parse().ok())
                .expect("Invalid input!");
            let color = split.next().expect("Invalid input!");

            match (color, count) {
                ("red", x) => set.red += x,
                ("green", x) => set.green += x,
                ("blue", x) => set.blue += x,
                _ => panic!("Invalid input!"),
            }
        }
        Ok(set)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        let sum: usize = input
            .lines()
            .map(|line| Game::from_str(line).expect("Invalid input!"))
            .map(|game| game.minimum_set_of_cubes().power_of_set())
            .sum();
        assert_eq!(sum, 2286);
    }
}
