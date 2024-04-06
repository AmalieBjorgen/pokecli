
use crate::berry::structs::Berry;
use crate::item::structs::Item;

pub fn beautify_berry_output(berry_item: (Berry, Item)) {
    let (berry, item) = berry_item;
    println!("---------------------------------------");
    println!("Name: {}", item.name);
    println!("Natural Gift Type: {}", berry.natural_gift_type.name);
    println!("Effect: {}", item.effect_entries[0].short_effect);
    println!("---------------------------------------");
}