use core::hash::Hash;
use core::hash::Hasher;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Hand {
    FiveOfAKind(Card, Card, Card, Card, Card),
    FourOfAKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    ThreeOfAKind(Card, Card, Card, Card, Card),
    TwoPairs(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card, Card),
    Distinct(Card, Card, Card, Card, Card),
}

impl Hand {
    fn value(&self) -> usize {
        match self {
            Hand::FiveOfAKind(_, _, _, _, _) => 7,
            Hand::FourOfAKind(_, _, _, _, _) => 6,
            Hand::FullHouse(_, _, _, _, _) => 5,
            Hand::ThreeOfAKind(_, _, _, _, _) => 4,
            Hand::TwoPairs(_, _, _, _, _) => 3,
            Hand::OnePair(_, _, _, _, _) => 2,
            Hand::Distinct(_, _, _, _, _) => 1,
        }
    }

    pub fn from(hand: &str) -> Result<Hand, String> {
        let cards = Card::from_hand(hand)?;
        if let Ok(hand) = Hand::from_cards(cards) {
            return Ok(hand);
        }

        Err("Invalid hand".to_string())
    }

    fn iter(&self) -> HandIterator {
        HandIterator::new(self.clone())
    }

    fn from_cards(cards: Vec<Card>) -> Result<Hand, String> {
        let map = cards
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, c| {
                let count = acc.entry(c).or_insert(0);
                *count += 1;
                acc
            });

        let mut values: Vec<usize> = map.values().copied().collect();
        values.sort();

        match values.as_slice() {
            [1, 1, 1, 1, 1] => Ok(joker_haha(
                map,
                Hand::Distinct(cards[0], cards[1], cards[2], cards[3], cards[4]),
            )),
            [1, 1, 1, 2] => Ok(joker_haha(
                map,
                Hand::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4]),
            )),
            [1, 2, 2] => Ok(joker_haha(
                map,
                Hand::TwoPairs(cards[0], cards[1], cards[2], cards[3], cards[4]),
            )),
            [1, 1, 3] => Ok(joker_haha(
                map,
                Hand::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            )),
            [2, 3] => Ok(joker_haha(
                map,
                Hand::FullHouse(cards[0], cards[1], cards[2], cards[3], cards[4]),
            )),
            [1, 4] => Ok(joker_haha(
                map,
                Hand::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            )),
            // no need to calculate joker
            [5] => Ok(Hand::FiveOfAKind(
                cards[0], cards[1], cards[2], cards[3], cards[4],
            )),
            _ => Err("Invalid hand".to_string()),
        }
    }
}

fn joker_haha(cards: HashMap<&Card, usize>, hand: Hand) -> Hand {
    let count = cards.get(&Card::Joker).unwrap_or(&0_usize);
    if *count == 0 {
        return hand;
    }
    match hand {
        Hand::Distinct(c1, c2, c3, c4, c5) => match count {
            // joker can only have one possible value
            // so in this case joker can make a one pair
            1 => Hand::OnePair(c1, c2, c3, c4, c5),
            _ => panic!("Invalid hand"),
        },
        Hand::OnePair(c1, c2, c3, c4, c5) => match count {
            // if joker is _not_ in the pair, the count should be 1
            // so there is one pair like A, A, J and other two different cards
            // in this case joker can make a three of a kind
            1 => Hand::ThreeOfAKind(c1, c2, c3, c4, c5),
            // if joker is in the pair, the count should be 2
            // so there is one pair like J, J and three different cards
            // in this case the joker can make a three of a kind with another random card
            2 => Hand::ThreeOfAKind(c1, c2, c3, c4, c5),
            _ => panic!("Invalid hand"),
        },
        Hand::TwoPairs(c1, c2, c3, c4, c5) => match count {
            // if joker is _not_ in the pairs, the count should be 1
            // so there are two pairs like A, A, Q, Q, and the joker
            // in this case joker can make a full house
            1 => Hand::FullHouse(c1, c2, c3, c4, c5),
            // if joker is in one of the pairs, the count should be 2
            // so there are two pairs like A, A, J, J and another different card
            // in this case joker can make a four of a kind
            2 => Hand::FourOfAKind(c1, c2, c3, c4, c5),
            _ => panic!("Invalid hand"),
        },
        Hand::ThreeOfAKind(c1, c2, c3, c4, c5) => match count {
            // if joker is _not_ in the three of a kind, the count should be 1
            // so there is a three of a kind like A, A, A, J and another different card
            // in this case joker can make a four of a kind
            1 => Hand::FourOfAKind(c1, c2, c3, c4, c5),
            // if joker is in the three of a kind, the count should be 3
            // so there is a three of a kind like J, J, J and two different cards
            // in this case joker can make a four of a kind
            3 => Hand::FourOfAKind(c1, c2, c3, c4, c5),
            _ => panic!("Invalid hand"),
        },
        Hand::FullHouse(c1, c2, c3, c4, c5) => match count {
            // if joker is in the full house as the lesser pair, the count should be 2
            // so there is a full house like A, A, A, J, J
            // in this case joker can make a five of a kind
            2 => Hand::FiveOfAKind(c1, c2, c3, c4, c5),
            // if joker is in the full house as the bigger pair, the count should be 2
            // so there is a full house like A, A, J, J, J
            // in this case joker can make a five of a kind
            3 => Hand::FiveOfAKind(c1, c2, c3, c4, c5),
            _ => panic!("Invalid hand"),
        },
        Hand::FourOfAKind(c1, c2, c3, c4, c5) => match count {
            // if joker is _not_ in the four of a kind, the count should be 1
            // so there is a four of a kind like A, A, A, A, J
            // in this case joker can make a five of a kind
            1 => Hand::FiveOfAKind(c1, c2, c3, c4, c5),
            // if joker is in the four of a kind, the count should be 4
            // so there is a four of a kind like J, J, J, J and another different card
            // in this case joker can make a five of a kind
            4 => Hand::FiveOfAKind(c1, c2, c3, c4, c5),
            _ => panic!("Invalid hand"),
        },
        // there are 5 jokers, no need to change
        Hand::FiveOfAKind(c1, c2, c3, c4, c5) => Hand::FiveOfAKind(c1, c2, c3, c4, c5),
    }
}

struct HandIterator {
    hand: Hand,
    current: usize,
}

impl HandIterator {
    fn new(hand: Hand) -> HandIterator {
        HandIterator { hand, current: 0 }
    }
}

impl Iterator for HandIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        let card = match self.hand {
            Hand::FiveOfAKind(c1, c2, c3, c4, c5) => match self.current {
                0 => Some(c1),
                1 => Some(c2),
                2 => Some(c3),
                3 => Some(c4),
                4 => Some(c5),
                _ => None,
            },
            Hand::FourOfAKind(c1, c2, c3, c4, c5) => match self.current {
                0 => Some(c1),
                1 => Some(c2),
                2 => Some(c3),
                3 => Some(c4),
                4 => Some(c5),
                _ => None,
            },
            Hand::FullHouse(c1, c2, c3, c4, c5) => match self.current {
                0 => Some(c1),
                1 => Some(c2),
                2 => Some(c3),
                3 => Some(c4),
                4 => Some(c5),
                _ => None,
            },
            Hand::ThreeOfAKind(c1, c2, c3, c4, c5) => match self.current {
                0 => Some(c1),
                1 => Some(c2),
                2 => Some(c3),
                3 => Some(c4),
                4 => Some(c5),
                _ => None,
            },
            Hand::TwoPairs(c1, c2, c3, c4, c5) => match self.current {
                0 => Some(c1),
                1 => Some(c2),
                2 => Some(c3),
                3 => Some(c4),
                4 => Some(c5),
                _ => None,
            },
            Hand::OnePair(c1, c2, c3, c4, c5) => match self.current {
                0 => Some(c1),
                1 => Some(c2),
                2 => Some(c3),
                3 => Some(c4),
                4 => Some(c5),
                _ => None,
            },
            Hand::Distinct(c1, c2, c3, c4, c5) => match self.current {
                0 => Some(c1),
                1 => Some(c2),
                2 => Some(c3),
                3 => Some(c4),
                4 => Some(c5),
                _ => None,
            },
        };
        self.current += 1;
        card
    }
}

impl Hash for Hand {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value().hash(state);
        self.iter().for_each(|c| c.hash(state));
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.value() == other.value() {
            return true;
        }
        self.iter().eq(other.iter())
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.value().cmp(&other.value());
        match ord {
            std::cmp::Ordering::Equal => self.iter().cmp(other.iter()),
            _ => ord,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
pub enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn value(&self) -> usize {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::Joker => 1,
        }
    }

    fn from(c: char) -> Result<Card, String> {
        match c {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            'J' => Ok(Card::Joker),
            _ => Err("Invalid card".to_string()),
        }
    }

    fn from_hand(hand: &str) -> Result<Vec<Card>, String> {
        let mut cards: Vec<Card> = Vec::new();
        for c in hand.chars() {
            cards.push(Card::from(c)?);
        }
        Ok(cards)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}
