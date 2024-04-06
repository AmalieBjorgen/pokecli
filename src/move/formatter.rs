use crate::r#move::structs::Move;

pub fn format_move(move_json: &str) {
    let move_deserialised: Move = serde_json::from_str(&move_json).unwrap();
    beautify_move_output(&move_deserialised);
}

fn beautify_move_output(move_json: &Move) {
    println!("---------------------------------------");
    println!("Name: {}", move_json.names[0].name); // TODO: Make this always print the English name
    println!("Type: {}", move_json.r#type.name);
    println!("Power: {}", move_json.power);
    println!("Damage Class: {}", move_json.damage_class.name);
    println!("PP: {}", move_json.pp);
    println!("Accuracy: {}", move_json.accuracy);
    println!("Priority: {}", move_json.priority);
    println!("Effect: {}", move_json.effect_entries[0].short_effect);
    println!("Description: {}", move_json.flavor_text_entries[0].flavor_text);
    println!("Target: {}", move_json.target.name);
    println!("Ailment Effect: {}", move_json.meta.ailment.name);
    println!("Ailment Chance: {:?}", move_json.meta.ailment_chance);
    println!("Crit Rate: {:?}", move_json.meta.crit_rate);
    println!("Stat Chance: {:?}", move_json.meta.stat_chance);
    println!("Move Category: {}", move_json.meta.category.name);
    println!("---------------------------------------");
}