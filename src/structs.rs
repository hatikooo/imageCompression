use super::results::return_straight;

pub struct Game {
    pub player_1: Player,
    pub player_2: Player,
    pub board: Vec<(usize, usize)>,
}

pub struct Player {
    pub name: String,
    pub hand: Vec<(usize, usize)>,
}

pub struct Score {
    pub name: String,
    pub hand: Vec<(usize, usize)>,
    pub full_board: Vec<(usize, usize)>,
    pub high_card: Vec<(usize, usize)>,
    pub flush: (bool, Vec<(usize, usize)>),
    pub pairs: (bool, Vec<usize>),
    pub trips: (bool, Vec<usize>),
    pub quads: (bool, Vec<usize>),
    pub straight: (bool, Vec<usize>),
}

impl Score {
    pub fn print_score(&self) {
        println!("full board: {:?}", self.full_board);
        println!("high cards: {:?}", self.high_card);
        println!("pairs: {:?}", self.pairs);
        println!("trips: {:?}", self.trips);
        println!("straight: {:?}", self.straight);
        println!("flush: {:?}", self.flush);
        println!("full house: {:?}", self.full_house());
        println!("quads: {:?}", self.quads);
        println!("straight flush: {:?}", self.straight_flush());
    }
}

impl Score {
    pub fn full_house(&self) -> (bool, usize, usize) {
        if self.trips.0 == true && self.trips.1.len() == 2 {
            let full_house_return: (bool, usize, usize) = (
                true,
                self.trips.1[0],
                self.trips.1[1],
            );
            return full_house_return 
        }
        else if self.trips.0 == true && self.pairs.0 == true {
            let full_house_return: (bool, usize, usize) = (
                true,
                self.trips.1[0],
                self.pairs.1[0],
            );
            return full_house_return 
        }
        else {
            return (false, 0, 0)
        }
    }
}

impl Score {
    pub fn straight_flush(&self) -> (bool, usize, usize) {
        if self.flush.0 == true {
            let mut clone_cards = self.full_board.clone();
            clone_cards.retain(|&x| x.0 == self.flush.1[0].0);
            let straight = return_straight(&clone_cards);
            if straight.0 == true {
                return (true, self.flush.1[0].0, straight.1[0]);
            }
        }
        (false, 0, 0)
    }
}

impl Score {
    pub fn final_score(&self,) -> (usize, String) {
        let suit_map: Vec<String> = vec![
            String::from("hearts"), 
            String::from("diamonds"), 
            String::from("clubs"), 
            String::from("spades"), 
        ];
        let card_map: Vec<String> = vec![
            String::from("ace"), 
            String::from("2"), 
            String::from("3"), 
            String::from("4"), 
            String::from("5"), 
            String::from("6"), 
            String::from("7"), 
            String::from("8"), 
            String::from("9"), 
            String::from("10"), 
            String::from("jack"), 
            String::from("queen"), 
            String::from("king"),
            String::from("ace"),
        ];

        let mut player_score: usize = 0; 
        let mut score_string: String = String::new();
        if self.straight_flush().0 == true {
            let suit: String = suit_map[self.straight_flush().1].clone();
            player_score = 8;
            score_string = format!("a {} high straight flush, {}", card_map[self.straight_flush().2], suit);
        }
        else if self.quads.0 == true  {
            player_score = 7;
            score_string = format!("quad {}s", card_map[self.quads.1[0]]);
        }
        else if self.full_house().0 == true  {
            player_score = 6;
            score_string = format!("a full house, {}s full of {}s", card_map[self.full_house().1], card_map[self.full_house().2]);
        }
        else if self.flush.0 == true  {
            let suit: String = suit_map[self.flush.1[0].0].clone();
            player_score = 5;
            score_string = format!("a {} high flush, {}", card_map[(self.flush.1[0].1)], suit);
        }
        else if self.straight.0 == true  {
            player_score = 4;
            score_string = format!("a {} high straight", card_map[self.straight.1[0]]);
        }
        else if self.trips.0 == true  {
            player_score = 3;
            score_string = format!("trip {}s", card_map[self.trips.1[0]]);
        }
        else if self.pairs.0 == true && self.pairs.1.len() > 1  {
            player_score = 2;
            score_string = format!("2 pairs, {}s and {}s", card_map[self.pairs.1[0]], card_map[self.pairs.1[1]]);
        }
        else if self.pairs.0 == true  {
            player_score = 1;
            score_string = format!("a pair of {}s", card_map[self.pairs.1[0]]);
        }
        else {
            score_string = format!("{} high", card_map[(self.high_card[0].1)]);
        }
        (player_score, score_string)
    }
}

// pub struct Result {
//     winning_string: 
// }
// Length 1 things
// -- pairs
// -- trips
// -- straight
// -- high card?
