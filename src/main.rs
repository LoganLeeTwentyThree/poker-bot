use std::{io};
use rs_poker::{core::*};

mod pokerbot;
use pokerbot::*;

fn prompt(msg: &str) -> String {
    println!("{}", msg);

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    String::from(user_input.trim())
}




fn main() {
    
    let num_opps : i32 = prompt("Enter the number of opponents").parse::<i32>().unwrap();
    let hand_string = prompt("Enter your hand");
    let to_call = prompt("Enter the blind").parse::<i32>().unwrap();
    let stack = prompt("Enter your stack").parse::<i32>().unwrap();
    let blind = prompt("Are you the blind? (y/n)");

    let mut temp_bet = 0;
    if blind.to_lowercase() == "y"
    {
        temp_bet = to_call;
    }

    

    let my_hand = Hand::new_from_str(&hand_string)
        .map_err(|e|e.to_string())
        .unwrap();


    let mut state = GameState
    {
        my_hand: my_hand,
        board: Vec::new(),
        pot_size: to_call + to_call / 2,
        to_call: to_call,
        stack: stack,
        num_opponents: num_opps,
        mine_in_pot: temp_bet,
        start_blind: to_call,
    };

    

    let mut bot = PokerBot { state: state };


    loop {
        println!("\n===== GAME STATE =====");
        println!("Pot size: {}", bot.state.pot_size);
        println!("To call: {}", bot.state.to_call);
        println!("Board: {:?}", bot.state.board);
        println!("Stack: {}", bot.state.stack);
        println!("num_opps: {}", bot.state.num_opponents);
        println!("======================");

        
        // to_call
        let to_call_change = prompt("Did to_call change? (y/n)");
        if to_call_change.to_lowercase() == "y" {
            bot.state.to_call = prompt("Enter new to_call").parse().unwrap();
        }

        // pot
        bot.state.pot_size = prompt("Enter new pot size").parse().unwrap();

        // Ask player if board has changed
        let update = prompt("Did a new card appear on the board? (y/n)");
        if update.to_lowercase() == "y" {
            let num_cards : i32 = prompt("How many?").parse().unwrap();
            for _ in 0..num_cards
            {
                let new_card_str = prompt("Enter new card (e.g. Ah)");
                bot.add_to_board(Hand::new_from_str(&new_card_str).unwrap().cards().collect::<Vec<Card>>()[0]);
            }
        }

        // Ask player if anyone folded
        let player_change = prompt("Did anyone fold?");
        if player_change.to_lowercase() == "y"
        {
            let num_folds : i8 = prompt("How many?").parse().unwrap();
            bot.remove_opps(num_folds);
        }
        

        // Ask bot for action
        if bot.state.to_call == 0
        {
            bot.decide(true)
        }else {
            bot.decide(false)
        }
        
    }

    
}
