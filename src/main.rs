#![feature(iterator_fold_self)]
#![feature(drain_filter)]
#![feature(vec_remove_item)]
#![allow(dead_code)]

extern crate strum;
#[macro_use] extern crate strum_macros;

use std::cmp::Ordering;
use itertools::Itertools;

mod card;
use card::Card;
use card::Cards;
use card::CardVector;
use card::fmt_cards;

mod deck;
use deck::make_deck;
use deck::make_shuffled_deck;

mod poker_hand;
use poker_hand::PokerHand;
use poker_hand::make_poker_hand;

mod win_lose_split;
use win_lose_split::WinLoseSplit;

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

    // let mut evals = Vec::new();
    // for pocket in &pockets {
    //     let mut cards = pocket.to_vec();
    //     cards.extend(board.to_vec());

    //     let poker_hand = make_poker_hand(&cards, &None);
    //     evals.push((pocket, poker_hand));
    // }

    // evals.sort_by(|a, b| a.1.cmp(&b.1));
    // evals.reverse();

    // for eval in evals {
    //     let (pocket, poker_hand) = eval;
    //     println!("Pocket: {} -> {}", Cards(&pocket), poker_hand);
    // }
}

fn find_winners(pockets: &Vec<Vec<Card>>, board: &Vec<&Card>) -> Vec<usize> {
    let mut vec = Vec::new();
    
    let mut best_hand = None;
    for (index, pocket) in pockets.iter().enumerate() {
        let mut current = board.to_vec();
        current.extend(pocket);
        let hand = make_poker_hand(&current, &None);
        
        if let Some(max) = &best_hand {
            match hand.cmp(&max) {
                Ordering::Equal => vec.push(index),
                Ordering::Greater => {
                    vec.clear();
                    vec.push(index);
                    best_hand = Some(hand)
                },
                Ordering::Less => {}
            }
        } else {
            vec.push(index);
            best_hand = Some(hand);
        }
    }
    
    return vec;
}

fn hold_em_odds(pockets: &Vec<Vec<Card>>, board: &Vec<Card>) -> Vec<WinLoseSplit> {
    let mut deck = make_deck();
    for pocket in pockets {
        for card in pocket {
            deck.remove_item(&card);
        }
    }
    
    for card in board {
        deck.remove_item(&card);
    }

    let mut results = vec![WinLoseSplit::new(); pockets.len()];

    let n = 5 - board.len();
    for combination in deck.iter().combinations(n) {
        let complete_board = board.iter().chain(combination).collect::<Vec<_>>();
        
        let winners = find_winners(pockets, &complete_board);
        for index in 0..results.len() { 
            if winners.contains(&index) {
                if winners.len() == 1 {
                    results[index].wins += 1;
                } else {
                    results[index].splits += 1;
                }
            } else {
                results[index].losses += 1;
            }
        }
    }
    
    return results;
}

fn random_deals() {
    for _n in 0..100 {
        let mut deck = make_shuffled_deck();
        deal(&mut deck, 8);
    }
}

fn enumerate_deals() {
    let pocket_ace_king = CardVector::parse("Ac Kc");
    let pocket_eights = CardVector::parse("8s 8d");

    let mut pockets = Vec::new();
    pockets.push(pocket_ace_king.0);
    pockets.push(pocket_eights.0);

    // let board = CardVector::parse("7c 8c 3s");
    let board = Vec::new();
    let results = hold_em_odds(&pockets, &board);

    println!("Board: {}", fmt_cards(&board));
    for i in 0..results.len() {
        let p = &pockets[i];
        let r = results[i];
        println!("- {} - {}", fmt_cards(&p), r);
    }
}

fn main() {
    // random_deals();
    enumerate_deals();
}
