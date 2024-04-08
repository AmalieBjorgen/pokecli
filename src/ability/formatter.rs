use crate::ability::structs::Ability;

pub fn format_ability(ability_json: &str) {
    let ability_deserialised: Ability = serde_json::from_str(&ability_json).unwrap();
    beautify_ability_output(&ability_deserialised);
}

fn beautify_ability_output(ability_json: &Ability) {
    let english_name = ability_json
        .names
        .iter()
        .find(|name| name.language.name == "en")
        .unwrap();
    let english_effect = ability_json
        .effect_entries
        .iter()
        .find(|effect| effect.language.name == "en")
        .unwrap();
    println!("---------------------------------------");
    println!("Name: {}", english_name.name);
    println!("Effect: {}", english_effect.short_effect);
    println!(
        "Generation Introduced: {}",
        beautify_generation_text(ability_json.generation.name.to_owned())
    );
    println!("---------------------------------------");
}

fn beautify_generation_text(generation_text: String) -> String {
    match generation_text.as_str() {
        "generation-i" => "Generation 1".to_string(),
        "generation-ii" => "Generation 2".to_string(),
        "generation-iii" => "Generation 3".to_string(),
        "generation-iv" => "Generation 4".to_string(),
        "generation-v" => "Generation 5".to_string(),
        "generation-vi" => "Generation 6".to_string(),
        "generation-vii" => "Generation 7".to_string(),
        "generation-viii" => "Generation 8".to_string(),
        "generation-ix" => "Generation 9".to_string(),
        _ => "Unknown".to_string(),
    }
}
