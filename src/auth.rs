use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use std::env;

const SESSION_COOKIE: &str = "session";

pub fn verify_password(password: &str) -> bool {
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".to_string());
    password == admin_password
}

pub fn verify_config_access(provided_password: Option<&str>, config_password: Option<&str>) -> bool {
    match (provided_password, config_password) {
        (None, None) => true, // No password required
        (Some(provided), Some(required)) => provided == required,
        (None, Some(_)) => false, // Password required but not provided
        (Some(_), None) => true, // Password provided but not required
    }
}

pub async fn auth_middleware(
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for login and download endpoints
    let path = request.uri().path();
    if path == "/login" || path == "/" || path.starts_with("/download") || path.starts_with("/static") {
        return Ok(next.run(request).await);
    }
    
    // Check session cookie
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        if cookie.value() == "authenticated" {
            return Ok(next.run(request).await);
        }
    }
    
    // Redirect to login
    let response = axum::response::Redirect::to("/").into_response();
    Ok(response)
}