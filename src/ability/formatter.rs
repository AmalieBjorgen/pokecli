use crate::ability::structs::Ability;

pub fn format_ability(ability_json: &str) {
    let ability_deserialised: Ability = serde_json::from_str(&ability_json).unwrap();
    beautify_ability_output(&ability_deserialised);
}

fn beautify_ability_output(ability_json: &Ability) {
    println!("---------------------------------------");
    println!("Name: {}", ability_json.names[0].name); // TODO: Make this always print the English name
    println!("Effect: {}", ability_json.effect_entries[0].short_effect); // TODO: Make this always print the English effect
    println!("Generation Introduced: {}", ability_json.generation.name);
    println!("---------------------------------------");
}
