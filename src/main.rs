use std::{thread, time};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::bot::Bot;
use crate::deck::{new_deck, Symbol};
use crate::player::Player;

mod bot;
mod deck;
mod player;

fn main() {
    println!("RUNO Game");

    let mut bank = new_deck();
    let mut pile = Vec::new();
    bank.shuffle(&mut thread_rng());

    let mut bots: Vec<Bot> = Vec::new();
    bots.push(Bot {
        cards: bank.split_off(bank.len() - 7),
    });
    bots.push(Bot {
        cards: bank.split_off(bank.len() - 7),
    });
    bots.push(Bot {
        cards: bank.split_off(bank.len() - 7),
    });

    let mut player = Player {
        cards: bank.split_off(bank.len() - 7),
    };

    pile.push(bank.pop().expect("weird beginning"));

    let mut dt_counter = 0;
    let mut super_joker = false;
    let mut bot_counter: i32 = 0;
    let mut order: i32 = 1;
    let winner: i32;

    println!("First card is {}", pile.last().unwrap());

    loop {
        let top_card = pile.last().unwrap();

        if bot_counter == 3 {
            // player turn

            // super joker
            if super_joker {
                println!("Player draws 4 cards");
                player.draw(&mut bank.split_off(bank.len() - 4));
                super_joker = false;
            }
            // draw two
            else if dt_counter > 0 {
                if let Some(card) = player.react_to_drawtwo() {
                    println!("Player plays {}", card);
                    pile.push(card);
                    dt_counter += 1;
                } else {
                    println!("Player draws {} cards", 2 * dt_counter);
                    player.draw(&mut bank.split_off(bank.len() - 2 * dt_counter));
                    dt_counter = 0;
                }
            }
            // play normally
            else if let Some(card) = player.play(top_card) {
                println!("Player plays {}", card);
                pile.push(card);
                match card.symbol {
                    Symbol::DrawTwo => dt_counter += 1,
                    Symbol::SuperJoker => super_joker = true,
                    Symbol::Reverse => order *= -1,
                    _ => (),
                }
            }
            // draw one card
            else {
                println!("Player draws 1 card");
                player.draw(&mut bank.split_off(bank.len() - 1));
            }

            if player.cards.len() == 0 {
                winner = bot_counter;
                break;
            }
        } else {
            // bots turn
            let bot: &mut Bot = &mut bots[bot_counter as usize];

            // super joker
            if super_joker {
                println!("Bot {} draws 4 cards", bot_counter);
                bot.draw(&mut bank.split_off(bank.len() - 4));
                super_joker = false;
            }
            // draw two
            else if dt_counter > 0 {
                if let Some(card) = bot.react_to_drawtwo() {
                    println!("Bot {} plays {}", bot_counter, card);
                    pile.push(card);
                    dt_counter += 1;
                } else {
                    println!("Bot {} draws {} cards", bot_counter, 2 * dt_counter);
                    bot.draw(&mut bank.split_off(bank.len() - 2 * dt_counter));
                    dt_counter = 0;
                }
            }
            // play normally
            else if let Some(card) = bot.play(top_card) {
                println!("Bot {} plays {}", bot_counter, card);
                pile.push(card);
                match card.symbol {
                    Symbol::DrawTwo => dt_counter += 1,
                    Symbol::SuperJoker => super_joker = true,
                    Symbol::Reverse => order *= -1,
                    _ => (),
                }
            }
            // draw one card
            else {
                println!("Bot {} draws 1 card", bot_counter);
                bot.draw(&mut bank.split_off(bank.len() - 1));
            }

            if bot.len() == 0 {
                winner = bot_counter;
                break;
            }
        }

        bot_counter += order;
        if bot_counter >= 4 {
            bot_counter = 0;
        } else if bot_counter < 0 {
            bot_counter = 3;
        }

        thread::sleep(time::Duration::from_secs(1));
    }

    println!("The winner is {}!", winner);
}
