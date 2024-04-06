use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Berry {
    pub id: i16,
    pub name: String,
    pub item: BerryItem,
    pub natural_gift_type: NaturalGiftType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BerryItem {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NaturalGiftType {
    pub name: String,
    pub url: String,
}