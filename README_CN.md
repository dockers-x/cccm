# Claude Code 配置管理器 (CCCM)

一个现代化的Claude Code配置管理Web应用，使用Rust + Axum构建，支持一键脚本下载和多语言支持。

**中文** | [English](README.md)

## ✨ 主要特性

### 🌍 多语言支持
- **语言切换**: 在所有页面中英文自由切换
- **国际化**: 完整的i18n支持，语言偏好持久化
- **登录界面**: 登录页面也支持语言选择

### 🔐 安全认证
- **管理员密码**: 支持环境变量 `ADMIN_PASSWORD` 设置（默认: admin123）
- **配置访问控制**: 每个配置可设置独立的访问密码
- **URL密码验证**: 下载时通过URL参数验证访问权限

### 🎨 现代化界面
- **自定义SVG图标**: 整个界面使用专业图标
- **Pico CSS**: 简洁现代的设计框架
- **丰富动画**: 流畅的交互动画和过渡效果
- **响应式设计**: 支持移动端友好界面

### ⚙️ 灵活脚本生成
- **环境变量**: 生成使用环境变量的脚本
- **配置文件**: 传统的settings.json配置方式
- **用户选择**: 让用户选择偏好的设置方式
- **智能检测**: 智能Shell检测和配置

### 📋 配置管理
- **CRUD操作**: 完整的配置生命周期管理
- **多种认证**: 支持API Key和Auth Token
- **一键脚本**: 动态生成bash配置脚本
- **即时下载**: 通过wget/curl直接下载

## 🚀 快速开始

### 运行应用

**使用Cargo:**
```bash
# 使用默认密码启动
cargo run

# 使用自定义管理员密码
ADMIN_PASSWORD=my_secure_password cargo run
```

**使用Docker:**
```bash
# 简单Docker运行
docker build -t cccm .
docker run -p 3000:3000 -e ADMIN_PASSWORD=your_password cccm

# 使用docker-compose
docker-compose up -d
```

### 访问应用
- 打开浏览器访问: `http://localhost:3000`
- 使用管理员密码登录（默认: admin123）
- 使用右上角的语言选择器切换语言

## 📖 使用说明

### 1. 添加配置
1. 登录管理后台
2. 填写配置信息：
   - **配置名称**: 易识别的名称
   - **Anthropic Base URL**: API端点地址
   - **API Key / Auth Token**: 选择一种认证方式
   - **访问密码**: 可选，用于下载保护
   - **设置方式**: 选择环境变量或配置文件

### 2. 下载脚本

**无密码保护的配置:**
```bash
curl -O http://localhost:3000/download/{config_id}
# 或
wget http://localhost:3000/download/{config_id}
```

**有密码保护的配置:**
```bash
curl -O "http://localhost:3000/download/{config_id}?password=your_password"
# 或
wget "http://localhost:3000/download/{config_id}?password=your_password"
```

### 3. 执行配置脚本
```bash
chmod +x setup_*.sh
./setup_*.sh
```

## 🔧 环境变量

| 变量名 | 描述 | 默认值 |
|--------|------|--------|
| `ADMIN_PASSWORD` | 管理员登录密码 | admin123 |
| `DATABASE_PATH` | SQLite数据库文件路径 | ./config.db |
| `RUST_LOG` | 日志级别 | info |
| `TZ` | 时区 | UTC |

## 🐳 Docker部署

### 基础Docker Compose
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

### 生产环境配置
使用 `docker-compose.production.yml` 进行生产环境部署，包含Nginx反向代理：

```bash
cp .env.example .env  # 配置环境变量
docker-compose -f docker-compose.production.yml up -d
```

## 🏗️ 技术栈

- **后端**: Rust + Axum + SQLite3
- **前端**: HTML + CSS + JavaScript + Pico CSS
- **模板**: Askama模板引擎
- **数据库**: SQLite3，自动迁移
- **国际化**: 基于JSON的消息文件
- **Docker**: 多阶段构建，优化镜像

## 📁 项目结构

```
src/
├── main.rs              # 应用程序入口
├── auth.rs             # 认证模块
├── database.rs         # 数据库初始化
├── i18n.rs             # 国际化
├── handlers/           # HTTP请求处理器
├── models/             # 数据模型
└── templates/          # HTML模板
    ├── login.html      # 登录页面
    ├── dashboard.html  # 管理面板
    └── download_password.html # 密码输入页面

locales/
├── en.json             # 英文翻译
└── zh.json             # 中文翻译

migrations/             # 数据库迁移
nginx/                  # Nginx配置文件
```

## 🔒 安全特性

- **密码保护**: 管理面板需要认证
- **配置级访问控制**: 每个配置独立密码
- **会话管理**: 基于Cookie的会话控制
- **输入验证**: 严格的验证和错误处理
- **SQL注入防护**: 使用SQLx预编译语句
- **XSS防护**: 基于模板的HTML转义

## 🎯 生成脚本特性

### 环境变量方式
- **Shell检测**: 自动检测Shell配置文件
- **备份创建**: 自动备份现有配置
- **干净安装**: 移除冲突的环境变量
- **会话变量**: 为当前会话设置变量

### 配置文件方式
- **依赖检查**: 自动检查jq工具
- **备份功能**: 自动备份现有设置
- **JSON验证**: 确保配置文件有效
- **彩色输出**: 用户友好的命令行界面
- **错误处理**: 完善的错误消息
- **跨平台**: 支持macOS/Linux/WSL

## 🌐 国际化

应用支持多种语言：

- **英文** (默认)
- **简体中文**

语言偏好：
- 存储在localStorage中持久化
- 应用于包括登录页在内的所有页面
- 语言选择器与页面内容同步

## 📝 许可证

MIT License

## 🤝 贡献

欢迎贡献！请随时提交Issues和Pull Requests。

### 开发环境设置
```bash
# 克隆仓库
git clone <your-repo-url>
cd cccm

# 安装依赖并运行
cargo run

# 运行测试
cargo test
```

## 📊 更新日志

### v1.1.0
- ✅ 修复多语言切换问题
- ✅ 添加登录页面语言选择器
- ✅ 修复语言选择器同步问题
- ✅ 添加环境变量脚本生成
- ✅ 增强Docker配置，支持数据持久化
- ✅ 添加完整文档

### v1.0.0
- 🎉 初始版本发布
- ✅ 基础配置管理
- ✅ 脚本生成和下载
- ✅ 管理员认证
- ✅ SQLite数据库自动迁移