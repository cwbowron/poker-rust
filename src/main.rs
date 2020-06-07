extern crate strum;
#[macro_use] extern crate strum_macros;

mod card;
use card::Card;
use card::Cards;

mod deck;
use deck::make_shuffled_deck;

mod poker_hand;
use poker_hand::PokerHand;

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

    evals.sort_by(|a, b| a.1.cmp(&b.1));
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
