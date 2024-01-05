use reqwest;
use serde_json::json;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok(); // This will load environment variables from the .env file
    
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");
    let client = reqwest::Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "prompt": "Hello, World!",
            "max_tokens": 5
        }))
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("Response from OpenAI: {}", response_text);
    Ok(())
}