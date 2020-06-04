use std::collections::HashMap;
use strum::IntoEnumIterator;

use super::card::Rank;
use super::card::Card;

pub struct RankMap(HashMap<Rank, Vec<Card>>);

impl RankMap {
    pub fn new(cards: &[Card]) -> RankMap {
        let mut rank_map = HashMap::new();
        for rank in Rank::iter() {
            rank_map.insert(rank, Vec::new());
        }
        
        for card in cards {
            if let Some(rank_vector) = rank_map.get_mut(&card.rank) {
                rank_vector.push(Card::copy(card));
            }
        }
        
        return RankMap(rank_map);
    }

    pub fn flatten(&self) -> Vec<Card> {
        let mut cards = Vec::new();
        for ranked_cards in self.0.values() {
            for card in ranked_cards {
                cards.push(Card::copy(card));
            }
        }
        
        return cards;
    }

    pub fn get(&self, rank: &Rank) -> Option<&Vec<Card>> {
        self.0.get(rank)
    }
}
