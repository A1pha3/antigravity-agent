# 术语表 (Glossary)

本文档提供 Antigravity Agent 项目中使用的专业术语的中英文对照和详细解释。

## 目录

- [通用术语](#通用术语)
- [技术术语](#技术术语)
- [功能术语](#功能术语)
- [开发术语](#开发术语)

## 通用术语

### A

**Account (账户)**
- 用户在 Antigravity 系统中的身份标识
- 包含用户名、邮箱、API 密钥等信息

**Agent (代理)**
- 指 Antigravity Agent 应用程序本身
- 作为 Antigravity 的管理工具运行

**API (应用程序编程接口)**
- Application Programming Interface
- 程序之间通信的接口规范

**API Key (API 密钥)**
- 用于身份验证的密钥字符串
- 用于访问 Antigravity 服务

### B

**Backup (备份)**
- 将账户配置和数据保存到文件
- 用于数据恢复和迁移

**Bundle (打包)**
- 将应用程序打包成可分发的安装包
- 包括所有必要的依赖和资源

### C

**Cache (缓存)**
- 临时存储的数据，用于提高性能
- 减少重复计算和数据库查询

**CLI (命令行界面)**
- Command Line Interface
- 通过文本命令操作程序的界面

**Configuration (配置)**
- 应用程序的设置和参数
- 存储在配置文件中

**Cross-platform (跨平台)**
- 能在多个操作系统上运行
- 如 Windows、macOS、Linux

### D

**Database (数据库)**
- 存储应用程序数据的系统
- 本项目使用 SQLite

**Debug (调试)**
- 查找和修复程序错误的过程
- 使用调试工具分析问题

**Dependency (依赖)**
- 项目所需的外部库或包
- 通过包管理器安装

**Desktop Application (桌面应用)**
- 运行在桌面操作系统上的应用程序
- 区别于 Web 应用或移动应用

### E

**Encryption (加密)**
- 将数据转换为密文以保护安全
- 本项目使用 AES-256 加密备份文件

**Environment (环境)**
- 程序运行的系统配置
- 包括开发环境、测试环境、生产环境

**Export (导出)**
- 将数据保存到外部文件
- 用于备份或迁移

### F

**Frontend (前端)**
- 用户界面层
- 本项目使用 React + TypeScript

**Framework (框架)**
- 提供基础功能的软件平台
- 如 Tauri、React

### G

**GUI (图形用户界面)**
- Graphical User Interface
- 通过图形元素操作程序的界面

### H

**Hot Reload (热重载)**
- 代码修改后自动刷新应用
- 无需重启整个程序

**Hook (钩子)**
- React 中的特殊函数
- 用于在函数组件中使用状态和副作用

### I

**Import (导入)**
- 从外部文件加载数据
- 用于恢复备份或迁移数据

**IPC (进程间通信)**
- Inter-Process Communication
- Tauri 中前端和后端的通信机制

**Installation (安装)**
- 将应用程序部署到系统的过程
- 包括复制文件、配置系统等

### L

**Library (库)**
- 可重用的代码集合
- 提供特定功能

**Logging (日志记录)**
- 记录程序运行信息
- 用于调试和问题排查

### M

**Migration (迁移)**
- 将数据或应用从一个环境转移到另一个
- 包括数据库迁移、账户迁移等

**Module (模块)**
- 独立的代码单元
- 实现特定功能

### P

**Package (包)**
- 打包的软件组件
- 可以通过包管理器安装

**Performance (性能)**
- 程序运行的效率
- 包括速度、内存使用、CPU 占用等

**Plugin (插件)**
- 扩展应用功能的组件
- 可以动态加载和卸载

**Process (进程)**
- 运行中的程序实例
- 操作系统分配资源的基本单位

### R

**Release (发布)**
- 软件的正式版本
- 经过测试和优化

**Repository (仓库)**
- 存储代码的地方
- 通常指 Git 仓库

**Restore (恢复)**
- 从备份文件恢复数据
- 与备份操作相对应

**Runtime (运行时)**
- 程序执行时的环境
- 包括运行时库和系统资源

### S

**SQLite**
- 轻量级的嵌入式数据库
- 本项目用于存储账户和配置数据

**State (状态)**
- 程序在某一时刻的数据
- React 中用于管理组件数据

**System Tray (系统托盘)**
- 操作系统任务栏的通知区域
- 应用可以在此显示图标和菜单

### T

**Tauri**
- 使用 Web 技术构建桌面应用的框架
- 使用 Rust 作为后端

**TypeScript**
- JavaScript 的超集
- 添加了静态类型检查

### U

**UI (用户界面)**
- User Interface
- 用户与程序交互的界面

**Update (更新)**
- 升级到新版本
- 包括功能改进和错误修复

### V

**Version (版本)**
- 软件的特定发布
- 使用语义化版本号（如 1.0.3）

### W

**WebView**
- 嵌入式浏览器组件
- 用于渲染 Web 内容

**Workspace (工作空间)**
- 项目的根目录
- 包含所有源代码和配置文件

## 技术术语

### 前端技术

**React**
- 用于构建用户界面的 JavaScript 库
- 采用组件化开发模式

**Component (组件)**
- React 中的 UI 构建块
- 可以是函数组件或类组件

**Props (属性)**
- 传递给组件的参数
- 用于组件间通信

**State Management (状态管理)**
- 管理应用程序状态的方法
- 本项目使用 Zustand

**Zustand**
- 轻量级的 React 状态管理库
- 基于 hooks 的 API

**Radix UI**
- 无样式的可访问 UI 组件库
- 提供行为和可访问性

**Tailwind CSS**
- 实用优先的 CSS 框架
- 通过类名快速构建 UI

**Vite**
- 现代化的前端构建工具
- 提供快速的开发服务器和构建

### 后端技术

**Rust**
- 系统编程语言
- 提供内存安全和高性能

**Cargo**
- Rust 的包管理器和构建工具
- 管理依赖和编译项目

**Crate**
- Rust 中的包或库
- 通过 Cargo 管理

**Tokio**
- Rust 的异步运行时
- 用于处理并发任务

**Serde**
- Rust 的序列化/反序列化框架
- 用于数据格式转换

**Tracing**
- Rust 的日志和追踪库
- 提供结构化日志

### 数据库术语

**Query (查询)**
- 从数据库检索数据的操作
- 使用 SQL 语言

**Transaction (事务)**
- 一组数据库操作
- 要么全部成功，要么全部失败

**Index (索引)**
- 加速数据库查询的数据结构
- 类似于书籍的索引

**Schema (模式)**
- 数据库的结构定义
- 包括表、列、关系等

**WAL (Write-Ahead Logging)**
- SQLite 的日志模式
- 提高并发性能

## 功能术语

### 账户管理

**Account Switching (账户切换)**
- 在不同 Antigravity 账户间切换
- 自动保存和加载账户数据

**Multi-Account (多账户)**
- 支持管理多个账户
- 每个账户独立存储

**Current Account (当前账户)**
- 正在使用的账户
- 在界面上高亮显示

### 备份功能

**Encrypted Backup (加密备份)**
- 使用密码加密的备份文件
- 保护敏感数据

**Backup File (备份文件)**
- 包含账户配置的压缩文件
- 通常以 `.zip` 或 `.agb` 扩展名

**Incremental Backup (增量备份)**
- 只备份变化的数据
- 节省空间和时间（计划功能）

### 进程管理

**Process Monitor (进程监控)**
- 监控 Antigravity 进程状态
- 检测进程是否运行

**Auto-restart (自动重启)**
- 进程崩溃时自动重启
- 提高可用性

**Process ID (进程 ID)**
- 操作系统分配的进程标识符
- 简称 PID

## 开发术语

### 版本控制

**Git**
- 分布式版本控制系统
- 跟踪代码变更

**Commit (提交)**
- 保存代码变更到仓库
- 包含变更说明

**Branch (分支)**
- 代码的独立开发线
- 用于并行开发功能

**Pull Request (拉取请求)**
- 请求合并代码变更
- 简称 PR

**Merge (合并)**
- 将一个分支的变更合并到另一个
- 整合不同的开发工作

### 测试

**Unit Test (单元测试)**
- 测试单个函数或组件
- 验证最小可测试单元

**Integration Test (集成测试)**
- 测试多个组件的协作
- 验证系统集成

**End-to-End Test (端到端测试)**
- 测试完整的用户流程
- 简称 E2E 测试

**Test Coverage (测试覆盖率)**
- 代码被测试覆盖的百分比
- 衡量测试完整性

### CI/CD

**Continuous Integration (持续集成)**
- 自动构建和测试代码
- 简称 CI

**Continuous Deployment (持续部署)**
- 自动部署到生产环境
- 简称 CD

**GitHub Actions**
- GitHub 的 CI/CD 平台
- 自动化工作流程

### 代码质量

**Linter (代码检查器)**
- 检查代码风格和潜在错误
- 如 ESLint、Clippy

**Formatter (代码格式化器)**
- 自动格式化代码
- 如 Prettier、rustfmt

**Code Review (代码审查)**
- 审查他人的代码变更
- 确保代码质量

**Refactoring (重构)**
- 改进代码结构而不改变功能
- 提高可维护性

## 缩写词

| 缩写 | 全称 | 中文 |
|------|------|------|
| API | Application Programming Interface | 应用程序编程接口 |
| CLI | Command Line Interface | 命令行界面 |
| CPU | Central Processing Unit | 中央处理器 |
| CSS | Cascading Style Sheets | 层叠样式表 |
| DB | Database | 数据库 |
| DMG | Disk Image | 磁盘映像（macOS） |
| E2E | End-to-End | 端到端 |
| FAQ | Frequently Asked Questions | 常见问题 |
| GUI | Graphical User Interface | 图形用户界面 |
| HTML | HyperText Markup Language | 超文本标记语言 |
| HTTP | HyperText Transfer Protocol | 超文本传输协议 |
| HTTPS | HTTP Secure | 安全的 HTTP |
| IDE | Integrated Development Environment | 集成开发环境 |
| IPC | Inter-Process Communication | 进程间通信 |
| JSON | JavaScript Object Notation | JavaScript 对象表示法 |
| LTS | Long Term Support | 长期支持 |
| macOS | Mac Operating System | Mac 操作系统 |
| MB | Megabyte | 兆字节 |
| OS | Operating System | 操作系统 |
| PID | Process ID | 进程标识符 |
| PR | Pull Request | 拉取请求 |
| RAM | Random Access Memory | 随机存取存储器 |
| SQL | Structured Query Language | 结构化查询语言 |
| SSD | Solid State Drive | 固态硬盘 |
| TOC | Table of Contents | 目录 |
| UI | User Interface | 用户界面 |
| URL | Uniform Resource Locator | 统一资源定位符 |
| UTF-8 | 8-bit Unicode Transformation Format | 8 位 Unicode 转换格式 |
| UUID | Universally Unique Identifier | 通用唯一识别码 |
| WAL | Write-Ahead Logging | 预写式日志 |
| XML | eXtensible Markup Language | 可扩展标记语言 |

## 相关文档

- [用户指南](user-guide/user-guide.md) - 了解功能术语的使用
- [开发指南](development/development-guide.md) - 了解开发术语
- [系统架构](development/architecture.md) - 了解技术架构术语
- [API 参考](user-guide/api-reference.md) - 了解 API 相关术语

---

**最后更新**: 2025-12-04  
**文档版本**: 1.0.3
