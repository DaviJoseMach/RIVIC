use mongodb::{Client, Collection};
use bson::{doc, Document};
use tokio;

#[tokio::main]
async fn main() {
    let client = connect_to_db().await.expect("Failed to connect to MongoDB");
    let db = client.database("mydb");
    insert_document(&db, "users", doc! { "name": "John Doe", "age": 30 }).await.expect("Failed to insert document");
}

async fn connect_to_db() -> mongodb::error::Result<Client> {
    let resolver_config = ResolverConfig::cloudflare_tls();
    let mut client_options = ClientOptions::parse_with_resolver_config(
        "mongodb://localhost:27017/",
        resolver_config,
    )
    .await?;
    client_options.app_name = Some("My Rust App".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}

async fn insert_document(db: &Database, collection_name: &str, document: bson::Document) -> mongodb::error::Result<()> {
    let collection = db.collection(collection_name);
    collection.insert_one(document, None).await?;
    Ok(())
}