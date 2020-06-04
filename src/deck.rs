use strum::IntoEnumIterator;

use rand::Rng;

use super::card::Suit;
use super::card::Rank;
use super::card::Card;

pub fn make_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);
    for rank in Rank::iter() {
        if rank != Rank::LowAce {
            for suit in Suit::iter() {
                deck.push(Card::new(rank, suit));
            }
        }
    }
    
    return deck;
}

pub fn shuffle(deck: &mut Vec<Card>) {
    let mut rng = rand::thread_rng();
    let n = deck.len();
    for i in 0 .. n - 2 {
        let j = rng.gen_range(i, n);
        deck.swap(i, j);
    }
}

pub fn shuffle_deck(deck: &Vec<Card>) -> Vec<Card> {
    let mut copy = deck.to_vec();
    shuffle(&mut copy);
    return copy;
}

pub fn make_shuffled_deck() -> Vec<Card> {
    shuffle_deck(&make_deck())
}

