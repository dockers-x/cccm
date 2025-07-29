# Claude Code Configuration Manager (CCCM)

一个现代化的Claude Code配置管理Web应用，使用Rust + Axum构建，支持一键脚本下载和访问控制。

## ✨ 主要特性

### 🔐 安全认证
- **管理员密码**: 支持环境变量 `ADMIN_PASSWORD` 设置（默认: admin123）
- **配置访问控制**: 每个配置可设置独立的访问密码
- **URL密码验证**: 下载时通过URL参数验证访问权限

### 🎨 现代化界面
- **SVG图标**: 使用自定义SVG图标替代文字按钮
- **Pico CSS**: 简洁现代的界面设计
- **CSS动画**: 丰富的交互动画效果
- **响应式设计**: 支持移动端访问

### 📋 配置管理
- **CRUD操作**: 完整的配置增删改查功能
- **多种认证**: 支持API Key和Auth Token
- **一键脚本**: 动态生成bash配置脚本
- **即时下载**: wget/curl直接下载配置脚本

## 🚀 快速开始

### 运行应用
```bash
# 使用默认密码启动
cargo run

# 使用自定义管理员密码
ADMIN_PASSWORD=my_secure_password cargo run
```

### 访问应用
- 打开浏览器访问: `http://localhost:3000`
- 使用管理员密码登录（默认: admin123）

## 📖 使用说明

### 1. 添加配置
1. 登录管理后台
2. 填写配置信息：
   - **配置名称**: 易识别的名称
   - **Anthropic Base URL**: API端点地址
   - **API Key / Auth Token**: 二选一填写
   - **访问密码**: 可选，用于下载保护

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

## 🏗️ 技术栈

- **后端**: Rust + Axum + SQLite3
- **前端**: HTML + CSS + JavaScript + Pico CSS
- **模板**: Askama
- **数据库**: SQLite3（自动初始化）

## 📁 项目结构

```
src/
├── main.rs           # 主程序入口
├── auth.rs           # 认证模块
├── database.rs       # 数据库初始化
├── handlers/         # HTTP处理器
├── models/           # 数据模型
└── templates/        # HTML模板
    ├── login.html    # 登录页面
    └── dashboard.html # 管理面板
```

## 🔒 安全特性

- **密码保护**: 管理后台需要密码登录
- **配置级访问控制**: 每个配置可独立设置访问密码
- **会话管理**: 基于Cookie的会话控制
- **参数验证**: 严格的输入验证和错误处理

## 🎯 生成的脚本特性

- **依赖检查**: 自动检查jq工具
- **备份功能**: 自动备份现有配置
- **彩色输出**: 友好的命令行界面
- **错误处理**: 完善的错误提示
- **跨平台**: 支持macOS/Linux/WSL

## 📝 许可证

MIT License

## 🤝 贡献

欢迎提交Issue和Pull Request！