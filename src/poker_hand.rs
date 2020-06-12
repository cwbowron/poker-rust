use strum::IntoEnumIterator;
use std::cmp::Ordering;

use super::card::{Suit, Rank, Card, IsWildCard, fmt_cards};

pub fn remove_cards<'a>(a: &'a [&Card], b: &[Card]) -> Vec<&'a Card> {
    let mut vec = a.to_vec();
    vec.drain_filter(|card| b.contains(card));
    return vec;
}

pub fn remove_card<'a>(a: &[&'a Card], b: &Card) -> Vec<&'a Card> {
    let mut vec = a.to_vec();
    vec.remove_item(b);
    return vec;
}

fn find_set(cards: &[&Card], wild_cards: &[&Card], n: usize) -> Option<Vec<Card>> {
    for rank in Rank::iter() {
        if rank != Rank::LowAce && rank != Rank::Joker {
            let count = cards.iter()
                .filter(|card| card.rank == rank)
                .count();
            
            if count + wild_cards.len() >= n {
                return Some(cards.iter()
                            .filter(|card| card.rank == rank)
                            .chain(wild_cards.iter())
                            .take(n)
                            .map(|card| card.scored_as(rank))
                            .collect());
            }
        }
    }

    return None;
}

fn make_sets(cards: &[&Card], wild_cards: &[&Card], sizes: &Vec<usize>, size_index: usize, result: &mut Vec<Card>) -> bool {
    if size_index >= sizes.len() {
        return true;
    } else if let Some(set) = find_set(cards, wild_cards, sizes[size_index]) {
        let next = remove_cards(cards, &set);
        if make_sets(&next, &vec![], sizes, size_index + 1, result) {
            result.extend(set);
            return true;
        }
    }

    return false;
}

macro_rules! define_set_maker {
    ($fn_name: ident, $set: tt) => {
        fn $fn_name(cards: &[&Card], wild_cards: &[&Card]) -> Option<Vec<Card>> {
            let mut r = Vec::new();
            if make_sets(cards, wild_cards, &vec!$set, 0, &mut r) {
                r.reverse();
                return Some(r);
            } else {
                return None;
            }
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

fn as_high_card(cards: &[&Card], _wild_cards: &[&Card]) -> Option<Vec<Card>> {
    return Some(top_five(cards.iter().cloned().cloned().collect()));
}

fn fill_straight(cards: &[&Card], wild_cards:&[&Card], rank_ordinal: usize, n: usize) -> Option<Vec<Card>>{
    if n >= 5 {
        return Some(Vec::new());
    } else {
        if let Some(card) = cards.iter()
            .find(|card| card.rank.is_ordinal(rank_ordinal)) {
                if let Some(mut result) = fill_straight(cards, wild_cards, rank_ordinal - 1, n + 1) {
                    result.push((*card).clone());
                    return Some(result);
                }
            }

        if wild_cards.len() > 0 {
            let rank = Rank::for_ordinal(rank_ordinal);
            if let Some(mut result) = fill_straight(cards, &wild_cards[1..], rank_ordinal - 1, n + 1) {
                result.push(wild_cards[0].scored_as(rank));
                return Some(result);
            }
        }

        return None;
    }
}

fn as_straight(cards: &[&Card], wild_cards: &[&Card]) -> Option<Vec<Card>> {
    for rank_ordinal in (Rank::Five as usize .. Rank::Ace as usize + 1).rev() {
        if let Some(mut result) = fill_straight(cards, wild_cards, rank_ordinal, 0) {
            result.reverse();
            return Some(result);
        }
    }

    return None;
}

fn find_missing_rank(cards: &[Card]) -> Option<Rank> {
    for rank in Rank::iter() {
        if !cards.iter().any(|card| card.scoring_rank == rank) {
            return Some(rank);
        }
    }

    return None;
}

fn as_flush(cards: &[&Card], wild_cards: &[&Card]) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited_count = cards.iter()
            .filter(|card| card.suit == suit)
            .count();

        if suited_count + wild_cards.len() >= 5 {
            let mut suited = cards.iter()
                .filter(|card| card.suit == suit)
                .cloned()
                .cloned()
                .collect::<Vec<_>>();
            
            for w in wild_cards {
                if let Some(rank) = find_missing_rank(&suited) {
                    suited.push(w.scored_as(rank));
                } else {
                    break;
                }
            }

            return Some(top_five(suited));
        }
    }

    return None;
}

fn as_straight_flush(cards: &[&Card], wild_cards: &[&Card]) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let count = cards
            .iter()
            .filter(|card| card.suit == suit)
            .count();
        
        if count + wild_cards.len() >= 5 {
            let suited_cards = cards
                .iter()
                .filter(|card| card.suit == suit)
                .cloned()
                .collect::<Vec<_>>();
                
            let option = as_straight(&suited_cards, wild_cards);
            if option.is_some() {
                return option;
            }
        }
    }
    
    return None;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Display, EnumIter)]
pub enum HandRank {
    #[strum(to_string = "Straight Flush")]
    StraightFlush = 8,
    #[strum(to_string = "Four of a Kind")]
    Quads = 7,
    #[strum(to_string = "Full House")]
    FullHouse = 6,
    #[strum(to_string = "Flush")]
    Flush = 5,
    #[strum(to_string = "Straight")]
    Straight = 4,
    #[strum(to_string = "Triplets")]
    Triplets = 3,
    #[strum(to_string = "Two Pair")]
    TwoPair = 2,
    #[strum(to_string = "Pair")]
    OnePair = 1,
    #[strum(to_string = "High Card")]
    HighCard = 0
}

impl HandRank {
    pub fn build(&self) -> fn(&[&Card], &[&Card]) -> Option<Vec<Card>> {
        match self {
            HandRank::StraightFlush => as_straight_flush,
            HandRank::Quads => as_quads,
            HandRank::FullHouse => as_full_house,
            HandRank::Flush => as_flush,
            HandRank::Straight => as_straight,
            HandRank::Triplets => as_trips,
            HandRank::TwoPair => as_two_pair,
            HandRank::OnePair => as_pair,
            HandRank::HighCard => as_high_card
        }
    }

    fn score_cards(&self, cards: &[Card]) -> i32 {
        return cards.iter()
            .fold(*self as i32, |acc, card| acc * 16 + (card.scoring_rank as i32));
    }
}

pub struct PokerHand {
    rank: HandRank,
    cards: Vec<Card>,
    score: i32
}

impl PokerHand {
    pub fn build(all_cards: &[&Card], is_wild: &Option<IsWildCard>) -> PokerHand {
        let (wild_cards, cards): (Vec<&Card>, Vec<&Card>) = all_cards.iter()
            .cloned()
            .partition(|card| card.is_wild(is_wild));
        
        for rank in HandRank::iter() {
            if let Some(cards) = rank.build()(&cards, &wild_cards) {
                return PokerHand::new(rank, cards);
            }
        }

        unreachable!();
    }

    fn new(hand_rank: HandRank, cards: Vec<Card>) -> Self {
        let score = hand_rank.score_cards(&cards);
        PokerHand {
            rank: hand_rank,
            cards: cards,
            score: score
        }
    }
    
    fn name(&self) -> std::string::String {
        self.rank.to_string()
    }
    
    fn cards(&self) -> &[Card] {
        &self.cards
    }
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &PokerHand) -> bool {
        self.score == other.score
    }
}

impl Eq for PokerHand {}

impl Ord for PokerHand {
    fn cmp(&self, other: &PokerHand) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &PokerHand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for PokerHand {
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
    use HandRank::*;

    fn _parse_hand(card_string: &str, is_wild: &Option<IsWildCard>) -> PokerHand {
        let card_vector = CardVector::parse(card_string);
        let foo = card_vector.iter().collect::<Vec<_>>();
        let hand = PokerHand::build(&foo, is_wild);
        println!("{} -> {}", card_string, hand);
        return hand;
    }

    fn parse_hand(card_string: &str) -> PokerHand {
        _parse_hand(card_string, &Some(Card::is_joker))
    }

    fn parse_hand_suicide_king(card_string: &str) -> PokerHand {
        _parse_hand(card_string, &Some(Card::is_suicide_king))
    }

    #[test]
    fn test_println() {
        let poker_hand = parse_hand("Ac As Ad Ah Jd");
        println!("{}", poker_hand);
    }

    #[test]
    fn test_straight_flush() {
        assert_eq!(parse_hand("Ac Kc Qc Tc Jc").rank, StraightFlush);
    }

    #[test]
    fn test_straight_flush_with_jokers() {
        assert_eq!(parse_hand("Ac Kc Qc ?? Jc").rank, StraightFlush);
        assert_eq!(parse_hand("Ac Kc ?? ?? Jc").rank, StraightFlush);
    }

    #[test]
    fn test_quads() {
        let poker_hand = parse_hand("Ac As Ad Ah Jd");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.rank, Quads);

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
        assert_eq!(poker_hand.rank, FullHouse);
        
        assert_eq!(cards[0].rank, Ace);
        assert_eq!(cards[1].rank, Ace);
        assert_eq!(cards[2].rank, Ace);
        assert_eq!(cards[3].rank, Jack);
        assert_eq!(cards[4].rank, Jack);
    }

    #[test]
    fn test_one_joker() {
        assert_eq!(parse_hand("Ac As Ad ?? Jd").rank, Quads);
        assert_eq!(parse_hand("Ac As ?? Jc Jd").rank, FullHouse);
        assert_eq!(parse_hand("Ac As ?? Jc Td").rank, Triplets);
        assert_eq!(parse_hand("Ac ?? Jc Td 7c").rank, OnePair);
    }

    #[test]
    fn test_two_joker() {
        assert_eq!(parse_hand("Ac As ?? ?? Jd").rank, Quads);
        assert_eq!(parse_hand("Ac ?? ?? Td 7c").rank, Triplets);
    }

    #[test]
    fn test_flush() {
        let poker_hand = parse_hand("Ac Kc 7c Tc Jc");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.rank, Flush);
        
        assert_eq!(cards[0].rank, Ace);
        assert_eq!(cards[1].rank, King);
        assert_eq!(cards[2].rank, Jack);
        assert_eq!(cards[3].rank, Ten);
        assert_eq!(cards[4].rank, Seven);
    }
    
    #[test]
    fn test_flush_with_jokers() {
        assert_eq!(parse_hand("Ac Kc 7c Tc ??").rank, Flush);
        assert_eq!(parse_hand("Ac Kc ?? Jc 7c").rank, Flush);
    }
    
    #[test]
    fn test_straight() {
        assert_eq!(parse_hand("Ac Kc Qc Ts Jd").rank, Straight);
    }

    #[test]
    fn test_low_straight() {
        assert_eq!(parse_hand("Ac 5c 4s 3s 2d").rank, Straight);
        assert_eq!(parse_hand("5c 4s 3s 2d ??").rank, Straight);
        assert_eq!(parse_hand("?? 4s 3s 2d Ac").rank, Straight);
    }

    #[test]
    fn test_straight_with_jokers() {
        assert_eq!(parse_hand("Ac Kc Qc Jd ??").rank, Straight);
        assert_eq!(parse_hand("Ac Kc ?? ?? Ts").rank, Straight);
    }

    #[test]
    fn test_triplets() {
        assert_eq!(parse_hand("Ac Ah As Ts Jd").rank, Triplets);
    }

    #[test]
    fn test_two_pair() {
        assert_eq!(parse_hand("Ac Ah Qs Qd Jd").rank, TwoPair);
    }

    #[test]
    fn test_pair() {
        assert_eq!(parse_hand("Ac Ah Qs Td Jd").rank, OnePair);
    }

    #[test]
    fn test_high_card() {
        let poker_hand = parse_hand("Ac Jh 9s 7d 5d");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.rank, HighCard);

        assert_eq!(cards[0], Ace.of(Clubs));
        assert_eq!(cards[1], Jack.of(Hearts));
        assert_eq!(cards[2], Nine.of(Spades));
        assert_eq!(cards[3], Seven.of(Diamonds));
        assert_eq!(cards[4], Five.of(Diamonds));
    }

    #[test]
    fn test_suicide_king() {
        assert_eq!(parse_hand_suicide_king("9c Kh 7c 6c 5c").rank, StraightFlush);
        assert_eq!(parse_hand_suicide_king("Ac Kh As Ad 7d").rank, Quads);
        assert_eq!(parse_hand_suicide_king("Kc Kh 7c 6c 5c").rank, Flush);
        assert_eq!(parse_hand_suicide_king("9c Kh 7s 6d 5d").rank, Straight);
        assert_eq!(parse_hand_suicide_king("Ac Kh As 7c 7d").rank, FullHouse);
        assert_eq!(parse_hand_suicide_king("Ac Kh As 6c 7d").rank, Triplets);
        assert_eq!(parse_hand_suicide_king("Ac Kh 5c 6c 7d").rank, OnePair);
    }

    #[test]
    fn test_two_pair_edge_case() {
        let poker_hand = parse_hand("K♣ K♦ 5♠ 5♣ 3♥ 3♣");
        assert_eq!(poker_hand.rank, TwoPair);

        println!("two_pair_edge_case: {}", poker_hand);
        assert_eq!(poker_hand.cards().len(), 5);
    }

    #[test]
    fn test_cmp_wild_full_house() {
        let natural = parse_hand("Ac As Ad Jh Jd");
        let wild = parse_hand("Ac As ?? Jh Jd");

        assert_eq!(natural.rank, FullHouse);
        assert_eq!(wild.rank, FullHouse);

        assert_eq!(wild.cmp(&natural), Ordering::Equal);
        assert_eq!(natural.cmp(&wild), Ordering::Equal);
    }

    #[test]
    fn test_cmp_wild_straight() {
        let natural = parse_hand("Ac Ks Qd Jh Td");
        let wild_one = parse_hand("Ac Ks ?? Jh Td");
        let wild_two = parse_hand("Ac Ks ?? ?? Td");

        assert_eq!(natural.rank, Straight);
        assert_eq!(wild_one.rank, Straight);
        assert_eq!(wild_two.rank, Straight);

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

        assert_eq!(natural.rank, Flush);
        assert_eq!(wild_one.rank, Flush);
        assert_eq!(wild_two.rank, Flush);

        assert_eq!(natural.cmp(&wild_one), Ordering::Equal);
        assert_eq!(natural.cmp(&wild_two), Ordering::Equal);

        assert_eq!(wild_one.cmp(&natural), Ordering::Equal);
        assert_eq!(wild_one.cmp(&wild_two), Ordering::Equal);

        assert_eq!(wild_two.cmp(&natural), Ordering::Equal);
        assert_eq!(wild_two.cmp(&wild_one), Ordering::Equal);
    }
}
