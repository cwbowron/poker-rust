use strum::IntoEnumIterator;

type ParseError = &'static str;

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Ord, PartialOrd, Display)]
pub enum Suit {
    #[strum(to_string = "♣")]
    Clubs,

    #[strum(to_string = "♦")]
    Diamonds,

    #[strum(to_string = "♥")]
    Hearts,
    
    #[strum(to_string = "♠")]
    Spades,

    #[strum(to_string = "?")]
    Joker
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
        } else if str == "?" {
            Ok(Suit::Joker)
        } else {
            Err("Invalid Suit")
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
    LowAce = 1,
    #[strum(to_string = "?")]
    Joker = 0,
}

#[allow(dead_code)]
impl Rank {
    pub fn is_ordinal (&self, ordinal: usize) -> bool {
        *self as usize == ordinal
            || (self == &Rank::Ace && Rank::LowAce as usize == ordinal)
    }

    pub fn for_ordinal(ordinal: usize) -> Self {
        for rank in Self::iter() {
            if rank as usize == ordinal {
                return rank;
            }
        }

        panic!("Invalid ordinal for Rank!");
    }
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
        return Err("Invalid Rank")
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub scoring_rank: Rank,
}

pub type IsWildCard = fn(&Card) -> bool;

#[allow(dead_code)]
impl Card {
    pub const fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank: rank, suit: suit, scoring_rank: rank }
    }

    pub const fn scored_as(&self, rank: Rank) -> Card {
        Card { rank: self.rank, suit: self.suit, scoring_rank: rank }
    }

    pub fn is_one_eyed_jack(card: &Card) -> bool {
        card.rank == Rank::Jack
            && (card.suit == Suit::Spades || card.suit == Suit::Hearts)
    }

    pub fn is_suicide_king(card: &Card) -> bool {
        card.rank == Rank::King && card.suit == Suit::Hearts
    }

    pub fn is_joker(card: &Card) -> bool {
        card.rank == Rank::Joker
    }

    pub fn is_wild(&self, is_wild_option: &Option<IsWildCard>) -> bool {
        if let Some(is_wild) = is_wild_option {
            is_wild(self)
        } else {
            false
        }
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
        self.scoring_rank.cmp(&other.scoring_rank)
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

#[allow(dead_code)]
impl CardVector {
    pub fn parse(cards_string: &str) -> CardVector {
        cards_string.parse::<CardVector>().unwrap()
    }
}

impl std::ops::Deref for CardVector {
    type Target = Vec<Card>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for CardVector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", fmt_cards(&self.0))
    }
}

pub fn remove_cards(a: &[Card], b: &[Card]) -> Vec<Card> {
    return a.iter()
        .filter(|card| !b.contains(card))
        .map(Card::clone)
        .collect();
}

pub fn remove_card(a: &[Card], b: &Card) -> Vec<Card> {
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
        .map(Card::clone)
        .collect();
}

pub fn add_cards(a: &[Card], b: &[Card]) -> Vec<Card> {
    let mut r = a.to_vec();
    r.extend(b.iter().map(Card::clone));
    return r;
}

#[cfg(test)]
mod tests {
    use super::*;
    use Rank::*;
    use Suit::*;
    
    #[test]
    fn test_rank_is_ordinal() {
        assert!(Ace.is_ordinal(14));
        assert!(!Ace.is_ordinal(13));
        assert!(Ace.is_ordinal(1));
    }

    #[test]
    fn test_card_to_string() {
        assert_eq!(Card::new(Ace, Clubs).to_string(), "A♣");
        assert_eq!(Card::new(King, Hearts).to_string(), "K♥");
        assert_eq!(Card::new(Queen, Diamonds).to_string(), "Q♦");
        assert_eq!(Card::new(Jack, Spades).to_string(), "J♠");
        assert_eq!(Card::new(Ten, Diamonds).to_string(), "T♦");

        assert_eq!(Rank::Joker.of(Suit::Joker).to_string(), "??");
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
        assert_eq!("c".parse::<Suit>(), Ok(Clubs));
        assert_eq!("d".parse::<Suit>(), Ok(Diamonds));
        assert_eq!("h".parse::<Suit>(), Ok(Hearts));
        assert_eq!("s".parse::<Suit>(), Ok(Spades));
        assert_eq!("♣".parse::<Suit>(), Ok(Clubs));
        assert_eq!("♦".parse::<Suit>(), Ok(Diamonds));
        assert_eq!("♥".parse::<Suit>(), Ok(Hearts));
        assert_eq!("♠".parse::<Suit>(), Ok(Spades));
    }

    #[test]
    fn test_rank_parsing() {
        assert_eq!("a".parse::<Rank>(), Ok(Ace));
        assert_eq!("k".parse::<Rank>(), Ok(King));
        assert_eq!("q".parse::<Rank>(), Ok(Queen));
        assert_eq!("j".parse::<Rank>(), Ok(Jack));
        assert_eq!("t".parse::<Rank>(), Ok(Ten));
        assert_eq!("9".parse::<Rank>(), Ok(Nine));
        assert_eq!("8".parse::<Rank>(), Ok(Eight));
        assert_eq!("7".parse::<Rank>(), Ok(Seven));
        assert_eq!("6".parse::<Rank>(), Ok(Six));
        assert_eq!("5".parse::<Rank>(), Ok(Five));
        assert_eq!("4".parse::<Rank>(), Ok(Four));
        assert_eq!("3".parse::<Rank>(), Ok(Three));
        assert_eq!("2".parse::<Rank>(), Ok(Two));
        assert_eq!("?".parse::<Rank>(), Ok(Rank::Joker));
    }

    #[test]
    fn test_card_parsing() {
        assert_eq!("Ac".parse::<Card>(), Ok(Ace.of(Clubs)));
        assert_eq!(" Kd".parse::<Card>(), Ok(King.of(Diamonds)));
        assert_eq!("Qh ".parse::<Card>(), Ok(Queen.of(Hearts)));
        assert_eq!("Js".parse::<Card>(), Ok(Jack.of(Spades)));
        assert_eq!("Tc".parse::<Card>(), Ok(Ten.of(Clubs)));
        assert_eq!("9s".parse::<Card>(), Ok(Nine.of(Spades)));
        assert_eq!("8h".parse::<Card>(), Ok(Eight.of(Hearts)));
        assert_eq!("7h".parse::<Card>(), Ok(Seven.of(Hearts)));
        assert_eq!("6♥".parse::<Card>(), Ok(Six.of(Hearts)));
        assert_eq!("5♣".parse::<Card>(), Ok(Five.of(Clubs)));
        assert_eq!("4♦".parse::<Card>(), Ok(Four.of(Diamonds)));
        assert_eq!("3♥".parse::<Card>(), Ok(Three.of(Hearts)));
        assert_eq!("     2♠        ".parse::<Card>(), Ok(Two.of(Spades)));
        assert_eq!("     ??        ".parse::<Card>(), Ok(Rank::Joker.of(Suit::Joker)));
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

    // #[test]
    // fn test_card_vector_from() {
    //     let cards = "AcKd".parse::<CardVector>().unwrap();
    //     let vec = Vec::from(cards);
    //     assert_eq!(vec[0], Ace.of(Clubs));
    //     assert_eq!(vec[1], King.of(Diamonds));
    // }

    // #[test]
    // fn test_card_vector_into() {
    //     let cards = "AcKd".parse::<CardVector>().unwrap();
    //     let vec: Vec<Card> = cards.into();
    //     assert_eq!(vec[0], Ace.of(Clubs));
    //     assert_eq!(vec[1], King.of(Diamonds));
    // }

    #[test]
    fn test_convert_card_vector_into_cards() {
        let card_vector = "AcKd".parse::<CardVector>().unwrap();
        println!("{}", Cards(&card_vector));
    }

    #[test]
    fn test_is_one_eyed_jack() {
        assert!(Card::is_one_eyed_jack(&Jack.of(Hearts)));
        assert!(Card::is_one_eyed_jack(&Jack.of(Spades)));
        assert!(!Card::is_one_eyed_jack(&Jack.of(Diamonds)));
        assert!(!Card::is_one_eyed_jack(&Jack.of(Clubs)));
        assert!(!Card::is_one_eyed_jack(&King.of(Spades)));
        assert!(!Card::is_one_eyed_jack(&King.of(Hearts)));
        
    }

    #[test]
    fn test_is_suicide_king() {
        assert!(Card::is_suicide_king(&King.of(Hearts)));
        assert!(!Card::is_suicide_king(&King.of(Spades)));
        assert!(!Card::is_suicide_king(&King.of(Diamonds)));
        assert!(!Card::is_suicide_king(&King.of(Clubs)));
        assert!(!Card::is_suicide_king(&Jack.of(Spades)));
        assert!(!Card::is_suicide_king(&Jack.of(Hearts)));

        assert!(King.of(Hearts).is_wild(&Some(Card::is_suicide_king)));
        assert!(!Jack.of(Hearts).is_wild(&Some(Card::is_suicide_king)));
        assert!(!King.of(Hearts).is_wild(&None));
        assert!(!Jack.of(Hearts).is_wild(&None));
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
}
