---
title: 开发环境搭建指南
description: Antigravity Agent 开发环境配置和开发流程说明
category: development
language: zh-CN
version: 1.0.3
lastUpdated: 2025-12-04
tags: [开发, 环境搭建, 构建, 调试]
---

# 开发环境搭建指南

本文档详细说明如何搭建 Antigravity Agent 的开发环境，包括依赖安装、项目初始化、开发服务器启动、构建打包以及调试方法。

## 目录

- [开发环境要求](#开发环境要求)
- [依赖安装](#依赖安装)
- [项目克隆和初始化](#项目克隆和初始化)
- [开发服务器](#开发服务器)
- [构建和打包](#构建和打包)
- [调试方法](#调试方法)
- [测试运行](#测试运行)
- [常见问题](#常见问题)

## 开发环境要求

### 操作系统

- **Windows**: Windows 10/11 (64-bit)
- **macOS**: macOS 10.15 (Catalina) 或更高版本
- **Linux**: Ubuntu 20.04+ / Debian 11+ / Fedora 35+ 或其他主流发行版

### 必需软件

#### 1. Node.js

**版本要求**: Node.js 18.0 或更高版本

**安装方法**:

**Windows / macOS**:
- 从 [Node.js 官网](https://nodejs.org/) 下载安装包
- 推荐使用 LTS 版本

**Linux**:
```bash
# Ubuntu/Debian
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Fedora
sudo dnf install nodejs

# 使用 nvm (推荐)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20
```

**验证安装**:
```bash
node --version  # 应显示 v18.0.0 或更高
npm --version   # 应显示 9.0.0 或更高
```

#### 2. Rust

**版本要求**: Rust 1.70 或更高版本

**安装方法**:

**Windows**:
1. 下载并安装 [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - 选择 "Desktop development with C++" 工作负载
2. 从 [rustup.rs](https://rustup.rs/) 下载并运行 `rustup-init.exe`

**macOS / Linux**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**验证安装**:
```bash
rustc --version  # 应显示 rustc 1.70.0 或更高
cargo --version  # 应显示 cargo 1.70.0 或更高
```

#### 3. 系统依赖

**Windows**:
- Visual Studio C++ Build Tools (已在 Rust 安装时安装)
- WebView2 Runtime (Windows 10/11 通常已预装)

**macOS**:
```bash
# 安装 Xcode Command Line Tools
xcode-select --install
```

**Linux (Ubuntu/Debian)**:
```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Linux (Fedora)**:
```bash
sudo dnf install \
    webkit2gtk4.1-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

### 推荐工具

#### 代码编辑器

**Visual Studio Code** (推荐)
- 下载: [https://code.visualstudio.com/](https://code.visualstudio.com/)
- 推荐扩展:
  - `rust-analyzer`: Rust 语言支持
  - `Tauri`: Tauri 开发支持
  - `ESLint`: JavaScript/TypeScript 代码检查
  - `Prettier`: 代码格式化
  - `Tailwind CSS IntelliSense`: Tailwind CSS 智能提示

#### 其他工具

- **Git**: 版本控制 ([下载](https://git-scm.com/))
- **Postman** 或 **Insomnia**: API 测试（可选）


## 依赖安装

### 1. 克隆项目

```bash
# 使用 HTTPS
git clone https://github.com/MonchiLin/antigravity-agent.git

# 或使用 SSH
git clone git@github.com:MonchiLin/antigravity-agent.git

# 进入项目目录
cd antigravity-agent
```

### 2. 安装前端依赖

```bash
# 使用 npm
npm install

# 或使用 yarn
yarn install

# 或使用 pnpm
pnpm install
```

**依赖说明**:

主要依赖包括:
- `react` & `react-dom`: UI 框架
- `@tauri-apps/api`: Tauri 前端 API
- `zustand`: 状态管理
- `@radix-ui/*`: UI 组件库
- `tailwindcss`: CSS 框架
- `lucide-react`: 图标库

开发依赖包括:
- `typescript`: TypeScript 编译器
- `vite`: 构建工具
- `@tauri-apps/cli`: Tauri CLI 工具

### 3. 安装 Rust 依赖

Rust 依赖会在首次构建时自动下载，无需手动安装。

**主要依赖**:
- `tauri`: Tauri 框架
- `tokio`: 异步运行时
- `rusqlite`: SQLite 数据库
- `serde`: 序列化框架
- `tracing`: 日志系统

如需提前下载依赖:
```bash
cd src-tauri
cargo fetch
cd ..
```

## 项目克隆和初始化

### 项目结构

```
antigravity-agent/
├── src/                    # 前端源代码
│   ├── components/        # React 组件
│   ├── hooks/            # 自定义 Hooks
│   ├── services/         # 服务层
│   ├── modules/          # 业务模块
│   ├── utils/            # 工具函数
│   ├── App.tsx           # 应用主组件
│   └── main.tsx          # 应用入口
├── src-tauri/             # 后端源代码
│   ├── src/              # Rust 源代码
│   │   ├── commands/    # Tauri 命令
│   │   ├── antigravity/ # 业务逻辑
│   │   ├── platform/    # 平台适配
│   │   └── main.rs      # 应用入口
│   ├── Cargo.toml       # Rust 依赖配置
│   └── tauri.conf.json  # Tauri 配置
├── docs/                  # 文档
├── package.json          # Node.js 依赖配置
├── vite.config.js        # Vite 配置
├── tailwind.config.js    # Tailwind 配置
└── tsconfig.json         # TypeScript 配置
```

### 配置文件说明

#### package.json

定义前端依赖和脚本命令:

```json
{
  "scripts": {
    "dev": "vite",                    // 启动前端开发服务器
    "build": "vite build",            // 构建前端
    "tauri:dev": "tauri dev",         // 启动 Tauri 开发模式
    "tauri:build": "tauri build",     // 构建 Tauri 应用
    "type-check": "tsc --noEmit"      // TypeScript 类型检查
  }
}
```

#### src-tauri/tauri.conf.json

Tauri 应用配置:

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",      // 开发前执行的命令
    "beforeBuildCommand": "npm run build",  // 构建前执行的命令
    "devUrl": "http://localhost:1420",      // 开发服务器地址
    "frontendDist": "../dist"               // 前端构建输出目录
  }
}
```

#### src-tauri/Cargo.toml

Rust 依赖配置，定义后端依赖包和版本。

### 初始化检查

运行以下命令确保环境配置正确:

```bash
# 检查 Node.js
node --version

# 检查 npm
npm --version

# 检查 Rust
rustc --version
cargo --version

# 检查 Tauri CLI
npm run tauri -- --version
```


## 开发服务器

### 启动开发模式

#### 方式 1: 使用 Tauri 开发模式（推荐）

```bash
npm run tauri:dev
```

这个命令会:
1. 启动 Vite 前端开发服务器 (http://localhost:1420)
2. 编译 Rust 后端代码
3. 启动 Tauri 应用窗口
4. 启用热重载（前端代码修改后自动刷新）

**首次运行**: 首次运行会下载和编译所有 Rust 依赖，可能需要 5-10 分钟。

#### 方式 2: 分别启动前后端

**终端 1 - 启动前端**:
```bash
npm run dev
```

**终端 2 - 启动 Tauri**:
```bash
npm run tauri dev
```

### 开发服务器特性

#### 热重载 (Hot Reload)

- **前端**: 修改 TypeScript/React 代码后自动刷新
- **后端**: 修改 Rust 代码后需要重新编译（自动触发）

#### 开发者工具

在开发模式下，可以使用以下快捷键:

- **Windows/Linux**: `Ctrl + Shift + I`
- **macOS**: `Cmd + Option + I`

或在代码中添加:

```typescript
// 在 App.tsx 或其他组件中
import { useDevToolsShortcut } from './hooks/useDevToolsShortcut';

function App() {
  useDevToolsShortcut(); // 启用 F12 打开开发者工具
  // ...
}
```

#### 日志查看

**前端日志**:
- 在浏览器开发者工具的 Console 面板查看
- 使用 `console.log()` 输出日志

**后端日志**:
- 在终端查看 Rust 日志输出
- 使用 `tracing::info!()` 等宏输出日志

```rust
tracing::info!("这是一条信息日志");
tracing::error!("这是一条错误日志");
```

### 开发模式配置

#### 修改开发服务器端口

编辑 `vite.config.js`:

```javascript
export default {
  server: {
    port: 1420,  // 修改为其他端口
    strictPort: true,
  }
}
```

同时修改 `src-tauri/tauri.conf.json`:

```json
{
  "build": {
    "devUrl": "http://localhost:1420"  // 与 Vite 端口保持一致
  }
}
```

#### 启用详细日志

设置环境变量:

```bash
# Windows (PowerShell)
$env:RUST_LOG="debug"
npm run tauri:dev

# macOS/Linux
RUST_LOG=debug npm run tauri:dev
```

## 构建和打包

### 开发构建

用于测试构建流程，不进行优化:

```bash
# 构建前端
npm run build

# 构建 Tauri 应用（开发模式）
npm run tauri build -- --debug
```

### 生产构建

生成优化的可分发应用:

```bash
npm run tauri:build
```

这个命令会:
1. 构建优化的前端代码
2. 编译优化的 Rust 代码
3. 生成平台特定的安装包

### 构建输出

构建完成后，安装包位于:

```
src-tauri/target/release/bundle/
├── nsis/              # Windows 安装程序 (.exe)
├── dmg/               # macOS 磁盘映像 (.dmg)
├── appimage/          # Linux AppImage
└── deb/               # Debian 包 (.deb)
```

### 构建选项

#### 指定目标平台

```bash
# 仅构建 NSIS 安装程序 (Windows)
npm run tauri build -- --bundles nsis

# 仅构建 DMG (macOS)
npm run tauri build -- --bundles dmg

# 构建多个格式
npm run tauri build -- --bundles nsis,msi
```

#### 调试构建

```bash
# 构建未优化的调试版本（更快，但体积更大）
npm run tauri build -- --debug
```

#### 清理构建缓存

```bash
# 清理前端构建
rm -rf dist

# 清理 Rust 构建
cd src-tauri
cargo clean
cd ..
```

### 构建优化

#### 减小包体积

1. **启用 LTO (Link Time Optimization)**

编辑 `src-tauri/Cargo.toml`:

```toml
[profile.release]
lto = true
opt-level = "z"  # 优化体积
strip = true     # 移除调试符号
```

2. **压缩前端资源**

Vite 默认会压缩和优化前端资源。

#### 加快构建速度

1. **使用增量编译**

```bash
# 设置环境变量
export CARGO_INCREMENTAL=1
```

2. **使用 sccache**

```bash
# 安装 sccache
cargo install sccache

# 配置 Cargo 使用 sccache
export RUSTC_WRAPPER=sccache
```


## 调试方法

### 前端调试

#### 1. 使用浏览器开发者工具

在开发模式下打开开发者工具:

- **Windows/Linux**: `Ctrl + Shift + I` 或 `F12`
- **macOS**: `Cmd + Option + I`

**主要功能**:
- **Console**: 查看日志和错误
- **Sources**: 设置断点调试 TypeScript 代码
- **Network**: 查看 IPC 调用
- **React DevTools**: 检查 React 组件树和状态

#### 2. 使用 console.log

```typescript
// 基本日志
console.log('用户信息:', user);

// 分组日志
console.group('账户切换');
console.log('旧账户:', oldAccount);
console.log('新账户:', newAccount);
console.groupEnd();

// 表格显示
console.table(accounts);
```

#### 3. 使用自定义 logger

项目提供了统一的日志工具:

```typescript
import { logger } from '@/utils/logger';

logger.info('操作成功', {
  module: 'Account',
  action: 'switch',
  accountId: '123'
});

logger.error('操作失败', {
  module: 'Account',
  error: error.message
});
```

#### 4. React DevTools

安装 React DevTools 浏览器扩展:
- [Chrome](https://chrome.google.com/webstore/detail/react-developer-tools/fmkadmapgofadopljbjfkapdkoienihi)
- [Firefox](https://addons.mozilla.org/en-US/firefox/addon/react-devtools/)

**功能**:
- 检查组件树
- 查看组件 props 和 state
- 分析组件性能
- 追踪 Hooks 状态

### 后端调试

#### 1. 使用 tracing 日志

```rust
use tracing::{info, debug, error, warn};

#[tauri::command]
async fn switch_account(account_id: String) -> Result<()> {
    info!(account_id = %account_id, "开始切换账户");
    
    debug!("检查账户是否存在");
    // ...
    
    if let Err(e) = result {
        error!(error = %e, "切换账户失败");
        return Err(e);
    }
    
    info!("账户切换成功");
    Ok(())
}
```

#### 2. 设置日志级别

```bash
# 显示所有日志
RUST_LOG=trace npm run tauri:dev

# 只显示特定模块的日志
RUST_LOG=antigravity_agent::commands=debug npm run tauri:dev

# 多个模块
RUST_LOG=antigravity_agent::commands=debug,antigravity_agent::antigravity=info npm run tauri:dev
```

#### 3. 使用 Rust 调试器

**VS Code 配置**:

创建 `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Tauri Development Debug",
      "cargo": {
        "args": [
          "build",
          "--manifest-path=./src-tauri/Cargo.toml",
          "--no-default-features"
        ]
      },
      "preLaunchTask": "ui:dev"
    }
  ]
}
```

需要安装 VS Code 扩展:
- `CodeLLDB` (Rust 调试器)
- `rust-analyzer` (Rust 语言支持)

#### 4. 打印调试

```rust
// 简单打印
println!("账户 ID: {}", account_id);

// 格式化打印
dbg!(&account);  // 打印变量名和值

// 条件打印
#[cfg(debug_assertions)]
println!("调试信息: {:?}", data);
```

### IPC 调试

#### 查看 IPC 调用

在前端添加日志:

```typescript
import { invoke } from '@tauri-apps/api/core';

async function switchAccount(accountId: string) {
  console.log('IPC 调用: switch_antigravity_account', { accountId });
  
  try {
    const result = await invoke('switch_antigravity_account', { accountId });
    console.log('IPC 响应:', result);
    return result;
  } catch (error) {
    console.error('IPC 错误:', error);
    throw error;
  }
}
```

#### 监控 IPC 性能

```typescript
async function invokeWithTiming(command: string, args: any) {
  const start = performance.now();
  
  try {
    const result = await invoke(command, args);
    const duration = performance.now() - start;
    console.log(`IPC ${command} 耗时: ${duration.toFixed(2)}ms`);
    return result;
  } catch (error) {
    const duration = performance.now() - start;
    console.error(`IPC ${command} 失败 (${duration.toFixed(2)}ms):`, error);
    throw error;
  }
}
```

### 数据库调试

#### 查看 SQLite 数据库

使用 SQLite 客户端工具:

**推荐工具**:
- [DB Browser for SQLite](https://sqlitebrowser.org/)
- [DBeaver](https://dbeaver.io/)
- [SQLite Studio](https://sqlitestudio.pl/)

**数据库位置**:
- Windows: `%APPDATA%\.antigravity-agent\antigravity.db`
- macOS/Linux: `~/.config/.antigravity-agent/antigravity.db`

#### 执行 SQL 查询

```rust
// 在 Rust 代码中添加调试查询
#[cfg(debug_assertions)]
{
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM accounts")?;
    let accounts: Vec<_> = stmt.query_map([], |row| {
        Ok(format!("{}: {}", row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?.collect();
    dbg!(accounts);
}
```

### 性能分析

#### 前端性能分析

使用 React DevTools Profiler:

1. 打开 React DevTools
2. 切换到 Profiler 标签
3. 点击录制按钮
4. 执行要分析的操作
5. 停止录制并查看结果

#### 后端性能分析

使用 `tracing` 的时间跟踪:

```rust
use tracing::instrument;

#[instrument]
async fn expensive_operation() {
    // 自动记录函数执行时间
}
```

或手动计时:

```rust
use std::time::Instant;

let start = Instant::now();
// 执行操作
let duration = start.elapsed();
tracing::info!("操作耗时: {:?}", duration);
```


## 测试运行

### 前端测试

目前项目主要依赖手动测试，未来可以添加自动化测试。

#### 添加测试框架（可选）

**安装 Vitest**:

```bash
npm install -D vitest @testing-library/react @testing-library/jest-dom
```

**配置 vite.config.js**:

```javascript
import { defineConfig } from 'vite';

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts',
  },
});
```

**编写测试**:

```typescript
// src/components/UserListItem.test.tsx
import { render, screen } from '@testing-library/react';
import { UserListItem } from './UserListItem';

describe('UserListItem', () => {
  it('should render user name', () => {
    const user = { id: '1', name: 'Test User' };
    render(<UserListItem user={user} />);
    expect(screen.getByText('Test User')).toBeInTheDocument();
  });
});
```

**运行测试**:

```bash
npm run test
```

### 后端测试

#### 单元测试

Rust 内置测试支持:

```rust
// src-tauri/src/antigravity/backup.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_creation() {
        let result = create_backup("test_account");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_backup() {
        let result = async_backup("test_account").await;
        assert!(result.is_ok());
    }
}
```

**运行测试**:

```bash
cd src-tauri
cargo test
```

**运行特定测试**:

```bash
# 运行特定模块的测试
cargo test antigravity::backup

# 运行特定测试函数
cargo test test_backup_creation

# 显示测试输出
cargo test -- --nocapture
```

#### 集成测试

创建 `src-tauri/tests/integration_test.rs`:

```rust
use antigravity_agent::*;

#[tokio::test]
async fn test_full_backup_restore_flow() {
    // 创建测试账户
    let account = create_test_account().await;
    
    // 备份
    let backup_path = backup_account(&account).await.unwrap();
    
    // 恢复
    let restored = restore_account(&backup_path).await.unwrap();
    
    // 验证
    assert_eq!(account.id, restored.id);
}
```

**运行集成测试**:

```bash
cd src-tauri
cargo test --test integration_test
```

### 端到端测试

#### 使用 Tauri 测试工具

安装 WebDriver:

```bash
npm install -D @tauri-apps/cli
```

**编写 E2E 测试**:

```typescript
// tests/e2e/account-switch.spec.ts
import { test, expect } from '@playwright/test';

test('should switch account', async ({ page }) => {
  await page.goto('http://localhost:1420');
  
  // 点击账户
  await page.click('[data-testid="account-item"]');
  
  // 验证切换成功
  await expect(page.locator('[data-testid="current-account"]'))
    .toHaveText('Test Account');
});
```

### 测试覆盖率

#### Rust 代码覆盖率

使用 `tarpaulin`:

```bash
# 安装
cargo install cargo-tarpaulin

# 运行
cd src-tauri
cargo tarpaulin --out Html
```

#### TypeScript 代码覆盖率

使用 Vitest:

```bash
npm run test -- --coverage
```

## 常见问题

### 构建问题

#### 问题: Rust 编译失败

**错误信息**: `error: linker 'cc' not found`

**解决方案**:
- **Windows**: 安装 Visual Studio C++ Build Tools
- **macOS**: 运行 `xcode-select --install`
- **Linux**: 安装 `build-essential` 或 `gcc`

#### 问题: WebView2 未找到 (Windows)

**错误信息**: `WebView2 runtime not found`

**解决方案**:
下载并安装 [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

#### 问题: 依赖下载失败

**错误信息**: `failed to download dependencies`

**解决方案**:
```bash
# 清理缓存
cargo clean
rm -rf ~/.cargo/registry

# 使用国内镜像（可选）
# 编辑 ~/.cargo/config.toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

### 运行时问题

#### 问题: 端口已被占用

**错误信息**: `Port 1420 is already in use`

**解决方案**:
```bash
# 查找占用端口的进程
# Windows
netstat -ano | findstr :1420

# macOS/Linux
lsof -i :1420

# 终止进程或修改 vite.config.js 中的端口
```

#### 问题: 热重载不工作

**解决方案**:
1. 检查文件监听限制（Linux）:
```bash
# 增加文件监听限制
echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

2. 重启开发服务器
3. 清理缓存: `rm -rf node_modules/.vite`

#### 问题: Antigravity 进程无法启动

**解决方案**:
1. 检查 Antigravity 是否已安装
2. 验证可执行文件路径
3. 查看日志文件: `~/.config/.antigravity-agent/logs/`

### 开发工具问题

#### 问题: VS Code 无法识别 Rust 代码

**解决方案**:
1. 安装 `rust-analyzer` 扩展
2. 重新加载窗口: `Ctrl+Shift+P` -> `Reload Window`
3. 检查 Rust 工具链: `rustup show`

#### 问题: TypeScript 类型错误

**解决方案**:
```bash
# 重新生成类型定义
npm run type-check

# 清理并重新安装依赖
rm -rf node_modules package-lock.json
npm install
```

## 开发工作流建议

### 日常开发流程

1. **拉取最新代码**:
```bash
git pull origin main
```

2. **创建功能分支**:
```bash
git checkout -b feature/your-feature-name
```

3. **启动开发服务器**:
```bash
npm run tauri:dev
```

4. **编写代码并测试**

5. **提交代码**:
```bash
git add .
git commit -m "feat: add new feature"
git push origin feature/your-feature-name
```

6. **创建 Pull Request**

### 代码质量检查

运行以下命令确保代码质量:

```bash
# TypeScript 类型检查
npm run type-check

# Rust 代码检查
cd src-tauri
cargo clippy

# Rust 代码格式化
cargo fmt

# 前端代码格式化（如果配置了 Prettier）
npm run format
```

## 相关文档

### 开发文档
- [系统架构](./architecture.md) - 系统整体架构设计
- [贡献指南](./contributing.md) - 如何参与项目贡献
- [代码规范](./code-style.md) - 代码风格和最佳实践

### 入门文档
- [项目概览](../getting-started/README.md) - 了解项目的基本信息
- [安装指南](../getting-started/installation.md) - 详细的安装步骤和系统要求
- [快速开始](../getting-started/quickstart.md) - 5 分钟快速上手教程

### 使用文档
- [使用手册](../user-guide/user-guide.md) - 完整的功能说明和操作指南
- [API 参考](../user-guide/api-reference.md) - 所有命令和接口说明

### 进阶文档
- [设计原理](../advanced/design-principles.md) - 核心设计思路和技术选型
- [性能优化](../advanced/performance.md) - 性能分析和优化建议
- [问题排查](../advanced/troubleshooting.md) - 常见问题诊断和解决

### 返回
- [文档首页](../../README.md) - 返回文档导航页

## 参考资源

- [Tauri 开发指南](https://tauri.app/v1/guides/)
- [Vite 文档](https://vitejs.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [React 文档](https://react.dev/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

---

**最后更新**: 2025-12-04  
**文档版本**: 1.0.3
