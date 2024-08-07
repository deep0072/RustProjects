use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub async fn get_database() -> Database {
    let client_uri = "nkjkj";
    let client_options = ClientOptions::parse(client_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    client.database("paytm_rust")
}
