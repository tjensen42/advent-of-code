use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let cards: Vec<_> = input.lines().map(|l| l.parse::<Card>().unwrap()).collect();
    let mut card_counter = vec![1; cards.len()];

    for card in cards {
        if card.matching_numbers != 0 {
            for i in 0..card.matching_numbers {
                card_counter[card.index + i] += card_counter[card.index - 1];
            }
        }
    }
    card_counter.iter().sum()
}

#[derive(Debug, PartialEq, Clone)]
struct Card {
    index: usize,
    winning_numbers: Vec<usize>,
    chosen_numbers: Vec<usize>,
    matching_numbers: usize,
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
        let matching_numbers = chosen_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();
        Ok(Self {
            index,
            winning_numbers,
            chosen_numbers,
            matching_numbers,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 30);
    }
}
