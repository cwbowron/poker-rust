#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, ToString, Ord, PartialOrd)]
pub enum Suit {
    #[strum(to_string = "♣")]
    Clubs,

    #[strum(to_string = "♦")]
    Diamonds,

    #[strum(to_string = "♥")]
    Hearts,
    
    #[strum(to_string = "♠")]
    Spades
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, ToString, Ord, PartialOrd, Hash)]
pub enum Rank {
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
    Two = 2,
    #[strum(to_string = "A")]
    LowAce = 1
}

#[derive(Clone, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit
}

#[allow(dead_code)]
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank: rank, suit: suit }
    }

    pub fn copy(&self) -> Card {
        Card { rank: self.rank, suit: self.suit }
    }
    
    pub fn to_string(&self) -> String {
        let mut str = String::with_capacity(2);
        str.push_str(&self.rank.to_string());
        str.push_str(&self.suit.to_string());
        return str;
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}

pub fn cmp_card_rank(a: &Card, b: &Card) -> std::cmp::Ordering {
    b.rank.cmp(&a.rank)
}

pub fn cmp_card_suit(a: &Card, b: &Card) -> std::cmp::Ordering {
    b.suit.cmp(&a.suit)
}

pub fn rank_sort(deck: &mut Vec<Card>) {
    deck.sort_by(cmp_card_rank);
}

pub fn suit_sort(deck: &mut Vec<Card>) {
    deck.sort_by(cmp_card_suit);
}

pub fn sort(deck: &mut Vec<Card>) {
    suit_sort(deck);
    rank_sort(deck);
}

pub fn fmt_cards(cards: &[Card]) -> String {
    return cards.iter()
        .map(|card| card.to_string())
        .collect::<Vec<String>>()
        .join(" ");
}
