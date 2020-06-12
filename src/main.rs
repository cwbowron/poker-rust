#![feature(iterator_fold_self)]
#![feature(drain_filter)]
#![feature(vec_remove_item)]
#![allow(dead_code)]

extern crate strum;
#[macro_use] extern crate strum_macros;

use std::cmp::Ordering;
use itertools::Itertools;
use clap::{App, Arg};
use strum::IntoEnumIterator;

mod card;
use card::{Card, Cards, CardVector, fmt_cards};

mod deck;
use deck::{make_deck, make_shuffled_deck};

mod poker_hand;
use poker_hand::{PokerHand, HandRank};

mod win_lose_split;
use win_lose_split::WinLoseSplit;

struct HandRankCount(Vec<usize>);
impl HandRankCount {
    pub fn new() -> HandRankCount {
        HandRankCount(vec![0; 1 + HandRank:: StraightFlush as usize])
    }

    pub fn inc(&mut self, rank: HandRank) {
        self.0[rank as usize] += 1;
    }
}

impl std::ops::Deref for HandRankCount {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for HandRankCount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let total = self.0.iter()
            .fold(0, |acc, current| current + acc) as f32;
        
        for rank in HandRank::iter() {
            let count = self.0[rank as usize] as f32;
            let p = 100.0 * count / total;
            write!(f, "{:14} - {:5.2} %%\n", rank.to_string(), p);
        }
        Ok(())
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

        let poker_hand = PokerHand::build(&cards.iter().collect::<Vec<_>>(), &None);
        evals.push((pocket, poker_hand));
    }

    evals.sort_by(|a, b| a.1.cmp(&b.1));
    evals.reverse();

    for eval in evals {
        let (pocket, poker_hand) = eval;
        println!("Pocket: {} -> {}", Cards(&pocket), poker_hand);
    }
}

fn find_winners(pockets: &Vec<Vec<Card>>, board: &Vec<&Card>, hand_rank_counts: &mut Vec<HandRankCount>) -> Vec<usize> {
    let mut vec = Vec::new();
    
    let mut best_hand = None;
    for (index, pocket) in pockets.iter().enumerate() {
        let mut current = board.to_vec();
        current.extend(pocket);
        let hand = PokerHand::build(&current, &None);

        hand_rank_counts[index].inc(hand.rank);
        
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

fn hold_em_odds(pockets: &Vec<Vec<Card>>, board: &Vec<Card>, hand_rank_counts: &mut Vec<HandRankCount>) -> Vec<WinLoseSplit> {
    let mut deck = make_deck();
    for card in pockets.iter().flatten().chain(board.iter()) {
        deck.remove_item(&card);
    }

    let mut results = vec![WinLoseSplit::new(); pockets.len()];

    let n = 5 - board.len();
    for combination in deck.iter().combinations(n) {
        let complete_board = board.iter().chain(combination).collect::<Vec<_>>();
        
        let winners = find_winners(pockets, &complete_board, hand_rank_counts);
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

fn enumerate_deals(pockets: Vec<Vec<Card>>, board: &Vec<Card>) {
    let mut hand_rank_counts = Vec::new();
    for _i in 0..pockets.len() {
        hand_rank_counts.push(HandRankCount::new());
    }

    let results = hold_em_odds(&pockets, board, &mut hand_rank_counts);

    if board.len() > 0 {
        println!("Board: {}", fmt_cards(&board));
    }

    for i in 0..results.len() {
        let p = &pockets[i];
        let r = results[i];
        println!("- {} - {}", fmt_cards(&p), r);
    }

    println!("\n");
    for i in 0..hand_rank_counts.len() {
        println!("{}", fmt_cards(&pockets[i]));
        println!("{}", hand_rank_counts[i]);
    }
}

fn main() {
    let matches = App::new("poker-rust")
        .version("1.0")
        .author("Chris Bowron <cwbowron@gmail.com>")
        .about("Calculate poker odds")
        .arg(Arg::new("board")
             .short('b')
             .long("board")
             .about("Board")
             .takes_value(true))
        .arg(Arg::new("pocket")
             .multiple(true)
             .index(1)
             .about("Pocket cards"))
        .subcommand(App::new("montecarlo")
                    .about("Monte Carlo Texas Hold 'em Simulation"))
        .get_matches();
             
    if matches.is_present("montecarlo") {
        random_deals();
    } else {
        let board_string = matches.value_of("board").unwrap_or("");
        let board = CardVector::parse(board_string);
        
        if let Some(pocket_strings) = matches.values_of("pocket") {
            let pockets  = pocket_strings
                .map(|str| CardVector::parse(str).to_vec())
                .collect::<Vec<Vec<Card>>>();
            
            enumerate_deals(pockets, &board);
        }
    }
}
