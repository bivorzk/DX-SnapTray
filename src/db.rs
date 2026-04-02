use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    #[serde(rename = "is2Active", default)]
    pub is_2active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthResult {
    User(User),
    Requires2FA { email: String },
}

/// Server function: runs on the server, clients call it via HTTP automatically.
/// MongoDB, bcrypt, dotenvy are only compiled when the "server" feature is active.
#[server]
pub async fn authenticate_user(username: String, password: String) -> Result<Option<AuthResult>, ServerFnError> {
    use mongodb::{bson::doc, Client};
    use mongodb::options::FindOneOptions;
    use tokio::sync::OnceCell;

    static CLIENT: OnceCell<Client> = OnceCell::const_new();

    #[derive(serde::Deserialize)]
    struct LoginProjection { password: String }
    #[derive(serde::Deserialize)]
    struct EmailProjection { email: String, #[serde(rename = "is2Active", default)] is_2active: bool }

    let client = CLIENT.get_or_try_init(|| async {
        dotenvy::dotenv().ok();
        let uri = match std::env::var("MONGODB_URI") {
            Ok(u) => u,
            Err(_) => return Err(mongodb::error::Error::custom("MONGODB_URI environment variable not set")),
        };
        Client::with_uri_str(&uri).await
    }).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let db_name = std::env::var("MONGODB_DB").unwrap_or_else(|_| "Projekt_vizsgaremek".to_string());
    let col = client.database(&db_name).collection::<LoginProjection>("users");

    let result = col
        .find_one(doc! { "username": &username })
        .with_options(FindOneOptions::builder().projection(doc! { "password": 1 }).build())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    match result {
        Some(proj) if bcrypt::verify(&password, &proj.password).unwrap_or(false) => {
            let col2 = client.database(&db_name).collection::<EmailProjection>("users");
            let info = col2.find_one(doc! { "username": &username }).await
                .map_err(|e| ServerFnError::new(e.to_string()))?
                .ok_or_else(|| ServerFnError::new("User not found"))?;
            if info.is_2active {
                Ok(Some(AuthResult::Requires2FA { email: info.email }))
            } else {
                Ok(Some(AuthResult::User(User { username, email: info.email, is_2active: false })))
            }
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{Client, bson::doc, bson::Document};
    use futures::TryStreamExt;

    async fn make_client() -> Client {
        dotenvy::dotenv().ok();
        let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not set");
        Client::with_uri_str(&uri).await.expect("Failed to connect")
    }

    fn db_name() -> String {
        std::env::var("MONGODB_DB").unwrap_or_else(|_| "Projekt_vizsgaremek".to_string())
    }

    #[tokio::test]
    async fn test_list_all() {
        let client = make_client().await;
        let db = client.database(&db_name());

        let collections = db
            .list_collection_names()
            .await
            .expect("Failed to list collections");

        if collections.is_empty() {
            println!("Database '{}' has no collections.", db_name());
            return;
        }

        println!("=== Database: {} ===", db_name());
        for col_name in &collections {
            println!("\n--- Collection: {} ---", col_name);
            let col = db.collection::<Document>(col_name);
            let mut cursor = col.find(doc! {}).await.expect("find failed");
            let mut count = 0;
            while let Some(doc) = cursor.try_next().await.expect("cursor error") {
                println!("{:#?}", doc);
                count += 1;
            }
            if count == 0 {
                println!("  (empty)");
            } else {
                println!("  Total: {} document(s)", count);
            }
        }
    }
}
