use strum::IntoEnumIterator;

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Ord, PartialOrd, Hash, Display)]
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

impl std::str::FromStr for Rank {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let lower_case = str.to_ascii_lowercase();
        for rank in Self::iter() {
            if rank != Rank::LowAce {
                if lower_case == rank.to_string().to_ascii_lowercase() {
                    return Ok(rank);
                }
            }
        }
        return Err(ParseError)
    }
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

impl std::str::FromStr for Card {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let trimmed = str.trim();
        let mut chars = trimmed.chars();
        let rank = chars.next().unwrap().to_string().parse::<Rank>()?;
        let suit = chars.next().unwrap().to_string().parse::<Suit>()?;
        Ok(Card::new(rank, suit))
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

pub struct CardVector(pub Vec<Card>);

impl std::str::FromStr for CardVector {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let cards = str.replace(" ", "")
            .replace(",", "")
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| {
                let rank = chunk[0].to_string().parse::<Rank>().unwrap();
                let suit = chunk[1].to_string().parse::<Suit>().unwrap();
                Card::new(rank, suit)
            }).collect::<Vec<Card>>();

        Ok(CardVector(cards))
    }
}

impl std::ops::Index<usize> for CardVector {
    type Output = Card;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<CardVector> for Vec<Card> {
    fn from(card_vector: CardVector) -> Vec<Card> {
        card_vector.0
    }
}

impl std::fmt::Display for CardVector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", fmt_cards(&self.0))
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

    #[test]
    fn test_rank_parsing() {
        assert_eq!("a".parse::<Rank>().unwrap(), Ace);
        assert_eq!("k".parse::<Rank>().unwrap(), King);
        assert_eq!("q".parse::<Rank>().unwrap(), Queen);
        assert_eq!("j".parse::<Rank>().unwrap(), Jack);
        assert_eq!("t".parse::<Rank>().unwrap(), Ten);
        assert_eq!("9".parse::<Rank>().unwrap(), Nine);
        assert_eq!("8".parse::<Rank>().unwrap(), Eight);
        assert_eq!("7".parse::<Rank>().unwrap(), Seven);
        assert_eq!("6".parse::<Rank>().unwrap(), Six);
        assert_eq!("5".parse::<Rank>().unwrap(), Five);
        assert_eq!("4".parse::<Rank>().unwrap(), Four);
        assert_eq!("3".parse::<Rank>().unwrap(), Three);
        assert_eq!("2".parse::<Rank>().unwrap(), Two);
    }

    #[test]
    fn test_card_parsing() {
        assert_eq!("Ac".parse::<Card>().unwrap(), Ace.of(Clubs));
        assert_eq!(" Kd".parse::<Card>().unwrap(), King.of(Diamonds));
        assert_eq!("Qh ".parse::<Card>().unwrap(), Queen.of(Hearts));
        assert_eq!("Js".parse::<Card>().unwrap(), Jack.of(Spades));
        assert_eq!("Tc".parse::<Card>().unwrap(), Ten.of(Clubs));
        assert_eq!("9s".parse::<Card>().unwrap(), Nine.of(Spades));
        assert_eq!("8h".parse::<Card>().unwrap(), Eight.of(Hearts));
        assert_eq!("7h".parse::<Card>().unwrap(), Seven.of(Hearts));
        assert_eq!("6♥".parse::<Card>().unwrap(), Six.of(Hearts));
        assert_eq!("5♣".parse::<Card>().unwrap(), Five.of(Clubs));
        assert_eq!("4♦".parse::<Card>().unwrap(), Four.of(Diamonds));
        assert_eq!("3♥".parse::<Card>().unwrap(), Three.of(Hearts));
        assert_eq!("     2♠        ".parse::<Card>().unwrap(), Two.of(Spades));
    }

    #[test]
    fn test_cards_parsing() {
        let cards = "AcKd".parse::<CardVector>().unwrap();
        assert_eq!(cards[0], Ace.of(Clubs));
        assert_eq!(cards[1], King.of(Diamonds));
    }

    #[test]
    fn test_cards_parsing_with_whitespace() {
        let cards = "    Ac Kd   ".parse::<CardVector>().unwrap();
        assert_eq!(cards[0], Ace.of(Clubs));
        assert_eq!(cards[1], King.of(Diamonds));
    }

    #[test]
    fn test_cards_parsing_with_commas() {
        let cards = "Ac,Kd".parse::<CardVector>().unwrap();
        assert_eq!(cards[0], Ace.of(Clubs));
        assert_eq!(cards[1], King.of(Diamonds));
    }

    #[test]
    fn test_card_vector_from() {
        let cards = "AcKd".parse::<CardVector>().unwrap();
        let vec = Vec::from(cards);
        assert_eq!(vec[0], Ace.of(Clubs));
        assert_eq!(vec[1], King.of(Diamonds));
    }

    #[test]
    fn test_card_vector_into() {
        let cards = "AcKd".parse::<CardVector>().unwrap();
        let vec: Vec<Card> = cards.into();
        assert_eq!(vec[0], Ace.of(Clubs));
        assert_eq!(vec[1], King.of(Diamonds));
    }
}
