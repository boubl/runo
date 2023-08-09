use crate::deck::{Card, Symbol, remove_card, Color};

pub struct Bot {
    pub cards: Vec<Card>
}

impl Bot {
    pub fn len(&self) -> usize {
        self.cards.len()
    }
    pub fn first_color_or(&self, fallback: Color) -> Color {
        for card in &self.cards {
            if card.color != Color::Joker {
                return card.color
            }
        }

        fallback
    }
    pub fn play(&mut self, top_card: &Card) -> Option<Card> {
        let mut playable_cards_color = Vec::new();
        let mut playable_cards_symbol = Vec::new();
        let mut playable_cards_joker = Vec::new();
        for card in &self.cards {
            if card.symbol == Symbol::Joker || card.symbol == Symbol::SuperJoker {
                playable_cards_joker.push(card.clone())
            }
            else if card.color == top_card.color {
                playable_cards_color.push(card.clone())
            }
            else if card.symbol == top_card.symbol {
                playable_cards_symbol.push(card.clone())
            }
        }

        if playable_cards_color.len() + playable_cards_symbol.len() + playable_cards_joker.len() == 0 {
            return None
        }

        // color: if attack play it, else last number
        let mut last_neutral = None;
        for card in &playable_cards_color {
            if card.is_neutral() {
                last_neutral = Some(card.clone());
            }
            else {
                remove_card(&mut self.cards, card);
                return Some(*card)
            }
        }
        if let Some(card) = last_neutral {
            remove_card(&mut self.cards, &card);
            return Some(card)
        }

        //symbol: quite the same (add later choosing an advantageous color)
        let mut last_neutral = None;
        for card in &playable_cards_symbol {
            if card.is_neutral() {
                last_neutral = Some(card.clone());
            }
            else {
                remove_card(&mut self.cards, card);
                return Some(*card)
            }
        }
        if let Some(card) = last_neutral {
            remove_card(&mut self.cards, &card);
            return Some(card)
        }

        // must use a joker, same strategy: attack first or neutral
        let mut last_neutral = None;
        for card in &playable_cards_joker {
            if card.is_neutral() {
                last_neutral = Some(card.clone());
            }
            else {
                remove_card(&mut self.cards, card);
                return Some(Card::new(self.first_color_or(Color::Red), card.symbol))
            }
        }
        if let Some(card) = last_neutral {
            remove_card(&mut self.cards, &card);
            return Some(Card::new(self.first_color_or(Color::Red), card.symbol))
        }

        // no card playable
        None
    }
    pub fn draw(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards);
    }
    pub fn react_to_drawtwo(&mut self) -> Option<Card> {
        let mut card_to_use = None;
        for card in &self.cards {
            if card.symbol == Symbol::DrawTwo {
                card_to_use = Some(card.clone());
                break;
            }
        }

        if let Some(card) = card_to_use {
            remove_card(&mut self.cards, &card);
            return Some(card)
        }

        None
    }
}