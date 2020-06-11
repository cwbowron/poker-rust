use strum::IntoEnumIterator;
use std::cmp::Ordering;

use super::card::Suit;
use super::card::Rank;
use super::card::Card;
use super::card::IsWildCard;
use super::card::fmt_cards;
use super::card::remove_cards;
use super::card::remove_card;

fn filter_suit<'a>(cards: &'a [Card], suit: Suit, is_wild: &Option<IsWildCard>) -> Vec<&'a Card> {
    return cards
        .iter()
        .filter(|card| card.is_wild_or_suit(suit, is_wild))
        .collect();
}

fn find_set(cards: &[Card], n: usize, is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for rank in Rank::iter() {
        if rank != Rank::LowAce && rank != Rank::Joker {
            let count = cards.iter()
                .filter(|card| card.is_wild_or_rank(rank, is_wild))
                .count();

            if count >= n {
                return Some(cards.iter()
                            .filter(|card| card.is_wild_or_rank(rank, is_wild))
                            .take(n)
                            .map(|card| card.scored_as(rank))
                            .collect());
            }
        }
    }

    return None;
}

fn make_sets(cards: &[Card], sizes: &Vec<usize>, size_index: usize, is_wild: &Option<IsWildCard>, result: &mut Vec<Card>) -> Option<Vec<Card>> {
    if size_index >= sizes.len() {
        return Some(result.to_vec());
    } else if let Some(set) = find_set(cards, sizes[size_index], is_wild) {
        let next_cards = remove_cards(cards, &set);
        result.extend(set);
        return make_sets(&next_cards, sizes, size_index + 1, is_wild, result);
    } else {
        return None;
    }
}

macro_rules! define_set_maker {
    ($fn_name: ident, $set: tt) => {
        fn $fn_name(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
            return make_sets(cards, &vec!$set, 0, is_wild, &mut Vec::new());
        }
    }
}

define_set_maker!(as_quads, [4, 1]);
define_set_maker!(as_full_house, [3, 2]);
define_set_maker!(as_trips, [3, 1, 1]);
define_set_maker!(as_two_pair, [2, 2, 1]);
define_set_maker!(as_pair, [2, 1, 1, 1]);

fn top_five(mut cards: Vec<Card>) -> Vec<Card> {
    cards.sort();
    cards.reverse();
    cards.truncate(5);
    return cards;
}

fn as_high_card(cards: &[Card], _is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return Some(top_five(cards.to_vec()))
}

fn fill_straight(cards: &[Card], is_wild:&Option<IsWildCard>, rank_ordinal: usize, result: &mut Vec<Card>) -> bool {
    if result.len() >= 5 {
        return true;
    } else {
        if let Some(card) = cards.iter()
            .filter(|card| !card.is_wild(is_wild))
            .find(|card| card.rank.is_ordinal(rank_ordinal)) {
                result.push(card.clone());
                if fill_straight(cards, is_wild, rank_ordinal - 1, result) {
                    return true;
                }
                result.pop();
            }

        if let Some(wild) = cards.iter()
            .find(|card| card.is_wild(is_wild)) {
                let rank = Rank::for_ordinal(rank_ordinal);
                result.push(wild.scored_as(rank));
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

fn contains_scoring_rank(cards: &[Card], rank: Rank) -> bool {
    return cards.iter()
        .find(|card| card.scoring_rank == rank)
        .is_some();
}

fn build_flush(partition: (Vec<&Card>, Vec<&Card>)) -> Option<Vec<Card>> {
    if (partition.0.len() + partition.1.len() >= 5) {
        let mut r = Vec::new();
        
        for n in partition.1 {
            r.push(n.clone());
        }
        
        for w in partition.0 {
            for rank in Rank::iter() {
                if !contains_scoring_rank(&r, rank) {
                    r.push(w.scored_as(rank));
                    break;
                }
            }
        }
        
        return Some(top_five(r));
    } else {
        return None;
    }
}

fn as_flush(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let option = build_flush(cards.iter()
                                 .filter(|card| card.is_wild_or_suit(suit, is_wild))
                                 .partition(|card| card.is_wild(is_wild)));

        if option.is_some() {
            return option;
        }
    }

    return None;
}

fn as_straight_flush(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited = filter_suit(cards, suit, is_wild);
        
        if suited.len() >= 5 {
            let suited_cards = suited.iter()
                .map(|foo :&&Card| (*foo).clone())
                .collect::<Vec<_>>();
                
            let option = as_straight(&suited_cards, is_wild);
            if option.is_some() {
                return option;
            }
        }
    }
    
    return None;
}

pub trait PokerHand {
    fn new(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Self> where Self: Sized;
    
    fn score_hand(hand_rank: i32, cards: &[Card]) -> i32 where Self: Sized {
        return cards.iter()
            .fold(hand_rank, |acc, card| acc * 16 + (card.scoring_rank as i32));
    }

    fn name(&self) -> &'static str;
    fn ord(&self) -> i32;
    fn cards(&self) -> &[Card];
    fn score(&self) -> i32;
}

macro_rules! define_hand {
    ($ordinal: literal, $symbol_struct: ident, $string: literal, $as_fn: expr) => {
        pub struct $symbol_struct(Vec<Card>, i32);

        impl $symbol_struct {
            const ORDINAL: i32 = $ordinal;
            const NAME: &'static str = $string;
        }
        
        impl PokerHand for $symbol_struct {
            fn new(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Self> {
                if let Some(hand) = $as_fn(cards, is_wild) {
                    let score = Self::score_hand(Self::ORDINAL, &hand);
                    Some($symbol_struct(hand, score))
                } else {
                    None
                }
            }
            
            fn name(&self) -> &'static str { Self::NAME }
            fn ord(&self) -> i32 { Self::ORDINAL }
            fn score(&self) -> i32 { self.1 }
            fn cards(&self) -> &[Card] { &self.0 }
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
        // self.ord() == other.ord() && self.cards().cmp(&other.cards()) == Ordering::Equal
        self.score() == other.score()
    }
}

impl<'a> Eq for dyn PokerHand + 'a {}

impl<'a> Ord for dyn PokerHand + 'a {
    fn cmp(&self, other: &dyn PokerHand) -> Ordering {
        self.score().cmp(&other.score())
        // match self.ord().cmp(&other.ord()) {
        //     Ordering::Greater => Ordering::Greater,
        //     Ordering::Less => Ordering::Less,
        //     Ordering::Equal => self.cards().cmp(&other.cards())
        // }
    }
}

impl<'a> PartialOrd for dyn PokerHand + 'a {
    fn partial_cmp(&self, other: &dyn PokerHand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> std::fmt::Display for dyn PokerHand + 'a {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}", fmt_cards(self.cards()), self.name())
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

    #[test]
    fn test_cmp_wild_full_house() {
        let natural = parse_hand("Ac As Ad Jh Jd");
        let wild = parse_hand("Ac As ?? Jh Jd");

        assert_eq!(natural.ord(), FullHouse::ORDINAL);
        assert_eq!(wild.ord(), FullHouse::ORDINAL);

        assert_eq!(wild.cmp(&natural), Ordering::Equal);
        assert_eq!(natural.cmp(&wild), Ordering::Equal);
    }

    #[test]
    fn test_cmp_wild_straight() {
        let natural = parse_hand("Ac Ks Qd Jh Td");
        let wild_one = parse_hand("Ac Ks ?? Jh Td");
        let wild_two = parse_hand("Ac Ks ?? ?? Td");

        assert_eq!(natural.ord(), Straight::ORDINAL);
        assert_eq!(wild_one.ord(), Straight::ORDINAL);
        assert_eq!(wild_two.ord(), Straight::ORDINAL);

        assert_eq!(natural.cmp(&wild_one), Ordering::Equal);
        assert_eq!(natural.cmp(&wild_two), Ordering::Equal);

        assert_eq!(wild_one.cmp(&natural), Ordering::Equal);
        assert_eq!(wild_one.cmp(&wild_two), Ordering::Equal);

        assert_eq!(wild_two.cmp(&natural), Ordering::Equal);
        assert_eq!(wild_two.cmp(&wild_one), Ordering::Equal);
    }

    #[test]
    fn test_cmp_wild_flush() {
        let natural = parse_hand("Ac Kc Qc 7c 6c");
        let wild_one = parse_hand("Ac ?? Qc 7c 6c");
        let wild_two = parse_hand("Ac ?? ?? 7c 6c");

        assert_eq!(natural.ord(), Flush::ORDINAL);
        assert_eq!(wild_one.ord(), Flush::ORDINAL);
        assert_eq!(wild_two.ord(), Flush::ORDINAL);

        assert_eq!(natural.cmp(&wild_one), Ordering::Equal);
        assert_eq!(natural.cmp(&wild_two), Ordering::Equal);

        assert_eq!(wild_one.cmp(&natural), Ordering::Equal);
        assert_eq!(wild_one.cmp(&wild_two), Ordering::Equal);

        assert_eq!(wild_two.cmp(&natural), Ordering::Equal);
        assert_eq!(wild_two.cmp(&wild_one), Ordering::Equal);
    }
}
