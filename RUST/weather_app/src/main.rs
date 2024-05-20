use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::{env, error::Error, process};

async fn call_api(city: &str, api_key: &str) -> Result<(), Box<dyn Error>> {
    let request_url = format!(
        "https://api.tomorrow.io/v4/weather/realtime?location={}&apikey={}",
        city, api_key
    );

    let client = Client::new();

    let response = client.get(&request_url).send().await?;
    let body = response.text().await?;

    let json: Value = serde_json::from_str(&body)?;

    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("TOMORROW_IO_API_KEY")?;
    let city: Vec<String> = env::args().collect();

    if city.len() != 2 {
        println!("Usage: cargo run <city>",);
        process::exit(0);
    }

    call_api(&city[1], &api_key).await?;

    Ok(())
}
