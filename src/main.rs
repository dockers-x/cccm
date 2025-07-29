use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use std::env;

mod auth;
mod database;
mod handlers;
mod models;
mod i18n;

use database::init_db;
use handlers::*;
use auth::auth_middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🤖 Starting Claude Code Config Manager...");
    
    let pool = init_db().await?;
    println!("✅ Database initialized");
    
    let app = Router::new()
        .route("/", get(login_page))
        .route("/login", post(login_post))
        .route("/dashboard", get(dashboard))
        .route("/logout", get(logout))
        .route("/api/configs", post(create_config))
        .route("/api/configs/:id", delete(delete_config))
        .route("/api/configs/:id/password", get(get_config_password))
        .route("/download/:id", get(download_script))
        .layer(middleware::from_fn(auth_middleware))
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(pool);
    
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".to_string());
    
    println!("🚀 Server starting on http://0.0.0.0:3000");
    if admin_password == "admin123" {
        println!("⚠️  Using default admin password: admin123");
        println!("   Set ADMIN_PASSWORD environment variable for production");
    } else {
        println!("✅ Using custom admin password from ADMIN_PASSWORD");
    }
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}