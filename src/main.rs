use std::{env, process::ExitCode};

use http::method;
use reqwest::Body;

mod getter;
mod structs;
mod formatter;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let poke_method = &args[1];
    let input_pokemon = &args[2];

    match poke_method.as_str() {
        "list" => match getter::get_pokedex().await {
            Ok(body) => formatter::format_pokedex(&body),
            Err(e) => println!("Error: {}", e),
        },
        "search" => match getter::get_pokemon(input_pokemon).await {
            Ok(body) => formatter::format_pokemon(&body),
            Err(e) => println!("Error: {}", e),
    },
         _ => println!("No valid input given. Type pokecli help for additional information.")

    }
}