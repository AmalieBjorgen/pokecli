use std::env;

use pokemon::{formatter as pokemon_formatter, getter as pokemon_getter};
use berry::{formatter as berry_formatter, getter as berry_getter};
use item::{formatter as item_formatter, getter as item_getter};
use r#move::{formatter as move_formatter, getter as move_getter};
mod pokemon;
mod berry;
mod item;
mod r#move;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.len() >= 4 {
        println!("Bad input. Type pokecli help for additional information.");
        std::process::exit(1);
    }
    let poke_method = &args[1];
    if &args.len() == &2 {
        match poke_method.as_str() {
            "pokedex" => match pokemon_getter::get_pokedex().await {
                Ok(body) => pokemon_formatter::format_pokedex(&body),
                Err(e) => println!("Error: {}", e),
            },
            "help" => {
                println!("pokecli help: This command will display the help menu.");
                println!("pokecli pokedex: This command will display the Pokedex.");
                println!("pokecli pokemon <pokemon_name>: This command will display the information of the Pokemon.");
            }
            _ => println!("Type pokecli help for additional information."),
        }
    } else if &args.len() >= &3 {
        let input = &args[2];
        match poke_method.as_str() {
            "pokemon" => match pokemon_getter::get_pokemon(input).await {
                Ok(body) => pokemon_formatter::format_pokemon(&body),
                Err(e) => println!("Error: {}", e),
            },
            "berry" => match berry_getter::get_berry(input).await {
                Ok(body) => berry_formatter::beautify_berry_output(body),
                Err(e) => println!("Error: {}", e),
            },
            "move" => match move_getter::get_move(input).await {
                Ok(body) => move_formatter::format_move(&body),
                Err(e) => println!("Error: {}", e),
            },
            "item" => match item_getter::get_item(input).await {
                Ok(body) => item_formatter::format_item(&body),
                Err(e) => println!("Error: {}", e),
            },
            "ability" => match pokemon_getter::get_pokemon(input).await {
                Ok(body) => pokemon_formatter::format_pokemon(&body),
                Err(e) => println!("Error: {}", e),
            },
            _ => println!("Type pokecli help for additional information."),
        }
    } else {
        println!("Type pokecli help for additional information.");
        std::process::exit(1);
    }
}
