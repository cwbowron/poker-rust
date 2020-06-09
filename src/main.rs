extern crate strum;
#[macro_use] extern crate strum_macros;

use std::cmp::Ordering;
use itertools::Itertools;

mod card;
use card::Card;
use card::Cards;
use card::CardVector;
use card::fmt_cards;
use card::remove_cards;
use card::add_cards;

mod deck;
use deck::make_deck;

mod poker_hand;
use poker_hand::PokerHand;
use poker_hand::make_poker_hand;

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

        let poker_hand = make_poker_hand(&cards, &None);
        evals.push((pocket, poker_hand));
    }

    evals.sort_by(|a, b| a.1.cmp(&b.1));
    evals.reverse();

    for eval in evals {
        let (pocket, poker_hand) = eval;
        println!("Pocket: {} -> {}", Cards(&pocket), poker_hand);
    }
}

pub struct WinLoseSplit {
    wins: i32,
    losses: i32,
    splits: i32
}

impl WinLoseSplit {
    pub fn new() -> WinLoseSplit {
        WinLoseSplit {
            wins: 0,
            losses: 0,
            splits: 0
        }
    }
}

impl Copy for WinLoseSplit {}

impl Clone for WinLoseSplit {
    fn clone(&self) -> Self {
        WinLoseSplit {
            wins: self.wins,
            losses: self.losses,
            splits: self.splits
        }
    }
}


fn hold_em_odds(deck: &[Card], pockets: &Vec<Vec<Card>>, board: &Vec<Card>) -> Vec<WinLoseSplit> {
    let mut results = vec![WinLoseSplit::new(); pockets.len()];
    let mut count = 0;
    for combination in deck.iter()
        .combinations(5 - board.len()) {
            let mut b = board.to_vec();
            for card in combination {
                b.push(card.copy());
            }

            let mut poker_hands = pockets.iter()
                .map(|pocket| add_cards(pocket, &b))
                .map(|cards| make_poker_hand(&cards, &None))
                .enumerate()
                .collect::<Vec<_>>();

            poker_hands.sort_by(|a, b| a.1.cmp(&b.1));
            poker_hands.reverse();

            let best = &poker_hands.first().unwrap().1;

            let winners = poker_hands.iter()
                .filter(|tuple| tuple.1.cmp(&best) == Ordering::Equal)
                .collect::<Vec<_>>();

            for winner in winners.iter() {
                if (winners.len() == 1) {
                    results[winner.0].wins += 1;
                } else {
                    results[winner.0].splits += 1;
                }
            }
                
            let losers = poker_hands.iter()
                .filter(|tuple| tuple.1.cmp(&best) != Ordering::Equal);

            for loser in losers {
                results[loser.0].losses += 1;
            }

            count += 1;
        }

    return results;
}

fn main() {
    // for _n in 0..100 {
    //     let mut deck = make_shuffled_deck();
    //     deal(&mut deck, 8);
    // }
    let mut deck = make_deck();
    let pocket_ace_king = CardVector::parse("Ac Kc");
    deck = remove_cards(&deck, &pocket_ace_king);
    
    let pocket_eights = CardVector::parse("8s 8d");
    deck = remove_cards(&deck, &pocket_eights);

    let mut pockets = Vec::new();
    pockets.push(pocket_ace_king.0);
    pockets.push(pocket_eights.0);

    let board = CardVector::parse("7c 5c 4s");
    let results = hold_em_odds(&deck, &pockets, &board);

    println!("Board: {}", fmt_cards(&board));
    for i in 0..results.len() {
        let p = &pockets[i];
        let r = results[i];
        println!("- {} - {} {} {}", fmt_cards(&p), r.wins, r.losses, r.splits);
    }
}
