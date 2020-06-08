// TODO handle sorting cards with wild cards
use strum::IntoEnumIterator;
use std::cmp::Ordering;

use super::card::Suit;
use super::card::Rank;
use super::card::Card;
use super::card::IsWildCard;
use super::card::fmt_cards;

fn remove_cards(a: &[Card], b: &[Card]) -> Vec<Card> {
    return a.iter()
        .filter(|card| !b.contains(card))
        .map(Card::copy)
        .collect();
}

fn remove_card(a: &[Card], b: &Card) -> Vec<Card> {
    let mut found = false;
    return a.iter()
        .filter(|card| {
            if found || *card != b {
                true
            } else {
                found = true;
                false
            }
        })
        .map(Card::copy)
        .collect();
}

fn filter_suit(cards: &[Card], suit: Suit, is_wild: &Option<IsWildCard>) -> Vec<Card> {
    return cards
        .iter()
        .filter(|card| card.suit == suit || card.is_wild(is_wild))
        .map(Card::copy)
        .collect();
}

fn find_set(cards: &[Card], n: usize, is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for rank in Rank::iter() {
        if rank != Rank::LowAce && rank != Rank::Joker {
            let filtered = cards.iter()
                .filter(|card| card.rank == rank || card.is_wild(is_wild))
                .collect::<Vec<_>>();

            if filtered.len() >= n {
                return Some(filtered.iter()
                            .take(n)
                            .map(|card_ref_ref| Card::copy(*card_ref_ref))
                            .collect());
            }
        }
    }

    return None;
}

fn make_sets_worker(cards: &[Card], sizes: &mut Vec<usize>, is_wild: &Option<IsWildCard>, result: &mut Vec<Card>) -> bool {
    if let Some(set_size) = sizes.pop() {
        if let Some(set) = find_set(cards, set_size, is_wild) {
            let next_cards = remove_cards(cards, &set);
            result.extend(set);
            return make_sets_worker(&next_cards, sizes, is_wild, result);
        }
        return false;
    } else {
        return true;
    }
}

fn make_sets(cards: &[Card], set_sizes: &Vec<usize>, is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    let mut sizes_copy = set_sizes.to_vec();
    sizes_copy.reverse();
    let mut hand = Vec::new();
    if make_sets_worker(cards, &mut sizes_copy, is_wild, &mut hand) {
        return Some(hand);
    } else {
        return None;
    }
}

fn as_quads(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![4, 1], is_wild);
}

fn as_full_house(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![3, 2], is_wild);
}

fn as_trips(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![3, 1, 1], is_wild);
}

fn as_two_pair(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![2, 2, 1], is_wild);
}

fn as_pair(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![2, 1, 1, 1], is_wild);
}

fn as_high_card(cards: &[Card], _is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort();
    sorted_cards.reverse();
    return Some(sorted_cards[0..5].to_vec());
}

fn fill_straight(cards: &[Card], is_wild:&Option<IsWildCard>, rank_ordinal: usize, result: &mut Vec<Card>) -> bool {
    if result.len() >= 5 {
        return true;
    } else {
        if let Some(card) = cards.iter()
            .filter(|card| !card.is_wild(is_wild))
            .find(|card| card.rank.is_ordinal(rank_ordinal)) {
                result.push(Card::copy(card));

                if fill_straight(cards, is_wild, rank_ordinal - 1, result) {
                    return true;
                }
                result.pop();
            }

        if let Some(wild) = cards.iter()
            .find(|card| card.is_wild(is_wild)) {
                result.push(Card::copy(wild));
                let remaining_cards = remove_card(cards, wild);
                if fill_straight(&remaining_cards, is_wild, rank_ordinal - 1, result) {
                    return true;
                }
                result.pop();
            }

        return false;
    }
}

fn as_straight(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for rank_ordinal in (Rank::Five as usize .. Rank::Ace as usize + 1).rev() {
        let mut result = Vec::with_capacity(5);
        if fill_straight(cards, is_wild, rank_ordinal, &mut result) {
            return Some(result);   
        }
    }

    return None;
}

fn as_flush(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let mut suited = filter_suit(cards, suit, is_wild);
        
        if suited.len() >= 5 {
            suited.sort();
            suited.reverse();
            return Some(suited.iter()
                        .take(5)
                        .map(|card_ref| Card::copy(card_ref))
                        .collect());
        }
    }

    return None;
}

fn as_straight_flush(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited = filter_suit(cards, suit, is_wild);
        
        if suited.len() >= 5 {
            if let Some(straight) = as_straight(&suited, is_wild) {
                return Some(straight);
            }
        }
    }
    
    return None;
}

pub trait PokerHand: std::fmt::Display {
    fn new(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Self> where Self: Sized;
    
    fn name(&self) -> &'static str;
    fn ord(&self) -> i32;
    fn cards(&self) -> &[Card];
}

macro_rules! define_hand {
    ($ordinal: literal, $symbol_struct: ident, $string: literal, $as_fn: expr) => {
        pub struct $symbol_struct(Vec<Card>);

        impl $symbol_struct {
            const ORDINAL: i32 = $ordinal;
            const NAME: &'static str = $string;
        }
        
        impl PokerHand for $symbol_struct {
            fn new(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Self> {
                if let Some(hand) = $as_fn(cards, is_wild) {
                    Some($symbol_struct(hand))
                } else {
                    None
                }
            }
            
            fn name(&self) -> &'static str { Self::NAME }
            fn ord(&self) -> i32 { Self::ORDINAL }
            fn cards(&self) -> &[Card] { &self.0 }
        }

        impl std::fmt::Display for $symbol_struct {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} -> {}", fmt_cards(self.cards()), self.name())
            }
        }
    }
}

define_hand!(0, HighCard, "High Card", as_high_card);
define_hand!(1, OnePair, "Pair", as_pair);
define_hand!(2, TwoPair, "Two Pair", as_two_pair);
define_hand!(3, Triplets, "Three of a Kind", as_trips);
define_hand!(4, Straight, "Straight", as_straight);
define_hand!(5, Flush, "Flush", as_flush);
define_hand!(6, FullHouse, "Full House", as_full_house);
define_hand!(7, Quads, "Four of a Kind", as_quads);
define_hand!(8, StraightFlush, "Straight Flush", as_straight_flush);

macro_rules! try_make_hand {
    ($struct_type: ident, $cards: ident, $is_wild: ident) => {
        if let Some(hand) = $struct_type::new($cards, $is_wild) {
            return Box::new(hand);
        }
    }
}

pub fn make_poker_hand(cards: &[Card], is_wild: &Option<IsWildCard>) -> Box<dyn PokerHand> {
    try_make_hand!(StraightFlush, cards, is_wild);
    try_make_hand!(Quads, cards, is_wild);
    try_make_hand!(FullHouse, cards, is_wild);
    try_make_hand!(Flush, cards, is_wild);
    try_make_hand!(Straight, cards, is_wild);
    try_make_hand!(Triplets, cards, is_wild);
    try_make_hand!(TwoPair, cards, is_wild);
    try_make_hand!(OnePair, cards, is_wild);
    try_make_hand!(HighCard, cards, is_wild);
    unreachable!();
}

impl<'a> PartialEq for dyn PokerHand + 'a {
    fn eq(&self, other: &dyn PokerHand) -> bool {
        return self.ord() == other.ord();
    }
}

impl<'a> Eq for dyn PokerHand + 'a {}

impl<'a> Ord for dyn PokerHand + 'a {
    // TODO handle wild cards
    fn cmp(&self, other: &dyn PokerHand) -> Ordering {
        self.ord().cmp(&other.ord())
    }
}

impl<'a> PartialOrd for dyn PokerHand + 'a {
    fn partial_cmp(&self, other: &dyn PokerHand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardVector;
    use Rank::*;
    use Suit::*;

    fn _parse_hand(card_string: &str, is_wild: &Option<IsWildCard>) -> Box<dyn PokerHand> {
        let card_vector = &CardVector::parse(card_string);
        return make_poker_hand(card_vector, is_wild);
    }

    fn parse_hand(card_string: &str) -> Box<dyn PokerHand> {
        _parse_hand(card_string, &Some(Card::is_joker))
    }

    fn parse_hand_suicide_king(card_string: &str) -> Box<dyn PokerHand> {
        _parse_hand(card_string, &Some(Card::is_suicide_king))
    }

    #[test]
    fn test_remove_card() {
        let cards0 = CardVector::parse("Kc ?? ??");
        assert_eq!(cards0.len(), 3);

        let cards1 = remove_card(&cards0, &Rank::Joker.of(Suit::Joker));
        assert_eq!(cards1.len(), 2);

        let cards2 = remove_card(&cards1, &Rank::Joker.of(Suit::Joker));
        assert_eq!(cards2.len(), 1);
    }

    #[test]
    fn test_println() {
        let poker_hand = parse_hand("Ac As Ad Ah Jd");
        println!("{}", poker_hand);
    }

    #[test]
    fn test_straight_flush() {
        assert_eq!(parse_hand("Ac Kc Qc Tc Jc").ord(), StraightFlush::ORDINAL);
    }

    #[test]
    fn test_straight_flush_with_jokers() {
        assert_eq!(parse_hand("Ac Kc Qc ?? Jc").ord(), StraightFlush::ORDINAL);
        assert_eq!(parse_hand("Ac Kc ?? ?? Jc").ord(), StraightFlush::ORDINAL);
    }

    #[test]
    fn test_quads() {
        let poker_hand = parse_hand("Ac As Ad Ah Jd");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.ord(), Quads::ORDINAL);

        assert_eq!(cards[0].rank, Ace);
        assert_eq!(cards[1].rank, Ace);
        assert_eq!(cards[2].rank, Ace);
        assert_eq!(cards[3].rank, Ace);
        assert_eq!(cards[4].rank, Jack);
    }

    #[test]
    fn test_full_house() {
        let poker_hand = parse_hand("Ac As Ad Jh Jd");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.ord(), FullHouse::ORDINAL);
        
        assert_eq!(cards[0].rank, Ace);
        assert_eq!(cards[1].rank, Ace);
        assert_eq!(cards[2].rank, Ace);
        assert_eq!(cards[3].rank, Jack);
        assert_eq!(cards[4].rank, Jack);
    }

    #[test]
    fn test_one_joker() {
        assert_eq!(parse_hand("Ac As Ad ?? Jd").ord(), Quads::ORDINAL);
        assert_eq!(parse_hand("Ac As ?? Jc Jd").ord(), FullHouse::ORDINAL);
        assert_eq!(parse_hand("Ac As ?? Jc Td").ord(), Triplets::ORDINAL);
        assert_eq!(parse_hand("Ac ?? Jc Td 7c").ord(), OnePair::ORDINAL);
    }

    #[test]
    fn test_two_joker() {
        assert_eq!(parse_hand("Ac As ?? ?? Jd").ord(), Quads::ORDINAL);
        assert_eq!(parse_hand("Ac ?? ?? Td 7c").ord(), Triplets::ORDINAL);
    }

    #[test]
    fn test_flush() {
        let poker_hand = parse_hand("Ac Kc 7c Tc Jc");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.ord(), Flush::ORDINAL);
        
        assert_eq!(cards[0].rank, Ace);
        assert_eq!(cards[1].rank, King);
        assert_eq!(cards[2].rank, Jack);
        assert_eq!(cards[3].rank, Ten);
        assert_eq!(cards[4].rank, Seven);
    }
    
    #[test]
    fn test_flush_with_jokers() {
        assert_eq!(parse_hand("Ac Kc 7c Tc ??").ord(), Flush::ORDINAL);
        assert_eq!(parse_hand("Ac Kc ?? Jc 7c").ord(), Flush::ORDINAL);
    }
    
    #[test]
    fn test_straight() {
        assert_eq!(parse_hand("Ac Kc Qc Ts Jd").ord(), Straight::ORDINAL);
    }

    #[test]
    fn test_low_straight() {
        assert_eq!(parse_hand("Ac 5c 4s 3s 2d").ord(), Straight::ORDINAL);
        assert_eq!(parse_hand("5c 4s 3s 2d ??").ord(), Straight::ORDINAL);
        assert_eq!(parse_hand("?? 4s 3s 2d Ac").ord(), Straight::ORDINAL);
    }

    #[test]
    fn test_straight_with_jokers() {
        assert_eq!(parse_hand("Ac Kc Qc Jd ??").ord(), Straight::ORDINAL);
        assert_eq!(parse_hand("Ac Kc ?? ?? Ts").ord(), Straight::ORDINAL);
    }

    #[test]
    fn test_triplets() {
        assert_eq!(parse_hand("Ac Ah As Ts Jd").ord(), Triplets::ORDINAL);
    }

    #[test]
    fn test_two_pair() {
        assert_eq!(parse_hand("Ac Ah Qs Qd Jd").ord(), TwoPair::ORDINAL);
    }

    #[test]
    fn test_pair() {
        assert_eq!(parse_hand("Ac Ah Qs Td Jd").ord(), OnePair::ORDINAL);
    }

    #[test]
    fn test_high_card() {
        let poker_hand = parse_hand("Ac Jh 9s 7d 5d");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.ord(), HighCard::ORDINAL);

        assert_eq!(cards[0], Ace.of(Clubs));
        assert_eq!(cards[1], Jack.of(Hearts));
        assert_eq!(cards[2], Nine.of(Spades));
        assert_eq!(cards[3], Seven.of(Diamonds));
        assert_eq!(cards[4], Five.of(Diamonds));
    }

    #[test]
    fn test_suicide_king() {
        assert_eq!(parse_hand_suicide_king("9c Kh 7c 6c 5c").ord(), StraightFlush::ORDINAL);
        assert_eq!(parse_hand_suicide_king("Ac Kh As Ad 7d").ord(), Quads::ORDINAL);
        assert_eq!(parse_hand_suicide_king("Kc Kh 7c 6c 5c").ord(), Flush::ORDINAL);
        assert_eq!(parse_hand_suicide_king("9c Kh 7s 6d 5d").ord(), Straight::ORDINAL);
        assert_eq!(parse_hand_suicide_king("Ac Kh As 7c 7d").ord(), FullHouse::ORDINAL);
        assert_eq!(parse_hand_suicide_king("Ac Kh As 6c 7d").ord(), Triplets::ORDINAL);
        assert_eq!(parse_hand_suicide_king("Ac Kh 5c 6c 7d").ord(), OnePair::ORDINAL);
    }

    #[test]
    fn test_two_pair_edge_case() {
        let poker_hand = parse_hand("K♣ K♦ 5♠ 5♣ 3♥ 3♣");
        assert_eq!(poker_hand.ord(), TwoPair::ORDINAL);

        println!("two_pair_edge_case: {}", poker_hand);
        assert_eq!(poker_hand.cards().len(), 5);
    }
}
