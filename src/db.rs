// MongoDB database module — desktop / mobile (non-wasm) only.
// Requires MONGODB_URI and MONGODB_DB environment variables (or a .env file).

use mongodb::{bson::{doc, DateTime, Document}, Client, Collection};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

// ---------------------------------------------------------------------------
// User model — adjust fields to match your collection schema
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub username: String,
    /// bcrypt hash — field is named `password` in MongoDB
    pub password: String,
    pub email: String,
    #[serde(rename = "isVerified", default)]
    pub is_verified: bool,
    #[serde(rename = "usertype")]
    pub user_type: String,
    pub balance: f64,
    #[serde(rename = "isBanned", default)]
    pub is_banned: bool,
    /// Nested identity object (contents vary — kept as raw BSON)
    pub identity: Option<Document>,
    /// Nested encryption object
    pub encryption: Option<Document>,
    /// Whether 2FA is active for this user
    #[serde(rename = "is2Active", default)]
    pub is_2active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime>,
    #[serde(rename = "lastActive")]
    pub last_active: Option<DateTime>,
    #[serde(rename = "userPersonalInfo", default)]
    pub user_personal_info: Vec<Document>,
    #[serde(default)]
    pub devices: Vec<Document>,
    /// Mongoose internal version key
    #[serde(rename = "__v", default)]
    pub version: i32,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

async fn get_client() -> Result<&'static Client, String> {
    CLIENT
        .get_or_try_init(|| async {
            let uri = std::env::var("MONGODB_URI")
                .map_err(|_| "MONGODB_URI environment variable not set".to_string())?;
            Client::with_uri_str(&uri)
                .await
                .map_err(|e| e.to_string())
        })
        .await
}

async fn collection() -> Result<Collection<User>, String> {
    let client = get_client().await?;
    let db_name = std::env::var("MONGODB_DB").unwrap_or_else(|_| "Projekt_vizsgaremek".to_string());
    Ok(client.database(&db_name).collection::<User>("users"))
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Look up a user by username (does **not** check the password).
pub async fn get_user_by_username(username: &str) -> Result<Option<User>, String> {
    let col = collection().await?;
    col.find_one(doc! { "username": username })
        .await
        .map_err(|e| e.to_string())
}

/// Verify credentials using only username + password.
/// Returns the full `User` on success, `None` on bad credentials.
pub async fn authenticate_user(username: &str, password: &str) -> Result<Option<User>, String> {
    #[derive(Deserialize)]
    struct LoginProjection {
        password: String,
    }

    use mongodb::options::FindOneOptions;

    let client = get_client().await?;
    let db_name = std::env::var("MONGODB_DB").unwrap_or_else(|_| "Projekt_vizsgaremek".to_string());
    let col = client
        .database(&db_name)
        .collection::<LoginProjection>("users");

    let opts = FindOneOptions::builder()
        .projection(doc! { "password": 1 })
        .build();

    let result = col
        .find_one(doc! { "username": username })
        .with_options(opts)
        .await
        .map_err(|e| e.to_string())?;

    match result {
        Some(proj) if bcrypt::verify(password, &proj.password).unwrap_or(false) => {
            // Password matched — now fetch the full user
            get_user_by_username(username).await
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{Client, bson::Document};
    use futures::TryStreamExt;

    async fn make_client() -> Client {
        dotenvy::dotenv().ok();
        let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not set");
        Client::with_uri_str(&uri).await.expect("Failed to connect")
    }

    fn db_name() -> String {
        std::env::var("MONGODB_DB").unwrap_or_else(|_| "Projekt_vizsgaremek".to_string())
    }

    /// Lists all collections and all documents in every collection.
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
