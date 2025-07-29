use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConfigRecord {
    pub id: String,
    pub name: String,
    pub anthropic_base_url: String,
    pub anthropic_api_key: Option<String>,
    pub anthropic_auth_token: Option<String>,
    pub access_password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateConfigRequest {
    pub name: String,
    pub anthropic_base_url: String,
    pub anthropic_api_key: Option<String>,
    pub anthropic_auth_token: Option<String>,
    pub access_password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub name: Option<String>,
    pub anthropic_base_url: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub anthropic_auth_token: Option<String>,
    pub access_password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub password: String,
}

impl ConfigRecord {
    pub fn new(req: CreateConfigRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: req.name,
            anthropic_base_url: req.anthropic_base_url,
            anthropic_api_key: req.anthropic_api_key,
            anthropic_auth_token: req.anthropic_auth_token,
            access_password: req.access_password,
            created_at: now,
            updated_at: now,
        }
    }
}