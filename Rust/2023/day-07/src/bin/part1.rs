use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    ty: HandType,
    bet: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ty.cmp(&other.ty) != Ordering::Equal {
            return self.ty.cmp(&other.ty);
        }
        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            if a < b {
                return Ordering::Less;
            } else if a > b {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }
}

fn main() {
    let input = std::fs::read_to_string("../inputs/07-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> usize {
    let mut hands = input.lines().map(|line| parse_line(line)).collect::<Vec<_>>();
    hands.sort();
    hands.iter().enumerate().map(|(i, hand)| (i + 1) * hand.bet).sum::<usize>()
}

fn parse_line(line: &str) -> Hand {
    let (left, right) = line.split_once(" ").expect("invalid input: should have a space");

    // Convert cards to numbers
    let cards: [u8; 5] = left.chars().map(|c| {
        match c {
            x @ '2'..='9' => x as u8 - 50, // '2'..='9' -> 0..=7
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => Err("invalid input: incorrect card").unwrap(),
        }
    }).collect::<Vec<_>>().try_into().expect("invalid input: incorrect number of cards");

    // Figure out HandType
    let mut counts = HashMap::<u8, u8>::new();
    cards.iter().for_each(|c| {
        counts.insert(*c, counts.get(&c).unwrap_or(&0) + 1);
    });
    let counts = counts.values().cloned().collect::<Vec<_>>();
    let ty = match counts.len() {
        1 => HandType::FiveKind,
        2 => if counts[0] == 4 || counts[1] == 4 {
            HandType::FourKind
        } else {
            HandType::FullHouse
        },
        3 => {
            if counts[0] == 3 || counts[1] == 3 || counts[2] == 3 {
                HandType::ThreeKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::Pair,
        5 => HandType::HighCard,
        _ => unreachable!(),
    };

    let bet = right.parse().expect("incorrect input: bet is not a number");

    Hand { cards, ty, bet }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".to_string();

        assert_eq!(solve(&input), 6440)
    }
}
