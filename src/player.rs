use crate::deck::{Card, Symbol, remove_card};
use text_io::read;

pub struct Player {
    pub cards: Vec<Card>
}

impl Player {
    pub fn play(&mut self, top_card: &Card) -> Option<Card> {
        let mut playable_cards = Vec::new();
        let mut pos = 0;
        for card in &self.cards {
            if card.symbol == Symbol::Joker || card.symbol == Symbol::SuperJoker || card.color == top_card.color || card.symbol == top_card.symbol {
                playable_cards.push(pos);
            }
            pos += 1;
        }

        if playable_cards.len() == 0 {
            println!("Can't play any card.");
            return None
        }

        println!("Cards in your hand:");
        let mut pos = 0;
        for card in &self.cards {
            let playable = playable_cards.contains(&pos);
            print!("{} - {}", pos, card);
            if playable {
                println!(" - Playable");
            }
            else {
                println!();
            }
            pos += 1;
        }

        let mut choice: i32;
        loop {
            choice = read!("{}\n");
            if playable_cards.contains(&choice) {
                break;
            }
        }

        let card = self.cards[choice as usize].clone();
        remove_card(&mut self.cards, &card);
        Some(card)
    }
    
    pub fn draw(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards);
    }

    pub fn react_to_drawtwo(&mut self) -> Option<Card> {
        let mut playable_cards = Vec::new();
        let mut pos = 0;
        for card in &self.cards {
            if card.symbol == Symbol::DrawTwo {
                playable_cards.push(pos);
            }
            pos += 1;
        }

        if playable_cards.len() == 0 {
            println!("Can't play any card.");
            return None
        }

        println!("Cards in your hand:");
        let mut pos = 0;
        for card in &self.cards {
            let playable = playable_cards.contains(&pos);
            print!("{} - {}", pos, card);
            if playable {
                println!(" - Playable");
            }
            else {
                println!();
            }
            pos += 1;
        }

        let mut choice: i32;
        loop {
            choice = read!("{}\n");
            if playable_cards.contains(&choice) {
                break;
            }
        }

        let card = self.cards[choice as usize].clone();
        remove_card(&mut self.cards, &card);
        Some(card)
    }
}