pub async fn request(url: String) -> Result<String, reqwest::Error> {
    let mut result = String::new();

    match reqwest::get(&url).await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                result = text;
            }
            Err(_) => {
                println!("read response text error");
            }
        },
        Err(_) => println!("request get error"),
    }
    Ok(result)
}