use crate::pokemon::structs::{Pokedex, Pokemon};

pub fn format_pokemon(pokemon_json: &str) {
    let pokemon_deserialised: Pokemon = serde_json::from_str(&pokemon_json).unwrap();
    beautify_pokemon_output(&pokemon_deserialised);
}

fn beautify_pokemon_output(pokemon_json: &Pokemon) {
    println!("---------------------------------------");
    println!("Name: {}", pokemon_json.name);
    for poketype in &pokemon_json.types {
        println!("Type: {}", poketype.r#type.name)
    }
    for stat in &pokemon_json.stats {
        println!("{}: {}", stat.stat.name, stat.base_stat)
    }
    println!("---------------------------------------");
}

pub fn format_pokedex(pokedex_json: &str) {
    let pokedex_deserialised: Pokedex = serde_json::from_str(&pokedex_json).unwrap();
    beautify_pokedex_output(&pokedex_deserialised);
}

fn beautify_pokedex_output(pokedex_json: &Pokedex) {
    println!("---------------------------------------");
    for pokemon in &pokedex_json.results {
        println!("{}", pokemon.name);
    }
    println!("---------------------------------------");
}
