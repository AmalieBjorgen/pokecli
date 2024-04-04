use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatDetail {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub base_stat: i8,
    pub effort: i8,
    pub stat: StatDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeDetail {
    pub name: String,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Types {
    pub slot: i8,
    pub r#type: TypeDetail
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: i16,
    pub name: String,
    pub stats: Vec<Stats>,
    pub types: Vec<Types>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonDetail {
    pub name: String,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokedex {
    pub results: Vec<PokemonDetail>
}