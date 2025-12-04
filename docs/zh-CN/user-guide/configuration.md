---
title: Antigravity Agent 配置说明
description: 详细的配置选项说明和配置文件管理
category: user-guide
language: zh-CN
version: 1.0.3
lastUpdated: 2025-12-04
tags: [配置, 设置, 系统托盘, 日志]
---

# Antigravity Agent 配置说明

## 概述

Antigravity Agent 使用 JSON 格式的配置文件来存储应用程序设置和用户数据。本文档详细说明了配置文件的位置、各项配置选项的含义、默认值以及如何修改这些配置。

## 目录

- [配置文件位置](#配置文件位置)
- [应用设置配置](#应用设置配置)
- [系统托盘设置](#系统托盘设置)
- [静默启动设置](#静默启动设置)
- [日志配置](#日志配置)
- [数据存储位置](#数据存储位置)
- [Antigravity 路径配置](#antigravity-路径配置)
- [配置文件示例](#配置文件示例)
- [配置管理最佳实践](#配置管理最佳实践)

## 配置文件位置

Antigravity Agent 的配置文件存储在不同操作系统的标准配置目录中。

### Windows

配置目录位于：

```
%APPDATA%\.antigravity-agent\
```

完整路径示例：

```
C:\Users\[用户名]\AppData\Roaming\.antigravity-agent\
```

### macOS

配置目录位于：

```
~/.config/.antigravity-agent/
```

完整路径示例：

```
/Users/[用户名]/.config/.antigravity-agent/
```

### Linux

配置目录位于：

```
~/.config/.antigravity-agent/
```

完整路径示例：

```
/home/[用户名]/.config/.antigravity-agent/
```

### 配置目录结构

配置目录包含以下文件和子目录：

```
.antigravity-agent/
├── app_settings.json          # 应用程序设置
├── window_state.json          # 窗口状态（位置、大小）
├── antigravity_path.json      # Antigravity 可执行文件路径
├── backups/                   # 账户备份数据
│   └── [账户备份文件]
└── logs/                      # 日志文件
    └── antigravity-agent.YYYY-MM-DD
```


## 应用设置配置

### 配置文件：`app_settings.json`

这是主要的应用程序设置文件，包含系统托盘和静默启动等选项。

### 配置结构

```json
{
  "system_tray_enabled": false,
  "silent_start_enabled": false
}
```

### 配置选项说明

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `system_tray_enabled` | boolean | `false` | 是否启用系统托盘功能 |
| `silent_start_enabled` | boolean | `false` | 是否启用静默启动（启动时最小化） |

### 如何修改

**方法 1：通过应用界面**（推荐）

1. 打开 Antigravity Agent
2. 点击右上角的设置图标
3. 在设置对话框中调整相应选项
4. 设置会自动保存

**方法 2：手动编辑配置文件**

1. 关闭 Antigravity Agent
2. 找到配置文件（参考上方路径）
3. 使用文本编辑器打开 `app_settings.json`
4. 修改配置值
5. 保存文件并重启应用

**注意**：手动编辑时请确保 JSON 格式正确，否则可能导致配置加载失败。

## 系统托盘设置

### 功能说明

系统托盘功能允许 Antigravity Agent 在后台运行，并通过系统托盘图标快速访问。启用后，关闭主窗口不会退出应用，而是最小化到系统托盘。

### 配置选项

**配置项**：`system_tray_enabled`

**默认值**：`false`（默认不启用，避免打扰用户）

**可选值**：
- `true`：启用系统托盘
- `false`：禁用系统托盘

### 启用系统托盘

**通过界面启用**：

1. 打开 Antigravity Agent
2. 点击右上角的设置图标
3. 找到"系统托盘"选项
4. 切换开关至"开启"状态
5. 系统托盘图标会立即出现在任务栏

**通过配置文件启用**：

编辑 `app_settings.json`：

```json
{
  "system_tray_enabled": true,
  "silent_start_enabled": false
}
```

### 系统托盘功能

启用系统托盘后，您可以：

- **最小化到托盘**：关闭主窗口时，应用会最小化到系统托盘而不是退出
- **快速访问**：点击托盘图标可以快速显示/隐藏主窗口
- **右键菜单**：右键点击托盘图标可以访问常用功能
- **后台运行**：应用在后台持续运行，保持账户监控功能

### 托盘图标位置

- **Windows**：任务栏右下角的通知区域
- **macOS**：菜单栏右上角
- **Linux**：系统托盘区域（取决于桌面环境）

### 托盘菜单功能

右键点击系统托盘图标，可以访问以下功能：

- **显示/隐藏窗口**：快速切换主窗口显示状态
- **退出应用**：完全退出 Antigravity Agent

### 禁用系统托盘

如果不需要系统托盘功能：

1. 打开设置对话框
2. 关闭"系统托盘"开关
3. 托盘图标会立即消失
4. 关闭主窗口将直接退出应用

### 使用场景

**适合启用系统托盘的场景**：

- 需要应用长期在后台运行
- 频繁切换账户，需要快速访问
- 希望减少任务栏占用
- 需要实时监控账户状态

**不适合启用的场景**：

- 偶尔使用应用
- 不希望应用占用系统资源
- 桌面环境不支持系统托盘


## 静默启动设置

### 功能说明

静默启动功能允许 Antigravity Agent 在系统启动时自动运行，并最小化到后台或系统托盘，不显示主窗口。这对于需要应用始终在后台运行的用户非常有用。

### 配置选项

**配置项**：`silent_start_enabled`

**默认值**：`false`（默认不启用，让用户看到应用界面）

**可选值**：
- `true`：启用静默启动
- `false`：禁用静默启动（正常启动并显示窗口）

### 启用静默启动

**通过界面启用**：

1. 打开 Antigravity Agent
2. 点击右上角的设置图标
3. 找到"静默启动"选项
4. 切换开关至"开启"状态
5. 下次启动时将自动应用

**通过配置文件启用**：

编辑 `app_settings.json`：

```json
{
  "system_tray_enabled": true,
  "silent_start_enabled": true
}
```

**注意**：建议同时启用系统托盘和静默启动，这样应用会在后台运行并可通过托盘图标访问。

### 静默启动行为

启用静默启动后：

- **自动启动**：应用随系统启动（需要在操作系统中配置开机自启）
- **隐藏窗口**：启动时不显示主窗口
- **后台运行**：应用在后台运行，保持所有功能
- **托盘访问**：如果启用了系统托盘，可以通过托盘图标访问应用

### 配置开机自启

静默启动功能需要配合操作系统的开机自启设置：

**Windows**：

1. 按 `Win + R` 打开运行对话框
2. 输入 `shell:startup` 并回车
3. 将 Antigravity Agent 的快捷方式复制到打开的文件夹

或者通过任务管理器：

1. 按 `Ctrl + Shift + Esc` 打开任务管理器
2. 切换到"启动"选项卡
3. 找到 Antigravity Agent 并启用

**macOS**：

1. 打开"系统偏好设置" > "用户与群组"
2. 选择当前用户，点击"登录项"
3. 点击"+"按钮，添加 Antigravity Agent
4. 勾选"隐藏"选项以实现静默启动

**Linux**：

根据桌面环境不同，通常在"启动应用程序"或"自动启动"设置中添加：

- **GNOME**：设置 > 应用程序 > 启动应用程序
- **KDE**：系统设置 > 启动和关闭 > 自动启动
- **XFCE**：设置 > 会话和启动 > 应用程序自动启动

### 禁用静默启动

如果希望应用启动时显示窗口：

1. 打开设置对话框
2. 关闭"静默启动"开关
3. 下次启动时将正常显示主窗口

### 使用场景

**适合启用静默启动的场景**：

- 需要应用随系统启动
- 希望应用在后台默默运行
- 不需要频繁查看主界面
- 主要通过系统托盘访问功能

**不适合启用的场景**：

- 不需要应用自动启动
- 希望看到应用启动状态
- 需要在启动时立即操作

### 静默启动与系统托盘的配合

| 系统托盘 | 静默启动 | 行为说明 |
|---------|---------|---------|
| 禁用 | 禁用 | 正常启动，显示窗口 |
| 启用 | 禁用 | 正常启动，显示窗口，可最小化到托盘 |
| 禁用 | 启用 | 启动时隐藏窗口，但无法通过托盘访问（不推荐） |
| 启用 | 启用 | 启动时隐藏窗口，通过托盘访问（推荐配置） |

**最佳实践**：如果启用静默启动，强烈建议同时启用系统托盘，否则应用启动后将无法访问。


## 日志配置

### 日志系统概述

Antigravity Agent 使用双层日志系统，同时输出到控制台和文件：

- **控制台日志**：开发模式下显示在终端，使用彩色输出，便于实时查看
- **文件日志**：持久化存储，使用 JSON 格式，便于分析和故障排查

### 日志文件位置

日志文件存储在配置目录的 `logs` 子目录中。

**Windows**：

```
%APPDATA%\.antigravity-agent\logs\
```

完整路径示例：

```
C:\Users\[用户名]\AppData\Roaming\.antigravity-agent\logs\
```

**macOS**：

```
~/.config/.antigravity-agent/logs/
```

完整路径示例：

```
/Users/[用户名]/.config/.antigravity-agent/logs/
```

**Linux**：

```
~/.config/.antigravity-agent/logs/
```

完整路径示例：

```
/home/[用户名]/.config/.antigravity-agent/logs/
```

### 日志文件命名

日志文件使用滚动策略，按日期自动创建新文件：

```
antigravity-agent.2025-12-04
antigravity-agent.2025-12-05
antigravity-agent.2025-12-06
```

每天会自动创建一个新的日志文件，旧文件会保留以便查看历史记录。

### 日志格式

日志文件使用 JSON 格式，每行一条日志记录：

```json
{
  "timestamp": "2025-12-04T10:30:45.123Z",
  "level": "INFO",
  "target": "app::startup",
  "message": "🚀 启动 Antigravity Agent",
  "fields": {
    "version": "1.0.3"
  }
}
```

**字段说明**：

| 字段 | 说明 |
|------|------|
| `timestamp` | 日志时间戳（ISO 8601 格式） |
| `level` | 日志级别（ERROR/WARN/INFO/DEBUG/TRACE） |
| `target` | 日志来源模块 |
| `message` | 日志消息内容 |
| `fields` | 附加字段（可选） |

### 日志级别

日志系统支持以下级别（从低到高）：

| 级别 | 说明 | 用途 | 示例 |
|------|------|------|------|
| `TRACE` | 追踪 | 最详细的调试信息 | 函数调用跟踪 |
| `DEBUG` | 调试 | 开发调试信息 | 变量值、状态变化 |
| `INFO` | 信息 | 一般运行信息 | 启动、关闭、操作成功 |
| `WARN` | 警告 | 潜在问题警告 | 配置缺失、性能问题 |
| `ERROR` | 错误 | 错误信息 | 操作失败、异常 |

**当前配置**：应用默认记录 `INFO` 及以上级别的日志。

### 日志内容

日志记录包含以下类型的信息：

**应用启动**：
```json
{"timestamp":"2025-12-04T10:30:00Z","level":"INFO","target":"app::startup","message":"🚀 启动 Antigravity Agent","fields":{"version":"1.0.3"}}
{"timestamp":"2025-12-04T10:30:00Z","level":"INFO","target":"app::startup","message":"🖥️ 系统信息","fields":{"os":"macos","arch":"aarch64"}}
```

**账户操作**：
```json
{"timestamp":"2025-12-04T10:35:00Z","level":"INFO","target":"account::switch","message":"切换账户","fields":{"from":"user1@example.com","to":"user2@example.com"}}
{"timestamp":"2025-12-04T10:35:01Z","level":"INFO","target":"account::backup","message":"备份账户成功","fields":{"email":"user2@example.com"}}
```

**数据库操作**：
```json
{"timestamp":"2025-12-04T10:40:00Z","level":"INFO","target":"database::operation","message":"🗄️ 数据库操作成功","fields":{"operation":"read","table":"ItemTable","success":true}}
```

**错误信息**：
```json
{"timestamp":"2025-12-04T10:45:00Z","level":"ERROR","target":"process::start","message":"启动 Antigravity 失败","fields":{"error":"可执行文件不存在"}}
```

### 查看日志

**方法 1：通过应用界面**（推荐）

1. 打开 Antigravity Agent
2. 点击右上角的"日志"按钮
3. 在日志查看器中浏览日志内容
4. 可以复制日志内容用于问题反馈

**方法 2：直接打开日志文件**

1. 导航到日志目录（参考上方路径）
2. 使用文本编辑器打开日志文件
3. 由于是 JSON 格式，建议使用支持 JSON 的编辑器（如 VS Code）

**方法 3：使用命令行工具**

**macOS/Linux**：

```bash
# 查看最新日志
tail -f ~/.config/.antigravity-agent/logs/antigravity-agent.$(date +%Y-%m-%d)

# 查看今天的所有日志
cat ~/.config/.antigravity-agent/logs/antigravity-agent.$(date +%Y-%m-%d)

# 搜索错误日志
grep "ERROR" ~/.config/.antigravity-agent/logs/antigravity-agent.*

# 使用 jq 格式化 JSON 日志
cat ~/.config/.antigravity-agent/logs/antigravity-agent.$(date +%Y-%m-%d) | jq '.'
```

**Windows PowerShell**：

```powershell
# 查看最新日志
Get-Content "$env:APPDATA\.antigravity-agent\logs\antigravity-agent.$(Get-Date -Format yyyy-MM-dd)" -Wait

# 搜索错误日志
Select-String -Path "$env:APPDATA\.antigravity-agent\logs\*" -Pattern "ERROR"
```

### 清理日志

日志文件会随时间累积，可能占用较多磁盘空间。

**通过应用界面清理**：

1. 打开 Antigravity Agent
2. 点击"日志"按钮
3. 点击"清理日志"按钮
4. 确认删除所有日志文件

**手动清理**：

直接删除日志目录中的旧日志文件：

**macOS/Linux**：

```bash
# 删除所有日志
rm ~/.config/.antigravity-agent/logs/antigravity-agent.*

# 删除 7 天前的日志
find ~/.config/.antigravity-agent/logs/ -name "antigravity-agent.*" -mtime +7 -delete

# 只保留最近 3 天的日志
find ~/.config/.antigravity-agent/logs/ -name "antigravity-agent.*" -mtime +3 -delete
```

**Windows PowerShell**：

```powershell
# 删除所有日志
Remove-Item "$env:APPDATA\.antigravity-agent\logs\*"

# 删除 7 天前的日志
Get-ChildItem "$env:APPDATA\.antigravity-agent\logs\" -Filter "antigravity-agent.*" | Where-Object {$_.LastWriteTime -lt (Get-Date).AddDays(-7)} | Remove-Item
```

### 日志配置选项

目前日志系统使用固定配置，未来版本可能支持以下配置：

| 配置项 | 说明 | 计划支持 |
|--------|------|---------|
| 日志级别 | 调整记录的日志级别（INFO/DEBUG/TRACE） | 未来版本 |
| 日志保留天数 | 自动清理超过指定天数的日志 | 未来版本 |
| 日志文件大小限制 | 限制单个日志文件的最大大小 | 未来版本 |
| 日志输出格式 | 选择 JSON 或纯文本格式 | 未来版本 |
| 日志压缩 | 自动压缩旧日志文件 | 未来版本 |

### 日志分析技巧

**查找特定时间段的日志**：

```bash
# 查找 10:30 到 10:40 之间的日志
grep '"timestamp":"2025-12-04T10:3[0-9]' ~/.config/.antigravity-agent/logs/antigravity-agent.2025-12-04
```

**统计错误数量**：

```bash
# 统计今天的错误数量
grep -c '"level":"ERROR"' ~/.config/.antigravity-agent/logs/antigravity-agent.$(date +%Y-%m-%d)
```

**提取特定模块的日志**：

```bash
# 提取账户相关的日志
grep '"target":"account::' ~/.config/.antigravity-agent/logs/antigravity-agent.* | jq '.'
```

### 日志故障排查

**问题 1：日志文件过大**

- **原因**：长时间运行未清理
- **解决**：定期清理旧日志，或手动删除不需要的日志文件

**问题 2：无法写入日志**

- **原因**：权限不足或磁盘空间不足
- **解决**：检查目录权限，确保有足够的磁盘空间

**问题 3：日志格式错误**

- **原因**：应用异常退出导致日志文件损坏
- **解决**：删除损坏的日志文件，应用会创建新文件


## 数据存储位置

### 存储目录概述

Antigravity Agent 将所有数据存储在配置目录中，包括账户备份、应用设置和日志文件。

### 主配置目录

**Windows**：

```
%APPDATA%\.antigravity-agent\
```

**macOS / Linux**：

```
~/.config/.antigravity-agent/
```

### 数据文件说明

| 文件/目录 | 说明 | 大小 | 重要性 |
|-----------|------|------|--------|
| `app_settings.json` | 应用程序设置 | < 1 KB | 中 |
| `window_state.json` | 窗口位置和大小 | < 1 KB | 低 |
| `antigravity_path.json` | Antigravity 可执行文件路径 | < 1 KB | 中 |
| `backups/` | 账户备份数据目录 | 取决于账户数量 | 高 |
| `logs/` | 日志文件目录 | 随时间增长 | 中 |

### 账户备份数据

账户备份数据存储在 `backups/` 子目录中，每个账户一个文件：

```
backups/
├── backup_user1_20251204_103045.json
├── backup_user2_20251203_154230.json
└── backup_user3_20251201_092015.json
```

**备份文件命名格式**：

```
backup_[用户名]_[时间戳].json
```

**备份文件内容**：

每个备份文件包含：

- 认证状态（API 密钥、令牌）
- 用户设置（偏好、配置）
- 个人资料信息（头像、邮箱）
- 会话数据（聊天历史索引）
- 其他 Antigravity 相关数据

**备份文件大小**：

单个备份文件通常在 10-100 KB 之间，取决于用户数据量。

**安全提示**：备份文件包含敏感信息（API 密钥），请妥善保管，不要分享给他人。

### 窗口状态数据

`window_state.json` 存储窗口的位置和大小信息：

```json
{
  "x": 100,
  "y": 100,
  "width": 1200,
  "height": 800,
  "maximized": false
}
```

应用会在关闭时保存窗口状态，下次启动时恢复到相同位置和大小。

### 数据备份建议

为了保护您的账户数据，建议定期备份配置目录：

**Windows**：

```powershell
# 备份到桌面
Copy-Item -Recurse "$env:APPDATA\.antigravity-agent" "$env:USERPROFILE\Desktop\antigravity-agent-backup"

# 备份到指定位置
Copy-Item -Recurse "$env:APPDATA\.antigravity-agent" "D:\Backup\antigravity-agent-$(Get-Date -Format yyyy-MM-dd)"
```

**macOS / Linux**：

```bash
# 备份到桌面
cp -r ~/.config/.antigravity-agent ~/Desktop/antigravity-agent-backup

# 备份到指定位置
cp -r ~/.config/.antigravity-agent ~/backup/antigravity-agent-$(date +%Y-%m-%d)

# 创建压缩备份
tar -czf ~/backup/antigravity-agent-$(date +%Y-%m-%d).tar.gz -C ~/.config .antigravity-agent
```

### 数据迁移

如果需要在不同计算机之间迁移数据：

**步骤 1：导出数据**

在旧计算机上：

1. 使用应用的"导出配置"功能（推荐）
2. 或手动复制整个配置目录

**步骤 2：传输数据**

- 使用 U 盘、云存储或网络传输
- 确保数据安全，避免泄露
- 如果使用"导出配置"功能，文件已加密

**步骤 3：导入数据**

在新计算机上：

1. 安装 Antigravity Agent
2. 使用应用的"导入配置"功能（推荐）
3. 或手动将配置目录复制到新位置

### 数据清理

如果需要完全卸载应用并清理所有数据：

**Windows**：

```powershell
# 删除配置目录
Remove-Item -Recurse -Force "$env:APPDATA\.antigravity-agent"
```

**macOS / Linux**：

```bash
# 删除配置目录
rm -rf ~/.config/.antigravity-agent
```

**警告**：此操作会永久删除所有账户备份和设置，无法恢复！

### 磁盘空间管理

**查看配置目录大小**：

**Windows PowerShell**：

```powershell
Get-ChildItem "$env:APPDATA\.antigravity-agent" -Recurse | Measure-Object -Property Length -Sum
```

**macOS / Linux**：

```bash
du -sh ~/.config/.antigravity-agent
du -sh ~/.config/.antigravity-agent/*
```

**优化建议**：

1. 定期清理日志文件（保留最近 7-30 天）
2. 删除不再使用的账户备份
3. 每个账户只保留最新的备份


## Antigravity 路径配置

### 配置文件：`antigravity_path.json`

此文件存储用户自定义的 Antigravity 可执行文件路径。

### 配置结构

```json
{
  "custom_executable_path": "/path/to/antigravity"
}
```

### 配置选项说明

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `custom_executable_path` | string \| null | `null` | 自定义的 Antigravity 可执行文件路径 |

### 自动检测

默认情况下，Antigravity Agent 会自动检测系统中安装的 Antigravity：

**Windows**：
- `C:\Users\[用户名]\AppData\Local\Programs\antigravity\Antigravity.exe`
- `C:\Program Files\Antigravity\Antigravity.exe`
- 其他常见安装位置

**macOS**：
- `/Applications/Antigravity.app/Contents/MacOS/Antigravity`
- `~/Applications/Antigravity.app/Contents/MacOS/Antigravity`

**Linux**：
- `/usr/bin/antigravity`
- `/usr/local/bin/antigravity`
- `~/.local/bin/antigravity`

### 自定义路径

如果 Antigravity 安装在非标准位置，可以手动指定路径：

**通过界面设置**：

1. 打开 Antigravity Agent
2. 点击"设置" > "Antigravity 路径"
3. 点击"浏览"按钮选择可执行文件
4. 或直接输入完整路径
5. 点击"保存"

**通过配置文件设置**：

编辑 `antigravity_path.json`：

**Windows 示例**：

```json
{
  "custom_executable_path": "D:\\Programs\\Antigravity\\Antigravity.exe"
}
```

**macOS 示例**：

```json
{
  "custom_executable_path": "/Users/username/Applications/Antigravity.app/Contents/MacOS/Antigravity"
}
```

**Linux 示例**：

```json
{
  "custom_executable_path": "/opt/antigravity/bin/antigravity"
}
```

**注意**：Windows 路径中的反斜杠需要转义（使用 `\\`）。

### 路径验证

应用会验证指定的路径是否有效：

- 文件是否存在
- 是否为可执行文件
- 是否有执行权限

如果路径无效，应用会显示错误提示并回退到自动检测。

### 清除自定义路径

如果想恢复自动检测：

**方法 1：通过界面**

1. 打开设置对话框
2. 点击"重置为默认"
3. 应用会删除自定义路径配置

**方法 2：手动删除**

删除 `antigravity_path.json` 文件或将其内容改为：

```json
{
  "custom_executable_path": null
}
```

### 使用场景

**需要自定义路径的场景**：

- Antigravity 安装在非标准位置
- 使用便携版 Antigravity
- 多个 Antigravity 版本共存
- 使用开发版本或测试版本

**不需要自定义的场景**：

- 使用标准安装程序安装
- Antigravity 在默认位置
- 只有一个 Antigravity 版本


## 配置文件示例

### 完整配置示例

以下是一个完整的配置目录结构和文件内容示例：

```
.antigravity-agent/
├── app_settings.json
├── window_state.json
├── antigravity_path.json
├── backups/
│   ├── backup_alice_20251204_103045.json
│   └── backup_bob_20251203_154230.json
└── logs/
    ├── antigravity-agent.2025-12-03
    └── antigravity-agent.2025-12-04
```

### app_settings.json 示例

**默认配置**（推荐新用户）：

```json
{
  "system_tray_enabled": false,
  "silent_start_enabled": false
}
```

**启用所有功能**（推荐高级用户）：

```json
{
  "system_tray_enabled": true,
  "silent_start_enabled": true
}
```

**仅启用系统托盘**：

```json
{
  "system_tray_enabled": true,
  "silent_start_enabled": false
}
```

**仅启用静默启动**（不推荐）：

```json
{
  "system_tray_enabled": false,
  "silent_start_enabled": true
}
```

> **注意**：不建议仅启用静默启动而不启用系统托盘，因为这样应用启动后将无法访问。

### window_state.json 示例

**标准窗口状态**：

```json
{
  "x": 100,
  "y": 100,
  "width": 1200,
  "height": 800,
  "maximized": false
}
```

**最大化窗口状态**：

```json
{
  "x": 0,
  "y": 0,
  "width": 1920,
  "height": 1080,
  "maximized": true
}
```

**小窗口模式**：

```json
{
  "x": 200,
  "y": 200,
  "width": 800,
  "height": 600,
  "maximized": false
}
```

### antigravity_path.json 示例

**使用默认路径**（自动检测）：

```json
{
  "custom_executable_path": null
}
```

**Windows 自定义路径**：

```json
{
  "custom_executable_path": "D:\\Programs\\Antigravity\\Antigravity.exe"
}
```

**macOS 自定义路径**：

```json
{
  "custom_executable_path": "/Users/username/Applications/Antigravity.app/Contents/MacOS/Antigravity"
}
```

**Linux 自定义路径**：

```json
{
  "custom_executable_path": "/opt/antigravity/bin/antigravity"
}
```

### 账户备份文件示例

**backup_alice_20251204_103045.json**（简化版）：

```json
{
  "username": "alice",
  "email": "alice@example.com",
  "profile_url": "https://example.com/avatar.jpg",
  "api_key": "ag_xxxxxxxxxxxxxxxxxxxxx",
  "created_at": "2025-12-04T10:30:45Z",
  "user_settings": {
    "theme": "dark",
    "language": "zh-CN",
    "notifications": true
  },
  "auth_status": {
    "authenticated": true,
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

**注意**：实际备份文件包含更多字段和敏感信息，请妥善保管。

### 日志文件示例

**antigravity-agent.2025-12-04**（部分内容）：

```json
{"timestamp":"2025-12-04T10:30:00.123Z","level":"INFO","target":"app::startup","message":"🚀 启动 Antigravity Agent","fields":{"version":"1.0.3"}}
{"timestamp":"2025-12-04T10:30:00.456Z","level":"INFO","target":"app::startup","message":"🖥️ 系统信息","fields":{"os":"macos","arch":"aarch64"}}
{"timestamp":"2025-12-04T10:30:01.789Z","level":"INFO","target":"app::startup","message":"📁 配置目录已初始化"}
{"timestamp":"2025-12-04T10:35:00.123Z","level":"INFO","target":"account::switch","message":"切换账户","fields":{"from":"user1@example.com","to":"user2@example.com"}}
{"timestamp":"2025-12-04T10:35:01.456Z","level":"INFO","target":"account::backup","message":"备份账户成功","fields":{"email":"user2@example.com"}}
```


## 配置管理最佳实践

### 安全建议

#### 1. 保护配置目录

配置目录包含敏感信息（API 密钥、令牌），需要妥善保护：

- **不要分享**：不要将配置文件上传到公共代码仓库或分享给他人
- **定期备份**：定期备份到安全位置（加密的外部硬盘或云存储）
- **使用加密**：使用应用的"导出配置"功能时设置强密码

#### 2. 权限设置

确保配置目录只有当前用户可访问：

**Windows**：

1. 右键点击配置目录
2. 选择"属性" > "安全"
3. 确保只有当前用户有完全控制权限

**macOS / Linux**：

```bash
# 设置目录权限为仅当前用户可访问
chmod 700 ~/.config/.antigravity-agent

# 设置文件权限
chmod 600 ~/.config/.antigravity-agent/*.json
```

#### 3. 加密备份

传输或存储配置文件时使用加密：

**使用应用内置功能**：

1. 使用"导出配置"功能
2. 设置强密码（至少 12 位，包含大小写字母、数字和符号）
3. 导出的文件已加密，可安全传输

**使用系统加密工具**：

**Windows**：使用 BitLocker 或 7-Zip 加密

```powershell
# 使用 7-Zip 创建加密压缩包
7z a -p -mhe=on antigravity-backup.7z "$env:APPDATA\.antigravity-agent"
```

**macOS**：使用磁盘工具创建加密映像

```bash
# 创建加密 DMG
hdiutil create -encryption -srcfolder ~/.config/.antigravity-agent antigravity-backup.dmg
```

**Linux**：使用 GPG 加密

```bash
# 创建加密压缩包
tar -czf - ~/.config/.antigravity-agent | gpg -c > antigravity-backup.tar.gz.gpg
```

### 性能优化

#### 1. 定期清理日志

日志文件会随时间增长，影响性能和磁盘空间：

**建议清理策略**：

- 每月清理一次旧日志
- 保留最近 7-30 天的日志
- 如果磁盘空间紧张，可以只保留 3-7 天

**自动清理脚本**（Linux/macOS）：

```bash
#!/bin/bash
# 删除 30 天前的日志
find ~/.config/.antigravity-agent/logs/ -name "antigravity-agent.*" -mtime +30 -delete
```

将此脚本添加到 crontab 实现自动清理：

```bash
# 每周日凌晨 2 点执行清理
0 2 * * 0 /path/to/cleanup-logs.sh
```

#### 2. 管理备份数量

避免累积过多账户备份：

- 删除不再使用的账户备份
- 每个账户只保留最新的备份
- 定期检查备份目录大小

#### 3. 优化启动速度

- 如果不需要后台运行，禁用静默启动
- 减少备份文件数量可以加快启动速度
- 定期清理日志文件

### 故障排查

#### 1. 配置文件损坏

**症状**：应用启动失败或设置丢失

**解决方案**：

1. 关闭应用
2. 备份当前配置目录
3. 删除损坏的配置文件（如 `app_settings.json`）
4. 重启应用，会自动创建默认配置

**验证配置文件**：

使用 JSON 验证工具检查格式：

```bash
# 使用 jq 验证 JSON 格式
jq empty ~/.config/.antigravity-agent/app_settings.json
```

如果有错误，jq 会显示具体位置。

#### 2. 权限问题

**症状**：无法保存配置或创建备份

**解决方案**：

**Windows**：

1. 以管理员身份运行应用
2. 检查配置目录权限
3. 确保防病毒软件没有阻止访问

**macOS / Linux**：

```bash
# 修改目录权限
chmod -R 755 ~/.config/.antigravity-agent

# 修改所有者
chown -R $USER:$USER ~/.config/.antigravity-agent
```

#### 3. 配置目录不存在

**症状**：应用无法找到配置目录

**解决方案**：

1. 手动创建配置目录：

```bash
# macOS / Linux
mkdir -p ~/.config/.antigravity-agent

# Windows PowerShell
New-Item -ItemType Directory -Force -Path "$env:APPDATA\.antigravity-agent"
```

2. 确保环境变量正确设置
3. 检查磁盘空间是否充足

#### 4. 日志文件过大

**症状**：磁盘空间不足或应用性能下降

**解决方案**：

1. 清理旧日志文件
2. 检查是否有异常日志（大量错误）
3. 如果日志增长异常快，可能是应用出现问题，需要排查

### 版本升级

升级 Antigravity Agent 时的注意事项：

#### 1. 升级前备份

```bash
# 创建带时间戳的备份
cp -r ~/.config/.antigravity-agent ~/backup/antigravity-agent-$(date +%Y-%m-%d)
```

#### 2. 检查兼容性

- 查看版本更新说明
- 了解配置格式是否有变化
- 确认是否需要迁移配置

#### 3. 迁移配置

大多数情况下配置会自动迁移，但如果遇到问题：

1. 导出旧版本的配置
2. 安装新版本
3. 导入配置或手动迁移

### 多设备同步

如果在多台设备上使用 Antigravity Agent：

#### 1. 不建议直接同步配置目录

原因：

- 不同设备的路径可能不同
- 窗口状态不应同步
- 可能导致配置冲突

#### 2. 仅同步账户备份

可以同步 `backups/` 目录：

**使用云存储**：

```bash
# 创建符号链接到云存储
ln -s ~/Dropbox/antigravity-backups ~/.config/.antigravity-agent/backups
```

**手动复制**：

```bash
# 从设备 A 复制到设备 B
scp -r ~/.config/.antigravity-agent/backups/ user@deviceB:~/.config/.antigravity-agent/
```

#### 3. 使用导入导出功能（推荐）

这是最安全的同步方式：

1. 在设备 A 上导出配置
2. 将导出文件传输到设备 B
3. 在设备 B 上导入配置

### 配置模板

为不同使用场景创建配置模板：

#### 开发者模式

```json
{
  "system_tray_enabled": true,
  "silent_start_enabled": false
}
```

适合：需要频繁查看应用状态的开发者

#### 后台运行模式

```json
{
  "system_tray_enabled": true,
  "silent_start_enabled": true
}
```

适合：需要应用始终在后台运行的用户

#### 临时使用模式

```json
{
  "system_tray_enabled": false,
  "silent_start_enabled": false
}
```

适合：偶尔使用应用的用户

## 相关文档

- [使用手册](./user-guide.md) - 完整的功能说明和操作指南
- [API 参考](./api-reference.md) - Tauri 命令和接口文档
- [问题排查](../advanced/troubleshooting.md) - 常见问题和解决方案
- [开发指南](../development/development-guide.md) - 开发环境搭建和配置

## 获取帮助

如果您在配置过程中遇到问题：

1. 查看[问题排查手册](../advanced/troubleshooting.md)
2. 查看[常见问题 FAQ](../advanced/faq.md)
3. 在 GitHub 上提交 Issue
4. 联系技术支持

---

**最后更新**：2025-12-04  
**文档版本**：1.0.3
