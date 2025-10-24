use rand::Rng;
use colored::*;
use rs_poker::{core::*, holdem::*};

pub struct GameState {
    pub my_hand: Hand,
    pub board: Vec<Card>,
    pub pot_size: i32,
    pub to_call: i32,
    pub stack: i32,
    pub num_opponents: i32,
    pub mine_in_pot: i32,
    pub start_blind: i32,
}

pub struct PokerBot {
    pub state: GameState,
}

impl PokerBot {
    pub fn decide(&mut self, can_check : bool) {
        let pot_odds = self.state.to_call as f32 / (self.state.pot_size + self.state.to_call) as f32;
        let pre_equity = self.get_equity();
        let mut rng = rand::rng();
        let bluff = rng.random_range(0.0..1.0);
        let aggression = 1.0
        + (1.0 - (self.state.num_opponents as f32 / 6.0)) * 0.3  // fewer opponents -> more aggro
        + (self.state.stack as f32 / 1000.0).min(1.0) * 0.2     // deep stack -> more aggro
        + if pre_equity > 0.5 {pre_equity - 0.5} else {-(0.5 - pre_equity) * 0.6}; //better hand -> more aggro


        let equity = pre_equity * aggression;

        println!(
            "Equity: {:.2}%, Pot odds: {:.2}%, Agression: {:.2}",
            equity * 100.0,
            pot_odds * 100.0,
            aggression
        );

        //preflop
        if self.state.board.is_empty() {
            if equity > 0.6 || bluff < 0.3 {
                let raise_amount = self.calculate_raise_amount(equity, aggression);
                self.raise(raise_amount);
            } else if self.state.to_call - self.state.mine_in_pot == 0
            {
                self.check();
            } 
            else {
                self.call();
            }
            return;
        }
    
        
        
        if equity < pot_odds && self.state.to_call != 0
        {
            self.fold();
        } else if (equity > pot_odds * 1.3 && equity >= 0.45) || (equity > 0.35 && bluff < 0.1) { 
            
            
            
            let raise_amount = self.calculate_raise_amount(equity, aggression);
            self.raise(raise_amount);

        } else if (can_check || self.state.to_call == 0) {
            if aggression < 1.0
            {
                self.check();
            }else {
                let raise_amount = self.calculate_raise_amount(equity, aggression);
                self.raise(raise_amount);
            }
            
        } else if !can_check && equity >= 0.15{
            self.call();
        }else {
            self.fold();
        }
    }

    fn calculate_raise_amount(&mut self, equity : f32, aggression : f32) -> i32
    {
        let min_raise = self.state.start_blind * 2; // double the current bet (minimum)
        let max_raise = self.state.stack;       // can't raise beyond stack

        // Base raise multiplier determined by equity strength
        let  factor = equity * (10.0 * aggression);

        // Compute raw raise
        let mut raise = (self.state.to_call as f32 * factor) as i32;

        // Make into multiple of blind 
        raise = ((raise as f32 / self.state.start_blind as f32).round() as i32) * self.state.start_blind;

        // Enforce min/max rules
        if raise < min_raise {
            raise = min_raise;
        } else if raise > max_raise {
            raise = max_raise;
        }

        raise
    }

    pub fn raise(&mut self, amount : i32) 
    {
        println!("{}: {}", "RAISE >:D".green(), amount);
        self.change_stack(-amount);
    }

    pub fn call(&mut self)
    {
        println!("{}", "CALL :) ".green());
        self.change_stack(-self.state.to_call);
    }

    pub fn check(&mut self) 
    {
        println!("{}", "CHECK :S".yellow());
    }

    pub fn fold(&mut self) 
    {
        println!("{}", "FOLD :C".red());
        std::process::exit(0);
    }


    pub fn add_to_board(&mut self, card : Card)
    {
        self.state.board.push(card);
    }

    pub fn remove_opps(&mut self, num : i8)
    {
        self.state.num_opponents -= num as i32;
    }

    pub fn change_stack(&mut self, num : i32)
    {
        self.state.stack += num;
        if num < 0 
        {
            self.state.mine_in_pot += num;
        }
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
