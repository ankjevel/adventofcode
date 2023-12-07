use std::{
    cmp::Ordering::{self, *},
    collections::{BTreeMap, BTreeSet},
};

use crate::Hand;

#[derive(Debug, Clone, Eq, PartialEq)]
enum HandOrder {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPair = 4,
    OnePair = 5,
    HighCard = 6,
}

use HandOrder::*;

impl HandOrder {
    fn char_to_card(c: char, joker_rule: bool) -> u8 {
        let joker_or_knight = if joker_rule { 1 } else { 11 };
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => joker_or_knight,
            'T' => 10,
            n => n.to_digit(10).unwrap() as u8,
        }
    }

    fn from_chars(hand: &Hand, joker_rule: bool) -> Vec<u8> {
        hand.to_owned()
            .into_iter()
            .map(|c| Self::char_to_card(c, joker_rule))
            .collect::<Vec<_>>()
    }

    fn to_map(hand: Vec<u8>) -> BTreeMap<u8, usize> {
        hand.into_iter().fold(BTreeMap::new(), |mut map, card| {
            match map.get_mut(&card) {
                Some(v) => *v += 1,
                _ => {
                    map.insert(card, 1);
                }
            };
            map
        })
    }

    pub fn from_hand(hand: &Hand) -> Self {
        let unique = BTreeSet::from_iter(hand.to_owned().into_iter());
        if unique.len() == 1 {
            return FiveOfAKind;
        }

        let map = Self::to_map(
            hand.to_owned()
                .into_iter()
                .map(|c| Self::char_to_card(c, false))
                .collect(),
        );

        if map.values().any(|v| v == &4) {
            return FourOfAKind;
        }

        if map.values().any(|v| v == &3) && map.values().any(|v| v == &2) {
            return FullHouse;
        }

        if map.values().any(|v| v == &3) {
            return ThreeOfAKind;
        }

        if map.values().filter(|v| v == &&2).collect::<Vec<_>>().len() == 2 {
            return TwoPair;
        }

        if map.values().any(|v| v == &2) {
            return OnePair;
        }

        HighCard
    }

    pub fn from_hand_with_joker_rule(hand: &Hand) -> Self {
        let mut jokers = 0;
        let map = Self::to_map(
            hand.to_owned()
                .into_iter()
                .filter(|c| {
                    if c == &'J' {
                        jokers += 1;
                        return false;
                    }
                    true
                })
                .map(|c| Self::char_to_card(c, true))
                .collect(),
        );

        if jokers == 5 || map.values().any(|v| (*v as u8 + jokers) == 5) {
            return FiveOfAKind;
        }

        if map.values().any(|v| (*v as u8 + jokers) >= 4) {
            return FourOfAKind;
        }

        let threes = map.values().filter(|v| v == &&3).collect::<Vec<_>>();
        let twos = map.values().filter(|v| v == &&2).collect::<Vec<_>>();

        if !threes.is_empty() && !twos.is_empty() {
            return FullHouse;
        }

        if threes.is_empty() && twos.len() == 2 && jokers == 1 {
            return FullHouse;
        }

        if jokers == 2 {
            return ThreeOfAKind;
        }

        if !threes.is_empty() || (!twos.is_empty() && jokers >= 1) {
            return ThreeOfAKind;
        }

        if map.values().filter(|v| v == &&2).collect::<Vec<_>>().len() == 2 {
            return TwoPair;
        }

        if map.values().any(|v| (*v as u8 + jokers) >= 2) {
            return OnePair;
        }

        HighCard
    }
}

fn rank(a_rank: HandOrder, b_rank: HandOrder, a: &Hand, b: &Hand, joker_rule: bool) -> Ordering {
    let to_vec = HandOrder::from_chars;
    match (b_rank as u8).cmp(&(a_rank as u8)) {
        Equal => {
            let a = to_vec(a, joker_rule);
            let b = to_vec(b, joker_rule);
            for (i, c) in a.into_iter().enumerate() {
                match c.cmp(b.get(i).unwrap()) {
                    Equal => continue,
                    order => return order,
                }
            }
            Equal
        }
        order => order,
    }
}

pub fn sort_hand(a: &Hand, b: &Hand) -> Ordering {
    let parse = HandOrder::from_hand;
    rank(parse(a), parse(b), a, b, false)
}

pub fn sort_hand_with_joker_rule(a: &Hand, b: &Hand) -> Ordering {
    let parse = HandOrder::from_hand_with_joker_rule;
    rank(parse(a), parse(b), a, b, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(input: &str) -> Vec<char> {
        input.chars().collect()
    }

    #[test]
    fn it_can_parse_hands() {
        let f = HandOrder::from_hand;
        let j = HandOrder::from_hand_with_joker_rule;
        assert_eq!(f(&c("22222")), FiveOfAKind);
        assert_eq!(j(&c("22222")), FiveOfAKind);
        assert_eq!(f(&c("22223")), FourOfAKind);
        assert_eq!(j(&c("22223")), FourOfAKind);
        assert_eq!(f(&c("22233")), FullHouse);
        assert_eq!(j(&c("22233")), FullHouse);
        assert_eq!(f(&c("22234")), ThreeOfAKind);
        assert_eq!(j(&c("22234")), ThreeOfAKind);
        assert_eq!(f(&c("22334")), TwoPair);
        assert_eq!(j(&c("22334")), TwoPair);
        assert_eq!(f(&c("22345")), OnePair);
        assert_eq!(j(&c("22345")), OnePair);
        assert_eq!(f(&c("23457")), HighCard);
        assert_eq!(j(&c("23457")), HighCard);
    }

    #[test]
    fn it_can_parse_hands_with_joker_rule() {
        let f = HandOrder::from_hand_with_joker_rule;
        assert_eq!(f(&c("JJJJJ")), FiveOfAKind);
        assert_eq!(f(&c("22JJJ")), FiveOfAKind);
        assert_eq!(f(&c("222JJ")), FiveOfAKind);
        assert_eq!(f(&c("2222J")), FiveOfAKind);
        assert_eq!(f(&c("222J3")), FourOfAKind);
        assert_eq!(f(&c("22JJ3")), FourOfAKind);
        assert_eq!(f(&c("2JJJ3")), FourOfAKind);
        assert_eq!(f(&c("22J33")), FullHouse);
        assert_eq!(f(&c("22J34")), ThreeOfAKind);
        assert_eq!(f(&c("2345J")), OnePair);
        assert_eq!(f(&c("23457")), HighCard);
    }

    #[test]
    fn it_orders_equal_hands() {
        let f = sort_hand;
        assert_eq!(f(&c("22222"), &c("22222")), Equal);
        assert_eq!(f(&c("22223"), &c("22224")), Less);
        assert_eq!(f(&c("22224"), &c("22223")), Greater);
        assert_eq!(f(&c("22233"), &c("22244")), Less);
        assert_eq!(f(&c("22244"), &c("22233")), Greater);
        assert_eq!(f(&c("22333"), &c("22444")), Less);
        assert_eq!(f(&c("22444"), &c("22333")), Greater);
        assert_eq!(f(&c("23333"), &c("24444")), Less);
        assert_eq!(f(&c("24444"), &c("23333")), Greater);
    }

    #[test]
    fn it_orders_equal_hands_using_joker_rule() {
        let f = sort_hand_with_joker_rule;
        assert_eq!(f(&c("J3333"), &c("22222")), Less);
        assert_eq!(f(&c("J2222"), &c("J2222")), Equal);
        assert_eq!(f(&c("222J4"), &c("22223")), Less);
        assert_eq!(f(&c("222J3"), &c("222J3")), Equal);
        assert_eq!(f(&c("2J444"), &c("23333")), Less);
        assert_eq!(f(&c("2J444"), &c("2J444")), Equal);
        assert_eq!(f(&c("JKKK2"), &c("QQQQ2")), Less);
    }

    #[test]
    fn it_orders() {
        let map = vec![
            c("22222"), // FiveOfAKind
            c("23333"), // FourOfAKind
            c("22333"), // FullHouse
            c("22234"), // ThreeOfAKind
            c("22334"), // TwoPair
            c("22345"), // OnePair
            c("23457"), // HighCard
        ];

        let inputs = map.clone();

        for (i, input) in inputs.into_iter().enumerate() {
            for (j, compare) in map.clone().into_iter().enumerate() {
                assert_eq!(sort_hand(&input, &compare), j.cmp(&i));
                assert_eq!(sort_hand_with_joker_rule(&input, &compare), j.cmp(&i));
            }
        }
    }
}
