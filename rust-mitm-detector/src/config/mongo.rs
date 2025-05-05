// === src/config/mongo.rs ===
use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub async fn init() -> mongodb::error::Result<Database> {
    let uri = env::var("MONGO_URI").expect("MONGO_URI nu este setată în .env");
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database("mitm_detector"))
}
