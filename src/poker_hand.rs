use strum::IntoEnumIterator;
use std::cmp::Ordering;

use super::card::Suit;
use super::card::Rank;
use super::card::Card;
use super::card::Cards;

use super::rank_map::RankMap;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, ToString, Ord, PartialOrd)]
pub enum HandCategory {
    #[strum(to_string = "High Card")]
    HighCard,

    #[strum(to_string = "Pair")]
    OnePair,

    #[strum(to_string = "Two Pair")]
    TwoPair,

    #[strum(to_string = "Three of a Kind")]
    Triplets,

    #[strum(to_string = "Straight")]
    Straight,

    #[strum(to_string = "Flush")]
    Flush,

    #[strum(to_string = "Full House")]
    FullHouse,

    #[strum(to_string = "Four of a Kind")]
    Quads,

    #[strum(to_string = "Straight Flush")]
    StraightFlush
}

fn make_sets_worker(rank_map: &RankMap, sizes: &mut Vec<usize>, result: &mut Vec<Card>) -> bool {
    if let Some(set_size) = sizes.pop() {
        if let Some(set) = rank_map.take_set(set_size) {
            let next_rank_map = rank_map.remove(&set);
            result.extend(set);
            return make_sets_worker(&next_rank_map, sizes, result);
        }
        return false;
    } else {
        return true;
    }
}

fn make_sets(rank_map: &RankMap, set_sizes: &Vec<usize>) -> Option<Vec<Card>> {
    let mut sizes_copy = set_sizes.to_vec();
    sizes_copy.reverse();
    let mut cards = Vec::new();
    if make_sets_worker(rank_map, &mut sizes_copy, &mut cards) {
        return Some(cards);
    } else {
        return None;
    }
}

fn as_quads(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &vec![4, 1]);
}

fn as_full_house(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &vec![3, 2]);
}

fn as_trips(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &vec![3, 1, 1]);
}

fn as_two_pair(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &vec![2, 2, 1]);
}

fn as_pair(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &vec![2, 1, 1, 1]);
}

fn as_high_card(cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort();
    sorted_cards.reverse();
    return Some(sorted_cards[0..5].to_vec());
}

fn as_straight_simple(cards: &[Card]) -> Option<Vec<Card>> {
    let mut vec: Vec<&Card> = Vec::new();
    for rank in Rank::iter() {
        match cards
            .iter()
            .find(|card| card.rank == rank) {
                Some(card) => {
                    vec.push(card);
                    if vec.len() >= 5 {
                        return Some(
                            vec.iter()
                                .map(|card_ref| Card::copy(card_ref))
                                .collect());
                    } else if vec.len() >= 4 && rank == Rank::Two {
                        if let Some(ace) = cards.iter().find(|card| card.rank == Rank::Ace) {
                            let mut result = vec.iter()
                                .map(|card_ref| Card::copy(card_ref))
                                .collect::<Vec<Card>>();

                            result.push(Card::new(Rank::LowAce, ace.suit));
                            return Some(result);
                        }
                    }
                }
                None => vec.clear()
            }
    }

    return None;
}

fn as_straight(cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    return as_straight_simple(cards);
}

fn as_flush(cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited: Vec<&Card> = cards
            .iter()
            .filter(|card| card.suit == suit)
            .collect();
        
        if suited.len() >= 5 {
            let mut suited_cards: Vec<Card> = suited.iter()
                .map(|card_ref| Card::copy(card_ref))
                .collect();

            suited_cards.sort();
            suited_cards.reverse();
            return Some(suited_cards.iter()
                        .take(5)
                        .map(|card_ref| Card::copy(card_ref))
                        .collect());
        }
    }
    return None;
}

fn as_straight_flush(cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited: Vec<&Card> = cards
            .iter()
            .filter(|card| card.suit == suit)
            .collect();
        
        if suited.len() >= 5 {
            let suited_cards = suited
                .iter()
                .map(|card_ref| Card::copy(card_ref))
                .collect::<Vec<Card>>();

            if let Some(straight) = as_straight_simple(&suited_cards) {
                return Some(straight);
            }
        }
    }
    return None;
}

impl HandCategory {
    fn get_test(&self) -> fn(&[Card], &RankMap) -> Option<Vec<Card>> {
        match self {
            HandCategory::HighCard => as_high_card,
            HandCategory::OnePair => as_pair,
            HandCategory::TwoPair => as_two_pair,
            HandCategory::Triplets => as_trips,
            HandCategory::Straight => as_straight,
            HandCategory::Flush => as_flush,
            HandCategory::FullHouse => as_full_house,
            HandCategory::Quads => as_quads,
            HandCategory::StraightFlush => as_straight_flush
        }
    }
}

#[derive(Eq)]
pub struct PokerHand {
    category: HandCategory,
    cards: Vec<Card>
}

impl PokerHand {
    pub fn new(cards: &[Card]) -> PokerHand {
        let rank_map = RankMap::new(&cards);
        for category in HandCategory::iter().rev() {
            if let Some(result_cards) = category.get_test()(&cards, &rank_map) {
                return PokerHand {
                    category: category,
                    cards: result_cards
                };
            }
        }
        
        unreachable!();
    }
}

impl std::fmt::Display for PokerHand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}", Cards(&self.cards), self.category.to_string())
    }
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &PokerHand) -> bool {
        self.category == other.category
            && self.cards[0] == other.cards[0]
    }
}

impl std::cmp::Ord for PokerHand {
    fn cmp(&self, other: &PokerHand) -> std::cmp::Ordering {
        match self.category.cmp(&other.category) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => return self.cards.cmp(&other.cards)
        }
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &PokerHand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardVector;
    use Rank::*;
    use Suit::*;
    use HandCategory::*;

    fn parse_hand(card_string: &str) -> PokerHand {
        let cards: Vec<Card> = card_string.parse::<CardVector>().unwrap().into();
        PokerHand::new(&cards)
    }
    
    #[test]
    fn test_straight_flush() {
        let poker_hand = parse_hand("Ac Kc Qc Tc Jc");
        assert_eq!(poker_hand.category, StraightFlush);
    }

    #[test]
    fn test_quads() {
        let poker_hand = parse_hand("Ac As Ad Ah Jd");
        assert_eq!(poker_hand.category, Quads);

        assert_eq!(poker_hand.cards[0].rank, Ace);
        assert_eq!(poker_hand.cards[1].rank, Ace);
        assert_eq!(poker_hand.cards[2].rank, Ace);
        assert_eq!(poker_hand.cards[3].rank, Ace);
        assert_eq!(poker_hand.cards[4].rank, Jack);
    }

    #[test]
    fn test_full_house() {
        let poker_hand = parse_hand("Ac As Ad Jh Jd");
        assert_eq!(poker_hand.category, FullHouse);
        
        assert_eq!(poker_hand.cards[0].rank, Ace);
        assert_eq!(poker_hand.cards[1].rank, Ace);
        assert_eq!(poker_hand.cards[2].rank, Ace);
        assert_eq!(poker_hand.cards[3].rank, Jack);
        assert_eq!(poker_hand.cards[4].rank, Jack);
    }

    #[test]
    fn test_flush() {
        let poker_hand = parse_hand("Ac Kc 7c Tc Jc");
        assert_eq!(poker_hand.category, Flush);
        
        assert_eq!(poker_hand.cards[0].rank, Ace);
        assert_eq!(poker_hand.cards[1].rank, King);
        assert_eq!(poker_hand.cards[2].rank, Jack);
        assert_eq!(poker_hand.cards[3].rank, Ten);
        assert_eq!(poker_hand.cards[4].rank, Seven);
    }
    
    #[test]
    fn test_straight() {
        let poker_hand = parse_hand("Ac Kc Qc Ts Jd");
        assert_eq!(poker_hand.category, Straight);
    }

    #[test]
    fn test_triplets() {
        let poker_hand = parse_hand("Ac Ah As Ts Jd");
        assert_eq!(poker_hand.category, Triplets);
    }

    #[test]
    fn test_two_pair() {
        let poker_hand = parse_hand("Ac Ah Qs Qd Jd");
        assert_eq!(poker_hand.category, TwoPair);
    }

    #[test]
    fn test_pair() {
        let poker_hand = parse_hand("Ac Ah Qs Td Jd");
        assert_eq!(poker_hand.category, OnePair);
    }

    #[test]
    fn test_high_card() {
        let poker_hand = parse_hand("Ac Jh 9s 7d 5d");
        assert_eq!(poker_hand.category, HighCard);

        assert_eq!(poker_hand.cards[0], Ace.of(Clubs));
        assert_eq!(poker_hand.cards[1], Jack.of(Hearts));
        assert_eq!(poker_hand.cards[2], Nine.of(Spades));
        assert_eq!(poker_hand.cards[3], Seven.of(Diamonds));
        assert_eq!(poker_hand.cards[4], Five.of(Diamonds));
    }
}
