use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt").trim();
    let bag = Set::new(12, 13, 14);
    let games: Vec<Game> = input
        .lines()
        .map(|line| Game::from_str(line).expect("Invalid input!"))
        .filter(|game| game.is_possible(&bag))
        .collect();
    // println!("Games: {:#?}", games);
    let sum: usize = games.iter().map(|g| g.index).sum();
    println!("Sum: {:?}", sum);
}

#[derive(Debug, Default)]
struct Game {
    index: usize,
    sets: Vec<Set>,
}

impl Game {
    pub fn is_possible(&self, bag: &Set) -> bool {
        self.sets
            .iter()
            .all(|set| set.red <= bag.red && set.green <= bag.green && set.blue <= bag.blue)
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

        let sets = s
            .next()
            .and_then(|s| Some(s.split(';')))
            .expect("Invalid input!");

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
        let bag = Set::new(12, 13, 14);
        let games: Vec<Game> = input
            .lines()
            .filter_map(|line| Game::from_str(line).ok())
            .filter(|game| game.is_possible(&bag))
            .collect();
        let sum: usize = games.iter().map(|g| g.index).sum();
        assert_eq!(sum, 8);
    }
}
