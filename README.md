# Antigravity Agent

> 开箱即用的 Antigravity 账户管理程序。

![dashborad.png](screenshots/dashboard.png)

## 📖 项目简介

Antigravity Agent 是一个专为 Antigravity 用户设计的桌面应用程序，提供便捷的多账户管理、配置备份和进程控制功能。通过直观的图形界面，用户可以轻松切换不同的 Antigravity 账户，导入导出加密配置，以及监控 Antigravity 进程状态。

### 项目目的

- **简化账户管理**：提供一键切换多个 Antigravity 账户的能力
- **保护用户数据**：支持加密导出和导入账户配置，确保数据安全
- **提升使用体验**：通过系统托盘和自动化功能，让 Antigravity 使用更加便捷
- **跨平台支持**：基于 Tauri 框架，提供原生性能和跨平台兼容性

## ✨ 主要功能

- 🔄 **多账户管理**：自动识别和记录多个 Antigravity 账户
- 🔀 **快速切换**：一键切换不同账户，自动处理数据迁移
- 🆕 **登录新账户**：安全清除当前数据，引导登录新账户
- 💾 **配置备份**：导出所有账户配置到加密文件
- 📥 **配置恢复**：从加密文件导入账户配置
- 🔐 **加密保护**：使用密码加密导入导出的配置文件
- 🚀 **进程管理**：启动、停止和监控 Antigravity 进程
- 📊 **数据库监控**：实时监控 SQLite 数据库变化
- 🎯 **系统托盘**：后台运行，快速访问常用功能
- 📝 **日志系统**：详细的操作日志，便于问题排查
- 🔕 **静默启动**：支持开机自动启动和后台运行

## 🛠️ 技术栈

### 前端技术
- **React 18** - 现代化的 UI 框架
- **TypeScript 5** - 类型安全的 JavaScript 超集
- **Vite 7** - 快速的前端构建工具
- **Tailwind CSS 3** - 实用优先的 CSS 框架
- **Radix UI** - 无障碍的 UI 组件库
- **Zustand** - 轻量级状态管理
- **Lucide Icons** - 精美的图标库

### 后端技术
- **Rust** - 高性能、内存安全的系统编程语言
- **Tauri 2** - 轻量级桌面应用框架
- **SQLite** - 嵌入式数据库

### 开发工具
- **ESLint** - 代码质量检查
- **Prettier** - 代码格式化
- **TypeScript Compiler** - 类型检查

## 💻 系统要求

### Windows
- **操作系统**：Windows 10 或更高版本（64 位）
- **内存**：至少 4GB RAM
- **磁盘空间**：至少 200MB 可用空间

### macOS
- **操作系统**：macOS 10.15 (Catalina) 或更高版本
- **架构**：支持 Intel 和 Apple Silicon (M1/M2/M3)
- **内存**：至少 4GB RAM
- **磁盘空间**：至少 200MB 可用空间

### Linux
- **状态**：计划中 🚧
- 未来将支持主流 Linux 发行版（Ubuntu、Fedora、Arch 等）

## 📦 平台支持

| 平台 | 状态 | 说明 |
|------|------|------|
| Windows | ✅ 已支持 | Windows 10+ (64-bit) |
| macOS | ✅ 已支持 | macOS 10.15+ (Intel & Apple Silicon) |
| Linux | 🚧 计划中 | 未来版本支持 |

## 下载安装

前往 [Releases](../../releases) 页面下载最新版本。

详细安装指南请参阅：[安装文档](docs/zh-CN/getting-started/installation.md)

## 🚀 快速开始

### 第一次使用
1. 启动 Antigravity Agent
2. 自动识别并记录当前登录用户

### 切换账户
- 从账户列表中选择要切换的账户
- 点击"切换"按钮自动切换

### 登录新账户
- 点击"登录新账户"按钮
- 确认操作后将清除当前用户数据
- 自动启动 Antigravity，登录新账户后自动记录在 Antigravity Agent

### 导入导出
- **导出**: 选择保存位置，设置密码，导出所有账户配置
- **导入**: 选择配置文件，输入密码，恢复账户数据

完整使用教程请参阅：[快速开始指南](docs/zh-CN/getting-started/quickstart.md)

## 📚 文档导航

- **入门文档**
  - [安装指南](docs/zh-CN/getting-started/installation.md) - 详细的安装步骤和系统要求
  - [快速开始](docs/zh-CN/getting-started/quickstart.md) - 5 分钟快速上手教程
  
- **使用文档**
  - [完整使用手册](docs/zh-CN/user-guide/user-guide.md) - 所有功能的详细说明
  - [API 参考](docs/zh-CN/user-guide/api-reference.md) - 命令和接口文档
  - [配置说明](docs/zh-CN/user-guide/configuration.md) - 配置选项和自定义设置
  - [使用示例](docs/zh-CN/user-guide/examples.md) - 常见场景和最佳实践

- **开发文档**
  - [系统架构](docs/zh-CN/development/architecture.md) - 架构设计和技术选型
  - [开发指南](docs/zh-CN/development/development-guide.md) - 环境搭建和开发流程
  - [贡献指南](docs/zh-CN/development/contributing.md) - 如何参与项目开发
  - [代码规范](docs/zh-CN/development/code-style.md) - 代码风格和最佳实践

- **进阶文档**
  - [设计原理](docs/zh-CN/advanced/design-principles.md) - 深入理解设计思路
  - [性能优化](docs/zh-CN/advanced/performance.md) - 性能分析和优化建议
  - [问题排查](docs/zh-CN/advanced/troubleshooting.md) - 常见问题和解决方案
  - [FAQ](docs/zh-CN/advanced/faq.md) - 常见问题解答

## 🐛 Bug 反馈

遇到问题时，请按照以下步骤提供日志以便快速定位和解决问题：

1. **查找日志文件**：
   - 日志文件存储位置：
     - Windows: `%APPDATA%\.antigravity-agent\logs\`
     - macOS: `~/.config/.antigravity-agent/logs/`
     - Linux: `~/.config/.antigravity-agent/logs/`
   - 查找当前日期的日志文件（格式：`antigravity-agent.YYYY-MM-DD.log`，例如：`antigravity-agent.2025-11-24.log`）
   - 或者查找最新的日志文件进行复制

2. **⚠️ 重要：检查敏感信息**
   - **打开日志文件**，仔细检查是否包含个人敏感信息
   - **特别注意以下内容**：
     - 完整的邮箱地址（系统已自动脱敏，但仍需确认）
     - API 密钥、令牌或密码
     - 用户路径中的真实用户名
     - 其他任何您认为敏感的信息
   - **如果发现敏感信息**：
     - 使用文本编辑器手动替换或删除敏感内容
     - 例如：`user@domain.com` → `u***r@domain.com`
     - 例如：`/home/john/` → `~/`

## 🤝 参与贡献

我们欢迎所有形式的贡献！无论是报告 Bug、提出新功能建议，还是提交代码改进。

- 查看 [贡献指南](docs/zh-CN/development/contributing.md) 了解如何参与
- 阅读 [代码规范](docs/zh-CN/development/code-style.md) 了解代码风格要求
- 参考 [开发指南](docs/zh-CN/development/development-guide.md) 搭建开发环境

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议。

Copyright (c) 2024 Antigravity Agent

## 📝 TODO 列表

- [ ] 外部端口
- [ ] 国际化支持
- [ ] 主题支持
- [ ] 检查更新
- [ ] Linux 支持
- [ ] 扩展支持
- [ ] 规范开发流程
- [ ] CHANGELOG

## 🔗 相关链接

- [项目主页](https://github.com/MonchiLin/antigravity-agent)
- [问题反馈](https://github.com/MonchiLin/antigravity-agent/issues)
- [发布页面](https://github.com/MonchiLin/antigravity-agent/releases)
- [完整文档](docs/README.md)
