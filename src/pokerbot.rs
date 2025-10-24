use rand::Rng;
use rs_poker::{core::*, holdem::*};

pub struct GameState {
    pub my_hand: Hand,
    pub board: Vec<Card>,
    pub pot_size: i32,
    pub to_call: i32,
    pub stack: i32,
    pub num_opponents: i32,
}

pub enum Action {
    Fold,
    Call(i32),
    Raise(i32),
    Check
}

pub struct PokerBot {
    pub state: GameState,
}

impl PokerBot {
    pub fn decide(&mut self) -> Action {
        let pot_odds = self.state.to_call as f32 / (self.state.pot_size + self.state.to_call) as f32;
        let equity = self.get_equity();

        println!(
            "Equity: {:.2}%, Pot odds: {:.2}%",
            equity * 100.0,
            pot_odds * 100.0
        );

        //preflop
        if self.state.board.is_empty() {
            return Action::Call(self.state.to_call)
        }
        
        
        if equity < pot_odds 
        {
            Action::Fold
        } else if equity > pot_odds * 2.0 && equity > 0.6 {
            // Very strong hand → raise
            let raise_amount = (self.state.to_call as f32 * 2.5) as i32;
            Action::Raise(raise_amount)
        } else if self.state.to_call == 0 {
            // Nothing to call → check
            Action::Check
        } else {
            // Profitable to call
            Action::Call(self.state.to_call)
        }
    }

    pub fn add_to_board(&mut self, card : Card)
    {
        self.state.board.push(card);
    }

    pub fn get_equity(&mut self) -> f32 {
        let mut rng = rand::rng();
        let mut all_hands : Vec<Hand> = Vec::new();
        let mut my_hand_board = Hand::new_with_cards(self.state.board.clone());
        let _ = self.state.my_hand.cards().map(|card| my_hand_board.insert(card));
        all_hands.push(my_hand_board);

        
        let flat_hands = RangeParser::parse_many("22+,A2s+,K9o+").unwrap();

        let mut wins = 0.0;

        for _ in 0..1000 {
            let mut this_hands = all_hands.clone();
            for _ in 0..self.state.num_opponents {
                let index = rng.random_range(0..flat_hands.len());
                let fh = &flat_hands[index];
                let mut cards: Vec<Card> = fh.cards().collect();
                let _ = self.state.board.iter().map(|card|cards.push(*card)); // add community cards
                this_hands.push(Hand::new_with_cards(cards));
            }

            let mut game = MonteCarloGame::new(this_hands).unwrap();
            wins += game.estimate_equity(1000)[0];
        }
        wins / 1000.0
    }
}
