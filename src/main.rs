extern crate strum;
#[macro_use] extern crate strum_macros;

use strum::IntoEnumIterator;
use std::cmp::Ordering;

mod card;
use card::Suit;
use card::Rank;
use card::Card;
use card::Cards;
use card::sort;

mod deck;
use deck::make_shuffled_deck;

mod rank_map;
use rank_map::RankMap;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, ToString, Ord, PartialOrd)]
enum HandCategory {
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

    #[strum(to_string = "Three of a Kind")]
    Triplets = 3,

    #[strum(to_string = "Two Pair")]
    TwoPair = 2,

    #[strum(to_string = "Pair")]
    OnePair = 1,

    #[strum(to_string = "High Card")]
    HighCard = 0
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

fn is_quads(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![4, 1]);
}

fn is_full_house(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![3, 2]);
}

fn is_trips(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![3, 1, 1]);
}

fn is_two_pair(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![2, 2, 1]);
}

fn is_pair(_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![2, 1, 1, 1]);
}

fn is_high_card(cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    let mut sorted_cards = cards.to_vec();
    sort(&mut sorted_cards);
    return Some(sorted_cards[0..5].to_vec());
}

fn is_straight_simple(cards: &[Card]) -> Option<Vec<Card>> {
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

fn is_straight(cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    return is_straight_simple(cards);
}

fn is_flush(cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited: Vec<&Card> = cards
            .iter()
            .filter(|card| card.suit == suit)
            .collect();
        
        if suited.len() >= 5 {
            let mut suited_cards = suited.iter()
                .map(|card_ref| Card::copy(card_ref))
                .collect();

            sort(&mut suited_cards);
            return Some(suited_cards.iter()
                        .take(5)
                        .map(|card_ref| Card::copy(card_ref))
                        .collect());
        }
    }
    return None;
}

fn is_straight_flush(cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
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

            if let Some(straight) = is_straight_simple(&suited_cards) {
                return Some(straight);
            }
        }
    }
    return None;
}


impl HandCategory {
    fn get_test(&self) -> fn(&[Card], &RankMap) -> Option<Vec<Card>> {
        match self {
            HandCategory::HighCard => is_high_card,
            HandCategory::OnePair => is_pair,
            HandCategory::TwoPair => is_two_pair,
            HandCategory::Triplets => is_trips,
            HandCategory::Straight => is_straight,
            HandCategory::Flush => is_flush,
            HandCategory::FullHouse => is_full_house,
            HandCategory::Quads => is_quads,
            HandCategory::StraightFlush => is_straight_flush
        }
    }
}

#[derive(Eq)]
struct PokerHand {
    category: HandCategory,
    cards: Vec<Card>
}

impl PokerHand {
    fn new(cards: &[Card]) -> PokerHand {
        // let rank_map = make_rank_map(&cards);
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

fn deal(cards: &mut Vec<Card>, n: usize) {
    let mut pockets = Vec::new();

    for card_index in 0..2 {
        for pocket_index in 0..n {
            if card_index == 0 {
                pockets.push(Vec::new());
            }

            if let Some(top) = cards.pop() {
                pockets[pocket_index].push(top);
            }
        }
    }

    let mut burns = Vec::new();
    let mut board = Vec::new();

    if let Some(burn) = cards.pop() { burns.push(burn); }
    if let Some(card) = cards.pop() { board.push(card); }
    if let Some(card) = cards.pop() { board.push(card); }
    if let Some(card) = cards.pop() { board.push(card); }
    if let Some(burn) = cards.pop() { burns.push(burn); }
    if let Some(card) = cards.pop() { board.push(card); }
    if let Some(burn) = cards.pop() { burns.push(burn); }
    if let Some(card) = cards.pop() { board.push(card); }

    println!("Board: {}", Cards(&board));

    let mut evals = Vec::new();
    for pocket in &pockets {
        let mut cards = pocket.to_vec();
        cards.extend(board.to_vec());

        let poker_hand = PokerHand::new(&cards);
        evals.push((pocket, poker_hand));
    }

    evals.sort_by(|a, b| {
        let (_pocket_a, poker_hand_a) = a;
        let (_pocket_b, poker_hand_b) = b;
        poker_hand_a.cmp(&poker_hand_b)
    });
    evals.reverse();

    for eval in evals {
        let (pocket, poker_hand) = eval;
        println!("Pocket: {} -> {}", Cards(&pocket), poker_hand);
    }
}

fn main() {
    for _n in 0..100 {
        let mut deck = make_shuffled_deck();
        deal(&mut deck, 8);
    }
}
