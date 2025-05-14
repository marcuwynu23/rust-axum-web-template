use dotenv::dotenv;
use mongodb::{Client, Database, options::ClientOptions};
use std::env;
use tracing::{error, info};

pub async fn connect_db() -> Database {
    // ✅ Load environment variables
    dotenv().ok();

    let client_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let db_name = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE must be set in .env");

    println!("{}", client_uri);
    println!("{}", db_name);
    // ✅ Parse connection options
    let options = match ClientOptions::parse(&client_uri).await {
        Ok(opts) => opts,
        Err(e) => {
            error!("Failed to parse MongoDB URI: {}", e);
            panic!("Database connection failed");
        }
    };

    // ✅ Create the client
    let client = match Client::with_options(options) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to initialize MongoDB client: {}", e);
            panic!("Database client initialization failed");
        }
    };

    info!(
        "✅ Successfully connected to MongoDB at URI: {}",
        client_uri
    );

    return client.database(&db_name);
}
