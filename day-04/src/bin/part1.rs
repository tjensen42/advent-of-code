use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let sum = input
        .lines()
        .map(|l| l.parse::<Card>().unwrap())
        .map(|c| c.worthiness())
        .sum();
    sum
}

#[derive(Debug, PartialEq)]
struct Card {
    index: usize,
    winning_numbers: Vec<usize>,
    chosen_numbers: Vec<usize>,
}

impl Card {
    fn worthiness(&self) -> usize {
        let mut worthiness = 0;
        for chosen_number in &self.chosen_numbers {
            if self.winning_numbers.contains(chosen_number) {
                if worthiness == 0 {
                    worthiness = 1;
                } else {
                    worthiness *= 2;
                }
            }
        }
        worthiness
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Card").expect("Invalid card");
        let card_split = s.split(&[':', '|']).collect::<Vec<_>>();
        let index = card_split[0]
            .trim()
            .parse::<usize>()
            .expect("Invalid game index");
        let winning_numbers = card_split[1]
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("Invalid number"))
            .collect::<Vec<_>>();
        let chosen_numbers = card_split[2]
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("Invalid number"))
            .collect::<Vec<_>>();
        Ok(Self {
            index,
            winning_numbers,
            chosen_numbers,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 13);
    }
}
