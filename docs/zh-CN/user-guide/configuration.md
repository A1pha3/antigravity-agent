# 配置说明

本文档详细说明 Antigravity Agent 的配置文件位置、配置选项和数据存储位置。

## 目录

- [配置文件位置](#配置文件位置)
- [配置文件说明](#配置文件说明)
- [应用设置](#应用设置)
- [数据存储位置](#数据存储位置)
- [环境变量](#环境变量)
- [配置示例](#配置示例)

---

## 配置文件位置

Antigravity Agent 的配置文件存储在系统标准配置目录中，不同操作系统的位置如下：

### Windows

```
%APPDATA%\.antigravity-agent\
```

**完整路径示例:**

```
C:\Users\YourUsername\AppData\Roaming\.antigravity-agent\
```

**备用路径:**

如果 `%APPDATA%` 环境变量不可用，会依次尝试：

1. `%USERPROFILE%\AppData\Roaming\.antigravity-agent\`
2. `%USERPROFILE%\.config\.antigravity-agent\`

### macOS

```
~/.config/.antigravity-agent/
```

**完整路径示例:**

```
/Users/YourUsername/.config/.antigravity-agent/
```

### Linux

```
~/.config/.antigravity-agent/
```

**完整路径示例:**

```
/home/yourname/.config/.antigravity-agent/
```

---

## 配置文件说明

配置目录包含以下文件和子目录：

```
.antigravity-agent/
├── app_settings.json          # 应用程序设置
├── window_state.json          # 窗口状态（位置、大小）
├── antigravity_config.json    # Antigravity 路径配置
├── antigravity-accounts/      # 账户备份目录
│   ├── user1@example.com.json
│   ├── user2@example.com.json
│   └── ...
├── backups/                   # 通用备份目录
│   └── ...
└── logs/                      # 日志文件目录
    ├── antigravity-agent.log
    └── antigravity-agent.backup.log
```

---

## 应用设置

### app_settings.json

应用程序的核心设置文件，包含系统托盘和静默启动等配置。

**文件位置:**

- Windows: `%APPDATA%\.antigravity-agent\app_settings.json`
- macOS/Linux: `~/.config/.antigravity-agent/app_settings.json`

**配置结构:**

```json
{
  "system_tray_enabled": false,
  "silent_start_enabled": false
}
```

**配置项说明:**

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `system_tray_enabled` | boolean | `false` | 是否启用系统托盘功能 |
| `silent_start_enabled` | boolean | `false` | 是否启用静默启动（启动时最小化） |

**修改方式:**

1. **通过界面修改**（推荐）：在应用设置页面直接修改
2. **手动编辑**：关闭应用后编辑 JSON 文件，重启应用生效

**示例配置:**

```json
{
  "system_tray_enabled": true,
  "silent_start_enabled": true
}
```

---

### window_state.json

窗口状态配置文件，保存窗口的位置、大小和显示状态。

**文件位置:**

- Windows: `%APPDATA%\.antigravity-agent\window_state.json`
- macOS/Linux: `~/.config/.antigravity-agent/window_state.json`

**配置结构:**

```json
{
  "x": 100,
  "y": 100,
  "width": 1200,
  "height": 800,
  "maximized": false,
  "visible": true
}
```

**配置项说明:**

| 配置项 | 类型 | 说明 |
|--------|------|------|
| `x` | number | 窗口 X 坐标 |
| `y` | number | 窗口 Y 坐标 |
| `width` | number | 窗口宽度 |
| `height` | number | 窗口高度 |
| `maximized` | boolean | 是否最大化 |
| `visible` | boolean | 是否可见 |

**注意:** 此文件由应用自动管理，通常不需要手动编辑。

---

### antigravity_config.json

Antigravity 路径配置文件，保存自定义的 Antigravity 可执行文件路径。

**文件位置:**

- Windows: `%APPDATA%\.antigravity-agent\antigravity_config.json`
- macOS/Linux: `~/.config/.antigravity-agent/antigravity_config.json`

**配置结构:**

```json
{
  "executable_path": "/path/to/antigravity"
}
```

**配置项说明:**

| 配置项 | 类型 | 说明 |
|--------|------|------|
| `executable_path` | string | Antigravity 可执行文件的完整路径 |

**修改方式:**

1. **通过界面修改**（推荐）：在设置页面配置 Antigravity 路径
2. **手动编辑**：编辑 JSON 文件，填入正确的可执行文件路径

**示例配置:**

```json
{
  "executable_path": "C:\\Program Files\\Antigravity\\antigravity.exe"
}
```

---

## 数据存储位置

### 账户备份目录

**路径:**

- Windows: `%APPDATA%\.antigravity-agent\antigravity-accounts\`
- macOS/Linux: `~/.config/.antigravity-agent/antigravity-accounts/`

**说明:**

存储所有备份的 Antigravity 账户数据，每个账户一个 JSON 文件。

**文件命名规则:**

```
{email}.json
```

例如：`user@example.com.json`

**文件内容示例:**

```json
{
  "account_email": "user@example.com",
  "backup_time": "2025-12-04 10:30:00",
  "antigravityAuthStatus": "{\"email\":\"user@example.com\",\"apiKey\":\"...\"}",
  "antigravityUserSettings.allUserSettings": "...",
  "antigravity.profileUrl": "data:image/png;base64,..."
}
```

---

### 日志文件目录

**路径:**

- Windows: `%APPDATA%\.antigravity-agent\logs\`
- macOS/Linux: `~/.config/.antigravity-agent/logs/`

**日志文件:**

| 文件名 | 说明 |
|--------|------|
| `antigravity-agent.log` | 当前日志文件（JSON 格式） |
| `antigravity-agent.backup.log` | 备份日志文件 |

**日志格式:**

日志文件使用 JSON 格式，每行一个 JSON 对象：

```json
{"timestamp":"2025-12-04T10:30:00.123Z","level":"INFO","target":"app::startup","message":"启动 Antigravity Agent"}
{"timestamp":"2025-12-04T10:30:01.456Z","level":"INFO","target":"account::switch","message":"切换账户成功"}
```

**日志级别:**

- `ERROR`: 错误信息
- `WARN`: 警告信息
- `INFO`: 一般信息
- `DEBUG`: 调试信息
- `TRACE`: 详细跟踪信息

**日志轮转:**

- 日志文件按天轮转（每天创建新文件）
- 旧日志文件自动归档
- 可通过应用界面清空日志

---

### 通用备份目录

**路径:**

- Windows: `%APPDATA%\.antigravity-agent\backups\`
- macOS/Linux: `~/.config/.antigravity-agent/backups/`

**说明:**

存储通用配置文件的 ZIP 备份（如果使用 `backup_profile` 命令）。

---

## 环境变量

Antigravity Agent 支持以下环境变量：

### APPDATA (Windows)

指定应用数据目录，默认为 `%USERPROFILE%\AppData\Roaming`。

**示例:**

```cmd
set APPDATA=D:\MyAppData
```

### HOME (macOS/Linux)

指定用户主目录，默认为 `~`。

**示例:**

```bash
export HOME=/custom/home
```

---

## 配置示例

### 启用系统托盘和静默启动

**app_settings.json:**

```json
{
  "system_tray_enabled": true,
  "silent_start_enabled": true
}
```

**效果:**

- 应用启动时会最小化到系统托盘
- 关闭窗口时不会退出应用，而是最小化到托盘
- 可通过托盘图标快速访问应用

---

### 配置自定义 Antigravity 路径

**antigravity_config.json:**

```json
{
  "executable_path": "C:\\CustomPath\\antigravity.exe"
}
```

**适用场景:**

- Antigravity 安装在非标准位置
- 使用便携版 Antigravity
- 多个 Antigravity 版本共存

---

### 自定义窗口大小和位置

**window_state.json:**

```json
{
  "x": 200,
  "y": 100,
  "width": 1400,
  "height": 900,
  "maximized": false,
  "visible": true
}
```

**注意:** 应用会自动保存窗口状态，通常不需要手动配置。

---

## 配置文件管理

### 备份配置

建议定期备份整个配置目录：

**Windows:**

```cmd
xcopy "%APPDATA%\.antigravity-agent" "D:\Backup\.antigravity-agent" /E /I /Y
```

**macOS/Linux:**

```bash
cp -r ~/.config/.antigravity-agent ~/backup/.antigravity-agent
```

---

### 恢复配置

从备份恢复配置：

**Windows:**

```cmd
xcopy "D:\Backup\.antigravity-agent" "%APPDATA%\.antigravity-agent" /E /I /Y
```

**macOS/Linux:**

```bash
cp -r ~/backup/.antigravity-agent ~/.config/.antigravity-agent
```

---

### 重置配置

删除配置目录以重置所有设置：

**Windows:**

```cmd
rmdir /S /Q "%APPDATA%\.antigravity-agent"
```

**macOS/Linux:**

```bash
rm -rf ~/.config/.antigravity-agent
```

**警告:** 此操作会删除所有配置和备份数据，无法恢复！

---

## 故障排查

### 配置文件损坏

**症状:** 应用启动失败或设置丢失

**解决方案:**

1. 关闭应用
2. 删除损坏的配置文件（如 `app_settings.json`）
3. 重启应用，会自动创建默认配置

---

### 权限问题

**症状:** 无法保存配置或创建备份

**解决方案:**

1. 检查配置目录权限
2. 以管理员身份运行应用（Windows）
3. 修改目录权限（macOS/Linux）：

```bash
chmod -R 755 ~/.config/.antigravity-agent
```

---

### 配置目录不存在

**症状:** 应用无法找到配置目录

**解决方案:**

1. 手动创建配置目录
2. 确保环境变量正确设置
3. 检查磁盘空间是否充足

---

## 相关文档

- [使用手册](./user-guide.md) - 功能使用说明
- [API 参考](./api-reference.md) - 命令接口文档
- [故障排查](../advanced/troubleshooting.md) - 常见问题解决
- [开发指南](../development/development-guide.md) - 开发环境配置

---

## 更新日志

- **v1.0.3** (2025-12-04): 完善配置说明文档
- **v1.0.0** (2024-12-01): 初始版本

