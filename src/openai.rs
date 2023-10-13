use serde_json::json;
use serde_json::Value;
use crate::openai::my_error::MyError;
// Create a new module for your custom error type
pub mod my_error {
    use serde_json::Error as SerdeJsonError;
    use reqwest::Error as ReqwestError;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum MyError {
        #[error("Serde JSON error: {0}")]
        SerdeJson(#[from] SerdeJsonError),
        #[error("Reqwest error: {0}")]
        Reqwest(#[from] ReqwestError),
    }
}

pub async fn chat_with_gpt(message: &str, api_key: &str) -> Result<String, MyError> {
    let endpoint = "https://api.openai.com/v1/chat/completions";
    
    let client = reqwest::Client::new();
    
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": message}],
            "max_tokens": 200,  // Adjust the token limit as needed
        }))
        .send()
        .await?;

    let body = response.text().await?;
    
    // Parse the JSON response and handle any potential errors
    let parsed_response: Value = serde_json::from_str(&body)?;
    
    // Extract the "content" field from the JSON
    let gpt_reply = parsed_response["choices"][0]["message"]["content"].as_str().unwrap_or("No content found");

    Ok(gpt_reply.to_string())
}
