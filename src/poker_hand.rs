// TODO handle wild cards in straight, flush, straight flush
use strum::IntoEnumIterator;
use std::cmp::Ordering;

use super::card::Suit;
use super::card::Rank;
use super::card::Card;
use super::card::Cards;
use super::card::IsWildCard;

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

fn remove_cards(a: &[Card], b: &[Card]) -> Vec<Card> {
    return a.iter()
        .filter(|card| !b.contains(card))
        .map(Card::copy)
        .collect::<Vec<Card>>();
}

fn remove_card(a: &[Card], b: &Card) -> Vec<Card> {
    let mut found = false;
    return a.iter()
        .filter(|card| {
            if found || *card != b {
                true
            } else {
                found = true;
                false
            }
        })
        .map(Card::copy)
        .collect::<Vec<_>>();
}

fn find_set(cards: &[Card], n: usize, is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for rank in Rank::iter() {
        if rank != Rank::LowAce && rank != Rank::Joker {
            let filtered = cards.iter()
                .filter(|card| card.rank == rank || card.is_wild(is_wild))
                .collect::<Vec<&Card>>();

            if filtered.len() >= n {
                return Some(filtered.iter()
                            .map(|card_ref_ref| Card::copy(*card_ref_ref))
                            .collect::<Vec<Card>>());
            }
        }
    }

    return None;
}

fn make_sets_worker(cards: &[Card], sizes: &mut Vec<usize>, is_wild: &Option<IsWildCard>, result: &mut Vec<Card>) -> bool {
    if let Some(set_size) = sizes.pop() {
        if let Some(set) = find_set(cards, set_size, is_wild) {
            let next_cards = remove_cards(cards, &set);
            result.extend(set);
            return make_sets_worker(&next_cards, sizes, is_wild, result);
        }
        return false;
    } else {
        return true;
    }
}

fn make_sets(cards: &[Card], set_sizes: &Vec<usize>, is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    let mut sizes_copy = set_sizes.to_vec();
    sizes_copy.reverse();
    let mut hand = Vec::new();
    if make_sets_worker(cards, &mut sizes_copy, is_wild, &mut hand) {
        return Some(hand);
    } else {
        return None;
    }
}

fn as_quads(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![4, 1], is_wild);
}

fn as_full_house(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![3, 2], is_wild);
}

fn as_trips(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![3, 1, 1], is_wild);
}

fn as_two_pair(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![2, 2, 1], is_wild);
}

fn as_pair(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    return make_sets(cards, &vec![2, 1, 1, 1], is_wild);
}

fn as_high_card(cards: &[Card], _is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort();
    sorted_cards.reverse();
    return Some(sorted_cards[0..5].to_vec());
}

// TODO do this better
fn build_straight(cards: &[Card], is_wild:&Option<IsWildCard>, ranks: &[Rank], rank_index: usize, result: &mut Vec<Card>) -> bool {
    if rank_index >= ranks.len() {
        return true;
    } else {
        let rank = ranks[rank_index];
        if let Some(card) = cards.iter()
            .filter(|card| !card.is_wild(is_wild))
            .find(|card| card.rank == rank || (card.rank == Rank::Ace && rank == Rank::LowAce)) {
                result.push(Card::copy(card));
                let remaining_cards = remove_card(cards, card);
                if build_straight(&remaining_cards, is_wild, ranks, rank_index + 1, result) {
                return true;
                } else {
                    result.pop();
                }
            }
        
        if let Some(card) = cards.iter()
            .find(|card| card.is_wild(is_wild)) {
                result.push(Card::copy(card));
                let remaining_cards = remove_card(cards, card);
                if build_straight(&remaining_cards, is_wild, ranks, rank_index + 1, result) {
                    return true;
                } else {
                    result.pop();
                }
            }
        
        return false;
    }
}

// TODO do this better
fn as_straight(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    let straights = vec![
        vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten],
        vec![Rank::King, Rank::Queen, Rank::Jack, Rank::Ten, Rank::Nine],
        vec![Rank::Queen, Rank::Jack, Rank::Ten, Rank::Nine, Rank::Eight],
        vec![Rank::Jack, Rank::Ten, Rank::Nine, Rank::Eight, Rank::Seven],
        vec![Rank::Ten, Rank::Nine, Rank::Eight, Rank::Seven, Rank::Six],
        vec![Rank::Nine, Rank::Eight, Rank::Seven, Rank::Six, Rank::Five],
        vec![Rank::Eight, Rank::Seven, Rank::Six, Rank::Five, Rank::Four],
        vec![Rank::Seven, Rank::Six, Rank::Five, Rank::Four, Rank::Three],
        vec![Rank::Six, Rank::Five, Rank::Four, Rank::Three, Rank::Two],
        vec![Rank::Five, Rank::Four, Rank::Three, Rank::Two, Rank::LowAce],
    ];

    for ranks in straights {
        let mut result = Vec::with_capacity(5);
        if build_straight(&cards, is_wild, &ranks, 0, &mut result) {
            return Some(result);
        }
    }

    return None;
}

fn as_flush(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited: Vec<&Card> = cards
            .iter()
            .filter(|card| card.suit == suit || card.is_wild(is_wild))
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

fn as_straight_flush(cards: &[Card], is_wild: &Option<IsWildCard>) -> Option<Vec<Card>> {
    for suit in Suit::iter() {
        let suited: Vec<&Card> = cards
            .iter()
            .filter(|card| card.suit == suit || card.is_wild(is_wild))
            .collect();
        
        if suited.len() >= 5 {
            let suited_cards = suited
                .iter()
                .map(|card_ref| Card::copy(card_ref))
                .collect::<Vec<Card>>();

            if let Some(straight) = as_straight(&suited_cards, is_wild) {
                return Some(straight);
            }
        }
    }
    return None;
}

impl HandCategory {
    fn get_test(&self) -> fn(&[Card], &Option<IsWildCard>) -> Option<Vec<Card>> {
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
    pub fn with_wild_cards(cards: &[Card], is_wild: &Option<IsWildCard>) -> PokerHand {
        for category in HandCategory::iter().rev() {
            if let Some(result_cards) = category.get_test()(&cards, is_wild) {
                return PokerHand {
                    category: category,
                    cards: result_cards
                };
            }
        }
        
        unreachable!();
    }

    pub fn new(cards: &[Card]) -> PokerHand {
        Self::with_wild_cards(cards, &None)
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
    // TODO handle wild cards
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
        PokerHand::with_wild_cards(&CardVector::parse(card_string), &Some(Card::is_joker))
    }

    #[test]
    fn test_remove_card() {
        let cards0 = CardVector::parse("Kc ?? ??");
        assert_eq!(cards0.len(), 3);

        let cards1 = remove_card(&cards0, &Rank::Joker.of(Suit::Joker));
        assert_eq!(cards1.len(), 2);

        let cards2 = remove_card(&cards1, &Rank::Joker.of(Suit::Joker));
        assert_eq!(cards2.len(), 1);
    }

    #[test]
    fn test_straight_flush() {
        let poker_hand = parse_hand("Ac Kc Qc Tc Jc");
        assert_eq!(poker_hand.category, StraightFlush);
    }

    #[test]
    fn test_straight_flush_with_jokers() {
        assert_eq!(parse_hand("Ac Kc Qc ?? Jc").category, StraightFlush);
        assert_eq!(parse_hand("Ac Kc ?? ?? Jc").category, StraightFlush);
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
    fn test_one_joker() {
        assert_eq!(parse_hand("Ac As Ad ?? Jd").category, Quads);
        assert_eq!(parse_hand("Ac As ?? Jc Jd").category, FullHouse);
        assert_eq!(parse_hand("Ac As ?? Jc Td").category, Triplets);
        assert_eq!(parse_hand("Ac ?? Jc Td 7c").category, OnePair);
    }

    #[test]
    fn test_two_joker() {
        assert_eq!(parse_hand("Ac As ?? ?? Jd").category, Quads);
        assert_eq!(parse_hand("Ac ?? ?? Td 7c").category, Triplets);
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
    fn test_flush_with_jokers() {
        assert_eq!(parse_hand("Ac Kc 7c Tc ??").category, Flush);
        assert_eq!(parse_hand("Ac Kc ?? Jc 7c").category, Flush);
    }
    
    #[test]
    fn test_straight() {
        let poker_hand = parse_hand("Ac Kc Qc Ts Jd");
        assert_eq!(poker_hand.category, Straight);
    }

    #[test]
    #[ignore]
    fn test_low_straight() {
        assert_eq!(parse_hand("Ac 5c 4s 3s 2d").category, Straight);
        assert_eq!(parse_hand("5c 4s 3s 2d ??").category, Straight);
        assert_eq!(parse_hand("?? 4s 3s 2d Ac").category, Straight);
    }

    #[test]
    #[ignore]
    fn test_straight_with_jokers() {
        assert_eq!(parse_hand("Ac Kc Qc Jd ??").category, Straight);
        assert_eq!(parse_hand("Ac Kc ?? ?? Ts").category, Straight);
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
