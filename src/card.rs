#[derive(Debug, Clone)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid strings")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Ord, PartialOrd, Display)]
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

impl std::str::FromStr for Suit {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str == "h" || str == "♥" {
            Ok(Suit::Hearts)
        } else if str == "c" || str == "♣" {
            Ok(Suit::Clubs)
        } else if str == "d" || str == "♦" {
            Ok(Suit::Diamonds)
        } else if str == "s" || str == "♠" {
            Ok(Suit::Spades)
        } else {
            Err(ParseError)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, Ord, PartialOrd, Hash, Display)]
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

#[derive(Clone, Debug, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit
}

#[allow(dead_code)]
impl Card {
    pub const fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank: rank, suit: suit }
    }

    pub const fn copy(&self) -> Card {
        Card { rank: self.rank, suit: self.suit }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[allow(dead_code)]
impl Rank {
    pub fn of(&self, suit: Suit) -> Card {
        Card::new(*self, suit)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn fmt_cards(cards: &[Card]) -> String {
    return cards.iter()
        .map(|card| card.to_string())
        .collect::<Vec<String>>()
        .join(" ");
}

pub struct Cards<'a>(pub &'a [Card]);

impl std::fmt::Display for Cards<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", fmt_cards(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Rank::*;
    use Suit::*;
    
    #[test]
    fn test_card_to_string() {
        assert_eq!(Card::new(Ace, Clubs).to_string(), "A♣");
        assert_eq!(Card::new(King, Hearts).to_string(), "K♥");
        assert_eq!(Card::new(Queen, Diamonds).to_string(), "Q♦");
        assert_eq!(Card::new(Jack, Spades).to_string(), "J♠");
        assert_eq!(Card::new(Ten, Diamonds).to_string(), "T♦");
    }

    #[test]
    fn test_card_cmp() {
        let ace_clubs = Ace.of(Clubs);
        let ace_diamonds = Ace.of(Diamonds);
        let king_diamonds = King.of(Diamonds);

        assert_eq!(ace_clubs.cmp(&ace_diamonds), std::cmp::Ordering::Equal);
        assert_eq!(ace_diamonds.cmp(&ace_clubs), std::cmp::Ordering::Equal);
        assert_eq!(ace_clubs.cmp(&king_diamonds), std::cmp::Ordering::Greater);
        assert_eq!(king_diamonds.cmp(&ace_diamonds), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_suit_parsing() {
        assert_eq!("c".parse::<Suit>().unwrap(), Clubs);
        assert_eq!("d".parse::<Suit>().unwrap(), Diamonds);
        assert_eq!("h".parse::<Suit>().unwrap(), Hearts);
        assert_eq!("s".parse::<Suit>().unwrap(), Spades);
        assert_eq!("♣".parse::<Suit>().unwrap(), Clubs);
        assert_eq!("♦".parse::<Suit>().unwrap(), Diamonds);
        assert_eq!("♥".parse::<Suit>().unwrap(), Hearts);
        assert_eq!("♠".parse::<Suit>().unwrap(), Spades);
    }
}
