pub async fn get_ability(ability: &str) -> Result<String, reqwest::Error> {
    let mut url = "https://pokeapi.co/api/v2/ability/".to_owned();
    url.push_str(ability);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    Ok(body)
}
