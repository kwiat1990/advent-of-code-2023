use std::cmp::Ordering;
use std::collections::BTreeMap;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut bids: Vec<(Hand, u32)> = input.lines().map(|line| parse(line, false)).collect();

    bids.sort();

    Some(
        bids.iter()
            .enumerate()
            .map(|(i, (_, bid))| (i as u32 + 1) * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bids: Vec<(Hand, u32)> = input.lines().map(|line| parse(line, true)).collect();

    bids.sort();

    Some(
        bids.iter()
            .enumerate()
            .map(|(i, (_, bid))| (i as u32 + 1) * bid)
            .sum(),
    )
}

fn parse(line: &str, has_joker: bool) -> (Hand, u32) {
    let (cards, bid) = line.split_once(char::is_whitespace).unwrap();
    let hand = cards
        .chars()
        .map(|c| Card::from((c, has_joker)))
        .collect::<Hand>();
    let bid = bid.parse::<u32>().expect("bid must be a number");

    (hand, bid)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    Joker = 1,
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

impl From<(char, bool)> for Card {
    fn from(val: (char, bool)) -> Self {
        match val.0 {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => match val.1 {
                true => Self::Joker,
                false => Self::Jack,
            },
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("unrecognizable card"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeKind = 4,
    FullHouse = 5,
    FourKind = 6,
    FiveKind = 7,
}

impl From<&Hand> for HandType {
    fn from(hand: &Hand) -> Self {
        let mut cards = hand.get_cards().iter().fold(BTreeMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let mut jokers = 0;

        if let Some(j) = cards.get(&Card::Joker) {
            jokers = *j;
        }

        cards.remove(&Card::Joker);
        let mut values: Vec<u32> = cards.values().copied().collect();
        values.sort();

        let (first, second) = (
            values.last().unwrap_or(&0) + jokers,
            values.get(values.len() - 2).unwrap_or(&0),
        );

        match (first, second) {
            (5, _) => HandType::FiveKind,
            (4, _) => HandType::FourKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeKind,
            (2, 2) => HandType::TwoPairs,
            (2, _) => HandType::OnePair,
            (_, _) => HandType::HighCard,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hand(Vec<Card>);

impl Hand {
    fn get_cards(&self) -> &Vec<Card> {
        &self.0
    }
}

impl Iterator for Hand {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().copied().next()
    }
}

impl FromIterator<Card> for Hand {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        let mut output: Vec<Card> = Vec::new();

        for i in iter {
            output.push(i);
        }

        Self(output)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = HandType::from(self);
        let b = HandType::from(other);

        match (a, b) {
            (a, b) if a > b => Ordering::Greater,
            (a, b) if a < b => Ordering::Less,
            (a, b) if a == b => self.0.partial_cmp(&other.0).unwrap(),
            (_, _) => panic!("cannot compare Hands!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
