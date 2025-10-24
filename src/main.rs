use std::{io};
use rs_poker::{core::*, holdem::*};
use colored::*;

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

    

    let my_hand = Hand::new_from_str(&hand_string)
        .map_err(|e|println!("{e}"))
        .unwrap();


    let mut state = GameState
    {
        my_hand: my_hand,
        board: Vec::new(),
        pot_size: to_call + to_call / 2,
        to_call: to_call,
        stack: stack,
        num_opponents: num_opps
    };

    let mut bot = PokerBot { state: state };

    loop {
        println!("\n===== GAME STATE =====");
        println!("Pot size: {}", bot.state.pot_size);
        println!("To call: {}", bot.state.to_call);
        println!("Board: {:?}", bot.state.board);
        println!("Stack: {}", bot.state.stack);

        // Ask bot for action
        let action = bot.decide();
        match action {
            Action::Fold => { fold(); break; }
            Action::Check => check(),
            Action::Call(x) => call(x),
            Action::Raise(x) => raise(x),
        }

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

        // Optionally update pot and to_call
        let change = prompt("Did pot or to_call change? (y/n)");
        if change.to_lowercase() == "y" {
            bot.state.pot_size = prompt("Enter new pot size").parse().unwrap();
            bot.state.to_call = prompt("Enter new to_call").parse().unwrap();
        }

        // Option to end hand
        let cont = prompt("Continue this hand? (y/n)");
        if cont.to_lowercase() != "y" {
            break;
        }
    }

    
}

fn raise(amount : i32)
{
    println!("{}: {}", "RAISE >:D".green(), amount)
}

fn call(amount : i32)
{
    println!("{}: {}", "CALL :) ".green(), amount)
}

fn check()
{
    println!("{}", "CHECK :S".yellow());
}

fn fold()
{
    println!("{}", "FOLD :C".red());
    std::process::exit(0);
}
