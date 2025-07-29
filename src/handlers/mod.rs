use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json, Form,
};
use axum_extra::extract::CookieJar;
use askama::Template;
use sqlx::SqlitePool;
use serde::Deserialize;

use crate::models::{ConfigRecord, CreateConfigRequest, LoginRequest};
use crate::auth::verify_password;
use crate::i18n::I18n;

#[derive(Deserialize)]
pub struct DownloadQuery {
    password: Option<String>,
    lang: Option<String>,
}

#[derive(Deserialize)]
pub struct LangQuery {
    lang: Option<String>,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
    i18n: I18n,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    configs: Vec<ConfigRecord>,
    i18n: I18n,
}

#[derive(Template)]
#[template(path = "download_password.html")]
struct DownloadPasswordTemplate {
    config_id: String,
    config_name: String,
    error: Option<String>,
    i18n: I18n,
}

pub async fn login_page(Query(query): Query<LangQuery>) -> Html<String> {
    let mut i18n = I18n::new();
    if let Some(lang) = query.lang {
        i18n.set_language(&lang);
    }
    let template = LoginTemplate { error: None, i18n };
    Html(template.render().unwrap())
}

pub async fn login_post(
    jar: CookieJar,
    Form(req): Form<LoginRequest>,
) -> Result<(CookieJar, axum::response::Redirect), (CookieJar, Html<String>)> {
    if verify_password(&req.password) {
        let jar = jar.add(("session", "authenticated"));
        Ok((jar, axum::response::Redirect::to("/dashboard")))
    } else {
        let i18n = I18n::new();
        let template = LoginTemplate { 
            error: Some(i18n.t("invalid_password")),
            i18n
        };
        Err((jar, Html(template.render().unwrap())))
    }
}

pub async fn dashboard(State(pool): State<SqlitePool>, Query(query): Query<LangQuery>) -> Result<Html<String>, StatusCode> {
    let configs = sqlx::query_as::<_, ConfigRecord>(
        "SELECT * FROM config_records ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut i18n = I18n::new();
    if let Some(lang) = query.lang {
        i18n.set_language(&lang);
    }
    
    let template = DashboardTemplate { configs, i18n };
    Ok(Html(template.render().unwrap()))
}

pub async fn create_config(
    State(pool): State<SqlitePool>,
    Json(req): Json<CreateConfigRequest>,
) -> Result<Json<ConfigRecord>, StatusCode> {
    let config = ConfigRecord::new(req);
    
    sqlx::query(
        r#"
        INSERT INTO config_records (id, name, anthropic_base_url, anthropic_api_key, anthropic_auth_token, access_password, setup_method, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&config.id)
    .bind(&config.name)
    .bind(&config.anthropic_base_url)
    .bind(&config.anthropic_api_key)
    .bind(&config.anthropic_auth_token)
    .bind(&config.access_password)
    .bind(&config.setup_method)
    .bind(&config.created_at)
    .bind(&config.updated_at)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(config))
}

pub async fn delete_config(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM config_records WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::OK)
}

pub async fn download_script(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Query(query): Query<DownloadQuery>,
) -> impl IntoResponse {
    // Set up i18n
    let mut i18n = I18n::new();
    if let Some(lang) = &query.lang {
        i18n.set_language(lang);
    }
    
    // Get config from database
    let config = match sqlx::query_as::<_, ConfigRecord>(
        "SELECT * FROM config_records WHERE id = ?"
    )
    .bind(&id) 
    .fetch_one(&pool)
    .await
    {
        Ok(config) => config,
        Err(_) => return (StatusCode::NOT_FOUND, "Configuration not found").into_response(),
    };
    
    // Check if config requires password
    if let Some(required_password) = &config.access_password {
        // Config requires password
        match &query.password {
            Some(provided_password) => {
                // Password provided, check if correct
                if provided_password.trim() != required_password.trim() {
                    // Wrong password, show password page with error
                    let template = DownloadPasswordTemplate {
                        config_id: id,
                        config_name: config.name,
                        error: Some(i18n.t("incorrect_password")),
                        i18n,
                    };
                    return Html(template.render().unwrap()).into_response();
                }
                // Password correct, continue with download
            }
            None => {
                // No password provided, show password input page
                let template = DownloadPasswordTemplate {
                    config_id: id,
                    config_name: config.name,
                    error: None,
                    i18n,
                };
                return Html(template.render().unwrap()).into_response();
            }
        }
    }
    
    // No password required or password verified, proceed with download
    let auth_token = match config.anthropic_api_key.as_ref()
        .or(config.anthropic_auth_token.as_ref())
    {
        Some(token) => token,
        None => return (StatusCode::BAD_REQUEST, "No authentication token configured").into_response(),
    };
    
    let filename = format!("setup_{}.sh", config.name.replace(" ", "_"));
    
    // Generate script based on setup method
    let setup_method = config.setup_method.as_deref().unwrap_or("config_file");
    let script = if setup_method == "env_vars" {
        generate_env_vars_script(&config, auth_token)
    } else {
        generate_config_file_script(&config, auth_token)
    };
    
    let headers = [
        ("Content-Type", "application/x-sh"),
        ("Content-Disposition", &format!("attachment; filename=\"{}\"", filename)),
    ];
    
    (headers, script).into_response()
}

fn generate_env_vars_script(config: &ConfigRecord, auth_token: &str) -> String {
    // Check if we should use API_KEY or AUTH_TOKEN
    let auth_var = if config.anthropic_api_key.is_some() {
        "ANTHROPIC_API_KEY"
    } else {
        "ANTHROPIC_AUTH_TOKEN"
    };
    
    let other_auth_var = if auth_var == "ANTHROPIC_API_KEY" {
        "ANTHROPIC_AUTH_TOKEN"
    } else {
        "ANTHROPIC_API_KEY"
    };

    // Use simple string concatenation to avoid format! macro issues
    let mut script = String::new();
    script.push_str("#!/bin/bash\n\n");
    script.push_str(&format!("# Claude Code Environment Configuration Script for {}\n", config.name));
    script.push_str("# This script configures Claude Code using environment variables\n\n");
    script.push_str("set -e\n\n");
    script.push_str("# Configuration\n");
    script.push_str(&format!("BASE_URL=\"{}\"\n", config.anthropic_base_url));
    script.push_str(&format!("AUTH_TOKEN=\"{}\"\n", auth_token));
    script.push_str(&format!("AUTH_VAR=\"{}\"\n", auth_var));
    script.push_str(&format!("OTHER_AUTH_VAR=\"{}\"\n\n", other_auth_var));
    
    script.push_str("# Main configuration function\n");
    script.push_str("main() {\n");
    script.push_str(&format!("    echo \"Claude Code Environment Configuration for {}\"\n", config.name));
    script.push_str("    echo \"=====================================================\"\n\n");
    
    script.push_str("    # Set environment variables for current session\n");
    script.push_str("    export ANTHROPIC_BASE_URL=\"$BASE_URL\"\n");
    script.push_str("    export $AUTH_VAR=\"$AUTH_TOKEN\"\n");
    script.push_str("    unset $OTHER_AUTH_VAR\n\n");
    
    script.push_str("    # Detect shell config file\n");
    script.push_str("    if [ -n \"$ZSH_VERSION\" ]; then\n");
    script.push_str("        CONFIG_FILE=\"$HOME/.zshrc\"\n");
    script.push_str("    elif [ -n \"$BASH_VERSION\" ]; then\n");
    script.push_str("        CONFIG_FILE=\"$HOME/.bashrc\"\n");
    script.push_str("    else\n");
    script.push_str("        CONFIG_FILE=\"$HOME/.profile\"\n");
    script.push_str("    fi\n\n");
    
    script.push_str("    # Backup existing config\n");
    script.push_str("    if [ -f \"$CONFIG_FILE\" ]; then\n");
    script.push_str("        cp \"$CONFIG_FILE\" \"$CONFIG_FILE.backup.$(date +%Y%m%d_%H%M%S)\"\n");
    script.push_str("    fi\n\n");
    
    script.push_str("    # Remove existing Claude Code config\n");
    script.push_str("    if [ -f \"$CONFIG_FILE\" ]; then\n");
    script.push_str("        sed -i.tmp '/ANTHROPIC_/d' \"$CONFIG_FILE\" 2>/dev/null || true\n");
    script.push_str("        sed -i.tmp '/Claude Code Configuration/d' \"$CONFIG_FILE\" 2>/dev/null || true\n");
    script.push_str("        rm -f \"$CONFIG_FILE.tmp\"\n");
    script.push_str("    fi\n\n");
    
    script.push_str("    # Add new configuration\n");
    script.push_str("    echo \"\" >> \"$CONFIG_FILE\"\n");
    script.push_str("    echo \"# Claude Code Configuration - $(date)\" >> \"$CONFIG_FILE\"\n");
    script.push_str("    echo \"export ANTHROPIC_BASE_URL=\\\"$BASE_URL\\\"\" >> \"$CONFIG_FILE\"\n");
    script.push_str("    echo \"export $AUTH_VAR=\\\"$AUTH_TOKEN\\\"\" >> \"$CONFIG_FILE\"\n");
    script.push_str("    echo \"unset $OTHER_AUTH_VAR\" >> \"$CONFIG_FILE\"\n");
    script.push_str("    echo \"\" >> \"$CONFIG_FILE\"\n\n");
    
    script.push_str("    echo \"Claude Code configured successfully!\"\n");
    script.push_str("    echo \"Environment variables added to: $CONFIG_FILE\"\n");
    script.push_str("    echo \"Restart your terminal or run: source $CONFIG_FILE\"\n");
    script.push_str("}\n\n");
    
    script.push_str("main \"$@\"\n");
    
    script
}

fn generate_config_file_script(config: &ConfigRecord, auth_token: &str) -> String {
    format!(
        r#"#!/bin/bash

# Claude Code Configuration Script for {}
# This script configures Claude Code to use your custom instance

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BASE_URL="{}"
API_KEY="{}"
CLAUDE_CONFIG_DIR="$HOME/.claude"
CLAUDE_SETTINGS_FILE="$CLAUDE_CONFIG_DIR/settings.json"

# Function to print colored output
print_info() {{
    echo -e "${{BLUE}}[INFO]${{NC}} $1"
}}

print_success() {{
    echo -e "${{GREEN}}[SUCCESS]${{NC}} $1"
}}

print_warning() {{
    echo -e "${{YELLOW}}[WARNING]${{NC}} $1"
}}

print_error() {{
    echo -e "${{RED}}[ERROR]${{NC}} $1"
}}

# Function to check if jq is installed
check_jq() {{
    if ! command -v jq &> /dev/null; then
        print_error "jq is required but not installed."
        print_info "Please install jq:"
        print_info "  macOS: brew install jq"
        print_info "  Ubuntu/Debian: sudo apt-get install jq"
        print_info "  CentOS/RHEL: sudo yum install jq"
        print_info "  Windows: Use Windows Subsystem for Linux (WSL) or install jq manually"
        exit 1
    fi
}}

# Function to backup existing settings
backup_settings() {{
    if [ -f "$CLAUDE_SETTINGS_FILE" ]; then
        local backup_file="${{CLAUDE_SETTINGS_FILE}}.backup.$(date +%Y%m%d_%H%M%S)"
        cp "$CLAUDE_SETTINGS_FILE" "$backup_file"
        print_info "Backed up existing settings to: $backup_file"
    fi
}}

# Function to create settings directory
create_settings_dir() {{
    if [ ! -d "$CLAUDE_CONFIG_DIR" ]; then
        mkdir -p "$CLAUDE_CONFIG_DIR"
        print_info "Created Claude configuration directory: $CLAUDE_CONFIG_DIR"
    fi
}}

# Function to create Claude Code settings
create_settings() {{
    local settings_json
    settings_json=$(cat <<EOF
{{
  "env": {{
    "ANTHROPIC_BASE_URL": "$BASE_URL",
    "ANTHROPIC_AUTH_TOKEN": "$API_KEY",
    "CLAUDE_CODE_MAX_OUTPUT_TOKENS": 20000,
    "DISABLE_TELEMETRY": 1,
    "DISABLE_ERROR_REPORTING": 1,
    "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": 1,
    "CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR": 1,
    "MAX_THINKING_TOKENS": 12000
  }},
  "permissions": {{
    "allow": [
      "Bash(*)",
      "LS(*)",
      "Read(*)",
      "Write(*)",
      "Edit(*)",
      "MultiEdit(*)",
      "Glob(*)",
      "Grep(*)",
      "Task(*)",
      "WebFetch(*)",
      "WebSearch(*)",
      "TodoWrite(*)",
      "NotebookRead(*)",
      "NotebookEdit(*)"
    ],
    "deny": []
  }},
  "model": "sonnet"
}}
EOF
    )
    
    # Validate JSON
    if ! echo "$settings_json" | jq . > /dev/null 2>&1; then
        print_error "Generated settings JSON is invalid"
        return 1
    fi
    
    # Write settings file
    echo "$settings_json" > "$CLAUDE_SETTINGS_FILE"
    print_success "Claude Code settings written to: $CLAUDE_SETTINGS_FILE"
}}

# Main function
main() {{
    print_info "Claude Code Configuration Script for {}"
    echo "======================================================="
    echo
    
    # Check dependencies
    check_jq
    
    print_info "Configuration:"
    print_info "  Base URL: $BASE_URL"
    print_info "  API Key: ${{API_KEY:0:8}}...${{API_KEY: -4}}"
    echo
    
    # Create settings directory
    create_settings_dir
    
    # Backup existing settings
    backup_settings
    
    # Create new settings
    if create_settings; then
        echo
        print_success "Claude Code has been configured successfully!"
        print_info "You can now use Claude Code with your custom API endpoint."
        print_info ""
        print_info "To verify the setup, run:"
        print_info "  claude --version"
        print_info ""
        print_info "Configuration file location: $CLAUDE_SETTINGS_FILE"
        
        if [ -f "$CLAUDE_SETTINGS_FILE" ]; then
            echo
            print_info "Current settings:"
            cat "$CLAUDE_SETTINGS_FILE" | jq .
        fi
    else
        print_error "Failed to create Claude Code settings"
        exit 1
    fi
}}

# Run main function
main "$@"
"#,
        config.name, config.anthropic_base_url, auth_token, config.name
    )
}

pub async fn get_config_password(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let config = sqlx::query_as::<_, ConfigRecord>(
        "SELECT * FROM config_records WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let response = serde_json::json!({
        "password": config.access_password
    });
    
    Ok(Json(response))
}

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    let jar = jar.remove("session");
    (jar, axum::response::Redirect::to("/"))
}