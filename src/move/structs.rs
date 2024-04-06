use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    pub name: String,
    pub id: i16,
    pub accuracy: i8,
    pub pp: i8,
    pub power: u8,
    pub effect_chance: Option<i8>,
    pub effect_entries: Vec<EffectEntry>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub damage_class: DamageClass,
    pub r#type: TypeDetail,
    pub names: Vec<Names>,
    pub target: Target,
    pub priority: i8,
    pub meta : Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub short_effect: String,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageClass {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeDetail {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Names {
    pub name: String,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub ailment: Ailment,
    pub category: Category,
    pub min_hits: Option<i8>,
    pub max_hits: Option<i8>,
    pub min_turns: Option<i8>,
    pub max_turns: Option<i8>,
    pub drain: i8,
    pub healing: i8,
    pub crit_rate: i8,
    pub ailment_chance: i8,
    pub flinch_chance: i8,
    pub stat_chance: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ailment {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub url: String,
}