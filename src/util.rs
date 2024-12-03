use reqwest::blocking::Client;
use std::error::Error;

pub fn get_input(url: &str, session_cookie: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "advent-of-code-fetcher")
        .header("Cookie", format!("session={}", session_cookie))
        .send()?;

    let input_data = response.text()?;
    Ok(input_data)
}
