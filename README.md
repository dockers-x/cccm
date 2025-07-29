# Claude Code Configuration Manager (CCCM)

A modern web application for managing Claude Code configurations, built with Rust + Axum, featuring one-click script downloads and multilingual support.

[中文](README_CN.md) | **English**

## ✨ Key Features

### 🌍 Multilingual Support
- **Language Toggle**: Switch between English and Chinese on all pages
- **Internationalization**: Full i18n support with persistent language preferences
- **Login Interface**: Language selection available on login page

### 🔐 Security & Authentication
- **Admin Password**: Configurable via `ADMIN_PASSWORD` environment variable (default: admin123)
- **Per-Config Access Control**: Individual access passwords for each configuration
- **URL Parameter Validation**: Password verification through URL parameters

### 🎨 Modern Interface
- **Custom SVG Icons**: Professional iconography throughout the interface
- **Pico CSS**: Clean, modern design framework
- **Rich Animations**: Smooth interactive animations and transitions
- **Responsive Design**: Mobile-friendly interface

### ⚙️ Flexible Script Generation
- **Environment Variables**: Generate scripts that use environment variables
- **Config File Method**: Traditional settings.json configuration approach
- **User Choice**: Let users choose their preferred setup method
- **Auto-Detection**: Smart shell detection and configuration

### 📋 Configuration Management
- **CRUD Operations**: Complete configuration lifecycle management
- **Multiple Auth Methods**: Support for both API Keys and Auth Tokens
- **One-Click Scripts**: Dynamically generated bash configuration scripts
- **Instant Downloads**: Direct download via wget/curl

## 🚀 Quick Start

### Running the Application

**Using Cargo:**
```bash
# Run with default password
cargo run

# Run with custom admin password
ADMIN_PASSWORD=my_secure_password cargo run
```

**Using Docker:**
```bash
# Simple Docker run
docker build -t cccm .
docker run -p 3000:3000 -e ADMIN_PASSWORD=your_password cccm

# Using docker-compose
docker-compose up -d
```

### Accessing the Application
- Open browser and navigate to: `http://localhost:3000`
- Login with admin password (default: admin123)
- Switch language using the language selector in the top-right corner

## 📖 Usage Guide

### 1. Adding a Configuration
1. Login to the admin dashboard
2. Fill in the configuration details:
   - **Configuration Name**: Easily identifiable name
   - **Anthropic Base URL**: API endpoint address
   - **API Key / Auth Token**: Choose one authentication method
   - **Access Password**: Optional, for download protection
   - **Setup Method**: Choose between environment variables or config file

### 2. Downloading Scripts

**For configurations without password protection:**
```bash
curl -O http://localhost:3000/download/{config_id}
# or
wget http://localhost:3000/download/{config_id}
```

**For password-protected configurations:**
```bash
curl -O "http://localhost:3000/download/{config_id}?password=your_password"
# or
wget "http://localhost:3000/download/{config_id}?password=your_password"
```

### 3. Running Configuration Scripts
```bash
chmod +x setup_*.sh
./setup_*.sh
```

## 🔧 Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ADMIN_PASSWORD` | Admin login password | admin123 |
| `DATABASE_PATH` | SQLite database file path | ./config.db |
| `RUST_LOG` | Logging level | info |
| `TZ` | Timezone | UTC |

## 🐳 Docker Deployment

### Basic Docker Compose
```yaml
version: '3.8'
services:
  cccm:
    build: .
    ports:
      - "3000:3000"
    environment:
      - ADMIN_PASSWORD=your_secure_password
      - DATABASE_PATH=/app/data/config.db
    volumes:
      - cccm_data:/app/data
    restart: unless-stopped

volumes:
  cccm_data:
```

### Production with Nginx
Use `docker-compose.production.yml` for a production setup with Nginx reverse proxy:

```bash
cp .env.example .env  # Configure your environment
docker-compose -f docker-compose.production.yml up -d
```

## 🏗️ Technology Stack

- **Backend**: Rust + Axum + SQLite3
- **Frontend**: HTML + CSS + JavaScript + Pico CSS
- **Templates**: Askama template engine
- **Database**: SQLite3 with automatic migrations
- **Internationalization**: JSON-based message files
- **Docker**: Multi-stage builds with optimized images

## 📁 Project Structure

```
src/
├── main.rs              # Application entry point
├── auth.rs             # Authentication module
├── database.rs         # Database initialization
├── i18n.rs             # Internationalization
├── handlers/           # HTTP request handlers
├── models/             # Data models
└── templates/          # HTML templates
    ├── login.html      # Login page
    ├── dashboard.html  # Management dashboard
    └── download_password.html # Password input page

locales/
├── en.json             # English translations
└── zh.json             # Chinese translations

migrations/             # Database migrations
nginx/                  # Nginx configuration files
```

## 🔒 Security Features

- **Password Protection**: Admin dashboard requires authentication
- **Per-Configuration Access Control**: Individual passwords for each configuration
- **Session Management**: Cookie-based session control
- **Input Validation**: Strict validation and error handling
- **SQL Injection Protection**: Using SQLx prepared statements
- **XSS Protection**: Template-based HTML escaping

## 🎯 Generated Script Features

### Environment Variables Method
- **Shell Detection**: Automatic shell configuration file detection
- **Backup Creation**: Automatic backup of existing configurations
- **Clean Installation**: Removes conflicting environment variables
- **Session Variables**: Sets variables for current session

### Config File Method
- **Dependency Checking**: Automatic jq tool verification
- **Backup Functionality**: Automatic backup of existing settings
- **JSON Validation**: Ensures valid configuration files
- **Colored Output**: User-friendly command-line interface
- **Error Handling**: Comprehensive error messages
- **Cross-Platform**: Supports macOS/Linux/WSL

## 🌐 Internationalization

The application supports multiple languages:

- **English** (default)
- **Chinese (Simplified)**

Language preferences are:
- Stored in localStorage for persistence
- Applied across all pages including login
- Synchronized between language selector and page content

## 📝 License

MIT License

## 🤝 Contributing

Contributions are welcome! Please feel free to submit Issues and Pull Requests.

### Development Setup
```bash
# Clone the repository
git clone <your-repo-url>
cd cccm

# Install dependencies and run
cargo run

# Run tests
cargo test
```

## 📊 Changelog

### v1.1.0
- ✅ Fixed multilingual switching issues
- ✅ Added language selector to login page
- ✅ Fixed language selector sync issues
- ✅ Added environment variable script generation
- ✅ Enhanced Docker configuration with persistence
- ✅ Added comprehensive documentation

### v1.0.0
- 🎉 Initial release
- ✅ Basic configuration management
- ✅ Script generation and download
- ✅ Admin authentication
- ✅ SQLite database with migrations