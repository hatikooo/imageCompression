pub mod structs;
pub mod new_game;
pub mod results;

#[cfg(test)]
mod tests {
    use super::results::return_flush;
    use super::results::return_straight;
    use super::results::count_duplicates;
    use super::results::multiples;
    use super::results::high_card;
    use super::results::declare_winner;
    use super::results::get_score;
    use super::new_game::check_for_dupes;
    use super::structs::Game;
    use super::structs::Score;
    use super::structs::Player;

    fn return_player_score(available_cards: Vec<(usize,usize)>) -> Score {
        let player_score = Score {
            name: String::from("PLAYER"),
            hand: vec![(0,0), (1,0)],
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

// NEW GAME TESTS

    #[test]
    fn test_check_for_dupes() {
        let cards_vec: Vec<(usize, usize)> = vec![
            (1,1),(2,2),(3,3),
        ];
        let new_card: (usize, usize) = (2,2);
        let dupe_report: bool = check_for_dupes(&cards_vec, &new_card);
        assert_eq!(dupe_report, true);
    }


// RESULTS TESTS

    #[test]
    fn test_royal_flush() {
        let available_cards: Vec<(usize,usize)> = vec![
            (1,0),(1,10),(1,11),(1,12),(1,9),(0,9),(0,11)
        ];
        let player_score = return_player_score(available_cards);
        let straight_flush_report: (bool, usize, usize) = player_score.straight_flush();
        assert_eq!(straight_flush_report.0, true);
        assert_eq!(straight_flush_report.1, 1);
        assert_eq!(straight_flush_report.2, 13);
    }

    #[test]
    fn test_straight_flush() {
        let available_cards: Vec<(usize,usize)> = vec![
            (2,1),(2,2),(0,1),(1,1),(2,3),(2,4),(2,5)
        ];
        let player_score = return_player_score(available_cards);
        let straight_flush_report: (bool, usize, usize) = player_score.straight_flush();
        assert_eq!(straight_flush_report.0, true);
        assert_eq!(straight_flush_report.1, 2);
        assert_eq!(straight_flush_report.2, 5);
    }

    #[test]
    fn test_full_house() {
        let available_cards: Vec<(usize,usize)> = vec![
            (1,10),(0,10),(1,2),(3,10),(1,3),(0,2),(0,3)
        ];
        let player_score = return_player_score(available_cards);
        let full_house_report: (bool, usize, usize) = player_score.full_house();
        assert_eq!(full_house_report.0, true);
        assert_eq!(full_house_report.1, 10);
        assert_eq!(full_house_report.2, 3);
    }

    #[test]
    fn test_full_house_two_trips() {
        let available_cards: Vec<(usize,usize)> = vec![
            (1,10),(0,10),(1,2),(3,10),(1,3),(3,3),(0,3)
        ];
        let player_score = return_player_score(available_cards);
        let full_house_report: (bool, usize, usize) = player_score.full_house();
        assert_eq!(full_house_report.0, true);
        assert_eq!(full_house_report.1, 10);
        assert_eq!(full_house_report.2, 3);
    }

    #[test]
    fn test_no_flush() {
        let available_cards: Vec<(usize,usize)> = vec![
            (0,8),(1,2),(1,4),(2,5),(2,7),(0,9),(0,11)
        ];
        let flush_report: (bool, Vec<(usize, usize)>) = return_flush(&available_cards);
        assert_eq!(false, flush_report.0);
    }

    #[test]
    fn test_low_flush_with_more_than_5_cards() {
        let available_cards: Vec<(usize,usize)> = vec![
            (0,8),(0,2),(0,4),(0,5),(0,7),(0,9),(0,11)
        ];
        let flush_report: (bool, Vec<(usize, usize)>) = return_flush(&available_cards);
        assert_eq!(true, flush_report.0);
        assert_eq!((0,11), flush_report.1[0]);
    }

    #[test]
    fn test_high_flush_with_more_than_5_cards() {
        let available_cards: Vec<(usize,usize)> = vec![
            (0,1),(0,9),(0,10),(0,11),(0,12),(0,13),(3,3)
        ];
        let flush_report: (bool, Vec<(usize, usize)>) = return_flush(&available_cards);
        assert_eq!(true, flush_report.0);
        assert_eq!((0,13), flush_report.1[0]);
    }
  
    #[test]
    fn test_no_straight() {
        let available_cards: Vec<(usize,usize)> = vec![
            (0,2),(0,7),(0,9),(1,13),(2,5),(3,6),(3,3)
        ];
        let straight_report: (bool, Vec<(usize)>) = return_straight(&available_cards);
        assert_eq!(false, straight_report.0);
    }

    #[test]
    fn test_low_straight_with_6_cards_and_ace() {
        let available_cards: Vec<(usize,usize)> = vec![
            (0,2),(0,1),(0,3),(1,4),(2,5),(3,6),(3,3)
        ];
        let straight_report: (bool, Vec<(usize)>) = return_straight(&available_cards);
        assert_eq!(true, straight_report.0);
        assert_eq!(6, straight_report.1[0]);
    }

    #[test]
    fn test_high_straight_with_6_cards_and_ace() {
        let available_cards: Vec<(usize,usize)> = vec![
            (0,1),(0,9),(0,10),(1,11),(2,12),(3,13),(3,3)
        ];
        let straight_report: (bool, Vec<(usize)>) = return_straight(&available_cards);
        assert_eq!(true, straight_report.0);
        assert_eq!(13, straight_report.1[0]);
    }

    #[test]
    fn test_duplicate_counter_card_numbers() {
        let available_cards: Vec<(usize, usize)> = vec![
            (0,0),(0,3),(0,1),(1,1),(2,2),(3,2),(3,3)
        ];
        let index: usize = 1;
        let limit: usize = 14;
        let duplicates: Vec<usize> = count_duplicates(&available_cards, index, limit);
        assert_eq!(duplicates.len(), 14);
        assert_eq!(duplicates, [0,2,2,2,0,0,0,0,0,0,0,0,0,1]);
    }

    #[test]
    fn test_count_duplicates_with_one_pair_of_aces() {
        let available_cards: Vec<(usize, usize)> = vec![
            (0,0),(0,0),(0,1),(1,2),(2,12),(3,4),(3,9)
        ];
        let index: usize = 1;
        let limit: usize = 14;
        let duplicates: Vec<usize> = count_duplicates(&available_cards, index, limit);
        assert_eq!(duplicates.len(), 14);
        assert_eq!(duplicates, [0,1,1,0,1,0,0,0,0,1,0,0,1,2]);
    }

    #[test]
    fn test_count_duplicates_with_quads() {
        let available_cards: Vec<(usize, usize)> = vec![
            (3,1),(2,1),(0,1),(1,1),(2,2),(3,2),(3,3)
        ];
        let index: usize = 1;
        let limit: usize = 14;
        let duplicates: Vec<usize> = count_duplicates(&available_cards, index, limit);
        assert_eq!(duplicates.len(), 14);
        assert_eq!(duplicates, [0,4,2,1,0,0,0,0,0,0,0,0,0,0]);
    }

    #[test]
    fn test_duplicate_counter_suits() {
        let available_cards: Vec<(usize, usize)> = vec![
            (0,0),(0,0),(0,1),(1,1),(2,2),(3,2),(3,3)
        ];
        let index: usize = 0;
        let limit: usize = 4;
        let duplicates = count_duplicates(&available_cards, index, limit);
        assert_eq!(duplicates.len(), 4);
        assert_eq!(duplicates, [3,1,1,2]);
    }

    #[test]
    fn test_multiples_report_pairs() {
        let available_cards: Vec<(usize, usize)> = vec![
            (0,0),(0,0),(0,1),(1,1),(2,2),(3,2),(3,3)
        ];
        let amount: usize = 2;
        let multiples_report: (bool, Vec<usize>) = multiples(&available_cards, amount);
        assert_eq!(true, multiples_report.0);
        assert_eq!(multiples_report.1, [13,2,1]);
    }

    #[test]
    fn test_multiples_report_quads() {
        let available_cards: Vec<(usize, usize)> = vec![
            (3,1),(2,1),(0,1),(1,1),(2,2),(3,2),(3,3)
        ];
        let amount: usize = 4;
        let multiples_report: (bool, Vec<usize>) = multiples(&available_cards, amount);
        assert_eq!(true, multiples_report.0);
        assert_eq!(multiples_report.1, [1]);
    }

    #[test]
    fn test_high_card_with_ace(){
        let available_cards: Vec<(usize, usize)> = vec![
            (0,0),(0,9),(0,1),(1,5),(2,12),(3,2),(3,3)
        ];
        let high_cards: Vec<(usize,usize)> = high_card(&available_cards);
        assert_eq!(high_cards[0].1, 13);
    }

    // TESTING WINNERS

    fn return_game(player_1_cards: Vec<(usize, usize)>, player_2_cards: Vec<(usize, usize)>,  board: Vec<(usize, usize)>) -> Game {
        let player_1 = Player {
            name: String::from("Player 1"),
            hand: player_1_cards,
        };
        let player_2 = Player {
            name: String::from("Player 2"),
            hand: player_2_cards,
        };
        Game {
            player_1: player_1,
            player_2: player_2,
            board: board,
        }
    }

    fn get_result(player_1_cards: Vec<(usize, usize)>, player_2_cards: Vec<(usize, usize)>,  board: Vec<(usize, usize)>) -> (String, String, usize, Score, Score) {
        let new_game: Game = return_game(player_1_cards, player_2_cards, board);
        let player_1_score: Score = get_score(&new_game.player_1, &new_game.board);
        let player_2_score: Score = get_score(&new_game.player_2, &new_game.board);
        declare_winner(player_1_score, player_2_score)
    }

    #[test]
    fn test_winner_high_card_low() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(2,7),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (3,1),(2,8),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,0),(2,12),(1,11),(1,10),(3,2),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 2 wins with ace high"),
            "Result was not correct, value was `{}`", result.0
        );      
    }

    #[test]
    fn test_winner_one_pair() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(2,1),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (3,3),(2,2),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,5),(2,8),(1,7),(1,9),(3,12),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 1 wins with a pair of 2s"),
            "Result was not correct, value was `{}`", result.0
        );
    }

    #[test]    
    fn test_winner_two_pair() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(2,0),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (3,3),(2,3),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,5),(2,5),(1,7),(1,9),(3,12),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 2 wins with 2 pairs, 6s and 4s"),
            "Result was not correct, value was `{}`", result.0
        );
    }

    #[test]    
    fn test_winner_two_pair_third_pair() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(2,0),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (3,4),(2,4),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,5),(2,5),(1,3),(1,3),(3,12),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 2 wins with 2 pairs, 6s and 5s"),
            "Result was not correct, value was `{}`", result.0
        );
    }

    #[test]    
    fn test_winner_trips() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(2,0),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (3,3),(2,3),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,5),(2,1),(1,7),(1,3),(3,12),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 2 wins with trip 4s"),
            "Result was not correct, value was `{}`", result.0
        );
    }

    #[test]    
    fn test_winner_straight() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(2,2),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (3,3),(2,3),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,5),(2,4),(1,6),(1,3),(3,12),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 1 wins with a 7 high straight"),
            "Result was not correct, value was `{}`", result.0
        );
    }

    #[test]    
    fn test_winner_two_straights() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(2,2),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (3,7),(3,2),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,5),(2,4),(1,6),(1,3),(3,12),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 2 wins with a 8 high straight"),
            "Result was not correct, value was `{}`", result.0
        );
    }

    #[test]    
    fn test_winner_flush() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(1,2),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (3,3),(2,3),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,5),(2,4),(1,6),(1,3),(1,12),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 1 wins with a king high flush, diamonds"),
            "Result was not correct, value was `{}`", result.0
        );
    }

   #[test]    
    fn test_winner_full_house() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(2,1),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (3,3),(2,3),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,5),(2,6),(1,6),(3,6),(1,12),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 2 wins with a full house, 7s full of 4s"),
            "Result was not correct, value was `{}`", result.0
        );
    }
   
   #[test]    
    fn test_split_full_house() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (1,1),(2,1),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (0,1),(3,1),
        ];
        let board: Vec<(usize,usize)> = vec![
            (3,5),(2,6),(1,6),(3,6),(1,12),
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 1 and Player 2 split the pot with a full house, 7s full of 2s"),
            "Result was not correct, value was `{}`", result.0
        );
    }

   #[test]    
    fn test_winner_quads() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (2,1),(3,1),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (0,5),(3,8),
        ];
        let board: Vec<(usize,usize)> = vec![
            (0,1),(1,1),(2,2),(3,7),(3,3)
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 1 wins with quad 2s"),
            "Result was not correct, value was `{}`", result.0
        );
    }

   #[test]    
    fn test_winner_straight_flush() {
        let player_1_cards: Vec<(usize,usize)> = vec![
            (2,1),(2,2),
        ];
        let player_2_cards: Vec<(usize,usize)> = vec![
            (0,5),(3,8),
        ];
        let board: Vec<(usize,usize)> = vec![
            (0,1),(1,1),(2,3),(2,4),(2,5)
        ];
        let result = get_result(player_1_cards, player_2_cards, board);
        assert!(
            result.0.contains("Player 1 wins with a 6 high straight flush, clubs"),
            "Result was not correct, value was `{}`", result.0
        );
    }
}
