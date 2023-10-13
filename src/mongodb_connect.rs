use mongodb::{Client, options::ClientOptions};

// Define a function to create and return a MongoDB client
pub async fn get_mongodb_client() -> Result<Client, mongodb::error::Error> {
    // Configure MongoDB client options
    let client_options = ClientOptions::parse("mongodb+srv://chamithwakista:HpfoBN26dN72uTSM@cluster0.pqapgw1.mongodb.net/discord?retryWrites=true&w=majority").await?;

    // You can specify additional options here, such as authentication

    // Create and return the MongoDB client
    let client = Client::with_options(client_options)?;

    Ok(client)
}