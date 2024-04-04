use std::env;

use pokemon::{formatter, getter};
mod pokemon;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let poke_method = &args[1];
    if poke_method == "help" {
        println!("pokecli help: This command will display the help menu.");
        println!("pokecli pokedex: This command will display the pokedex.");
        println!("pokecli pokemon <pokemon_name>: This command will display the information of the pokemon.");
        return;
    }

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
         _ => println!("Type pokecli help for additional information.")

    }

    //let input_pokemon = &args[2];

    match poke_method.as_str() {
        "pokemon" => {
            let input_pokemon = &args[2];
            match getter::get_pokemon(input_pokemon).await {
            Ok(body) => formatter::format_pokemon(&body),
            Err(e) => println!("Error: {}", e),
    }},
         _ => println!("No valid input given. Type pokecli help for additional information.")

    }
}