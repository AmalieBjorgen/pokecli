use crate::berry::structs::Berry;
use crate::item::structs::Item;

pub fn beautify_berry_output(berry_item: (Berry, Item)) {
    let (berry, item) = berry_item;
    let english_effect = item
        .effect_entries
        .iter()
        .find(|effect| effect.language.name == "en")
        .unwrap();
    println!("---------------------------------------");
    println!("Name: {}", item.name);
    println!("Natural Gift Type: {}", berry.natural_gift_type.name);
    println!("Effect: {}", english_effect.short_effect);
    println!("---------------------------------------");
}
