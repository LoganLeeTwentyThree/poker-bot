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
        if equity < pot_odds 
        {
            Action::Fold
        } else if equity > 0.7 
        {
            Action::Raise((self.state.to_call as f32 * 2.5) as i32)
        }else 
        {
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
        all_hands.push(self.state.my_hand);

        
        let flat_hands = RangeParser::parse_many("22+,A2s+,K9o+").unwrap();

        let mut wins = vec![0; (self.state.num_opponents + 1) as usize];

        for _ in 0..100_000 {
            let mut this_hands = all_hands.clone();
            for _ in 0..self.state.num_opponents {
                let index = rng.random_range(0..flat_hands.len());
                let fh = &flat_hands[index];
                let cards: Vec<Card> = fh.cards().collect();
                this_hands.push(Hand::new_with_cards(cards));
            }

            let mut mc_game = MonteCarloGame::new(this_hands).unwrap();
            let r = mc_game.simulate();
            mc_game.reset();

            if let Some(i) = r.0.ones().next() {
                wins[i] += 1;
            }
        }

        // compute probability 
        let total_games = wins.iter().sum::<i32>() as f32;
        wins[0] as f32 / total_games
    }
}
