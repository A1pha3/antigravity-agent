---
title: 常见问题 (FAQ)
description: Antigravity Agent 使用、开发和部署相关的常见问题解答
category: advanced
language: zh-CN
version: 1.0.3
lastUpdated: 2025-12-04
tags: [FAQ, 常见问题, 问答]
---

# 常见问题 (FAQ)

## 概述

本文档收集了 Antigravity Agent 使用、开发和部署过程中的常见问题及解答。如果您的问题在这里找不到答案，请参考[问题排查手册](./troubleshooting.md)或在 GitHub 上提交 Issue。

## 目录

- [使用相关问题](#使用相关问题)
- [开发相关问题](#开发相关问题)
- [部署相关问题](#部署相关问题)
- [已知限制和计划功能](#已知限制和计划功能)

## 使用相关问题

### 基本使用

#### Q1: Antigravity Agent 是什么？

**A:** Antigravity Agent 是一个开箱即用的 Antigravity 账户管理程序，使用 Tauri + React + TypeScript 构建。它提供以下核心功能：

- 多账户管理和快速切换
- 配置备份和恢复
- 加密的导入导出功能
- Antigravity 进程管理
- 数据库实时监控
- 系统托盘集成

#### Q2: Antigravity Agent 支持哪些操作系统？

**A:** 当前支持：
- ✅ Windows 10/11
- ✅ macOS 10.15 (Catalina) 及更高版本
- 🚧 Linux（计划支持中）

#### Q3: Antigravity Agent 是免费的吗？

**A:** 是的，Antigravity Agent 是完全免费的开源软件，采用 MIT 许可证。您可以自由使用、修改和分发。

#### Q4: 如何更新 Antigravity Agent？

**A:** 应用程序内置了自动更新功能：

1. 应用程序会自动检查更新
2. 发现新版本时会显示通知
3. 点击通知即可下载并安装更新
4. 或者手动检查：设置 → 关于 → 检查更新

您也可以从 [GitHub Releases](https://github.com/MonchiLin/antigravity-agent/releases) 手动下载最新版本。

#### Q5: 数据存储在哪里？

**A:** 应用程序数据存储在以下位置：

**Windows:**
```
%APPDATA%\.antigravity-agent\
├── data.db          # 数据库
├── config.json      # 配置文件
└── logs\            # 日志文件
```

**macOS:**
```
~/.config/.antigravity-agent/
├── data.db
├── config.json
└── logs/
```

### 账户管理

#### Q6: 如何添加新账户？

**A:** 有两种方式：

**方法 1: 登录新账户**
1. 点击"登录新账户"按钮
2. 在打开的 Antigravity 窗口中登录
3. 登录成功后，账户会自动添加到列表

**方法 2: 导入备份**
1. 点击"导入"按钮
2. 选择之前导出的备份文件
3. 输入加密密码
4. 账户会被导入到列表

#### Q7: 可以管理多少个账户？

**A:** 理论上没有限制，但建议不超过 20 个账户以保持最佳性能。如果需要管理更多账户，建议定期清理不再使用的账户。

#### Q8: 切换账户需要多长时间？

**A:** 通常在 1-3 秒内完成，具体取决于：
- 账户配置的大小
- 系统性能
- 磁盘速度

#### Q9: 切换账户时会丢失数据吗？

**A:** 不会。切换账户时：
1. 当前账户的所有数据会被保存
2. 目标账户的数据会被加载
3. 所有数据都安全存储在数据库中

#### Q10: 如何删除账户？

**A:**
1. 在账户列表中找到要删除的账户
2. 点击账户右侧的删除按钮（垃圾桶图标）
3. 确认删除操作

**注意：** 删除账户会永久删除该账户的所有配置数据，建议先导出备份。

### 备份和恢复

#### Q11: 如何备份账户配置？

**A:**
1. 选择要备份的账户
2. 点击"导出"按钮
3. 设置加密密码（强烈建议）
4. 选择保存位置
5. 备份文件会以 `.zip` 格式保存

#### Q12: 备份文件包含什么内容？

**A:** 备份文件包含：
- 账户配置信息
- 用户偏好设置
- 数据库数据
- 相关的配置文件

**不包含：**
- 应用程序本身
- 日志文件
- 临时文件

#### Q13: 备份文件是加密的吗？

**A:** 是的，如果您在导出时设置了密码，备份文件会使用 AES-256 加密。没有密码无法解密和恢复备份。

**重要：** 请妥善保管密码，忘记密码将无法恢复备份！

#### Q14: 可以在不同电脑之间迁移账户吗？

**A:** 可以！这正是备份功能的主要用途：

1. 在旧电脑上导出账户备份
2. 将备份文件复制到新电脑
3. 在新电脑上安装 Antigravity Agent
4. 导入备份文件

#### Q15: 备份文件可以在不同操作系统之间使用吗？

**A:** 可以。备份文件是跨平台的，可以在 Windows、macOS 和 Linux（未来）之间互相导入。

### 系统托盘

#### Q16: 如何启用系统托盘？

**A:**
1. 打开设置（点击右上角齿轮图标）
2. 找到"系统托盘"选项
3. 开启"启用系统托盘"开关
4. 应用程序会最小化到系统托盘

#### Q17: 关闭窗口后应用程序还在运行吗？

**A:** 如果启用了系统托盘，关闭窗口只会隐藏界面，应用程序仍在后台运行。要完全退出，需要：
- 右键点击托盘图标
- 选择"退出"

#### Q18: 如何设置开机自启动？

**A:**
1. 打开设置
2. 找到"启动选项"
3. 开启"开机自启动"开关
4. 可选：开启"静默启动"（启动时不显示窗口）


## 开发相关问题

### 环境搭建

#### Q19: 开发 Antigravity Agent 需要什么环境？

**A:** 需要以下工具：

**必需：**
- Node.js 18+ 和 npm/pnpm
- Rust 1.70+
- 系统特定的依赖：
  - Windows: Visual Studio Build Tools, WebView2
  - macOS: Xcode Command Line Tools
  - Linux: webkit2gtk, gtk3

**推荐：**
- VS Code 或其他代码编辑器
- Git

详细说明请参考[开发指南](../development/development-guide.md)。

#### Q20: 如何运行开发服务器？

**A:**
```bash
# 克隆仓库
git clone https://github.com/MonchiLin/antigravity-agent.git
cd antigravity-agent

# 安装依赖
npm install

# 运行开发服务器
npm run tauri:dev
```

开发服务器会自动启动前端和后端，支持热重载。

#### Q21: 如何构建生产版本？

**A:**
```bash
# 构建所有平台
npm run tauri:build

# 构建产物位置
# Windows: src-tauri/target/release/bundle/nsis/
# macOS: src-tauri/target/release/bundle/dmg/
# Linux: src-tauri/target/release/bundle/appimage/
```

#### Q22: 为什么构建很慢？

**A:** 首次构建 Rust 项目会比较慢（10-30 分钟），因为需要编译所有依赖。后续构建会快很多（1-5 分钟）。

**加速技巧：**
- 使用 `cargo build --release` 的增量编译
- 使用 `sccache` 缓存编译结果
- 使用更快的链接器（如 `lld`）

### 技术问题

#### Q23: 为什么选择 Tauri 而不是 Electron？

**A:** 主要原因：

| 指标 | Tauri | Electron |
|------|-------|----------|
| 安装包大小 | ~8 MB | ~60 MB |
| 内存占用 | ~70 MB | ~180 MB |
| 启动速度 | ~1.5s | ~3.5s |
| 安全性 | 更高 | 一般 |

详细对比请参考[设计原理](./design-principles.md#技术选型理由)。

#### Q24: 前端和后端如何通信？

**A:** 使用 Tauri 的 IPC (Inter-Process Communication) 机制：

**前端调用后端：**
```typescript
import { invoke } from '@tauri-apps/api/core';

const accounts = await invoke('get_accounts');
```

**后端定义命令：**
```rust
#[tauri::command]
async fn get_accounts() -> Result<Vec<Account>, String> {
    // 实现逻辑
}
```

#### Q25: 如何添加新功能？

**A:** 基本流程：

1. **后端（Rust）：**
   - 在 `src-tauri/src/commands/` 中添加新命令
   - 实现业务逻辑
   - 在 `main.rs` 中注册命令

2. **前端（TypeScript）：**
   - 在 `src/commands/` 中添加类型定义
   - 创建服务函数封装 `invoke` 调用
   - 在组件中使用

3. **测试：**
   - 编写单元测试
   - 手动测试功能

#### Q26: 如何调试应用程序？

**A:**

**前端调试：**
- 使用 Chrome DevTools（Ctrl+Shift+I / Cmd+Option+I）
- 在代码中使用 `console.log` 或 `debugger`

**后端调试：**
- 使用 `tracing` 库记录日志
- 使用 VS Code 的 Rust 调试器
- 使用 LLDB/GDB 命令行调试

详细说明请参考[问题排查手册](./troubleshooting.md#调试技巧)。

#### Q27: 如何贡献代码？

**A:**

1. Fork 仓库
2. 创建功能分支：`git checkout -b feature/my-feature`
3. 提交更改：`git commit -m "Add my feature"`
4. 推送到 Fork：`git push origin feature/my-feature`
5. 创建 Pull Request

详细流程请参考[贡献指南](../development/contributing.md)。

### 测试

#### Q28: 如何运行测试？

**A:**

**Rust 测试：**
```bash
cd src-tauri
cargo test
```

**前端测试：**
```bash
npm test
```

#### Q29: 测试覆盖率如何？

**A:** 当前测试覆盖率：
- 后端（Rust）: ~70%
- 前端（TypeScript）: ~60%

我们持续努力提高测试覆盖率。

#### Q30: 如何编写测试？

**A:**

**Rust 单元测试：**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let result = my_function();
        assert_eq!(result, expected);
    }
}
```

**TypeScript 测试：**
```typescript
import { describe, it, expect } from 'vitest';

describe('MyComponent', () => {
  it('should render correctly', () => {
    // 测试代码
  });
});
```


## 部署相关问题

### 打包和分发

#### Q31: 如何创建安装包？

**A:**

```bash
# 构建所有平台的安装包
npm run tauri:build

# 只构建特定平台
npm run tauri:build -- --target x86_64-pc-windows-msvc  # Windows
npm run tauri:build -- --target x86_64-apple-darwin     # macOS Intel
npm run tauri:build -- --target aarch64-apple-darwin    # macOS Apple Silicon
```

#### Q32: 安装包在哪里？

**A:** 构建完成后，安装包位于：

**Windows:**
```
src-tauri/target/release/bundle/nsis/
└── Antigravity Agent_1.0.3_x64-setup.exe
```

**macOS:**
```
src-tauri/target/release/bundle/dmg/
└── Antigravity Agent_1.0.3_x64.dmg
```

**Linux:**
```
src-tauri/target/release/bundle/appimage/
└── antigravity-agent_1.0.3_amd64.AppImage
```

#### Q33: 如何签名应用程序？

**A:**

**Windows (代码签名):**
```bash
# 需要代码签名证书
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com "Antigravity Agent.exe"
```

**macOS (代码签名和公证):**
```bash
# 签名
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" "Antigravity Agent.app"

# 公证
xcrun notarytool submit "Antigravity Agent.dmg" --apple-id "your@email.com" --password "app-specific-password" --team-id "TEAM_ID"
```

#### Q34: 如何设置自动更新？

**A:** Antigravity Agent 使用 Tauri 的内置更新器：

1. **配置更新端点**（`tauri.conf.json`）：
```json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://github.com/user/repo/releases/latest/download/latest.json"
      ],
      "pubkey": "YOUR_PUBLIC_KEY"
    }
  }
}
```

2. **生成密钥对**：
```bash
tauri signer generate -w ~/.tauri/myapp.key
```

3. **签名更新包**：
```bash
tauri signer sign path/to/app.exe
```

4. **创建 `latest.json`**：
```json
{
  "version": "1.0.3",
  "notes": "Release notes",
  "pub_date": "2025-12-04T00:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "...",
      "url": "https://github.com/.../app.exe"
    }
  }
}
```

#### Q35: 如何分发应用程序？

**A:** 几种方式：

1. **GitHub Releases**（推荐）
   - 创建 Release
   - 上传安装包
   - 自动更新会从这里获取

2. **直接下载**
   - 托管在自己的服务器
   - 提供下载链接

3. **应用商店**（未来计划）
   - Microsoft Store
   - Mac App Store

### CI/CD

#### Q36: 如何设置 CI/CD？

**A:** 使用 GitHub Actions 示例：

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [windows-latest, macos-latest, ubuntu-latest]

    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install dependencies
        run: npm install

      - name: Build
        run: npm run tauri:build

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.platform }}
          path: src-tauri/target/release/bundle/
```

#### Q37: 如何自动化发布流程？

**A:**

1. **创建 Git tag**：
```bash
git tag v1.0.3
git push origin v1.0.3
```

2. **GitHub Actions 自动构建**
3. **自动创建 Release**
4. **上传安装包**
5. **生成 `latest.json`**

### 性能和优化

#### Q38: 如何优化安装包大小？

**A:**

1. **Rust 优化**（`Cargo.toml`）：
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

2. **前端优化**：
   - 启用 Tree Shaking
   - 代码分割
   - 压缩资源

3. **移除未使用的依赖**

详细说明请参考[性能优化指南](./performance.md#打包体积优化)。

#### Q39: 如何提高应用程序性能？

**A:**

1. **启动性能**：
   - 延迟初始化
   - 并行加载
   - 减少启动时的 I/O

2. **运行时性能**：
   - 使用缓存
   - 优化数据库查询
   - 避免不必要的重渲染

3. **内存优化**：
   - 及时释放资源
   - 使用引用而非拷贝
   - 监控内存使用

详细说明请参考[性能优化指南](./performance.md)。

#### Q40: 如何监控生产环境的性能？

**A:**

1. **内置日志系统**：
   - 记录关键操作的耗时
   - 监控错误率

2. **用户反馈**：
   - 收集用户报告的性能问题
   - 分析日志文件

3. **性能指标**：
   - 启动时间
   - 内存使用
   - CPU 使用率


## 已知限制和计划功能

### 当前限制

#### Q41: Antigravity Agent 有哪些已知限制？

**A:**

**平台限制：**
- ❌ Linux 支持尚未完成（计划中）
- ⚠️ macOS 需要 10.15 或更高版本
- ⚠️ Windows 需要 WebView2 运行时

**功能限制：**
- 不支持同时运行多个 Antigravity 实例
- 备份文件不支持增量备份
- 不支持云同步功能
- 不支持账户分组或标签

**性能限制：**
- 建议账户数量不超过 20 个
- 大型备份文件（>500MB）可能较慢
- 数据库大小建议不超过 1GB

#### Q42: 为什么不支持 Linux？

**A:** Linux 支持正在开发中。主要挑战：

- 不同发行版的依赖差异
- 包管理器的多样性
- 桌面环境的兼容性

预计在 v1.1.0 版本中提供 Linux 支持。

#### Q43: 可以同时运行多个 Antigravity 实例吗？

**A:** 目前不支持。原因：

- 数据库锁定机制
- 配置文件冲突
- 进程管理复杂性

这是一个已知限制，未来版本可能会支持。

#### Q44: 支持云同步吗？

**A:** 目前不支持。但您可以：

- 使用备份/恢复功能手动同步
- 将配置目录放在云盘（如 OneDrive、iCloud）
- 使用 Git 同步配置文件

未来可能会添加内置的云同步功能。

### 计划功能

#### Q45: 未来会添加哪些功能？

**A:** 路线图（按优先级）：

**v1.1.0（计划 2025 Q1）：**
- ✅ Linux 支持
- ✅ 账户分组功能
- ✅ 增量备份
- ✅ 批量操作

**v1.2.0（计划 2025 Q2）：**
- 🔄 云同步支持
- 🔄 插件系统
- 🔄 主题定制
- 🔄 多语言支持（英文）

**v2.0.0（计划 2025 Q3）：**
- 🔮 移动端支持
- 🔮 Web 版本
- 🔮 团队协作功能
- 🔮 高级自动化

**图例：**
- ✅ 确定实现
- 🔄 开发中
- 🔮 计划中

#### Q46: 会支持移动端吗？

**A:** 是的，计划在 v2.0.0 中支持：

- iOS 应用
- Android 应用
- 与桌面版同步

但移动端功能可能会有所限制。

#### Q47: 会有 Web 版本吗？

**A:** 计划在 v2.0.0 中提供 Web 版本，用于：

- 远程访问
- 轻量级管理
- 跨平台兼容

但核心功能仍建议使用桌面版。

#### Q48: 会支持插件系统吗？

**A:** 是的，v1.2.0 计划添加插件系统，允许：

- 自定义功能扩展
- 第三方集成
- 社区贡献

插件 API 文档将随版本发布。

#### Q49: 会支持更多语言吗？

**A:** 是的，计划支持：

- ✅ 中文（已支持）
- 🔄 英文（v1.2.0）
- 🔮 日文（v2.0.0）
- 🔮 韩文（v2.0.0）
- 🔮 其他语言（社区贡献）

欢迎贡献翻译！

#### Q50: 如何参与功能开发？

**A:**

1. **提出建议**：
   - 在 GitHub Issues 中创建 Feature Request
   - 描述功能需求和使用场景
   - 参与讨论

2. **贡献代码**：
   - Fork 仓库
   - 实现功能
   - 提交 Pull Request

3. **测试反馈**：
   - 参与 Beta 测试
   - 报告 Bug
   - 提供改进建议

### 技术债务

#### Q51: 有哪些已知的技术债务？

**A:**

**代码质量：**
- 部分模块缺少单元测试
- 某些函数过于复杂，需要重构
- 错误处理不够统一

**性能：**
- 数据库查询可以进一步优化
- 某些操作可以并行化
- 缓存策略可以改进

**架构：**
- 需要更好的模块化
- 某些组件耦合度较高
- 需要更清晰的接口定义

我们正在持续改进这些问题。

#### Q52: 如何报告 Bug？

**A:**

1. **检查是否已报告**：
   - 搜索 GitHub Issues
   - 查看已知问题列表

2. **收集信息**：
   - 详细的重现步骤
   - 系统信息
   - 日志文件
   - 截图或录屏

3. **创建 Issue**：
   - 使用 Bug Report 模板
   - 提供完整信息
   - 标记严重程度

4. **跟进**：
   - 回答维护者的问题
   - 测试修复版本
   - 确认问题解决

### 社区和支持

#### Q53: 如何获取帮助？

**A:**

1. **文档**：
   - [用户指南](../user-guide/user-guide.md)
   - [问题排查手册](./troubleshooting.md)
   - [FAQ](./faq.md)（本文档）

2. **社区**：
   - GitHub Discussions
   - Issue 跟踪器
   - 用户论坛

3. **联系方式**：
   - GitHub: [@MonchiLin](https://github.com/MonchiLin)
   - Email: 通过 GitHub Profile 查看

#### Q54: 如何贡献文档？

**A:**

1. **发现问题**：
   - 文档错误
   - 缺失内容
   - 不清晰的说明

2. **提出改进**：
   - 创建 Issue
   - 或直接提交 PR

3. **编写文档**：
   - 遵循文档模板
   - 使用清晰的语言
   - 提供示例

4. **翻译**：
   - 翻译成其他语言
   - 保持术语一致
   - 定期同步更新

#### Q55: 项目的开发状态如何？

**A:**

**活跃度：**
- ✅ 积极维护中
- ✅ 定期发布更新
- ✅ 响应 Issues 和 PRs

**版本发布：**
- 小版本：每 1-2 个月
- 大版本：每 3-6 个月
- 补丁版本：根据需要

**社区：**
- GitHub Stars: 持续增长
- Contributors: 欢迎贡献
- Issues: 及时处理

#### Q56: 项目的长期规划是什么？

**A:**

**短期（6 个月）：**
- 完善核心功能
- 提高稳定性
- 改进用户体验

**中期（1 年）：**
- 跨平台支持（Linux）
- 云同步功能
- 插件系统

**长期（2 年）：**
- 移动端支持
- 企业版功能
- 生态系统建设

我们致力于将 Antigravity Agent 打造成最好用的 Antigravity 账户管理工具！

## 没有找到答案？

如果您的问题在这里找不到答案：

1. **查看其他文档**：
   - [用户指南](../user-guide/user-guide.md)
   - [问题排查手册](./troubleshooting.md)
   - [开发指南](../development/development-guide.md)

2. **搜索 Issues**：
   - [GitHub Issues](https://github.com/MonchiLin/antigravity-agent/issues)
   - 可能已有人问过相同问题

3. **提问**：
   - 创建新的 Issue
   - 在 Discussions 中讨论
   - 联系维护者

## 相关文档

### 进阶文档
- [问题排查](./troubleshooting.md) - 常见问题诊断和解决
- [性能优化](./performance.md) - 性能分析和优化建议
- [设计原理](./design-principles.md) - 核心设计思路和技术选型

### 使用文档
- [使用手册](../user-guide/user-guide.md) - 完整的功能说明和操作指南
- [配置说明](../user-guide/configuration.md) - 配置选项详解
- [API 参考](../user-guide/api-reference.md) - 所有命令和接口说明
- [使用示例](../user-guide/examples.md) - 常见场景示例

### 开发文档
- [开发指南](../development/development-guide.md) - 开发环境搭建和工作流程
- [贡献指南](../development/contributing.md) - 如何参与项目贡献
- [系统架构](../development/architecture.md) - 系统整体架构设计
- [代码规范](../development/code-style.md) - 代码风格和最佳实践

### 入门文档
- [项目概览](../getting-started/README.md) - 了解项目的基本信息
- [安装指南](../getting-started/installation.md) - 详细的安装步骤和系统要求
- [快速开始](../getting-started/quickstart.md) - 5 分钟快速上手教程

### 返回
- [文档首页](../../README.md) - 返回文档导航页

