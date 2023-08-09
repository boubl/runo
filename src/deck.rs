use std::fmt::Display;

pub fn new_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    deck.extend(deck_color(Color::Blue));
    deck.extend(deck_color(Color::Green));
    deck.extend(deck_color(Color::Red));
    deck.extend(deck_color(Color::Yellow));
    deck.extend([
        Card {color: Color::Joker, symbol: Symbol::Joker },
        Card {color: Color::Joker, symbol: Symbol::Joker },
        Card {color: Color::Joker, symbol: Symbol::Joker },
        Card {color: Color::Joker, symbol: Symbol::Joker },
        Card { color: Color::Joker, symbol: Symbol::SuperJoker },
        Card { color: Color::Joker, symbol: Symbol::SuperJoker },
        Card { color: Color::Joker, symbol: Symbol::SuperJoker },
        Card { color: Color::Joker, symbol: Symbol::SuperJoker }
    ]);

    deck
}

fn deck_color(color: Color) -> Vec<Card> {
    let mut deck = Vec::new();

    // single zero
    deck.push(Card { color, symbol: Symbol::Number(0) });

    // two times 1 to 9
    for i in 1..19 {
        if i >= 10 {
            deck.push(Card { color, symbol: Symbol::Number((i - 9) as i32) });
        }
        else {
            deck.push(Card { color, symbol: Symbol::Number(i as i32) });
        }
    }

    // two skips
    deck.push(Card { color, symbol: Symbol::Skip });
    deck.push(Card { color, symbol: Symbol::Skip });
    
    // two reverse
    deck.push(Card { color, symbol: Symbol::Reverse });
    deck.push(Card { color, symbol: Symbol::Reverse });
    
    // two draw two
    deck.push(Card { color, symbol: Symbol::DrawTwo });
    deck.push(Card { color, symbol: Symbol::DrawTwo });

    deck
}

pub fn remove_card(deck: &mut Vec<Card>, card: &Card) {
    deck.remove(deck.iter().position(|&x| x == *card).expect("couldn't find card deck"));
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    pub color: Color,
    pub symbol: Symbol
}

impl Card {
    pub fn new(color: Color, symbol: Symbol) -> Card {
        Card { color, symbol }
    }

    pub fn is_neutral(&self) -> bool {
        match self.symbol {
            Symbol::Number(_) => true,
            Symbol::Joker => true,
            _ => false
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.symbol {
            Symbol::Number(num) => 
                write!(f, "{} {:?}", num, self.color),
            _ => write!(f, "{:?} {:?}", self.symbol, self.color)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Joker
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Symbol {
    Number(i32),
    Skip,
    Reverse,
    DrawTwo,
    Joker,
    SuperJoker
}