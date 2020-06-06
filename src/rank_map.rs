use std::collections::HashMap;
use strum::IntoEnumIterator;

use super::card::Rank;
use super::card::Card;
use super::card::IsWildCard;

type _RankMap = HashMap<Rank, Vec<Card>>;

pub struct RankMap(_RankMap);

impl RankMap {
    fn _new() -> RankMap {
        RankMap(HashMap::new())
    }
    
    pub fn with_wild_cards(cards: &[Card], _is_wild: &Option<IsWildCard>) -> RankMap {
        let mut rank_map = Self::_new();
        
        for card in cards {
            match rank_map.0.get_mut(&card.rank) {
                Some(rank_vector) => rank_vector.push(Card::copy(card)),
                None => {
                    rank_map.0.insert(card.rank, vec![Card::copy(card)]);
                }
            }
        }
        
        rank_map
    }

    pub fn new(cards: &[Card]) -> RankMap {
        Self::with_wild_cards(cards, &None)
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

impl std::ops::Deref for RankMap {
    type Target = _RankMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl std::ops::Index<&Rank> for RankMap {
//     type Output = Vec<Card>;
//     fn index(&self, index: &Rank) -> &Self::Output {
//         &self.0.get(index).unwrap()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardVector;
    use Rank::*;

    #[test]
    fn test_rank_map() {
        let cards = CardVector::parse("Ac Ad As Qc Tc Jc");
        let rank_map = RankMap::new(&cards);

        assert_eq!(rank_map.0.get(&Ace).unwrap().len(), 3);
        assert_eq!(rank_map.0.get(&Queen).unwrap().len(), 1);
        assert_eq!(rank_map.0.get(&Jack).unwrap().len(), 1);
        assert_eq!(rank_map.0.get(&Ten).unwrap().len(), 1);
    }

    // #[test]
    // fn test_rank_map_with_wild_cards() {
    //     let cards = CardVector::parse("Ac Ad As Qc Tc Jc");
    //     let rank_map = RankMap::new(&cards);

    //     assert_eq!(rank_map.0.get(&Ace).unwrap().len(), 3);
    //     assert_eq!(rank_map.0.get(&Queen).unwrap().len(), 1);
    //     assert_eq!(rank_map.0.get(&Jack).unwrap().len(), 1);
    //     assert_eq!(rank_map.0.get(&Ten).unwrap().len(), 1);
    // }
}
