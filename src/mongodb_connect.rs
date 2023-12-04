use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;

pub async fn get_mongodb_client() -> Result<Client, mongodb::error::Error> {
    // Configure MongoDB client options
    // Retrieve the MongoDB URI from the env variable
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not set in .env file");

    // Configure MongoDB client options
    let client_options = ClientOptions::parse(&mongodb_uri).await?;
    // Create and return the MongoDB client
    let client = Client::with_options(client_options)?;

    Ok(client)
}