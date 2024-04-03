use std::string;

use http::{response, Request, Response};
use serde::{Deserialize, Serialize};

pub async fn get_pokemon(pokemon: &str) -> Result<String, reqwest::Error> {

    let mut url = "https://pokeapi.co/api/v2/pokemon/".to_owned();
    url.push_str(pokemon);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    Ok(body)
}

pub async fn get_pokedex() -> Result<String, reqwest::Error>{
    let url = "https://pokeapi.co/api/v2/pokemon?limit=10000";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    Ok(body)
}