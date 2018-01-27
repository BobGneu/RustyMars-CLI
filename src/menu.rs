use game::Game;
use std::io::{self, Read};

pub fn main_menu_prompt() -> char {
    println!("Welcome to RustyMars
    
Begin by creating a new game:
    
    N) Create new game
    L) Load game
    Q) Quit game
    
    >");

    get_input(false).remove(0)
}

fn get_input(allow_empty: bool) -> String {
    let mut buffer = String::new();

    loop {
        io::stdin().read_line(&mut buffer);
        buffer = buffer.trim().to_owned();

        if !buffer.is_empty() || allow_empty {
            break;
        }
    }

    (buffer.to_lowercase())
}

fn report_results(game_data: &Game) {
    println!("
-------------------------------------------");
    println!("    There are not currently any results\n\n\tbut between the two of us, \n\t\tYou won!");
}

fn print_header(game_data: &Game) {
    println!("RustyMars >>> {}
    Population: {}\t\tFunds: ${}", game_data.colony.name, game_data.colony.population, game_data.earth.funds);
}

fn print_menu() {
    print!("Menu: 
    L: Launch 

    ------------
    S: Save Game
    Q: Quit
        >");
}