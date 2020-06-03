extern crate strum;
#[macro_use] extern crate strum_macros;

use strum::IntoEnumIterator;
use std::string::String;
use rand::Rng;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::cmp::Ordering;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, ToString, Ord, PartialOrd)]
enum Suit {
    #[strum(to_string = "♣")]
    Clubs,

    #[strum(to_string = "♦")]
    Diamonds,

    #[strum(to_string = "♥")]
    Hearts,
    
    #[strum(to_string = "♠")]
    Spades
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, ToString, Ord, PartialOrd, Hash)]
enum Rank {
    #[strum(to_string = "A")]
    Ace = 14,
    #[strum(to_string = "K")]
    King = 13,
    #[strum(to_string = "Q")]
    Queen = 12,
    #[strum(to_string = "J")]
    Jack = 11,
    #[strum(to_string = "T")]
    Ten = 10,
    #[strum(to_string = "9")]
    Nine = 9,
    #[strum(to_string = "8")]
    Eight = 8,
    #[strum(to_string = "7")]
    Seven = 7,
    #[strum(to_string = "6")]
    Six = 6,
    #[strum(to_string = "5")]
    Five = 5,
    #[strum(to_string = "4")]
    Four = 4,
    #[strum(to_string = "3")]
    Three = 3,
    #[strum(to_string = "2")]
    Two = 2
}

#[derive(Clone, Debug)]
struct Card {
    rank: Rank,
    suit: Suit
}

#[allow(dead_code)]
impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank: rank, suit: suit }
    }

    fn copy(&self) -> Card {
        Card { rank: self.rank, suit: self.suit }
    }
    
    fn to_string(&self) -> String {
        let mut str = String::with_capacity(2);
        str.push_str(&self.rank.to_string());
        str.push_str(&self.suit.to_string());
        return str;
    }
}

fn make_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);
    for rank in Rank::iter() {
        for suit in Suit::iter() {
            let card = Card { rank: rank, suit: suit };
            deck.push(card);
        }
    }

    
    return deck;
}

fn shuffle(deck: &mut Vec<Card>) {
    let mut rng = rand::thread_rng();
    let n = deck.len();
    for i in 0 .. n - 2 {
        let j = rng.gen_range(i, n);
        deck.swap(i, j);
    }
}

fn shuffle_deck(deck: &Vec<Card>) -> Vec<Card> {
    let mut copy = deck.to_vec();
    shuffle(&mut copy);
    return copy;
}

fn make_shuffled_deck() -> Vec<Card> {
    shuffle_deck(&make_deck())
}

fn cmp_card_rank(a: &Card, b: &Card) -> std::cmp::Ordering {
    b.rank.cmp(&a.rank)
}

fn cmp_card_suit(a: &Card, b: &Card) -> std::cmp::Ordering {
    b.suit.cmp(&a.suit)
}

fn rank_sort(deck: &mut Vec<Card>) {
    deck.sort_by(cmp_card_rank);
}

fn suit_sort(deck: &mut Vec<Card>) {
    deck.sort_by(cmp_card_suit);
}

fn sort(deck: &mut Vec<Card>) {
    suit_sort(deck);
    rank_sort(deck);
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

fn fmt_cards(cards: &[Card]) -> String {
    return cards.iter()
        .map(|card| card.to_string())
        .collect::<Vec<String>>()
        .join(" ");
}

fn fmt_card_refs(cards: &[&Card]) -> String {
    return cards.iter()
        .map(|card| card.to_string())
        .collect::<Vec<String>>()
        .join(" ");
}

fn cmp_size<T>(a: &[T], b:&[T]) -> std::cmp::Ordering {
    return b.len().cmp(&a.len());
}

fn canonical_order<'a>(rank_map: &'a HashMap<Rank, Vec<&Card>>) -> Vec<Card> {
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
            // println!("{:?}", rank_cards);
            for card in rank_cards {
                canonical_cards.push(card.copy());
            }
        }
    }

    return canonical_cards;
}

fn is_quads(canonical_cards: &[Card]) -> bool {
    canonical_cards[0].rank == canonical_cards[1].rank
        && canonical_cards[1].rank == canonical_cards[2].rank
        && canonical_cards[2].rank == canonical_cards[3].rank
}

fn is_full_house(canonical_cards: &[Card]) -> bool {
    canonical_cards[0].rank == canonical_cards[1].rank
        && canonical_cards[1].rank == canonical_cards[2].rank
        && canonical_cards[3].rank == canonical_cards[4].rank
}

fn is_trips(canonical_cards: &[Card]) -> bool {
    canonical_cards[0].rank == canonical_cards[1].rank
        && canonical_cards[1].rank == canonical_cards[2].rank
}

fn is_two_pair(canonical_cards: &[Card]) -> bool {
    canonical_cards[0].rank == canonical_cards[1].rank
        && canonical_cards[2].rank == canonical_cards[3].rank
}

fn is_pair(canonical_cards: &[Card]) -> bool {
    canonical_cards[0].rank == canonical_cards[1].rank
}

fn evaluate(cards: &[Card]) -> (HandCategory, Vec<Card>) {
    // let rank_count = 15;
    // let mut byRank = Vec::with_capacity(rank_count);

    // for n in 0..rank_count {
    //     byRank.push(Vec::new());
    // }
    
    // for card in cards {
    //     let rank_ord = card.rank as usize;
    //     byRank[rank_ord].push(card);
    // }

    // for rank in byRank {
    //     println!("{:?}", rank);
    // }

    let mut rank_map = HashMap::new();
    for rank in Rank::iter() {
        rank_map.insert(rank, Vec::new());
    }
    
    for card in cards {
        if let Some(mut rank_vector) = rank_map.get_mut(&card.rank) {
            rank_vector.push(card);
        }
    }

    let canonical_cards = canonical_order(&rank_map);
    
    if is_quads(&canonical_cards) {
        return (HandCategory::Quads, canonical_cards);
    } else if is_full_house(&canonical_cards) {
        return (HandCategory::FullHouse, canonical_cards);
    } else if is_trips(&canonical_cards) {
        return (HandCategory::Triplets, canonical_cards);
    } else if is_two_pair(&canonical_cards) {
        return (HandCategory::OnePair, canonical_cards);
    } else if is_pair(&canonical_cards) {
        return (HandCategory::OnePair, canonical_cards);
    } else {
        return (HandCategory::HighCard, canonical_cards);
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
    for pocket in &pockets {
        let mut cards = pocket.to_vec();
        cards.extend(board.to_vec());

        let (category, sorted_cards) = evaluate(&cards);
        println!("Pocket: {} -> {} -> {}", fmt_cards(&pocket), fmt_cards(&sorted_cards), category.to_string());
    }
}

fn main() {
    let mut deck = make_shuffled_deck();
    // for card in &deck {
    //     println!("{}", card.to_string());
    // }

    deal(&mut deck, 3);
    
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
