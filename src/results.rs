use super::structs::Player;
use super::structs::Score;

pub fn declare_winner(player_1_score: Score, player_2_score: Score) -> (String, String, usize, Score, Score) {
    if player_1_score.final_score().0 > player_2_score.final_score().0 {
        let result = results_string(player_1_score, player_2_score);
        return result
    }
    else if player_2_score.final_score().0 > player_1_score.final_score().0 {
        let result = results_string(player_2_score, player_1_score);
        return result
    }
    else {
        if player_1_score.final_score().0 == 0 {
            for i in 0..5 {
                if player_1_score.high_card[i].1 > player_2_score.high_card[i].1 {
                    let result = results_string(player_1_score, player_2_score);
                    return result
                }
                else if player_2_score.high_card[i].1 > player_1_score.high_card[i].1 {
                    let result = results_string(player_2_score, player_1_score);
                    return result
                }
            }
            let result = split_pot_string(player_1_score, player_2_score);
            return result
        }
        else if player_1_score.final_score().0 == 1 {

            if player_1_score.pairs.1[0] > player_2_score.pairs.1[0] {
                let result = results_string(player_1_score, player_2_score);
                return result
            }
            else if player_2_score.pairs.1[0] > player_1_score.pairs.1[0] {
                let result = results_string(player_2_score, player_1_score);
                return result
            }
            else {
                let mut player_1_high_card = player_1_score.high_card.clone();
                player_1_high_card.retain(|&x| x.1 != player_1_score.pairs.1[0]);

                let mut player_2_high_card = player_2_score.high_card.clone();
                player_2_high_card.retain(|&x| x.1 != player_2_score.pairs.1[0]);

                for i in 0..2 {
                    if player_1_high_card[i].1 > player_2_high_card[i].1 {
                        let result = results_string(player_1_score, player_2_score);
                        return result
                    }
                    else if player_2_high_card[i].1 > player_1_high_card[i].1 {
                        let result = results_string(player_2_score, player_1_score);
                        return result
                    }
                }
            }
            let result: (String, String, usize, Score, Score) = split_pot_string(player_1_score, player_2_score);
            return result
        }
        else if player_1_score.final_score().0 == 2 {

            if player_1_score.pairs.1[1] > player_2_score.pairs.1[1] {
                let result = results_string(player_1_score, player_2_score);
                return result
            }
            else if player_2_score.pairs.1[1] > player_1_score.pairs.1[1] {
                let result = results_string(player_2_score, player_1_score);
                return result
            }
            else {
                let mut player_1_high_card = player_1_score.high_card.clone();
                player_1_high_card.retain(|&x| x.1 != player_1_score.pairs.1[0] && x.1 != player_1_score.pairs.1[1]);

                let mut player_2_high_card = player_2_score.high_card.clone();
                player_2_high_card.retain(|&x| x.1 != player_2_score.pairs.1[0] && x.1 != player_2_score.pairs.1[1]);

                if player_1_high_card[0].1 > player_2_high_card[0].1 {
                    let result = results_string(player_1_score, player_2_score);
                    return result
                }
                else if player_2_high_card[0].1 > player_1_high_card[0].1 {
                    let result = results_string(player_2_score, player_1_score);
                    return result
                }
            
            }
            let result: (String, String, usize, Score, Score) = split_pot_string(player_1_score, player_2_score);
            return result
        }
        else if player_1_score.final_score().0 == 3 {

            if player_1_score.trips.1[0] > player_2_score.trips.1[0] {
                let result = results_string(player_1_score, player_2_score);
                return result
            }
            else if player_2_score.trips.1[0] > player_1_score.trips.1[0] {
                let result = results_string(player_2_score, player_1_score);
                return result
            }
            else {
                let mut player_1_high_card = player_1_score.high_card.clone();
                player_1_high_card.retain(|&x| x.1 != player_1_score.trips.1[0]);

                let mut player_2_high_card = player_2_score.high_card.clone();
                player_2_high_card.retain(|&x| x.1 != player_2_score.trips.1[0]);

                for i in 0..1 {
                    if player_1_high_card[i].1 > player_2_high_card[i].1 {
                        let result = results_string(player_1_score, player_2_score);
                        return result
                    }
                    else if player_2_high_card[i].1 > player_1_high_card[i].1 {
                        let result = results_string(player_2_score, player_1_score);
                        return result
                    }
                }
            }
            let result: (String, String, usize, Score, Score) = split_pot_string(player_1_score, player_2_score);
            return result
        }
        else if player_1_score.final_score().0 == 4 {
            for i in 0..5 {
                if player_1_score.straight.1[i] > player_2_score.straight.1[i] {
                    let result = results_string(player_1_score, player_2_score);
                    return result
                }
                else if player_2_score.straight.1[i] > player_1_score.straight.1[i] {
                    let result = results_string(player_2_score, player_1_score);
                    return result
                }
            }
            let result: (String, String, usize, Score, Score) = split_pot_string(player_1_score, player_2_score);
            return result
        }
        else if player_1_score.final_score().0 == 5 {
            for i in 0..5 {
                if player_1_score.flush.1[i].1 > player_2_score.flush.1[i].1 {
                    let result = results_string(player_1_score, player_2_score);
                    return result
                }
                else if player_2_score.flush.1[i].1 > player_1_score.flush.1[i].1 {
                    let result = results_string(player_2_score, player_1_score);
                    return result
                }
            }
            let result: (String, String, usize, Score, Score) = split_pot_string(player_1_score, player_2_score);
            return result
        }
        else if player_1_score.final_score().0 == 6 {
            for i in 0..5 {
                if player_1_score.full_house().1 > player_2_score.full_house().1 {
                    let result = results_string(player_1_score, player_2_score);
                    return result
                }
                else if player_2_score.full_house().1 > player_1_score.full_house().1 {
                    let result = results_string(player_2_score, player_1_score);
                    return result
                }
                else if player_1_score.full_house().2 > player_2_score.full_house().2 {
                    let result = results_string(player_1_score, player_2_score);
                    return result
                }
                else if player_2_score.full_house().2 > player_1_score.full_house().2 {
                    let result = results_string(player_2_score, player_1_score);
                    return result
                }
            }
            let result: (String, String, usize, Score, Score) = split_pot_string(player_1_score, player_2_score);
            return result
        }
        else if player_1_score.final_score().0 == 7 {
            for i in 0..5 {
                if player_1_score.quads.1[0] > player_2_score.quads.1[0] {
                    let result = results_string(player_1_score, player_2_score);
                    return result
                }
                else if player_2_score.quads.1[0] > player_1_score.quads.1[0] {
                    let result = results_string(player_2_score, player_1_score);
                    return result
                }
                else {
                    let mut player_1_high_card = player_1_score.high_card.clone();
                    player_1_high_card.retain(|&x| x.1 != player_1_score.quads.1[0]);

                    let mut player_2_high_card = player_2_score.high_card.clone();
                    player_2_high_card.retain(|&x| x.1 != player_2_score.quads.1[0]);

                    if player_1_high_card[0].1 > player_2_high_card[0].1 {
                        let result = results_string(player_1_score, player_2_score);
                        return result
                    }
                    else if player_2_high_card[0].1 > player_1_high_card[0].1 {
                        let result = results_string(player_2_score, player_1_score);
                        return result
                    }
                }
                let result: (String, String, usize, Score, Score) = split_pot_string(player_1_score, player_2_score);
                return result
            }
        }
        else if player_1_score.final_score().0 == 8 {
            for i in 0..5 {
                if player_1_score.straight_flush().2 > player_2_score.straight_flush().2 {
                    let result = results_string(player_1_score, player_2_score);
                    return result
                }
                else if player_2_score.straight_flush().2 > player_1_score.straight_flush().2 {
                    let result = results_string(player_2_score, player_1_score);
                    return result
                }
            }
            let result: (String, String, usize, Score, Score) = split_pot_string(player_1_score, player_2_score);
            return result
        }
        return (String::from("WHY?"), String::from("HOW?"), 99, player_1_score, player_2_score)
    }
}

pub fn results_string(winner: Score, loser: Score) -> (String, String, usize, Score, Score) {
    let winner_string = format!("{} wins with {}", winner.name, winner.final_score().1);
    let loser_string = format!("{} has {}", loser.name, loser.final_score().1);
    let winner_score = winner.final_score().0;
    (winner_string, loser_string, winner_score, winner, loser)
}

pub fn split_pot_string(loser_1: Score, loser_2: Score) -> (String, String, usize, Score, Score) {
    let loser_1_string = format!("{} and {} split the pot with {}", loser_1.name, loser_2.name, loser_1.final_score().1);
    let loser_2_string = String::from(" ");
    let loser_1_score = loser_1.final_score().0;
    (loser_1_string, loser_2_string, loser_1_score, loser_1, loser_2)
}   

pub fn get_score(player: &Player, board: &Vec<(usize, usize)>) -> Score {
    let available_cards = merge_card_sets(&player.hand, &board);
    let player_score = Score {
        name: player.name.clone(),
        hand: player.hand.clone(),
        full_board: available_cards.clone(),
        high_card: high_card(&available_cards),
        flush: return_flush(&available_cards),
        pairs: multiples(&available_cards, 2_usize),
        trips: multiples(&available_cards, 3_usize),
        quads: multiples(&available_cards, 4_usize),
        straight: return_straight(&available_cards), 
    };
    player_score
}

pub fn merge_card_sets(player: &Vec<(usize, usize)>, board: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut available_cards: Vec<(usize, usize)> = Vec::new();
    for card in player {
        available_cards.push(card.clone());
    }
    for card in board {
        available_cards.push(card.clone());
    }
    available_cards
}

pub fn high_card(available_cards: &Vec<(usize, usize)>) -> Vec<(usize,usize)> {
    let mut clone_cards = available_cards.clone();
    clone_cards.sort_by(|a, b| a.1.cmp(&b.1));
    if clone_cards[0].1 == 0 {
         let mut high_ace = clone_cards[0].clone();
         high_ace.1 = 13;
         clone_cards.push(high_ace);
    }
    clone_cards.reverse();
    clone_cards.truncate(5);
    clone_cards
}

pub fn return_straight(available_cards: &Vec<(usize, usize)>) -> (bool, Vec<usize>) {
    let mut nums: Vec<usize> = Vec::new();
    let mut straight_on: Vec<usize> = Vec::new();
    for card in available_cards {
        nums.push(card.1);
    }
    nums.sort();
    nums.dedup();
    if nums[0] == 0 {
        nums.push(13);
    }
    if nums.len() >= 5 {
        for i in (0..(nums.len() - 4)).rev() {
            straight_on.clear();
            straight_on.push(nums[i]);
            for index in i..(i+4) {
                if nums[index]+1 == nums[(index+1)] {
                    straight_on.push(nums[(index+1)]);
                }
            }
            if straight_on.len() == 5 {
                straight_on.reverse();
                return (true, straight_on)
            }
        }
    }
    else {
        return (false, straight_on)
    }
    return (false, straight_on)
}


pub fn return_flush(available_cards: &Vec<(usize, usize)>) -> (bool, Vec<(usize, usize)>) {
    let mut clone_cards = available_cards.clone();
    let mut flush_report: Vec<(usize, usize)> = Vec::new();
    clone_cards.sort_by(|a, b| a.0.cmp(&b.0));
    if clone_cards[0].1 == 0 {
        let mut high_ace = clone_cards[0].clone();
        high_ace.1 = 13;
        clone_cards.push(high_ace);
        clone_cards.remove(0);
    }
    let suit_counter: Vec<usize> = count_duplicates(&clone_cards, 0, 4);
    for i in 0..suit_counter.len() {
        if suit_counter[i] >= 5 {
            clone_cards.retain(|&x| x.0 == i);
            clone_cards.sort_by(|a, b| a.1.cmp(&b.1));
            clone_cards.reverse();
            clone_cards.truncate(5);
            flush_report = clone_cards;
            return (true, flush_report)
        }
    }
    (false, flush_report)
}

// Return a report on duplicate cards. Returns simple vector of card numbers
pub fn multiples(available_cards: &Vec<(usize, usize)>, amount: usize) -> (bool, Vec<usize>) {
    let multiples_counter: Vec<usize> = count_duplicates(&available_cards, 1, 14);
    let mut multiples_report: Vec<(usize)> = Vec::new();
    for key in 0..multiples_counter.len() {
        if multiples_counter[key] == amount {
            multiples_report.push(key)
        }
    }
    if multiples_report.len() > 0 {
        multiples_report.reverse();
        return (true, multiples_report)    
    }
    else {
        return (false, multiples_report)
    }
}

pub fn count_duplicates(available_cards: &Vec<(usize, usize)>, index: usize, limit: usize) -> Vec<usize> {
    let mut multiples_counter: Vec<usize> = vec![0; limit];
    let mut num_value: usize = 99;
    for key in 0..limit {
        for card in available_cards {
            if index == 1 {
                num_value = card.1.clone();
            }
            else {
                num_value = card.0.clone();
            }
            if num_value == key {
                multiples_counter[key] = multiples_counter[key] + 1;
                if index == 1 {
                    if num_value == 0 {
                        multiples_counter[13] = multiples_counter[13] + 1;
                        multiples_counter[0] = 0;
                    }
                }
            }
        }
    }
    multiples_counter
}
