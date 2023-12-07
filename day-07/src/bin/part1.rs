use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| Hand::from_str(line).unwrap())
        .collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn identify_hand_type(cards: &[Card; 5]) -> HandType {
        let mut counts = [0; 15];
        for card in cards {
            counts[*card as usize] += 1;
        }
        let mut counts: Vec<_> = counts.iter().enumerate().collect();
        counts.sort_by(|a, b| b.1.cmp(a.1));
        let (_, count) = counts[0];
        match count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counts[1].1 == &2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if counts[1].1 == &2 {
                    HandType::TwoPairs
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("Invalid count: {}", count),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<_> = s.split_whitespace().collect();
        let cards: Vec<Card> = input[0].chars().map(Card::from).collect();
        let cards: [Card; 5] = cards.try_into().unwrap();
        let bid = input[1].parse().unwrap();
        Ok(Hand {
            hand_type: Self::identify_hand_type(&cards),
            cards,
            bid,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 6440);
    }
}
