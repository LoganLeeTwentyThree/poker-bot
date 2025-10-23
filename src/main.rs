use std::{io};
use rs_poker::{core::*, holdem::*};
use rand::*;
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
   
    let chance_to_win = bot.get_equity();
    println!("Your chance to win is {}%", chance_to_win * 100.0);


    for _ in 0..4
    {
        match bot.decide()
        {
            Action::Fold => fold(),
            Action::Check => check(),
            Action::Call(i) => call(i),
            Action::Raise(i) => raise(i),
        }

        loop {
            let next_action = prompt("Call changed (y/n)?");
            match next_action.as_str()
            {
                "y" => {
                    //can fold, raise, call
                    break
                },
                "n" => {
                    //can fold, check, raise, call
                    break
                },
                _ => println!("Invalid input. Try again.")
            }
        }
        

    }
    
    // maybe raise sometimes?

    //prompt for first card
    //check/raise/fold based on new probabilities
    //repeat for second and third cards
    
    

    

    

    
}

fn raise(amount : i32)
{
    println!("{}: {}", "RAISE >:D".green(), amount)
}

fn call(amount : i32)
{
    println!("{}: {}", "CALL :|".green(), amount)
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
