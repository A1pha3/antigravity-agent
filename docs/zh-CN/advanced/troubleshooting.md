---
title: 问题排查手册
description: Antigravity Agent 常见问题诊断和解决方案
category: advanced
language: zh-CN
version: 1.0.3
lastUpdated: 2025-12-04
tags: [问题排查, 故障诊断, 调试]
---

# 问题排查手册

## 概述

本文档提供 Antigravity Agent 常见问题的诊断方法和解决方案。通过系统化的排查流程，您可以快速定位和解决大多数问题。

## 目录

- [日志查看方法](#日志查看方法)
- [常见错误和解决方案](#常见错误和解决方案)
- [数据库问题排查](#数据库问题排查)
- [进程问题排查](#进程问题排查)
- [平台特定问题](#平台特定问题)
- [调试技巧](#调试技巧)

## 日志查看方法

### 日志位置

Antigravity Agent 使用双层日志系统：

**Windows:**
```
%APPDATA%\.antigravity-agent\logs\
├── antigravity-agent.log          # 主日志文件
├── antigravity-agent.log.2025-12-04  # 按日期归档
└── error.log                       # 错误日志
```

**macOS:**
```
~/.config/.antigravity-agent/logs/
├── antigravity-agent.log
├── antigravity-agent.log.2025-12-04
└── error.log
```

**Linux (计划中):**
```
~/.config/.antigravity-agent/logs/
├── antigravity-agent.log
├── antigravity-agent.log.2025-12-04
└── error.log
```

### 快速访问日志

**方法 1：通过应用程序**
1. 打开 Antigravity Agent
2. 点击右上角的用户面板
3. 选择"查看日志"
4. 日志文件会在系统文件管理器中打开

**方法 2：命令行**

```bash
# Windows (PowerShell)
cd $env:APPDATA\.antigravity-agent\logs
Get-Content antigravity-agent.log -Tail 50

# macOS/Linux
cd ~/.config/.antigravity-agent/logs
tail -f antigravity-agent.log
```

### 日志格式

日志采用结构化格式，包含以下信息：

```
2025-12-04T10:30:45.123Z INFO antigravity_agent::commands: Account switched successfully
  account_id: "user123"
  duration_ms: 245

2025-12-04T10:30:46.456Z ERROR antigravity_agent::backup: Backup failed
  error: "Permission denied"
  path: "/path/to/backup"
  span: backup_operation
```

**字段说明：**
- **时间戳**: ISO 8601 格式的 UTC 时间
- **级别**: TRACE, DEBUG, INFO, WARN, ERROR
- **模块**: 日志来源的模块路径
- **消息**: 日志内容
- **字段**: 结构化的上下文信息

### 日志级别

可以通过环境变量调整日志级别：

```bash
# Windows (PowerShell)
$env:RUST_LOG="debug"
.\antigravity-agent.exe

# macOS/Linux
RUST_LOG=debug ./antigravity-agent

# 更细粒度的控制
RUST_LOG="antigravity_agent=debug,tauri=info"
```

**日志级别说明：**
- `error`: 只记录错误
- `warn`: 警告和错误
- `info`: 一般信息（默认）
- `debug`: 调试信息
- `trace`: 详细跟踪信息

### 日志分析技巧

#### 1. 搜索特定错误

```bash
# 搜索错误日志
grep "ERROR" antigravity-agent.log

# 搜索特定模块的日志
grep "backup" antigravity-agent.log

# 搜索时间范围内的日志
grep "2025-12-04T10:" antigravity-agent.log
```

#### 2. 统计错误频率

```bash
# 统计各类错误的数量
grep "ERROR" antigravity-agent.log | cut -d' ' -f4 | sort | uniq -c

# 查看最近的错误
grep "ERROR" antigravity-agent.log | tail -20
```

#### 3. 跟踪操作流程

```bash
# 跟踪特定操作的完整流程
grep "span: backup_operation" antigravity-agent.log
```


## 常见错误和解决方案

### 启动问题

#### 问题 1: 应用程序无法启动

**症状：**
- 双击应用程序图标后没有反应
- 应用程序闪退
- 显示"应用程序无法启动"错误

**排查步骤：**

1. **检查日志文件**
   ```bash
   # 查看最近的错误
   tail -50 antigravity-agent.log | grep ERROR
   ```

2. **检查系统要求**
   - Windows: 需要 Windows 10 或更高版本
   - macOS: 需要 macOS 10.15 或更高版本
   - 确保安装了必要的运行时（WebView2 for Windows）

3. **检查文件权限**
   ```bash
   # macOS/Linux: 确保应用程序有执行权限
   chmod +x antigravity-agent
   ```

4. **清除配置文件**
   ```bash
   # 备份并删除配置目录
   # Windows
   mv %APPDATA%\.antigravity-agent %APPDATA%\.antigravity-agent.backup

   # macOS/Linux
   mv ~/.config/.antigravity-agent ~/.config/.antigravity-agent.backup
   ```

**常见原因和解决方案：**

| 错误信息 | 原因 | 解决方案 |
|---------|------|---------|
| "WebView2 not found" | 缺少 WebView2 运行时 | 安装 [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) |
| "Permission denied" | 文件权限问题 | 使用管理员权限运行或修改权限 |
| "Database locked" | 数据库被其他进程占用 | 关闭其他实例或重启系统 |
| "Config file corrupted" | 配置文件损坏 | 删除配置文件重新初始化 |

#### 问题 2: 启动缓慢

**症状：**
- 应用程序启动需要超过 5 秒
- 启动时界面卡顿

**排查步骤：**

1. **检查启动日志**
   ```bash
   grep "initialized in" antigravity-agent.log
   ```

2. **检查系统资源**
   - CPU 使用率是否过高
   - 磁盘 I/O 是否繁忙
   - 内存是否充足

3. **优化启动**
   - 关闭不必要的后台程序
   - 清理临时文件
   - 检查磁盘健康状态

**解决方案：**
- 参考 [性能优化指南](./performance.md#启动时间优化)
- 禁用不必要的功能
- 升级硬件（SSD、更多内存）

### 账户管理问题

#### 问题 3: 无法切换账户

**症状：**
- 点击切换账户后没有反应
- 显示"切换失败"错误
- 账户切换后数据不正确

**排查步骤：**

1. **检查账户状态**
   ```bash
   grep "account_switch" antigravity-agent.log
   ```

2. **验证账户数据**
   - 检查账户是否存在于数据库
   - 验证账户配置文件是否完整

3. **检查进程状态**
   - 确认 Antigravity 进程是否正在运行
   - 检查进程是否响应

**解决方案：**

```bash
# 1. 停止所有相关进程
# Windows
taskkill /F /IM antigravity.exe

# macOS/Linux
pkill -9 antigravity

# 2. 清除缓存
# 删除 .antigravity-agent/cache 目录

# 3. 重新启动应用程序
```

#### 问题 4: 账户数据丢失

**症状：**
- 账户列表为空
- 之前添加的账户不见了

**排查步骤：**

1. **检查数据库文件**
   ```bash
   # Windows
   dir %APPDATA%\.antigravity-agent\data.db

   # macOS/Linux
   ls -lh ~/.config/.antigravity-agent/data.db
   ```

2. **验证数据库完整性**
   ```bash
   sqlite3 data.db "PRAGMA integrity_check;"
   ```

3. **检查备份**
   - 查找自动备份文件
   - 检查导出的备份

**解决方案：**

```bash
# 从备份恢复
# 1. 找到最近的备份文件
# 2. 使用应用程序的导入功能恢复
# 3. 或手动替换数据库文件
```

### 备份和恢复问题

#### 问题 5: 备份失败

**症状：**
- 创建备份时显示错误
- 备份文件不完整
- 备份过程中断

**常见错误：**

| 错误信息 | 原因 | 解决方案 |
|---------|------|---------|
| "Insufficient disk space" | 磁盘空间不足 | 清理磁盘空间或选择其他位置 |
| "Permission denied" | 没有写入权限 | 选择有权限的目录 |
| "Source not found" | 源文件不存在 | 检查账户配置是否正确 |
| "Compression failed" | 压缩过程出错 | 检查文件是否损坏 |

**解决方案：**

```bash
# 1. 检查磁盘空间
# Windows
wmic logicaldisk get size,freespace,caption

# macOS/Linux
df -h

# 2. 手动创建备份
# 复制配置目录到安全位置

# 3. 使用命令行工具
# 如果 GUI 失败，尝试使用命令行
```

#### 问题 6: 恢复失败

**症状：**
- 导入备份时显示错误
- 恢复后数据不完整
- 密码验证失败

**排查步骤：**

1. **验证备份文件**
   ```bash
   # 检查文件是否损坏
   unzip -t backup.zip
   ```

2. **检查密码**
   - 确认密码输入正确
   - 检查键盘布局（大小写、数字键盘）

3. **查看详细错误**
   ```bash
   grep "restore" antigravity-agent.log | tail -20
   ```

**解决方案：**

```bash
# 1. 手动解压备份文件
unzip backup.zip -d temp_restore

# 2. 检查解压后的文件
ls -la temp_restore

# 3. 手动复制文件到配置目录
# 注意：先备份当前配置
```


## 数据库问题排查

### 数据库位置

```bash
# Windows
%APPDATA%\.antigravity-agent\data.db

# macOS/Linux
~/.config/.antigravity-agent/data.db
```

### 常见数据库问题

#### 问题 7: 数据库锁定

**症状：**
- 显示"database is locked"错误
- 操作超时
- 应用程序无响应

**原因：**
- 多个进程同时访问数据库
- 之前的进程未正常关闭
- 文件系统问题

**解决方案：**

```bash
# 1. 检查是否有多个实例运行
# Windows
tasklist | findstr antigravity

# macOS/Linux
ps aux | grep antigravity

# 2. 关闭所有实例
# Windows
taskkill /F /IM antigravity-agent.exe

# macOS/Linux
pkill -9 antigravity-agent

# 3. 删除锁文件（如果存在）
# Windows
del %APPDATA%\.antigravity-agent\data.db-shm
del %APPDATA%\.antigravity-agent\data.db-wal

# macOS/Linux
rm ~/.config/.antigravity-agent/data.db-shm
rm ~/.config/.antigravity-agent/data.db-wal

# 4. 重启应用程序
```

#### 问题 8: 数据库损坏

**症状：**
- 显示"database disk image is malformed"
- 查询返回错误结果
- 应用程序崩溃

**诊断：**

```bash
# 使用 SQLite 命令行工具检查
sqlite3 data.db "PRAGMA integrity_check;"

# 如果返回 "ok"，数据库完好
# 如果返回错误信息，数据库已损坏
```

**解决方案：**

```bash
# 方法 1: 尝试修复
sqlite3 data.db "PRAGMA integrity_check;"
sqlite3 data.db "REINDEX;"
sqlite3 data.db "VACUUM;"

# 方法 2: 导出并重建
sqlite3 data.db ".dump" > backup.sql
mv data.db data.db.corrupted
sqlite3 data.db < backup.sql

# 方法 3: 从备份恢复
# 使用应用程序的导入功能
# 或手动替换数据库文件
```

#### 问题 9: 数据库性能问题

**症状：**
- 查询缓慢
- 界面卡顿
- 操作延迟

**诊断：**

```bash
# 检查数据库大小
# Windows
dir %APPDATA%\.antigravity-agent\data.db

# macOS/Linux
ls -lh ~/.config/.antigravity-agent/data.db

# 分析查询性能
sqlite3 data.db "EXPLAIN QUERY PLAN SELECT * FROM accounts;"
```

**解决方案：**

```bash
# 1. 优化数据库
sqlite3 data.db "ANALYZE;"
sqlite3 data.db "VACUUM;"

# 2. 重建索引
sqlite3 data.db "REINDEX;"

# 3. 清理旧数据
# 删除不需要的日志和历史记录

# 4. 检查 WAL 文件大小
# 如果 WAL 文件过大，执行 checkpoint
sqlite3 data.db "PRAGMA wal_checkpoint(TRUNCATE);"
```

### 数据库维护

**定期维护任务：**

```bash
# 每月执行一次
sqlite3 data.db "ANALYZE;"
sqlite3 data.db "VACUUM;"
sqlite3 data.db "PRAGMA optimize;"

# 检查数据库健康状态
sqlite3 data.db "PRAGMA integrity_check;"
sqlite3 data.db "PRAGMA foreign_key_check;"
```

## 进程问题排查

### 进程管理

#### 问题 10: Antigravity 进程无法启动

**症状：**
- 点击启动按钮后没有反应
- 显示"启动失败"错误
- 进程立即退出

**排查步骤：**

1. **检查进程日志**
   ```bash
   grep "process_start" antigravity-agent.log
   ```

2. **手动启动进程**
   ```bash
   # 找到 Antigravity 可执行文件
   # 尝试手动启动并查看错误信息
   ```

3. **检查端口占用**
   ```bash
   # Windows
   netstat -ano | findstr :PORT

   # macOS/Linux
   lsof -i :PORT
   ```

**常见原因：**

| 错误 | 原因 | 解决方案 |
|------|------|---------|
| "Port already in use" | 端口被占用 | 关闭占用端口的程序或更改端口 |
| "Executable not found" | 可执行文件不存在 | 重新安装 Antigravity |
| "Permission denied" | 权限不足 | 使用管理员权限运行 |
| "Missing dependencies" | 缺少依赖 | 安装必要的运行时库 |

#### 问题 11: 进程意外终止

**症状：**
- 进程突然停止
- 显示"进程已退出"
- 连接断开

**排查步骤：**

1. **检查进程退出代码**
   ```bash
   grep "process_exit" antigravity-agent.log
   ```

2. **查看系统日志**
   ```bash
   # Windows: 事件查看器
   eventvwr.msc

   # macOS: Console.app
   # Linux: journalctl
   journalctl -u antigravity
   ```

3. **检查资源使用**
   - 内存是否耗尽
   - CPU 是否过载
   - 磁盘空间是否充足

**解决方案：**

```bash
# 1. 增加进程监控
# 应用程序会自动重启崩溃的进程

# 2. 检查系统资源
# 确保有足够的内存和 CPU

# 3. 更新到最新版本
# 可能是已知 bug，已在新版本中修复
```

#### 问题 12: 进程僵尸/无响应

**症状：**
- 进程存在但不响应
- 无法停止进程
- 资源占用异常

**诊断：**

```bash
# Windows
tasklist /FI "IMAGENAME eq antigravity.exe" /V

# macOS/Linux
ps aux | grep antigravity
top -p $(pgrep antigravity)
```

**解决方案：**

```bash
# 1. 尝试正常停止
# 使用应用程序的停止按钮

# 2. 强制终止
# Windows
taskkill /F /PID <PID>

# macOS/Linux
kill -9 <PID>

# 3. 清理残留
# 删除临时文件和锁文件

# 4. 重启应用程序
```

### 进程监控

**实时监控进程状态：**

```bash
# Windows (PowerShell)
while ($true) {
    Get-Process antigravity* | Format-Table Name, CPU, Memory -AutoSize
    Start-Sleep -Seconds 5
}

# macOS/Linux
watch -n 5 'ps aux | grep antigravity'
```


## 平台特定问题

### Windows 特定问题

#### 问题 13: WebView2 相关问题

**症状：**
- 应用程序无法启动
- 显示"WebView2 Runtime not found"
- 界面显示异常

**解决方案：**

1. **安装 WebView2 Runtime**
   - 下载：[Microsoft Edge WebView2](https://developer.microsoft.com/microsoft-edge/webview2/)
   - 选择"Evergreen Standalone Installer"
   - 运行安装程序

2. **验证安装**
   ```powershell
   # 检查 WebView2 版本
   Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
   ```

3. **手动修复**
   ```powershell
   # 重新注册 WebView2
   regsvr32 "C:\Program Files (x86)\Microsoft\EdgeWebView\Application\<version>\EmbeddedBrowserWebView.dll"
   ```

#### 问题 14: Windows Defender 误报

**症状：**
- 应用程序被 Windows Defender 阻止
- 文件被隔离
- 无法下载或运行

**解决方案：**

1. **添加排除项**
   ```powershell
   # 以管理员身份运行 PowerShell
   Add-MpPreference -ExclusionPath "C:\Program Files\Antigravity Agent"
   Add-MpPreference -ExclusionProcess "antigravity-agent.exe"
   ```

2. **恢复被隔离的文件**
   - 打开 Windows 安全中心
   - 转到"病毒和威胁防护"
   - 点击"保护历史记录"
   - 找到被隔离的文件并恢复

3. **报告误报**
   - 访问 [Microsoft 安全智能](https://www.microsoft.com/wdsi/filesubmission)
   - 提交误报报告

#### 问题 15: 权限问题

**症状：**
- 无法访问某些文件或目录
- 显示"Access denied"错误

**解决方案：**

```powershell
# 1. 以管理员身份运行
# 右键点击应用程序 -> "以管理员身份运行"

# 2. 修改文件权限
icacls "%APPDATA%\.antigravity-agent" /grant %USERNAME%:F /T

# 3. 检查 UAC 设置
# 控制面板 -> 用户账户 -> 更改用户账户控制设置
```

### macOS 特定问题

#### 问题 16: 应用程序无法打开（已损坏）

**症状：**
- 显示"应用程序已损坏，无法打开"
- 显示"来自身份不明的开发者"

**解决方案：**

```bash
# 方法 1: 移除隔离属性
xattr -cr /Applications/Antigravity\ Agent.app

# 方法 2: 允许任何来源的应用
sudo spctl --master-disable

# 方法 3: 手动允许
# 系统偏好设置 -> 安全性与隐私 -> 通用
# 点击"仍要打开"
```

#### 问题 17: 权限请求

**症状：**
- 应用程序请求各种权限
- 某些功能无法使用

**解决方案：**

```bash
# 检查权限状态
tccutil reset All com.antigravity-agent.app

# 手动授予权限
# 系统偏好设置 -> 安全性与隐私 -> 隐私
# 选择相应的权限类别并勾选 Antigravity Agent
```

**需要的权限：**
- ✅ 文件和文件夹访问
- ✅ 完全磁盘访问（如果需要）
- ❌ 相机（不需要）
- ❌ 麦克风（不需要）

#### 问题 18: Gatekeeper 问题

**症状：**
- 应用程序被 Gatekeeper 阻止
- 无法验证开发者

**解决方案：**

```bash
# 1. 临时禁用 Gatekeeper
sudo spctl --master-disable

# 2. 为特定应用添加例外
sudo spctl --add /Applications/Antigravity\ Agent.app

# 3. 重新启用 Gatekeeper
sudo spctl --master-enable

# 4. 验证签名
codesign -dv --verbose=4 /Applications/Antigravity\ Agent.app
```

### Linux 特定问题（计划中）

#### 问题 19: 依赖缺失

**症状：**
- 应用程序无法启动
- 显示"library not found"错误

**解决方案：**

```bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.0-37 libgtk-3-0

# Fedora
sudo dnf install webkit2gtk3 gtk3

# Arch Linux
sudo pacman -S webkit2gtk gtk3

# 检查依赖
ldd antigravity-agent
```

#### 问题 20: AppImage 权限

**症状：**
- AppImage 无法执行
- 显示权限错误

**解决方案：**

```bash
# 添加执行权限
chmod +x Antigravity-Agent-*.AppImage

# 运行 AppImage
./Antigravity-Agent-*.AppImage

# 如果仍然失败，尝试提取并运行
./Antigravity-Agent-*.AppImage --appimage-extract
cd squashfs-root
./AppRun
```


## 调试技巧

### 启用详细日志

#### 开发模式

```bash
# 设置环境变量启用详细日志
# Windows (PowerShell)
$env:RUST_LOG="debug"
$env:TAURI_DEBUG="true"
.\antigravity-agent.exe

# macOS/Linux
RUST_LOG=debug TAURI_DEBUG=1 ./antigravity-agent
```

#### 模块级别日志

```bash
# 只启用特定模块的调试日志
RUST_LOG="antigravity_agent::backup=debug,antigravity_agent::database=trace"

# 排除某些模块
RUST_LOG="debug,hyper=info,tokio=info"
```

### 使用开发者工具

#### 启用 DevTools

**方法 1: 快捷键**
- Windows/Linux: `Ctrl + Shift + I`
- macOS: `Cmd + Option + I`

**方法 2: 代码启用**

```rust
// src-tauri/src/main.rs
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### DevTools 使用技巧

**Console 面板：**
```javascript
// 查看应用状态
console.log(window.__TAURI__);

// 测试 Tauri 命令
await window.__TAURI__.invoke('get_accounts');

// 监控性能
console.time('operation');
// ... 执行操作
console.timeEnd('operation');
```

**Network 面板：**
- 监控 IPC 通信
- 查看资源加载时间
- 分析请求/响应

**Performance 面板：**
- 录制性能分析
- 识别性能瓶颈
- 查看帧率

**Memory 面板：**
- 拍摄堆快照
- 分析内存泄漏
- 监控内存使用

### 断点调试

#### 前端调试

```typescript
// 在代码中设置断点
function handleAccountSwitch(accountId: string) {
  debugger;  // 执行到这里会暂停

  // 调试代码
  console.log('Switching to account:', accountId);
}
```

#### 后端调试

**使用 VS Code：**

1. 安装 Rust 扩展
2. 创建 `.vscode/launch.json`：

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Tauri",
      "cargo": {
        "args": [
          "build",
          "--manifest-path=./src-tauri/Cargo.toml",
          "--no-default-features"
        ]
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

3. 在代码中设置断点
4. 按 F5 开始调试

**使用 LLDB（命令行）：**

```bash
# 编译 debug 版本
cargo build

# 启动 LLDB
lldb target/debug/antigravity-agent

# 设置断点
(lldb) breakpoint set --name main
(lldb) breakpoint set --file backup.rs --line 42

# 运行
(lldb) run

# 调试命令
(lldb) continue  # 继续执行
(lldb) step      # 单步进入
(lldb) next      # 单步跳过
(lldb) print var # 打印变量
```

### 网络调试

#### 抓包分析

**使用 Wireshark：**

```bash
# 过滤 HTTP/HTTPS 流量
http or tls

# 过滤特定端口
tcp.port == 443

# 过滤特定主机
ip.addr == 192.168.1.100
```

**使用 Charles Proxy：**

1. 配置系统代理
2. 安装 SSL 证书
3. 监控 HTTPS 流量

#### 模拟网络条件

```bash
# macOS: 使用 Network Link Conditioner
# 下载 Additional Tools for Xcode

# Linux: 使用 tc (traffic control)
sudo tc qdisc add dev eth0 root netem delay 100ms

# Windows: 使用 Clumsy
# 下载并运行 Clumsy
```

### 性能分析

#### CPU 分析

```bash
# 使用 cargo-flamegraph
cargo install flamegraph
cargo flamegraph --bin antigravity-agent

# 使用 perf (Linux)
perf record -g ./target/release/antigravity-agent
perf report

# 使用 Instruments (macOS)
instruments -t "Time Profiler" ./target/release/antigravity-agent
```

#### 内存分析

```bash
# 使用 valgrind (Linux)
valgrind --leak-check=full ./target/release/antigravity-agent

# 使用 heaptrack (Linux)
heaptrack ./target/release/antigravity-agent
heaptrack_gui heaptrack.antigravity-agent.*.gz
```

### 日志增强

#### 添加自定义日志

```rust
use tracing::{debug, info, warn, error, instrument};

#[instrument]
async fn debug_function(param: &str) {
    debug!("Function called with param: {}", param);

    // 添加结构化字段
    info!(
        param = param,
        timestamp = ?std::time::SystemTime::now(),
        "Processing started"
    );

    // 执行操作
    match perform_operation(param).await {
        Ok(result) => {
            info!(result = ?result, "Operation succeeded");
        }
        Err(e) => {
            error!(error = %e, "Operation failed");
        }
    }
}
```

#### 日志过滤和格式化

```rust
// 自定义日志格式
use tracing_subscriber::{fmt, EnvFilter};

fn setup_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .json()  // 使用 JSON 格式
        .init();
}
```

### 问题报告

#### 收集诊断信息

创建诊断报告脚本：

```bash
#!/bin/bash
# collect-diagnostics.sh

echo "=== System Information ===" > diagnostics.txt
uname -a >> diagnostics.txt
echo "" >> diagnostics.txt

echo "=== Application Version ===" >> diagnostics.txt
./antigravity-agent --version >> diagnostics.txt
echo "" >> diagnostics.txt

echo "=== Recent Logs ===" >> diagnostics.txt
tail -100 ~/.config/.antigravity-agent/logs/antigravity-agent.log >> diagnostics.txt
echo "" >> diagnostics.txt

echo "=== Error Logs ===" >> diagnostics.txt
grep "ERROR" ~/.config/.antigravity-agent/logs/antigravity-agent.log >> diagnostics.txt
echo "" >> diagnostics.txt

echo "=== Process Status ===" >> diagnostics.txt
ps aux | grep antigravity >> diagnostics.txt
echo "" >> diagnostics.txt

echo "=== Disk Space ===" >> diagnostics.txt
df -h >> diagnostics.txt

echo "Diagnostics collected in diagnostics.txt"
```

#### 提交问题

提交问题时应包含：

1. **问题描述**
   - 详细描述问题
   - 重现步骤
   - 预期行为 vs 实际行为

2. **环境信息**
   - 操作系统和版本
   - 应用程序版本
   - 硬件配置

3. **日志文件**
   - 相关的日志片段
   - 错误堆栈跟踪
   - 截图或录屏

4. **诊断信息**
   - 系统信息
   - 进程状态
   - 资源使用情况

## 常见问题快速参考

| 问题 | 快速解决方案 |
|------|-------------|
| 应用无法启动 | 检查日志 → 清除配置 → 重新安装 |
| 数据库锁定 | 关闭所有实例 → 删除锁文件 → 重启 |
| 账户切换失败 | 停止进程 → 清除缓存 → 重试 |
| 备份失败 | 检查磁盘空间 → 检查权限 → 更换目录 |
| 进程无响应 | 强制终止 → 清理残留 → 重启应用 |
| 性能问题 | 优化数据库 → 清理日志 → 检查资源 |

## 获取帮助

如果以上方法都无法解决问题：

1. **查看文档**
   - [用户指南](../user-guide/user-guide.md)
   - [FAQ](./faq.md)
   - [性能优化](./performance.md)

2. **社区支持**
   - GitHub Issues
   - 讨论论坛
   - 用户社区

3. **提交 Bug 报告**
   - 使用诊断脚本收集信息
   - 在 GitHub 上创建 Issue
   - 提供详细的重现步骤

## 相关文档

### 进阶文档
- [FAQ](./faq.md) - 常见问题解答
- [性能优化](./performance.md) - 性能分析和优化建议
- [设计原理](./design-principles.md) - 核心设计思路和技术选型

### 使用文档
- [使用手册](../user-guide/user-guide.md) - 完整的功能说明和操作指南
- [配置说明](../user-guide/configuration.md) - 配置选项详解
- [API 参考](../user-guide/api-reference.md) - 所有命令和接口说明

### 开发文档
- [开发指南](../development/development-guide.md) - 开发环境搭建和工作流程
- [系统架构](../development/architecture.md) - 系统整体架构设计
- [代码规范](../development/code-style.md) - 代码风格和最佳实践

### 入门文档
- [项目概览](../getting-started/README.md) - 了解项目的基本信息
- [安装指南](../getting-started/installation.md) - 详细的安装步骤和系统要求
- [快速开始](../getting-started/quickstart.md) - 5 分钟快速上手教程

### 返回
- [文档首页](../../README.md) - 返回文档导航页

