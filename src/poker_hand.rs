use strum::IntoEnumIterator;
use std::cmp::Ordering;

use super::card::Suit;
use super::card::Rank;
use super::card::Card;
use super::card::IsWildCard;
use super::card::fmt_cards;
use super::card::fmt_cards_refs;

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

fn contains_scoring_rank(cards: &[Card], rank: Rank) -> bool {
    return cards.iter()
        .find(|card| card.scoring_rank == rank)
        .is_some();
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
                for rank in Rank::iter() {
                    if !contains_scoring_rank(&suited, rank) {
                        suited.push(w.scored_as(rank));
                        break;
                    }
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Display)]
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

pub struct PokerHand {
    rank: HandRank,
    cards: Vec<Card>,
    score: i32
}

impl PokerHand {
    fn new(hand_rank: HandRank, cards: Vec<Card>) -> Self {
        let score = PokerHand::score_cards(hand_rank, &cards);
        PokerHand {
            rank: hand_rank,
            cards: cards,
            score: score
        }
    }
    
    fn score_cards(hand_rank: HandRank, cards: &[Card]) -> i32 {
        return cards.iter()
            .fold(hand_rank as i32, |acc, card| acc * 16 + (card.scoring_rank as i32));
    }

    fn name(&self) -> std::string::String {
        self.rank.to_string()
    }
    
    fn ord(&self) -> i32 {
        self.rank as i32
    }
    
    fn cards(&self) -> &[Card] {
        &self.cards
    }
    
    fn score(&self) -> i32 {
        self.score
    }
}

macro_rules! try_make_hand {
    ($hand_rank: ident, $cards: ident, $wild_cards: ident, $as_fn: ident) => {
        if let Some(cards) = $as_fn(&$cards, &$wild_cards) {
            return PokerHand::new(HandRank::$hand_rank, cards);
        }
    }
}

pub fn make_poker_hand(all_cards: &[&Card], is_wild: &Option<IsWildCard>) -> PokerHand {
    let (wild_cards, cards): (Vec<&Card>, Vec<&Card>) = all_cards.iter()
        .cloned()
        .partition(|card| card.is_wild(is_wild));

    try_make_hand!(StraightFlush, cards, wild_cards, as_straight_flush);
    try_make_hand!(Quads, cards, wild_cards, as_quads);
    try_make_hand!(FullHouse, cards, wild_cards, as_full_house);
    try_make_hand!(Flush, cards, wild_cards, as_flush);
    try_make_hand!(Straight, cards, wild_cards, as_straight);
    try_make_hand!(Triplets, cards, wild_cards, as_trips);
    try_make_hand!(TwoPair, cards, wild_cards, as_two_pair);
    try_make_hand!(OnePair, cards, wild_cards, as_pair);
    try_make_hand!(HighCard, cards, wild_cards, as_high_card);
    unreachable!();
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &PokerHand) -> bool {
        self.score() == other.score()
    }
}

impl Eq for PokerHand {}

impl Ord for PokerHand {
    fn cmp(&self, other: &PokerHand) -> Ordering {
        self.score().cmp(&other.score())
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
        let hand = make_poker_hand(&foo, is_wild);
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
        assert_eq!(parse_hand("Ac Kc Qc Tc Jc").ord(), StraightFlush as i32);
    }

    #[test]
    fn test_straight_flush_with_jokers() {
        assert_eq!(parse_hand("Ac Kc Qc ?? Jc").ord(), StraightFlush as i32);
        assert_eq!(parse_hand("Ac Kc ?? ?? Jc").ord(), StraightFlush as i32);
    }

    #[test]
    fn test_quads() {
        let poker_hand = parse_hand("Ac As Ad Ah Jd");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.ord(), Quads as i32);

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
        assert_eq!(poker_hand.ord(), FullHouse as i32);
        
        assert_eq!(cards[0].rank, Ace);
        assert_eq!(cards[1].rank, Ace);
        assert_eq!(cards[2].rank, Ace);
        assert_eq!(cards[3].rank, Jack);
        assert_eq!(cards[4].rank, Jack);
    }

    #[test]
    fn test_one_joker() {
        assert_eq!(parse_hand("Ac As Ad ?? Jd").ord(), Quads as i32);
        assert_eq!(parse_hand("Ac As ?? Jc Jd").ord(), FullHouse as i32);
        assert_eq!(parse_hand("Ac As ?? Jc Td").ord(), Triplets as i32);
        assert_eq!(parse_hand("Ac ?? Jc Td 7c").ord(), OnePair as i32);
    }

    #[test]
    fn test_two_joker() {
        assert_eq!(parse_hand("Ac As ?? ?? Jd").ord(), Quads as i32);
        assert_eq!(parse_hand("Ac ?? ?? Td 7c").ord(), Triplets as i32);
    }

    #[test]
    fn test_flush() {
        let poker_hand = parse_hand("Ac Kc 7c Tc Jc");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.ord(), Flush as i32);
        
        assert_eq!(cards[0].rank, Ace);
        assert_eq!(cards[1].rank, King);
        assert_eq!(cards[2].rank, Jack);
        assert_eq!(cards[3].rank, Ten);
        assert_eq!(cards[4].rank, Seven);
    }
    
    #[test]
    fn test_flush_with_jokers() {
        assert_eq!(parse_hand("Ac Kc 7c Tc ??").ord(), Flush as i32);
        assert_eq!(parse_hand("Ac Kc ?? Jc 7c").ord(), Flush as i32);
    }
    
    #[test]
    fn test_straight() {
        assert_eq!(parse_hand("Ac Kc Qc Ts Jd").ord(), Straight as i32);
    }

    #[test]
    fn test_low_straight() {
        assert_eq!(parse_hand("Ac 5c 4s 3s 2d").ord(), Straight as i32);
        assert_eq!(parse_hand("5c 4s 3s 2d ??").ord(), Straight as i32);
        assert_eq!(parse_hand("?? 4s 3s 2d Ac").ord(), Straight as i32);
    }

    #[test]
    fn test_straight_with_jokers() {
        assert_eq!(parse_hand("Ac Kc Qc Jd ??").ord(), Straight as i32);
        assert_eq!(parse_hand("Ac Kc ?? ?? Ts").ord(), Straight as i32);
    }

    #[test]
    fn test_triplets() {
        assert_eq!(parse_hand("Ac Ah As Ts Jd").ord(), Triplets as i32);
    }

    #[test]
    fn test_two_pair() {
        assert_eq!(parse_hand("Ac Ah Qs Qd Jd").ord(), TwoPair as i32);
    }

    #[test]
    fn test_pair() {
        assert_eq!(parse_hand("Ac Ah Qs Td Jd").ord(), OnePair as i32);
    }

    #[test]
    fn test_high_card() {
        let poker_hand = parse_hand("Ac Jh 9s 7d 5d");
        let cards = poker_hand.cards();
        assert_eq!(poker_hand.ord(), HighCard as i32);

        assert_eq!(cards[0], Ace.of(Clubs));
        assert_eq!(cards[1], Jack.of(Hearts));
        assert_eq!(cards[2], Nine.of(Spades));
        assert_eq!(cards[3], Seven.of(Diamonds));
        assert_eq!(cards[4], Five.of(Diamonds));
    }

    #[test]
    fn test_suicide_king() {
        assert_eq!(parse_hand_suicide_king("9c Kh 7c 6c 5c").ord(), StraightFlush as i32);
        assert_eq!(parse_hand_suicide_king("Ac Kh As Ad 7d").ord(), Quads as i32);
        assert_eq!(parse_hand_suicide_king("Kc Kh 7c 6c 5c").ord(), Flush as i32);
        assert_eq!(parse_hand_suicide_king("9c Kh 7s 6d 5d").ord(), Straight as i32);
        assert_eq!(parse_hand_suicide_king("Ac Kh As 7c 7d").ord(), FullHouse as i32);
        assert_eq!(parse_hand_suicide_king("Ac Kh As 6c 7d").ord(), Triplets as i32);
        assert_eq!(parse_hand_suicide_king("Ac Kh 5c 6c 7d").ord(), OnePair as i32);
    }

    #[test]
    fn test_two_pair_edge_case() {
        let poker_hand = parse_hand("K♣ K♦ 5♠ 5♣ 3♥ 3♣");
        assert_eq!(poker_hand.ord(), TwoPair as i32);

        println!("two_pair_edge_case: {}", poker_hand);
        assert_eq!(poker_hand.cards().len(), 5);
    }

    #[test]
    fn test_cmp_wild_full_house() {
        let natural = parse_hand("Ac As Ad Jh Jd");
        let wild = parse_hand("Ac As ?? Jh Jd");

        assert_eq!(natural.ord(), FullHouse as i32);
        assert_eq!(wild.ord(), FullHouse as i32);

        assert_eq!(wild.cmp(&natural), Ordering::Equal);
        assert_eq!(natural.cmp(&wild), Ordering::Equal);
    }

    #[test]
    fn test_cmp_wild_straight() {
        let natural = parse_hand("Ac Ks Qd Jh Td");
        let wild_one = parse_hand("Ac Ks ?? Jh Td");
        let wild_two = parse_hand("Ac Ks ?? ?? Td");

        assert_eq!(natural.ord(), Straight as i32);
        assert_eq!(wild_one.ord(), Straight as i32);
        assert_eq!(wild_two.ord(), Straight as i32);

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

        assert_eq!(natural.ord(), Flush as i32);
        assert_eq!(wild_one.ord(), Flush as i32);
        assert_eq!(wild_two.ord(), Flush as i32);

        assert_eq!(natural.cmp(&wild_one), Ordering::Equal);
        assert_eq!(natural.cmp(&wild_two), Ordering::Equal);

        assert_eq!(wild_one.cmp(&natural), Ordering::Equal);
        assert_eq!(wild_one.cmp(&wild_two), Ordering::Equal);

        assert_eq!(wild_two.cmp(&natural), Ordering::Equal);
        assert_eq!(wild_two.cmp(&wild_one), Ordering::Equal);
    }
}
