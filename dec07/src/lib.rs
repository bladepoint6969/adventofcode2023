use std::{cmp::Ordering, collections::HashMap, fmt::Debug};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum WildCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for WildCard {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Joker,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Not a card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Not a card"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

trait HandVal {
    fn hand_value(&self) -> HandType;
}

#[derive(Debug)]
struct Hand<T> {
    hand: Vec<T>,
    bid: usize,
}

impl HandVal for Hand<WildCard> {
    fn hand_value(&self) -> HandType {
        let jokers = self
            .hand
            .iter()
            .filter(|&card| card == &WildCard::Joker)
            .count();

        let mut card_map = HashMap::new();

        self.hand
            .iter()
            .filter(|&card| card != &WildCard::Joker)
            .for_each(|card| {
                let entry = card_map.entry(*card).or_insert(0);
                *entry += 1;
            });
        let mut best_hand = HandType::HighCard;

        card_map
            .values()
            .for_each(|count| match (count, best_hand) {
                (5, _) => best_hand = HandType::FiveOfAKind,
                (4, _) => best_hand = HandType::FourOfAKind,
                (3, HandType::Pair) => best_hand = HandType::FullHouse,
                (3, HandType::HighCard) => best_hand = HandType::ThreeOfAKind,
                (2, HandType::ThreeOfAKind) => best_hand = HandType::FullHouse,
                (2, HandType::Pair) => best_hand = HandType::TwoPair,
                (2, HandType::HighCard) => best_hand = HandType::Pair,
                (_, _) => {}
            });

        best_hand = match (jokers, best_hand) {
            (4..=5, _) => HandType::FiveOfAKind,
            (3, HandType::Pair) => HandType::FiveOfAKind,
            (3, HandType::HighCard) => HandType::FourOfAKind,
            (2, HandType::ThreeOfAKind) => HandType::FiveOfAKind,
            (2, HandType::Pair) => HandType::FourOfAKind,
            (2, HandType::HighCard) => HandType::ThreeOfAKind,
            (1, HandType::FourOfAKind) => HandType::FiveOfAKind,
            (1, HandType::ThreeOfAKind) => HandType::FourOfAKind,
            (1, HandType::TwoPair) => HandType::FullHouse,
            (1, HandType::Pair) => HandType::ThreeOfAKind,
            (1, HandType::HighCard) => HandType::Pair,
            (_, hand) => hand,
        };

        best_hand
    }
}

impl HandVal for Hand<Card> {
    fn hand_value(&self) -> HandType {
        let mut card_map = HashMap::new();

        self.hand.iter().for_each(|card| {
            let entry = card_map.entry(*card).or_insert(0);
            *entry += 1;
        });
        let mut best_hand = HandType::HighCard;

        card_map
            .values()
            .for_each(|count| match (count, best_hand) {
                (5, _) => best_hand = HandType::FiveOfAKind,
                (4, _) => best_hand = HandType::FourOfAKind,
                (3, HandType::Pair) => best_hand = HandType::FullHouse,
                (3, HandType::HighCard) => best_hand = HandType::ThreeOfAKind,
                (2, HandType::ThreeOfAKind) => best_hand = HandType::FullHouse,
                (2, HandType::Pair) => best_hand = HandType::TwoPair,
                (2, HandType::HighCard) => best_hand = HandType::Pair,
                (_, _) => {}
            });

        best_hand
    }
}

impl<T> Hand<T>
where
    T: From<char>,
{
    fn new(input: &str) -> Self {
        let mut split = input.split(' ');
        let hand = split.next().unwrap().chars().map(T::from).collect();
        let bid = split.next().unwrap().parse().unwrap();

        Self { hand, bid }
    }
}

impl<T> PartialEq for Hand<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let hands = self.hand.iter().zip(other.hand.iter());

        for (this, that) in hands {
            if this != that {
                return false;
            }
        }
        true
    }
}

impl<T> Eq for Hand<T> where Hand<T>: PartialEq {}

impl<T> PartialOrd for Hand<T>
where
    Hand<T>: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Hand<T>
where
    Hand<T>: HandVal,
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let type_cmp = self.hand_value().cmp(&other.hand_value());
        if type_cmp != Ordering::Equal {
            return type_cmp;
        }

        let zipped = self.hand.iter().zip(other.hand.iter());

        for (this, that) in zipped {
            let cmp = this.cmp(that);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        Ordering::Equal
    }
}

pub fn part1(input: &str) -> usize {
    part_x::<Card>(input)
}

pub fn part2(input: &str) -> usize {
    part_x::<WildCard>(input)
}

#[inline(always)]
fn part_x<T: From<char>>(input: &str) -> usize
where
    T: From<char> + Ord + Debug,
    Hand<T>: HandVal,
{
    let mut hands: Vec<_> = input.lines().map(Hand::<T>::new).collect();
    hands.sort();

    hands
        .windows(2)
        .for_each(|pair| assert!(pair[0] <= pair[1]));

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            println!("{hand:?} - {:?}", hand.hand_value());
            (idx + 1) * hand.bid
        })
        .sum();

    println!("{winnings}");

    winnings
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Card, Hand, HandType, HandVal};

    #[test]
    fn test_card_order() {
        assert!(Card::Ace > Card::Five);
        assert!(Card::Two < Card::Seven);
        assert_eq!(Card::Eight, Card::Eight);
    }

    #[test]
    fn test_hand_order() {
        assert!(HandType::FiveOfAKind > HandType::FullHouse);
        assert_eq!(HandType::HighCard, HandType::HighCard);

        let hand_1: Hand<Card> = Hand::new("32T3K 765");
        let hand_2: Hand<Card> = Hand::new("T55J5 684");
        let hand_3: Hand<Card> = Hand::new("32T3K 400");
        let hand_4: Hand<Card> = Hand::new("T233K 400");

        assert_eq!(hand_1, hand_3);
        assert_ne!(hand_1, hand_2);

        assert_eq!(hand_1.hand_value(), HandType::Pair);
        assert_eq!(hand_2.hand_value(), HandType::ThreeOfAKind);

        assert!(hand_2 > hand_1);
        assert!(hand_4 > hand_1);

        let hand_5: Hand<Card> = Hand::new("33332 400");
        let hand_6: Hand<Card> = Hand::new("2AAAA 500");
        assert!(hand_5 > hand_6);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test_input.txt");
        let result = part1(input);
        assert_eq!(result, 6440)
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test_input.txt");
        let result = part2(input);
        assert_eq!(result, 5905)
    }
}
