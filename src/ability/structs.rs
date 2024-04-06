use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    pub id: i16,
    pub name: String,
    pub is_main_series: bool,
    pub names: Vec<Names>,
    pub effect_entries: Vec<EffectEntry>,
    pub generation: Generation
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub short_effect: String,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Names {
    pub name: String,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Generation {
    pub name: String,
    pub url: String,
}