use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[actix_web::post("/login")]
async fn authenticate(req: web::Json<AuthRequest>) -> Result<HttpResponse> {
    match snap_tray_auth::db::authenticate_user(&req.username, &req.password).await {
        Ok(Some(snap_tray_auth::db::AuthResult::User(user))) => Ok(HttpResponse::Ok().json(user)),
        Ok(Some(snap_tray_auth::db::AuthResult::Requires2FA { email })) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({ "requires2FA": true, "email": email })))
        }
        Ok(None) => Ok(HttpResponse::Ok().json(None::<snap_tray_auth::db::User>)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({ "error": e }))),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(authenticate)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}