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
    #[strum(to_string = "Straight Flush")]
    StraightFlush,

    #[strum(to_string = "Four of a Kind")]
    Quads,

    #[strum(to_string = "Full House")]
    FullHouse,

    #[strum(to_string = "Flush")]
    Flush,

    #[strum(to_string = "Straight")]
    Straight,

    #[strum(to_string = "Three of a Kind")]
    Triplets,

    #[strum(to_string = "Two Pair")]
    TwoPair,

    #[strum(to_string = "Pair")]
    OnePair,

    #[strum(to_string = "High Card")]
    HighCard
}

fn make_sets(rank_map: &RankMap, set_sizes: &mut Vec<usize>) -> Option<Vec<Card>> {
    if set_sizes.len() > 0 {
        let set_size = set_sizes.remove(0);
        for rank in Rank::iter() {
            if let Some(ranked_cards) = rank_map.get(&rank) {
                if ranked_cards.len() >= set_size {
                    let mut set = ranked_cards[0..set_size].to_vec();

                    let cards = rank_map.flatten();
                    let mut filtered_cards = Vec::new();

                    for card in cards {
                        if !set.contains(&card) {
                            filtered_cards.push(card);
                        }
                    }
                    let next_rank_map = RankMap::new(&filtered_cards);
                    if let Some(sets) = make_sets(&next_rank_map, set_sizes) {
                        for card in sets {
                            set.push(card);
                        }

                        return Some(set);
                    }
                }
            }
        }
        return None;
    } else {
        return Some(Vec::new());
    }
}

fn as_quads(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![4, 1]);
}

fn as_full_house(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![3, 2]);
}

fn as_trips(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![3, 1, 1]);
}

fn as_two_pair(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![2, 2, 1]);
}

fn as_pair(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![2, 1, 1, 1]);
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
        for category in HandCategory::iter() {
            if let Some(result_cards) = category.get_test()(&cards, &rank_map) {
                return PokerHand {
                    category: category,
                    cards: result_cards
                };
            }
        }
        
        panic!();
    }

    fn cmp_ranks(a: &[Card], b: &[Card]) -> std::cmp::Ordering {
        for n in 0..std::cmp::min(a.len(), b.len()) {
            let card_a = &a[n];
            let card_b = &b[n];
            let cmp = card_a.rank.cmp(&card_b.rank);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        return Ordering::Equal;
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
            Ordering::Equal => return PokerHand::cmp_ranks(&self.cards, &other.cards)
        }
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &PokerHand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
