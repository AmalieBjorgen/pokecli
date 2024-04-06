pub async fn get_move(move_name: &str) -> Result<String, reqwest::Error> {
    let mut url = "https://pokeapi.co/api/v2/move/".to_owned();
    url.push_str(move_name);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    Ok(body)
    
}