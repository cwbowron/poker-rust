use std::collections::HashMap;
use strum::IntoEnumIterator;

use super::card::Rank;
use super::card::Card;

type _RankMap = HashMap<Rank, Vec<Card>>;

pub struct RankMap(_RankMap);

impl RankMap {
    pub fn new(cards: &[Card]) -> RankMap {
        let mut rank_map: _RankMap = HashMap::new();
        
        for card in cards {
            match rank_map.get_mut(&card.rank) {
                Some(rank_vector) => rank_vector.push(Card::copy(card)),
                None => {
                    rank_map.insert(card.rank, vec![Card::copy(card)]);
                }
            }
        }
        
        return RankMap(rank_map);
    }

    pub fn take_set(&self, size: usize) -> Option<Vec<Card>> {
        for rank in Rank::iter() {
            if let Some(ranked_cards) = &self.0.get(&rank) {
                if ranked_cards.len() >= size {
                    return Some(ranked_cards[0..size].to_vec());
                }
            }
        }

        return None;
    }

    pub fn remove(&self, cards: &[Card]) -> RankMap {
        let filtered_cards = self.0
            .values()
            .flatten()
            .filter(|card| !cards.contains(card))
            .map(Card::copy)
            .collect::<Vec<Card>>();
        
        return RankMap::new(&filtered_cards);
    }
}

// impl std::ops::Index<&Rank> for RankMap {
//     type Output = Vec<Card>;
//     fn index(&self, index: &Rank) -> &Self::Output {
//         &self.0.get(index).unwrap()
//     }
// }
