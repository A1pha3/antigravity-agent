# API 参考文档

本文档详细列出了 Antigravity Agent 所有可用的 Tauri 命令接口，包括功能说明、参数、返回值和使用示例。

## 目录

- [账户管理命令](#账户管理命令)
- [备份管理命令](#备份管理命令)
- [进程管理命令](#进程管理命令)
- [平台支持命令](#平台支持命令)
- [系统托盘命令](#系统托盘命令)
- [应用设置命令](#应用设置命令)
- [日志管理命令](#日志管理命令)
- [数据库监控命令](#数据库监控命令)
- [语言服务器命令](#语言服务器命令)

## 命令概览

Antigravity Agent 提供了 50+ 个 Tauri 命令，涵盖账户管理、备份恢复、进程控制、系统集成等核心功能。所有命令都通过 Tauri 的 `invoke` API 调用。

### 调用方式

```typescript
import { invoke } from '@tauri-apps/api/core';

// 基本调用
const result = await invoke('command_name', { param1: value1 });

// 错误处理
try {
  const result = await invoke('command_name', { param1: value1 });
  console.log('成功:', result);
} catch (error) {
  console.error('失败:', error);
}
```

---

## 账户管理命令

账户管理命令负责 Antigravity 账户的切换、备份、恢复和清除操作。


### switch_antigravity_account

切换到指定的 Antigravity 账户（已弃用，建议使用 `switch_to_antigravity_account`）。

**参数:**

- `account_id` (string): 账户 ID

**返回值:**

- `Result<String, String>`: 成功返回切换信息，失败返回错误消息

**示例:**

```typescript
const result = await invoke('switch_antigravity_account', {
  accountId: 'account_user@example.com'
});
```

**源代码:** [src-tauri/src/commands/account_commands.rs](../../../src-tauri/src/commands/account_commands.rs)

---

### get_antigravity_accounts

获取所有已备份的 Antigravity 账户列表。

**参数:** 无

**返回值:**

- `Result<Vec<AntigravityAccount>, String>`: 账户列表，按最后切换时间降序排列

**AntigravityAccount 结构:**

```typescript
interface AntigravityAccount {
  id: string;           // 账户 ID
  name: string;         // 用户名
  email: string;        // 邮箱地址
  api_key: string;      // API 密钥
  profile_url: string;  // 头像 URL
  user_settings: string; // 用户设置 JSON
  created_at: string;   // 创建时间
  last_switched: string; // 最后切换时间
}
```

**示例:**

```typescript
const accounts = await invoke('get_antigravity_accounts');
console.log('账户列表:', accounts);
```

**源代码:** [src-tauri/src/commands/account_commands.rs](../../../src-tauri/src/commands/account_commands.rs)

---

### get_current_antigravity_info

获取当前登录的 Antigravity 账户信息。

**参数:** 无

**返回值:**

- `Result<Value, String>`: 包含认证信息的 JSON 对象

**返回数据结构:**

```typescript
interface CurrentAntigravityInfo {
  email?: string;      // 邮箱
  name?: string;       // 用户名
  apiKey?: string;     // API 密钥
  db_path: string;     // 数据库路径
  // ... 其他认证信息
}
```

**示例:**

```typescript
const info = await invoke('get_current_antigravity_info');
console.log('当前用户:', info.email);
```

**源代码:** [src-tauri/src/commands/account_commands.rs](../../../src-tauri/src/commands/account_commands.rs)

---

### backup_antigravity_current_account

备份当前登录的 Antigravity 账户。使用智能备份策略，自动处理重复备份。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回备份信息，失败返回错误消息

**示例:**

```typescript
const result = await invoke('backup_antigravity_current_account');
console.log(result); // "Antigravity 账户 'user@example.com' 备份成功"
```

**源代码:** [src-tauri/src/commands/account_commands.rs](../../../src-tauri/src/commands/account_commands.rs)

---

### restore_antigravity_account

恢复指定的 Antigravity 账户数据到数据库。

**参数:**

- `account_name` (string): 账户名称（不含 .json 扩展名）

**返回值:**

- `Result<String, String>`: 成功返回恢复信息，失败返回错误消息

**示例:**

```typescript
const result = await invoke('restore_antigravity_account', {
  accountName: 'user@example.com'
});
```

**源代码:** [src-tauri/src/commands/account_commands.rs](../../../src-tauri/src/commands/account_commands.rs)

---

### switch_to_antigravity_account

切换到指定账户（推荐使用）。自动执行：关闭进程 → 恢复账户数据 → 重启进程。

**参数:**

- `account_name` (string): 账户名称

**返回值:**

- `Result<String, String>`: 包含所有步骤执行结果的消息

**示例:**

```typescript
const result = await invoke('switch_to_antigravity_account', {
  accountName: 'user@example.com'
});
console.log(result);
```

**源代码:** [src-tauri/src/commands/account_commands.rs](../../../src-tauri/src/commands/account_commands.rs)

---

### clear_all_antigravity_data

清除所有 Antigravity 数据（彻底注销）。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回清除信息，失败返回错误消息

**警告:** 此操作会删除所有 Antigravity 数据，无法恢复！

**示例:**

```typescript
const result = await invoke('clear_all_antigravity_data');
```

**源代码:** [src-tauri/src/commands/account_commands.rs](../../../src-tauri/src/commands/account_commands.rs)

---

## 备份管理命令

备份管理命令负责配置文件和账户的备份、恢复、删除等操作。


### backup_profile

创建配置文件备份（ZIP 格式）。

**参数:**

- `name` (string): 备份名称
- `source_path` (string): 源路径
- `state` (State): 应用状态（自动注入）

**返回值:**

- `Result<String, String>`: 成功返回备份文件路径

**示例:**

```typescript
const result = await invoke('backup_profile', {
  name: 'my-backup',
  sourcePath: '/path/to/config'
});
```

**源代码:** [src-tauri/src/commands/backup_commands.rs](../../../src-tauri/src/commands/backup_commands.rs)

---

### restore_profile

从 ZIP 备份恢复配置文件。

**参数:**

- `name` (string): 备份名称
- `target_path` (string): 目标路径
- `state` (State): 应用状态（自动注入）

**返回值:**

- `Result<String, String>`: 成功返回恢复信息

**示例:**

```typescript
const result = await invoke('restore_profile', {
  name: 'my-backup',
  targetPath: '/path/to/restore'
});
```

**源代码:** [src-tauri/src/commands/backup_commands.rs](../../../src-tauri/src/commands/backup_commands.rs)

---

### get_recent_accounts

获取最近使用的账户列表（按文件修改时间排序）。

**参数:**

- `limit` (number, 可选): 返回数量限制

**返回值:**

- `Result<Vec<String>, String>`: 账户名称列表

**示例:**

```typescript
const accounts = await invoke('get_recent_accounts', { limit: 5 });
console.log('最近使用:', accounts);
```

**源代码:** [src-tauri/src/commands/backup_commands.rs](../../../src-tauri/src/commands/backup_commands.rs)

---

### collect_backup_contents

收集所有备份文件的完整内容。

**参数:** 无

**返回值:**

- `Result<Vec<BackupData>, String>`: 备份数据列表

**BackupData 结构:**

```typescript
interface BackupData {
  filename: string;    // 文件名
  content: any;        // JSON 内容
  timestamp: number;   // 时间戳
}
```

**示例:**

```typescript
const backups = await invoke('collect_backup_contents');
```

**源代码:** [src-tauri/src/commands/backup_commands.rs](../../../src-tauri/src/commands/backup_commands.rs)

---

### restore_backup_files

批量恢复备份文件到本地。

**参数:**

- `backups` (Vec<BackupData>): 备份数据列表

**返回值:**

- `Result<RestoreResult, String>`: 恢复结果

**RestoreResult 结构:**

```typescript
interface RestoreResult {
  restoredCount: number;  // 成功恢复数量
  failed: Array<{         // 失败列表
    filename: string;
    error: string;
  }>;
}
```

**示例:**

```typescript
const result = await invoke('restore_backup_files', { backups });
console.log(`成功恢复 ${result.restoredCount} 个文件`);
```

**源代码:** [src-tauri/src/commands/backup_commands.rs](../../../src-tauri/src/commands/backup_commands.rs)

---

### delete_backup

删除指定的备份文件。

**参数:**

- `name` (string): 备份名称

**返回值:**

- `Result<String, String>`: 成功返回删除信息

**示例:**

```typescript
const result = await invoke('delete_backup', {
  name: 'user@example.com'
});
```

**源代码:** [src-tauri/src/commands/backup_commands.rs](../../../src-tauri/src/commands/backup_commands.rs)

---

### clear_all_backups

清空所有备份文件。

**参数:** 无

**返回值:**

- `Result<String, String>`: 返回删除统计信息

**示例:**

```typescript
const result = await invoke('clear_all_backups');
console.log(result); // "已清空所有用户备份，共删除 5 个文件"
```

**源代码:** [src-tauri/src/commands/backup_commands.rs](../../../src-tauri/src/commands/backup_commands.rs)

---

## 进程管理命令

进程管理命令负责 Antigravity 进程的启动、关闭、重启等操作。


### kill_antigravity

关闭所有 Antigravity 相关进程。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回关闭信息

**示例:**

```typescript
const result = await invoke('kill_antigravity');
console.log(result); // "已关闭 2 个 Antigravity 进程"
```

**源代码:** [src-tauri/src/commands/process_commands.rs](../../../src-tauri/src/commands/process_commands.rs)

---

### start_antigravity

启动 Antigravity 应用程序。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回启动信息

**示例:**

```typescript
const result = await invoke('start_antigravity');
console.log(result); // "Antigravity 启动成功"
```

**注意:** 需要先配置 Antigravity 可执行文件路径。

**源代码:** [src-tauri/src/commands/process_commands.rs](../../../src-tauri/src/commands/process_commands.rs)

---

### is_antigravity_running

检查 Antigravity 进程是否正在运行。

**参数:** 无

**返回值:**

- `bool`: true 表示正在运行，false 表示未运行

**示例:**

```typescript
const isRunning = await invoke('is_antigravity_running');
if (isRunning) {
  console.log('Antigravity 正在运行');
}
```

**源代码:** [src-tauri/src/commands/process_commands.rs](../../../src-tauri/src/commands/process_commands.rs)

---

### list_antigravity_processes

列出所有 Antigravity 相关进程（用于调试）。

**参数:** 无

**返回值:**

- `Result<Vec<Value>, String>`: 进程信息列表

**进程信息结构:**

```typescript
interface ProcessInfo {
  pid: string;              // 进程 ID
  name: string;             // 进程名称
  command: string;          // 完整命令
  matched_pattern: number;  // 匹配的模式索引
  pattern_description: string; // 模式描述
}
```

**示例:**

```typescript
const processes = await invoke('list_antigravity_processes');
console.log('找到进程:', processes.length);
```

**源代码:** [src-tauri/src/commands/process_commands.rs](../../../src-tauri/src/commands/process_commands.rs)

---

### backup_and_restart_antigravity

备份当前账户并重启 Antigravity（彻底注销并重新登录）。

**执行步骤:**

1. 关闭 Antigravity 进程
2. 备份当前账户信息
3. 清除所有 Antigravity 数据
4. 重新启动 Antigravity

**参数:** 无

**返回值:**

- `Result<String, String>`: 包含所有步骤执行结果的消息

**示例:**

```typescript
const result = await invoke('backup_and_restart_antigravity');
console.log(result);
```

**源代码:** [src-tauri/src/commands/process_commands.rs](../../../src-tauri/src/commands/process_commands.rs)

---

## 平台支持命令

平台支持命令负责获取平台信息、检测安装位置等跨平台操作。

### get_platform_info

获取当前平台的详细信息。

**参数:** 无

**返回值:**

- `Result<Value, String>`: 平台信息 JSON 对象

**返回数据结构:**

```typescript
interface PlatformInfo {
  os: string;                    // 操作系统 (windows/macos/linux)
  arch: string;                  // 架构 (x86_64/aarch64)
  family: string;                // 系统家族 (unix/windows)
  antigravity_available: boolean; // Antigravity 是否可用
  antigravity_paths: string[];   // 可能的数据库路径
  config_dir: string;            // 配置目录
  data_dir: string;              // 数据目录
  home_dir: string;              // 用户主目录
}
```

**示例:**

```typescript
const info = await invoke('get_platform_info');
console.log('操作系统:', info.os);
console.log('Antigravity 可用:', info.antigravity_available);
```

**源代码:** [src-tauri/src/commands/platform_commands.rs](../../../src-tauri/src/commands/platform_commands.rs)

---

### find_antigravity_installations

查找系统中所有可能的 Antigravity 安装位置。

**参数:** 无

**返回值:**

- `Result<Vec<String>, String>`: 安装路径列表

**示例:**

```typescript
const paths = await invoke('find_antigravity_installations');
console.log('找到安装位置:', paths);
```

**源代码:** [src-tauri/src/commands/platform_commands.rs](../../../src-tauri/src/commands/platform_commands.rs)

---

### detect_antigravity_installation

自动检测 Antigravity 数据库安装状态。

**参数:** 无

**返回值:**

- `Result<Value, String>`: 检测结果

**返回数据结构:**

```typescript
interface InstallationInfo {
  found: boolean;        // 是否找到
  path: string | null;   // 数据目录路径
  isCustomPath: boolean; // 是否为自定义路径
}
```

**示例:**

```typescript
const info = await invoke('detect_antigravity_installation');
if (info.found) {
  console.log('数据目录:', info.path);
}
```

**源代码:** [src-tauri/src/commands/platform_commands.rs](../../../src-tauri/src/commands/platform_commands.rs)

---

### detect_antigravity_executable

自动检测 Antigravity 可执行文件位置。

**参数:** 无

**返回值:**

- `Result<Value, String>`: 检测结果

**返回数据结构:**

```typescript
interface ExecutableInfo {
  found: boolean;        // 是否找到
  path: string | null;   // 可执行文件路径
  isCustomPath: boolean; // 是否为自定义路径
}
```

**示例:**

```typescript
const info = await invoke('detect_antigravity_executable');
if (info.found) {
  console.log('可执行文件:', info.path);
}
```

**源代码:** [src-tauri/src/commands/platform_commands.rs](../../../src-tauri/src/commands/platform_commands.rs)

---

### validate_antigravity_executable

验证指定路径是否为有效的 Antigravity 可执行文件。

**参数:**

- `path` (string): 可执行文件路径

**返回值:**

- `Result<bool, String>`: true 表示有效

**示例:**

```typescript
const isValid = await invoke('validate_antigravity_executable', {
  path: '/path/to/antigravity'
});
```

**源代码:** [src-tauri/src/commands/platform_commands.rs](../../../src-tauri/src/commands/platform_commands.rs)

---

### save_antigravity_executable

保存用户自定义的 Antigravity 可执行文件路径。

**参数:**

- `path` (string): 可执行文件路径

**返回值:**

- `Result<String, String>`: 成功返回保存信息

**示例:**

```typescript
const result = await invoke('save_antigravity_executable', {
  path: '/custom/path/to/antigravity'
});
```

**源代码:** [src-tauri/src/commands/platform_commands.rs](../../../src-tauri/src/commands/platform_commands.rs)

---

### get_current_paths

获取当前配置的所有路径信息。

**参数:** 无

**返回值:**

- `Result<Value, String>`: 路径信息

**返回数据结构:**

```typescript
interface CurrentPaths {
  executablePath: string | null; // 可执行文件路径
}
```

**示例:**

```typescript
const paths = await invoke('get_current_paths');
console.log('可执行文件:', paths.executablePath);
```

**源代码:** [src-tauri/src/commands/platform_commands.rs](../../../src-tauri/src/commands/platform_commands.rs)

---

## 系统托盘命令

系统托盘命令负责系统托盘的启用、禁用和窗口管理。


### enable_system_tray

启用系统托盘功能。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回 "系统托盘已启用"

**示例:**

```typescript
const result = await invoke('enable_system_tray');
```

**源代码:** [src-tauri/src/commands/tray_commands.rs](../../../src-tauri/src/commands/tray_commands.rs)

---

### disable_system_tray

禁用系统托盘功能。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回 "系统托盘已禁用"

**示例:**

```typescript
const result = await invoke('disable_system_tray');
```

**源代码:** [src-tauri/src/commands/tray_commands.rs](../../../src-tauri/src/commands/tray_commands.rs)

---

### toggle_system_tray

切换系统托盘状态（启用/禁用）。

**参数:** 无

**返回值:**

- `Result<Value, String>`: 包含新状态的 JSON 对象

**返回数据结构:**

```typescript
interface ToggleResult {
  enabled: boolean;  // 当前状态
  message: string;   // 状态消息
}
```

**示例:**

```typescript
const result = await invoke('toggle_system_tray');
console.log('托盘状态:', result.enabled);
```

**源代码:** [src-tauri/src/commands/tray_commands.rs](../../../src-tauri/src/commands/tray_commands.rs)

---

### get_system_tray_state

获取系统托盘当前状态。

**参数:** 无

**返回值:**

- `Result<bool, String>`: true 表示已启用

**示例:**

```typescript
const enabled = await invoke('get_system_tray_state');
```

**源代码:** [src-tauri/src/commands/tray_commands.rs](../../../src-tauri/src/commands/tray_commands.rs)

---

### is_system_tray_enabled

检查系统托盘是否启用（兼容旧接口）。

**参数:** 无

**返回值:**

- `Result<bool, String>`: true 表示已启用

**示例:**

```typescript
const enabled = await invoke('is_system_tray_enabled');
```

**源代码:** [src-tauri/src/commands/tray_commands.rs](../../../src-tauri/src/commands/tray_commands.rs)

---

### save_system_tray_state

保存系统托盘状态。

**参数:**

- `enabled` (bool): 是否启用

**返回值:**

- `Result<String, String>`: 成功返回 "状态已保存"

**示例:**

```typescript
const result = await invoke('save_system_tray_state', {
  enabled: true
});
```

**源代码:** [src-tauri/src/commands/tray_commands.rs](../../../src-tauri/src/commands/tray_commands.rs)

---

### minimize_to_tray

最小化窗口到系统托盘。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回 "已最小化到托盘"

**示例:**

```typescript
const result = await invoke('minimize_to_tray');
```

**源代码:** [src-tauri/src/commands/tray_commands.rs](../../../src-tauri/src/commands/tray_commands.rs)

---

### restore_from_tray

从系统托盘恢复窗口。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回 "已恢复窗口"

**示例:**

```typescript
const result = await invoke('restore_from_tray');
```

**源代码:** [src-tauri/src/commands/tray_commands.rs](../../../src-tauri/src/commands/tray_commands.rs)

---

## 应用设置命令

应用设置命令负责应用程序配置的管理和存储。

### is_silent_start_enabled

获取静默启动状态。

**参数:** 无

**返回值:**

- `Result<bool, String>`: true 表示已启用静默启动

**示例:**

```typescript
const enabled = await invoke('is_silent_start_enabled');
```

**源代码:** [src-tauri/src/commands/settings_commands.rs](../../../src-tauri/src/commands/settings_commands.rs)

---

### save_silent_start_state

保存静默启动状态。

**参数:**

- `enabled` (bool): 是否启用静默启动

**返回值:**

- `Result<String, String>`: 成功返回状态消息

**示例:**

```typescript
const result = await invoke('save_silent_start_state', {
  enabled: true
});
console.log(result); // "静默启动已启用"
```

**源代码:** [src-tauri/src/commands/settings_commands.rs](../../../src-tauri/src/commands/settings_commands.rs)

---

### get_all_settings

获取所有应用设置。

**参数:** 无

**返回值:**

- `Result<Value, String>`: 包含所有设置的 JSON 对象

**返回数据结构:**

```typescript
interface AppSettings {
  system_tray_enabled: boolean;   // 系统托盘是否启用
  silent_start_enabled: boolean;  // 静默启动是否启用
}
```

**示例:**

```typescript
const settings = await invoke('get_all_settings');
console.log('系统托盘:', settings.system_tray_enabled);
console.log('静默启动:', settings.silent_start_enabled);
```

**源代码:** [src-tauri/src/commands/settings_commands.rs](../../../src-tauri/src/commands/settings_commands.rs)

---

## 日志管理命令

日志管理命令提供日志查看、清理和加密导入导出功能。


### get_log_info

获取日志文件信息（路径、大小、修改时间等）。

**参数:** 无

**返回值:**

- `Result<LogInfo, String>`: 日志文件信息

**LogInfo 结构:**

```typescript
interface LogInfo {
  exists: boolean;       // 文件是否存在
  path: string;          // 文件路径
  size_bytes: number;    // 文件大小（字节）
  size_human: string;    // 人类可读的大小
  last_modified: string; // 最后修改时间
}
```

**示例:**

```typescript
const info = await invoke('get_log_info');
console.log('日志大小:', info.size_human);
console.log('最后修改:', info.last_modified);
```

**源代码:** [src-tauri/src/commands/logging_commands.rs](../../../src-tauri/src/commands/logging_commands.rs)

---

### clear_logs

清空日志文件内容（会自动备份）。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回 "日志文件已清空"

**示例:**

```typescript
const result = await invoke('clear_logs');
```

**源代码:** [src-tauri/src/commands/logging_commands.rs](../../../src-tauri/src/commands/logging_commands.rs)

---

### write_text_file

写入文本文件到指定路径。

**参数:**

- `path` (string): 文件路径
- `content` (string): 文件内容

**返回值:**

- `Result<String, String>`: 成功返回 "文件写入成功"

**示例:**

```typescript
const result = await invoke('write_text_file', {
  path: '/path/to/file.txt',
  content: 'Hello, World!'
});
```

**源代码:** [src-tauri/src/commands/logging_commands.rs](../../../src-tauri/src/commands/logging_commands.rs)

---

### decrypt_config_data

解密配置数据文件。

**参数:**

- `file_path` (string): 加密文件路径
- `password` (string): 解密密码

**返回值:**

- `Result<String, String>`: 成功返回解密后的 JSON 字符串

**加密算法:** XOR + Base64

**示例:**

```typescript
const decrypted = await invoke('decrypt_config_data', {
  filePath: '/path/to/encrypted.txt',
  password: 'my-password'
});
const data = JSON.parse(decrypted);
```

**源代码:** [src-tauri/src/commands/logging_commands.rs](../../../src-tauri/src/commands/logging_commands.rs)

---

### encrypt_config_data

加密配置数据。

**参数:**

- `json_data` (string): JSON 字符串
- `password` (string): 加密密码

**返回值:**

- `Result<String, String>`: 成功返回 Base64 编码的加密字符串

**加密算法:** XOR + Base64

**示例:**

```typescript
const jsonData = JSON.stringify({ key: 'value' });
const encrypted = await invoke('encrypt_config_data', {
  jsonData,
  password: 'my-password'
});
```

**源代码:** [src-tauri/src/commands/logging_commands.rs](../../../src-tauri/src/commands/logging_commands.rs)

---

### write_frontend_log

写入前端日志到统一日志系统（自动脱敏处理）。

**参数:**

- `log_entry` (Value): 日志条目 JSON 对象

**日志条目结构:**

```typescript
interface LogEntry {
  level: 'info' | 'warn' | 'error'; // 日志级别
  message: string;                   // 日志消息
  details?: string;                  // 详细信息
  sessionId?: string;                // 会话 ID
}
```

**返回值:**

- `Result<(), String>`: 成功返回空

**示例:**

```typescript
await invoke('write_frontend_log', {
  logEntry: {
    level: 'info',
    message: '用户登录成功',
    sessionId: 'session-123'
  }
});
```

**源代码:** [src-tauri/src/commands/logging_commands.rs](../../../src-tauri/src/commands/logging_commands.rs)

---

## 数据库监控命令

数据库监控命令提供 SQLite 数据库的实时监控功能。

### is_database_monitoring_running

获取数据库监控运行状态。

**参数:** 无

**返回值:**

- `Result<bool, String>`: 智能监控默认启用，总是返回 true

**示例:**

```typescript
const isRunning = await invoke('is_database_monitoring_running');
```

**源代码:** [src-tauri/src/commands/db_monitor_commands.rs](../../../src-tauri/src/commands/db_monitor_commands.rs)

---

### start_database_monitoring

手动启动数据库监控。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回 "数据库监控已启动"

**示例:**

```typescript
const result = await invoke('start_database_monitoring');
```

**源代码:** [src-tauri/src/commands/db_monitor_commands.rs](../../../src-tauri/src/commands/db_monitor_commands.rs)

---

### stop_database_monitoring

手动停止数据库监控。

**参数:** 无

**返回值:**

- `Result<String, String>`: 成功返回 "数据库监控已停止"

**示例:**

```typescript
const result = await invoke('stop_database_monitoring');
```

**源代码:** [src-tauri/src/commands/db_monitor_commands.rs](../../../src-tauri/src/commands/db_monitor_commands.rs)

---

## 语言服务器命令

语言服务器命令提供与 Antigravity 语言服务器的交互功能。

### language_server_get_user_status

获取语言服务器用户状态信息。

**参数:**

- `api_key` (string): API 密钥

**返回值:**

- `Result<Value, String>`: 用户状态 JSON 对象

**示例:**

```typescript
const status = await invoke('language_server_get_user_status', {
  apiKey: 'your-api-key'
});
console.log('用户状态:', status);
```

**源代码:** [src-tauri/src/language_server/commands.rs](../../../src-tauri/src/language_server/commands.rs)

---

### initialize_language_server_cache

初始化语言服务器缓存（预热）。

**参数:** 无

**返回值:**

- `Result<CacheInitResult, String>`: 初始化结果

**CacheInitResult 结构:**

```typescript
interface CacheInitResult {
  message: string;  // 初始化消息
}
```

**示例:**

```typescript
const result = await invoke('initialize_language_server_cache');
console.log(result.message);
```

**源代码:** [src-tauri/src/language_server/commands.rs](../../../src-tauri/src/language_server/commands.rs)

---

### clear_all_cache_command

清空所有语言服务器缓存。

**参数:** 无

**返回值:**

- `Result<(), String>`: 成功返回空

**示例:**

```typescript
await invoke('clear_all_cache_command');
console.log('缓存已清空');
```

**源代码:** [src-tauri/src/language_server/commands.rs](../../../src-tauri/src/language_server/commands.rs)

---

### get_cache_stats_command

获取缓存统计信息。

**参数:** 无

**返回值:**

- `Result<CacheStats, String>`: 缓存统计信息

**CacheStats 结构:**

```typescript
interface CacheStats {
  csrf_cache_size: number;  // CSRF 缓存大小
  ports_cache_size: number; // 端口缓存大小
}
```

**示例:**

```typescript
const stats = await invoke('get_cache_stats_command');
console.log('CSRF 缓存:', stats.csrf_cache_size);
console.log('端口缓存:', stats.ports_cache_size);
```

**源代码:** [src-tauri/src/language_server/commands.rs](../../../src-tauri/src/language_server/commands.rs)

---

## 错误处理

所有命令都遵循统一的错误处理模式：

### 错误类型

```typescript
// 成功返回
Result<T, String>

// 错误返回
String // 错误消息
```

### 错误处理示例

```typescript
try {
  const result = await invoke('command_name', { param: value });
  console.log('成功:', result);
} catch (error) {
  console.error('失败:', error);
  // 显示错误提示给用户
  alert(`操作失败: ${error}`);
}
```

### 常见错误

- `"未找到Antigravity安装位置"` - Antigravity 未安装或路径配置错误
- `"连接数据库失败"` - 数据库文件不存在或权限不足
- `"未检测到已登录用户"` - 当前没有登录的 Antigravity 账户
- `"解密后的数据不是有效的JSON格式"` - 解密密码错误
- `"文件内容为空"` - 尝试读取空文件

---

## 最佳实践

### 1. 错误处理

始终使用 try-catch 包裹命令调用：

```typescript
async function safeInvoke<T>(command: string, args?: any): Promise<T | null> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    console.error(`命令 ${command} 失败:`, error);
    return null;
  }
}
```

### 2. 状态检查

在执行操作前检查状态：

```typescript
// 检查 Antigravity 是否运行
const isRunning = await invoke('is_antigravity_running');
if (!isRunning) {
  console.log('Antigravity 未运行，正在启动...');
  await invoke('start_antigravity');
}
```

### 3. 批量操作

使用 Promise.all 并行执行独立操作：

```typescript
const [accounts, settings, platformInfo] = await Promise.all([
  invoke('get_antigravity_accounts'),
  invoke('get_all_settings'),
  invoke('get_platform_info')
]);
```

### 4. 超时处理

为长时间运行的命令添加超时：

```typescript
function invokeWithTimeout<T>(
  command: string,
  args?: any,
  timeout = 30000
): Promise<T> {
  return Promise.race([
    invoke<T>(command, args),
    new Promise<T>((_, reject) =>
      setTimeout(() => reject(new Error('操作超时')), timeout)
    )
  ]);
}
```

---

## 相关文档

- [使用手册](./user-guide.md) - 功能使用说明
- [配置说明](./configuration.md) - 配置文件详解
- [开发指南](../development/development-guide.md) - 开发环境搭建
- [架构设计](../development/architecture.md) - 系统架构说明

---

## 更新日志

- **v1.0.3** (2025-12-04): 完善 API 文档，添加所有命令说明
- **v1.0.0** (2024-12-01): 初始版本

