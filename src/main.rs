extern crate strum;
#[macro_use] extern crate strum_macros;

use strum::IntoEnumIterator;
use std::collections::HashMap;
use std::cmp::Ordering;

mod card;
use card::Suit;
use card::Rank;
use card::Card;
use card::sort;
use card::fmt_cards;

mod deck;
use deck::make_shuffled_deck;

type RankMap = HashMap<Rank, Vec<Card>>;

fn make_rank_map(cards: &[Card]) -> RankMap {
    let mut rank_map = HashMap::new();
    for rank in Rank::iter() {
        rank_map.insert(rank, Vec::new());
    }
    
    for card in cards {
        if let Some(rank_vector) = rank_map.get_mut(&card.rank) {
            rank_vector.push(Card::copy(card));
        }
    }

    return rank_map;
}

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

fn flatten_rank_map(rank_map: &RankMap) -> Vec<Card> {
    let mut cards = Vec::new();
    for ranked_cards in rank_map.values() {
        for card in ranked_cards {
            cards.push(Card::copy(card));
        }
    }

    return cards;
}

fn make_sets(rank_map: &RankMap, set_sizes: &mut Vec<usize>) -> Option<Vec<Card>> {
    if set_sizes.len() > 0 {
        let set_size = set_sizes.remove(0);
        for rank in Rank::iter() {
            if let Some(ranked_cards) = rank_map.get(&rank) {
                if ranked_cards.len() >= set_size {
                    let mut set = ranked_cards[0..set_size].to_vec();

                    let cards = flatten_rank_map(rank_map);
                    let mut filtered_cards = Vec::new();

                    for card in cards {
                        if !set.contains(&card) {
                            filtered_cards.push(card);
                        }
                    }
                    let next_rank_map = make_rank_map(&filtered_cards);
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

fn is_quads(_canonical_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![4, 1]);
}

fn is_full_house(_canonical_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![3, 2]);
}

fn is_trips(_canonical_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![3, 1, 1]);
}

fn is_two_pair(_canonical_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![2, 2, 1]);
}

fn is_pair(_canonical_cards: &[Card], rank_map: &RankMap) -> Option<Vec<Card>> {
    return make_sets(rank_map, &mut vec![2, 1, 1, 1]);
}

fn is_high_card(canonical_cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    let mut cards = canonical_cards.to_vec();
    sort(&mut cards);
    return Some(cards[0..5].to_vec());
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

fn is_straight(canonical_cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    return is_straight_simple(canonical_cards);
}

fn is_flush(canonical_cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited: Vec<&Card> = canonical_cards
            .iter()
            .filter(|card| card.suit == suit)
            .collect();
        
        if suited.len() >= 5 {
            let mut cards = suited.iter()
                .map(|card_ref| Card::copy(card_ref))
                .collect();

            sort(&mut cards);
            return Some(cards.iter()
                        .take(5)
                        .map(|card_ref| Card::copy(card_ref))
                        .collect());
        }
    }
    return None;
}

fn is_straight_flush(canonical_cards: &[Card], _rank_map: &RankMap) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited: Vec<&Card> = canonical_cards
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

fn cmp_size<T>(a: &[T], b:&[T]) -> std::cmp::Ordering {
    return b.len().cmp(&a.len());
}

fn canonical_order(rank_map: &RankMap) -> Vec<Card> {
    let mut sorted_keys: Vec<&Rank> = rank_map.keys().collect();
    sorted_keys.sort_by(|a, b| {
        match cmp_size(rank_map.get(a).unwrap(), rank_map.get(b).unwrap()) {
            Ordering::Equal => b.cmp(a),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater
        }
    });

    let mut canonical_cards: Vec<Card> = Vec::new();
    for rank in sorted_keys {
        if let Some(rank_cards) = rank_map.get(rank) {
            for card in rank_cards {
                canonical_cards.push(card.copy());
            }
        }
    }

    return canonical_cards;
}

type Score = (HandCategory, Vec<Card>);

fn evaluate(cards: &[Card]) -> Score {
    let rank_map = make_rank_map(&cards);
    let canonical_cards = canonical_order(&rank_map);

    for category in HandCategory::iter() {
        if let Some(result_cards) = category.get_test()(&canonical_cards, &rank_map) {
            return (category, result_cards);
        }
    }

    return (HandCategory::HighCard, canonical_cards);
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

fn cmp_score(a: &Score, b: &Score) -> std::cmp::Ordering {
    let (category_a, cards_a) = a;
    let (category_b, cards_b) = b;

    match category_a.cmp(category_b) {
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
        Ordering::Equal => return cmp_ranks(&cards_a, &cards_b)
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

    println!("Board: {}", fmt_cards(&board));

    // let evals = pockets.iter()
    //     .map(|pocket| {
    //         let mut cards = pocket.to_vec();
    //         cards.extend(board.to_vec());
    //         (pocket, cards)
    //     })
    //     .map(|(pocket, cards)| {
    //         let (category, sorted_cards) = evaluate(&cards);
    //         (pocket, category, sorted_cards)
    //     })
    //     .collect();
    
    let mut evals = Vec::new();
    for pocket in &pockets {
        let mut cards = pocket.to_vec();
        cards.extend(board.to_vec());

        // let (category, sorted_cards) = evaluate(&cards);
        let eval = evaluate(&cards);
        evals.push((pocket, eval));
        // println!("Pocket: {} -> {} -> {}", fmt_cards(&pocket), fmt_cards(&sorted_cards), category.to_string());

        // if category == HandCategory::FullHouse {
        //     panic!();
        // }
    }

    evals.sort_by(|a, b| {
        let (_pocket_a, score_a) = a;
        let (_pocket_b, score_b) = b;
        cmp_score(score_a, score_b)
    });
    evals.reverse();

    for eval in evals {
        let (pocket, score) = eval;
        let (category, sorted_cards) = score;
        println!("Pocket: {} -> {} -> {}", fmt_cards(&pocket), fmt_cards(&sorted_cards), category.to_string());
    }
}

fn main() {
    // for card in &deck {
    //     println!("{}", card.to_string());
    // }

    for _n in 0..100 {
        let mut deck = make_shuffled_deck();
        deal(&mut deck, 8);
    }

    // let mut hand = &deck[..5].to_vec();
    // println!("Hand: {:#?}", hand);
    // let mut hand = Vec::from_iter(deck[..5].iter().cloned());
    // sort(&mut hand);
    // println!("Hand: {}", foo(&hand));

    // let mut sorted_deck = deck.to_vec();
    // sort(&mut sorted_deck);
    // for card in sorted_deck {
    //     println!("{}", card.to_string());
    // }
}
