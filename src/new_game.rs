extern crate rand;

use self::rand::Rng;
use super::structs::Game;
use super::structs::Player;

pub fn create_game() -> Game {
    let mut cards_vec: Vec<(usize, usize)> = Vec::new(); 
    if cards_vec.len() == 0 {
        let card = return_one_card();
        cards_vec.push(card);
    }
    while cards_vec.len() < 9 {
        let card = return_one_card();
        let is_dupe: bool = check_for_dupes(&cards_vec, &card);
        if is_dupe == false {
            cards_vec.push(card);
        }
    }
    let player_1: Player = Player {
        name: String::from("Player 1"),
        hand: cards_vec[0..2].to_vec(),
    };
    let player_2: Player = Player {
        name: String::from("Player 2"),
        hand: cards_vec[2..4].to_vec(), 
    };
    Game {
        player_1: player_1,
        player_2: player_2,
        board: cards_vec[4..9].to_vec(),
    }
}

pub fn check_for_dupes(cards_vec: &Vec<(usize, usize)>, new_card: &(usize, usize)) -> bool {
    for card in cards_vec.iter() {
        if card.0 == new_card.0 && card.1 == new_card.1 {
            return true
        }
    }
    false
}

pub fn return_one_card() -> (usize, usize) {
    let suit = return_random_numbers(0,4);
    let card_number = return_random_numbers(0,13);
    (suit, card_number)
}

pub fn return_random_numbers(start: usize, end: usize) -> usize {
    let num = rand::thread_rng().gen_range(start, end);
    num
}