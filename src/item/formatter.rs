use crate::item::structs::Item;

pub fn format_item(item_json: &str) {
    let item_deserialised: Item = serde_json::from_str(&item_json).unwrap();
    beautify_item_output(&item_deserialised);
}

fn beautify_item_output(item_json: &Item) {
    let english_effect = item_json
        .effect_entries
        .iter()
        .find(|effect| effect.language.name == "en")
        .unwrap();
    let english_flavor_text = item_json
        .flavor_text_entries
        .iter()
        .find(|flavor_text| flavor_text.language.name == "en")
        .unwrap();
    println!("---------------------------------------");
    println!("Name: {}", item_json.name);
    println!("Cost: {}", item_json.cost);
    println!("Effect: {}", english_effect.short_effect);
    println!("Description: {}", english_flavor_text.text);
    println!("---------------------------------------");
}
