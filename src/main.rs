extern crate dealer;

use dealer::structs::Score;
use dealer::results::get_score;
use dealer::results::declare_winner;
use dealer::new_game::create_game;

// TO DO:
// - What happens in the event of a split pot?
// - Print out the actual result of the contest
// - Think about how different the game will need to be to accomodate live play...
//   - API for betting
//   - API for returning status of game, current cards, revealing cards as they are dealt

fn main() {
    let mut score_board: Vec<usize> = vec![0; 9];

    for i in 0..1000{
        let new_game = create_game();
        let player_1_score: Score = get_score(&new_game.player_1, &new_game.board);
        let player_2_score: Score = get_score(&new_game.player_2, &new_game.board);
        // player_1_score.print_score();
        // println!("FINAL SCORE: {}", player_1_score.final_score().0);
        // player_2_score.print_score();
        // println!("FINAL SCORE: {}", player_2_score.final_score().0);
        let winner: (String, String, usize, Score, Score) = declare_winner(player_1_score, player_2_score);
        score_board[winner.2] = score_board[winner.2] + 1;
        if winner.4.final_score().0 == 3 {
            println!("{:?}", winner.3.hand);
            println!("{}", winner.0);
            println!("{:?}", winner.4.hand);
            println!("{}", winner.1);
        }
    }
    // println!("Player 1 wins: {}", player_1_winners);
    // println!("Player 2 wins: {}", player_2_winners);
    println!("High cards: {}", score_board[0]);
    println!("Pair: {}", score_board[1]);
    println!("2 pair: {}", score_board[2]);
    println!("Trips: {}", score_board[3]);
    println!("Straights: {}", score_board[4]);
    println!("Flushes: {}", score_board[5]);
    println!("Full houses: {}", score_board[6]);
    println!("Quads: {}", score_board[7]);
    println!("Straight flushes: {}", score_board[8]);

}
