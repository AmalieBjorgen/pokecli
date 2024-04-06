use std::env;

use pokemon::{formatter, getter};
mod pokemon;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("{:?}", args.len());
    if args.len() == 1 || args.len() >= 4 {
        println!("Bad input. Type pokecli help for additional information.");
        std::process::exit(1);
    }
    let poke_method = &args[1];
    if &args.len() == &2 {
        match poke_method.as_str() {
            "pokedex" => match getter::get_pokedex().await {
                Ok(body) => formatter::format_pokedex(&body),
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
            "pokemon" => match getter::get_pokemon(input).await {
                Ok(body) => formatter::format_pokemon(&body),
                Err(e) => println!("Error: {}", e),
            },
            "berry" => match getter::get_pokemon(input).await {
                Ok(body) => formatter::format_pokemon(&body),
                Err(e) => println!("Error: {}", e),
            },
            "move" => match getter::get_pokemon(input).await {
                Ok(body) => formatter::format_pokemon(&body),
                Err(e) => println!("Error: {}", e),
            },
            "item" => match getter::get_pokemon(input).await {
                Ok(body) => formatter::format_pokemon(&body),
                Err(e) => println!("Error: {}", e),
            },
            _ => println!("Type pokecli help for additional information."),
        }
    } else {
        println!("Type pokecli help for additional information.");
        std::process::exit(1);
    }
}
