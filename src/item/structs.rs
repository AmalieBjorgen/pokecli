use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub id: i16,
    pub cost: i16,
    pub effect_entries: Vec<EffectEntry>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub short_effect: String,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub text: String,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub url: String,
}