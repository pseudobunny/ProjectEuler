
// Using Ace High Rules
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl CardValue {
    fn new(v_s: &str) -> CardValue {
        match v_s {
            "2" => CardValue::Two,
            "3" => CardValue::Three,
            "4" => CardValue::Four,
            "5" => CardValue::Five,
            "6" => CardValue::Six,
            "7" => CardValue::Seven,
            "8" => CardValue::Eight,
            "9" => CardValue::Nine,
            "T" => CardValue::Ten,
            "J" => CardValue::Jack,
            "Q" => CardValue::Queen,
            "K" => CardValue::King,
            "A" => CardValue::Ace,
            _ => CardValue::Ace,
        }
    }
}

const CARD_VALUE_ORDER: [CardValue; 13] = [
    CardValue::Two,
    CardValue::Three,
    CardValue::Four,
    CardValue::Five,
    CardValue::Six,
    CardValue::Seven,
    CardValue::Eight,
    CardValue::Nine,
    CardValue::Ten,
    CardValue::Jack,
    CardValue::Queen,
    CardValue::King,
    CardValue::Ace
];

const LOW_ACE_STRAIGHT: [CardValue; 5] = [
    CardValue::Two,
    CardValue::Three,
    CardValue::Four,
    CardValue::Five,
    CardValue::Ace
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardSuit {
    Spade,
    Diamond,
    Club,
    Heart
}

impl CardSuit {
    fn new(s_s: &str) -> CardSuit {
        match s_s {
            "S" => CardSuit::Spade,
            "D" => CardSuit::Diamond,
            "C" => CardSuit::Club,
            "H" => CardSuit::Heart,
            _ => CardSuit::Spade,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    val: CardValue,
    suit: CardSuit
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for Card { }

impl Card {
    fn new(card_str: &str) -> Card {
        let mut card_chars = card_str.chars().collect::<Vec<char>>();
        let s_s: &str = &card_chars.pop().unwrap().to_string()[..];
        let v_s: &str = &card_chars.iter().collect::<String>()[..];

        Card{val: CardValue::new(v_s), suit: CardSuit::new(s_s)}
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerHand {
    HighCard(Card, Card, Card, Card, Card), 
    OnePair(Card, Card, Card, Card),
    TwoPair(Card, Card, Card),
    ThreeOfAKind(Card, Card, Card),
    Straight(Card),
    Flush(Card, Card, Card, Card, Card),
    FullHouse(Card, Card),
    FourOfAKind(Card, Card),
    StraightFlush(Card),
}

impl PokerHand {
    pub fn new(hand_str: &str) -> PokerHand {
        let mut cards = hand_str.split_whitespace()
            .map(Card::new)
            .collect::<Vec<Card>>();
        
        cards.sort();
    
        let is_flush = cards.windows(2)
            .all(|cv| cv[0].suit == cv[1].suit);
        
        let is_straight = cards.iter()
            .map(|c| CARD_VALUE_ORDER.iter().position(|&v| v == c.val).unwrap() as i32)
            .collect::<Vec<i32>>()
            .windows(2)
            .all(|cv| (cv[0] - cv[1]).abs() == 1);

        let is_ace_low_straight = cards.iter()
            .zip(LOW_ACE_STRAIGHT)
            .filter(|(a,b)| a.val == *b)
            .count() == 5;

        if is_flush && is_straight {
            return PokerHand::StraightFlush(cards[4]);
        }

        if is_flush && is_ace_low_straight {
            return PokerHand::StraightFlush(cards[3]);
        }

        if is_straight {
            return PokerHand::Straight(cards[4]);
        }

        if is_ace_low_straight {
            return PokerHand::Straight(cards[3]);
        }

        if is_flush {
            return PokerHand::Flush(cards[4], cards[3], cards[2], cards[1], cards[0]);
        }

        let val_counts = CARD_VALUE_ORDER.iter()
            .map(|v| cards.iter().filter(|c| c.val == *v).count())
            .collect::<Vec<usize>>();

        if val_counts.iter().any(|&c| c == 4) {
            return PokerHand::FourOfAKind(
                *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[val_counts.iter().position(|&v| v == 4).unwrap()]).unwrap(),
                *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[val_counts.iter().position(|&v| v == 1).unwrap()]).unwrap()
            );
        }

        if val_counts.iter().any(|&c| c == 3) {
            if val_counts.iter().any(|&c| c == 2) {
                return PokerHand::FullHouse(
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[val_counts.iter().position(|&v| v == 3).unwrap()]).unwrap(),
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[val_counts.iter().position(|&v| v == 2).unwrap()]).unwrap()
                );
            } else {
                let ones_positions = val_counts.iter()
                                               .enumerate()
                                               .filter(|(_, &v)| v == 1)
                                               .map(|(i, _)| i)
                                               .collect::<Vec<usize>>();

                return PokerHand::ThreeOfAKind(
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[val_counts.iter().position(|&v| v == 3).unwrap()]).unwrap(),
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[ones_positions[1]]).unwrap(),
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[ones_positions[0]]).unwrap(),
                );
            }
        }

        match val_counts.iter().filter(|&&v| v == 2).count() {
            2 => {
                let twos_positions = val_counts.iter()
                                               .enumerate()
                                               .filter(|(_, &v)| v == 2)
                                               .map(|(i, _)| i)
                                               .collect::<Vec<usize>>();
                
                PokerHand::TwoPair(
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[twos_positions[1]]).unwrap(),
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[twos_positions[0]]).unwrap(),
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[val_counts.iter().position(|&v| v == 1).unwrap()]).unwrap()
                )
            },
            1 => {
                let ones_positions = val_counts.iter()
                                               .enumerate()
                                               .filter(|(_, &v)| v == 1)
                                               .map(|(i, _)| i)
                                               .collect::<Vec<usize>>();
                
                PokerHand::OnePair(
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[val_counts.iter().position(|&v| v == 2).unwrap()]).unwrap(),
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[ones_positions[2]]).unwrap(),
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[ones_positions[1]]).unwrap(),
                    *cards.iter().find(|c| c.val == CARD_VALUE_ORDER[ones_positions[0]]).unwrap()
                )
            }
            _ => {
                PokerHand::HighCard(cards[4], cards[3], cards[2], cards[1], cards[0])
            }
        }
    }
}