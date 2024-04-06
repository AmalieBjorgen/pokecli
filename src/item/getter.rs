pub async fn get_item(item: &str) -> Result<String, reqwest::Error> {
    let mut url = "https://pokeapi.co/api/v2/item/".to_owned();
    url.push_str(item);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    Ok(body)
}
