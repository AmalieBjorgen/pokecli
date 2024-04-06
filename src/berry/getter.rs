use super::structs::Berry;
use crate::item::structs::Item;

pub async fn get_berry(berry: &str) -> Result<(Berry, Item), reqwest::Error> {
    let mut url = "https://pokeapi.co/api/v2/berry/".to_owned();
    url.push_str(berry);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    match Ok(body) {
        Ok(berry_json) => {
            let berry_deserialised: Berry = serde_json::from_str(&berry_json).unwrap();
            let item = get_berry_as_item(&berry_deserialised).await.unwrap();
            let item_deserialised: Item = serde_json::from_str(&item).unwrap();
            return Ok((berry_deserialised, item_deserialised))
        }
        Err(e) => Err(e),
    }
}

async fn get_berry_as_item(berry: &Berry) -> Result<String, reqwest::Error> {
    let response = reqwest::get(&berry.item.url).await?;
    let body = response.text().await?;

    Ok(body)
}