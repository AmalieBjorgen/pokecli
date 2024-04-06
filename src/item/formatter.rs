use crate::item::structs::Item;

pub fn format_item(item_json: &str) {
    let item_deserialised: Item = serde_json::from_str(&item_json).unwrap();
    beautify_item_output(&item_deserialised);
}

fn beautify_item_output(item_json: &Item) {
    println!("---------------------------------------");
    println!("Name: {}", item_json.name);
    println!("Cost: {}", item_json.cost);
    println!("Effect: {}", item_json.effect_entries[0].short_effect);
    println!("Description: {}", item_json.flavor_text_entries[0].text);
    println!("---------------------------------------");
}