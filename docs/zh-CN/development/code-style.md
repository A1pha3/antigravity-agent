---
title: 代码规范文档
description: Antigravity Agent 项目代码风格和最佳实践指南
category: development
language: zh-CN
version: 1.0.3
lastUpdated: 2025-12-04
tags: [代码规范, 风格指南, 最佳实践]
---

# 代码规范文档

本文档定义了 Antigravity Agent 项目的代码规范和最佳实践。遵循这些规范可以确保代码的一致性、可读性和可维护性。

## 目录

- [TypeScript 代码规范](#typescript-代码规范)
- [Rust 代码规范](#rust-代码规范)
- [命名约定](#命名约定)
- [注释规范](#注释规范)
- [文件组织规范](#文件组织规范)
- [Linter 和 Formatter 配置](#linter-和-formatter-配置)

## TypeScript 代码规范

### 基本原则

1. **使用 TypeScript 严格模式**
2. **优先使用函数式编程风格**
3. **避免使用 `any` 类型**
4. **保持函数简短和单一职责**

### 类型定义

#### 使用接口定义对象类型

```typescript
// ✅ 好的做法
interface User {
  id: string;
  username: string;
  email: string;
  createdAt: Date;
}

// ❌ 避免使用 any
interface User {
  data: any;  // 不好
}
```

#### 使用类型别名定义联合类型

```typescript
// ✅ 好的做法
type Status = 'idle' | 'loading' | 'success' | 'error';
type Result<T> = { success: true; data: T } | { success: false; error: string };

// 使用泛型
type AsyncState<T> = {
  data: T | null;
  loading: boolean;
  error: string | null;
};
```

#### 避免使用 any

```typescript
// ❌ 不好
function processData(data: any) {
  return data.value;
}

// ✅ 好的做法
function processData(data: unknown) {
  if (typeof data === 'object' && data !== null && 'value' in data) {
    return (data as { value: string }).value;
  }
  throw new Error('Invalid data');
}

// ✅ 更好的做法
interface DataWithValue {
  value: string;
}

function processData(data: DataWithValue) {
  return data.value;
}
```

### React 组件

#### 函数组件

```typescript
// ✅ 好的做法 - 使用函数组件和 TypeScript
interface UserCardProps {
  user: User;
  onSelect?: (user: User) => void;
}

export function UserCard({ user, onSelect }: UserCardProps) {
  const handleClick = () => {
    onSelect?.(user);
  };

  return (
    <div onClick={handleClick}>
      <h3>{user.username}</h3>
      <p>{user.email}</p>
    </div>
  );
}
```

#### Props 类型定义

```typescript
// ✅ 好的做法 - 明确的 Props 类型
interface ButtonProps {
  children: React.ReactNode;
  variant?: 'primary' | 'secondary' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  onClick?: () => void;
}

export function Button({
  children,
  variant = 'primary',
  size = 'md',
  disabled = false,
  onClick,
}: ButtonProps) {
  // 实现
}
```

#### Hooks 使用

```typescript
// ✅ 好的做法 - 自定义 Hook
export function useAntigravityAccount() {
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchAccounts = useCallback(async () => {
    setLoading(true);
    setError(null);
    
    try {
      const data = await invoke<Account[]>('get_antigravity_accounts');
      setAccounts(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchAccounts();
  }, [fetchAccounts]);

  return { accounts, loading, error, refetch: fetchAccounts };
}
```

### 异步处理

#### 使用 async/await

```typescript
// ✅ 好的做法
async function switchAccount(accountId: string): Promise<void> {
  try {
    await invoke('switch_antigravity_account', { accountId });
    toast.success('账户切换成功');
  } catch (error) {
    console.error('切换账户失败:', error);
    toast.error('账户切换失败');
    throw error;
  }
}

// ❌ 避免使用 Promise 链
function switchAccount(accountId: string) {
  return invoke('switch_antigravity_account', { accountId })
    .then(() => {
      toast.success('账户切换成功');
    })
    .catch((error) => {
      console.error('切换账户失败:', error);
      toast.error('账户切换失败');
      throw error;
    });
}
```

### 错误处理

```typescript
// ✅ 好的做法 - 明确的错误类型
class AccountError extends Error {
  constructor(
    message: string,
    public code: string,
    public accountId?: string
  ) {
    super(message);
    this.name = 'AccountError';
  }
}

async function getAccount(id: string): Promise<Account> {
  try {
    return await invoke<Account>('get_account', { id });
  } catch (error) {
    if (error instanceof Error) {
      throw new AccountError(
        `Failed to get account: ${error.message}`,
        'ACCOUNT_NOT_FOUND',
        id
      );
    }
    throw new AccountError('Unknown error', 'UNKNOWN', id);
  }
}
```

### 代码组织

```typescript
// ✅ 好的做法 - 清晰的导入顺序
// 1. React 相关
import React, { useState, useEffect } from 'react';

// 2. 第三方库
import { invoke } from '@tauri-apps/api/core';
import toast from 'react-hot-toast';

// 3. 项目内部模块（按层级）
import { useAntigravityAccount } from '@/modules/use-antigravity-account';
import { AntigravityService } from '@/services/antigravity-service';
import { logger } from '@/utils/logger';

// 4. 类型定义
import type { Account } from '@/types';

// 5. 样式
import './UserPanel.css';
```


## Rust 代码规范

### 基本原则

1. **遵循 Rust 官方风格指南**
2. **使用 `rustfmt` 格式化代码**
3. **使用 `clippy` 检查代码质量**
4. **优先使用标准库和成熟的 crate**

### 命名和格式

```rust
// ✅ 好的做法 - 遵循 Rust 命名约定

// 模块名：snake_case
mod account_manager;
mod backup_service;

// 结构体：PascalCase
struct AntigravityAccount {
    id: String,
    username: String,
    email: String,
}

// 枚举：PascalCase，变体也是 PascalCase
enum AccountStatus {
    Active,
    Inactive,
    Suspended,
}

// 函数和变量：snake_case
fn switch_account(account_id: &str) -> Result<()> {
    let current_account = get_current_account()?;
    // ...
}

// 常量：SCREAMING_SNAKE_CASE
const MAX_RETRY_COUNT: u32 = 3;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

// 类型别名：PascalCase
type AccountId = String;
type Result<T> = std::result::Result<T, AccountError>;
```

### 错误处理

#### 使用 Result 类型

```rust
// ✅ 好的做法 - 明确的错误类型
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("Account not found: {0}")]
    NotFound(String),
    
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid account data: {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, AccountError>;

// 使用
fn get_account(id: &str) -> Result<Account> {
    let conn = get_connection()?;
    let account = conn.query_row(
        "SELECT * FROM accounts WHERE id = ?",
        [id],
        |row| {
            Ok(Account {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
            })
        },
    )?;
    Ok(account)
}
```

#### 使用 ? 操作符

```rust
// ✅ 好的做法
fn backup_account(account_id: &str) -> Result<PathBuf> {
    let account = get_account(account_id)?;
    let backup_dir = create_backup_directory()?;
    let backup_path = write_backup_file(&backup_dir, &account)?;
    Ok(backup_path)
}

// ❌ 避免过度使用 unwrap
fn backup_account(account_id: &str) -> PathBuf {
    let account = get_account(account_id).unwrap();  // 不好！
    let backup_dir = create_backup_directory().unwrap();
    write_backup_file(&backup_dir, &account).unwrap()
}
```

### Tauri 命令

```rust
// ✅ 好的做法 - Tauri 命令规范
use tauri::State;
use tracing::{info, error};

#[tauri::command]
pub async fn switch_antigravity_account(
    account_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    info!(account_id = %account_id, "开始切换账户");
    
    // 执行业务逻辑
    match switch_account_impl(&account_id, &state).await {
        Ok(()) => {
            info!("账户切换成功");
            Ok(())
        }
        Err(e) => {
            error!(error = %e, "账户切换失败");
            Err(e.to_string())
        }
    }
}

// 将实际逻辑分离到独立函数
async fn switch_account_impl(
    account_id: &str,
    state: &AppState,
) -> Result<()> {
    // 实现细节
    Ok(())
}
```

### 异步编程

```rust
// ✅ 好的做法 - 使用 async/await
use tokio::fs;

async fn read_config_file(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path).await?;
    Ok(content)
}

async fn process_accounts() -> Result<Vec<Account>> {
    // 并行处理多个账户
    let handles: Vec<_> = account_ids
        .into_iter()
        .map(|id| tokio::spawn(async move {
            process_account(&id).await
        }))
        .collect();
    
    let results = futures::future::join_all(handles).await;
    
    // 处理结果
    let accounts: Result<Vec<_>> = results
        .into_iter()
        .map(|r| r.map_err(|e| AccountError::from(e))?)
        .collect();
    
    accounts
}
```

### 所有权和借用

```rust
// ✅ 好的做法 - 明确的所有权
fn process_account(account: Account) -> Result<()> {
    // 获取所有权
    save_account(account)?;
    Ok(())
}

fn display_account(account: &Account) {
    // 借用，不获取所有权
    println!("Account: {}", account.username);
}

fn update_account(account: &mut Account, new_email: String) {
    // 可变借用
    account.email = new_email;
}

// 使用
let mut account = get_account("123")?;
display_account(&account);  // 借用
update_account(&mut account, "new@email.com".to_string());  // 可变借用
process_account(account);  // 转移所有权
// account 在这里不再可用
```

### 模式匹配

```rust
// ✅ 好的做法 - 使用模式匹配
fn handle_account_status(status: AccountStatus) -> String {
    match status {
        AccountStatus::Active => "账户活跃".to_string(),
        AccountStatus::Inactive => "账户未激活".to_string(),
        AccountStatus::Suspended => "账户已暂停".to_string(),
    }
}

// 处理 Option
fn get_account_email(account: Option<Account>) -> String {
    match account {
        Some(acc) => acc.email,
        None => "未知".to_string(),
    }
}

// 或使用 if let
fn process_if_active(account: Option<Account>) {
    if let Some(acc) = account {
        if acc.status == AccountStatus::Active {
            process_account(acc);
        }
    }
}
```

### 日志记录

```rust
// ✅ 好的做法 - 使用 tracing
use tracing::{info, debug, warn, error, instrument};

#[instrument(skip(state))]
async fn switch_account(
    account_id: &str,
    state: &AppState,
) -> Result<()> {
    info!(account_id = %account_id, "开始切换账户");
    
    debug!("检查账户是否存在");
    let account = get_account(account_id)?;
    
    debug!("终止当前进程");
    kill_current_process()?;
    
    debug!("更新数据库");
    update_current_account(&account)?;
    
    info!("账户切换成功");
    Ok(())
}

// 错误日志
fn handle_error(error: &AccountError) {
    error!(
        error = %error,
        error_type = ?error,
        "处理账户时发生错误"
    );
}
```

### 测试

```rust
// ✅ 好的做法 - 完整的测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_account_creation() {
        let account = Account {
            id: "123".to_string(),
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
        };
        
        assert_eq!(account.id, "123");
        assert_eq!(account.username, "test_user");
    }
    
    #[tokio::test]
    async fn test_async_backup() {
        let result = backup_account("test_id").await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_error_handling() {
        let result = get_account("nonexistent");
        assert!(matches!(result, Err(AccountError::NotFound(_))));
    }
}
```


## 命名约定

### 通用原则

1. **使用描述性名称**: 名称应清楚表达意图
2. **避免缩写**: 除非是广为人知的缩写（如 `id`, `url`, `db`）
3. **保持一致性**: 在整个项目中使用相同的命名风格

### TypeScript 命名

#### 变量和函数

```typescript
// ✅ 好的做法 - camelCase
const userName = 'John';
const accountList = [];
const isLoading = false;

function getUserAccount(userId: string) { }
function handleAccountSwitch() { }

// ❌ 避免
const user_name = 'John';  // snake_case
const UserName = 'John';   // PascalCase
function get_user() { }    // snake_case
```

#### 类和接口

```typescript
// ✅ 好的做法 - PascalCase
class AccountManager { }
interface UserAccount { }
type AccountStatus = 'active' | 'inactive';

// 接口不需要 I 前缀
interface Account { }  // ✅
interface IAccount { }  // ❌ 避免
```

#### 常量

```typescript
// ✅ 好的做法 - SCREAMING_SNAKE_CASE
const MAX_RETRY_COUNT = 3;
const API_BASE_URL = 'https://api.example.com';
const DEFAULT_TIMEOUT_MS = 5000;

// 枚举值使用 PascalCase
enum AccountStatus {
  Active = 'active',
  Inactive = 'inactive',
  Suspended = 'suspended',
}
```

#### 文件名

```typescript
// ✅ 好的做法
// 组件文件：PascalCase
UserCard.tsx
AccountList.tsx
SettingsDialog.tsx

// 工具文件：kebab-case
user-utils.ts
date-formatter.ts
api-client.ts

// Hook 文件：camelCase，use 前缀
useAntigravityAccount.ts
useCurrentUser.ts
useDebounce.ts

// 服务文件：kebab-case，-service 后缀
antigravity-service.ts
backup-service.ts
```

### Rust 命名

#### 模块和文件

```rust
// ✅ 好的做法 - snake_case
// 文件名
account_manager.rs
backup_service.rs
path_utils.rs

// 模块声明
mod account_manager;
mod backup_service;
```

#### 结构体和枚举

```rust
// ✅ 好的做法 - PascalCase
struct AntigravityAccount { }
struct BackupConfig { }
enum ProcessStatus { }
```

#### 函数和变量

```rust
// ✅ 好的做法 - snake_case
fn switch_account() { }
fn get_current_user() { }
let account_id = "123";
let is_running = true;
```

#### Trait

```rust
// ✅ 好的做法 - PascalCase，通常使用形容词
trait Serializable { }
trait Cloneable { }
trait Processable { }
```

### 特殊命名模式

#### Boolean 变量

```typescript
// ✅ 好的做法 - 使用 is/has/can/should 前缀
const isLoading = true;
const hasError = false;
const canEdit = true;
const shouldUpdate = false;

// Rust
let is_running = true;
let has_backup = false;
let can_switch = true;
```

#### 事件处理函数

```typescript
// ✅ 好的做法 - handle/on 前缀
function handleClick() { }
function handleAccountSwitch() { }
function onUserSelect() { }

// React 组件
<Button onClick={handleClick} />
<UserCard onSelect={handleUserSelect} />
```

#### 异步函数

```typescript
// ✅ 好的做法 - 使用动词，表明是异步操作
async function fetchAccounts() { }
async function loadUserData() { }
async function saveBackup() { }

// Rust
async fn fetch_accounts() { }
async fn load_user_data() { }
```

## 注释规范

### TypeScript 注释

#### JSDoc 注释

```typescript
/**
 * 切换到指定的 Antigravity 账户
 * 
 * @param accountId - 要切换到的账户 ID
 * @returns Promise，成功时 resolve，失败时 reject
 * @throws {AccountError} 当账户不存在或切换失败时
 * 
 * @example
 * ```typescript
 * await switchAccount('account-123');
 * ```
 */
async function switchAccount(accountId: string): Promise<void> {
  // 实现
}

/**
 * 用户账户接口
 */
interface Account {
  /** 账户唯一标识符 */
  id: string;
  
  /** 用户名 */
  username: string;
  
  /** 电子邮件地址 */
  email: string;
  
  /** 账户创建时间 */
  createdAt: Date;
}
```

#### 行内注释

```typescript
// ✅ 好的做法 - 解释"为什么"，而不是"是什么"

// 使用 setTimeout 避免阻塞 UI 线程
setTimeout(() => {
  processLargeData();
}, 0);

// 临时解决方案：等待上游库修复 #123
const workaround = true;

// ❌ 避免 - 显而易见的注释
// 设置 loading 为 true
setLoading(true);

// 调用 API
const data = await fetchData();
```

#### TODO 注释

```typescript
// TODO: 添加错误重试逻辑
// TODO(username): 优化性能
// FIXME: 修复内存泄漏问题
// HACK: 临时解决方案，需要重构
// NOTE: 这个实现依赖于 X 库的特定行为
```

### Rust 注释

#### 文档注释

```rust
/// 切换到指定的 Antigravity 账户
///
/// # Arguments
///
/// * `account_id` - 要切换到的账户 ID
///
/// # Returns
///
/// 成功时返回 `Ok(())`，失败时返回 `Err(AccountError)`
///
/// # Errors
///
/// 当账户不存在或切换失败时返回错误
///
/// # Examples
///
/// ```
/// let result = switch_account("account-123").await;
/// assert!(result.is_ok());
/// ```
pub async fn switch_account(account_id: &str) -> Result<()> {
    // 实现
}

/// 用户账户结构体
#[derive(Debug, Clone)]
pub struct Account {
    /// 账户唯一标识符
    pub id: String,
    
    /// 用户名
    pub username: String,
    
    /// 电子邮件地址
    pub email: String,
}
```

#### 模块文档

```rust
//! 账户管理模块
//!
//! 此模块提供 Antigravity 账户的管理功能，包括：
//! - 账户切换
//! - 账户备份
//! - 账户恢复
//!
//! # Examples
//!
//! ```
//! use antigravity_agent::account_manager::*;
//!
//! let accounts = get_all_accounts()?;
//! switch_account(&accounts[0].id).await?;
//! ```

use std::path::PathBuf;
// ...
```

### 注释最佳实践

#### 何时写注释

✅ **应该写注释的情况**:
- 复杂的算法或业务逻辑
- 非显而易见的解决方案
- 临时解决方案或已知问题
- 公开 API 和接口
- 重要的架构决策

❌ **不需要写注释的情况**:
- 显而易见的代码
- 可以通过重命名改善的代码
- 重复代码本身的信息

```typescript
// ❌ 不好 - 重复代码信息
// 获取用户名
const username = user.name;

// ✅ 好 - 解释为什么
// 使用 display name 而不是 username，因为某些用户没有设置 username
const displayName = user.displayName || user.username || 'Anonymous';
```


## 文件组织规范

### TypeScript 文件结构

```typescript
// 1. 文件头注释（可选）
/**
 * @fileoverview 账户管理服务
 * @author Your Name
 */

// 2. 导入语句（按类别分组）
// React 相关
import React, { useState, useEffect } from 'react';

// 第三方库
import { invoke } from '@tauri-apps/api/core';
import toast from 'react-hot-toast';

// 项目内部（按层级）
import { useAntigravityAccount } from '@/modules/use-antigravity-account';
import { AntigravityService } from '@/services/antigravity-service';
import { logger } from '@/utils/logger';

// 类型定义
import type { Account, AccountStatus } from '@/types';

// 样式
import './AccountManager.css';

// 3. 类型定义
interface AccountManagerProps {
  initialAccount?: Account;
}

// 4. 常量
const MAX_ACCOUNTS = 10;
const REFRESH_INTERVAL = 5000;

// 5. 主要导出
export function AccountManager({ initialAccount }: AccountManagerProps) {
  // 实现
}

// 6. 辅助函数（非导出）
function validateAccount(account: Account): boolean {
  // 实现
}

// 7. 其他导出
export { validateAccount };
```

### Rust 文件结构

```rust
// 1. 模块文档
//! 账户管理模块

// 2. 导入语句（按类别分组）
// 标准库
use std::path::PathBuf;
use std::fs;

// 第三方 crate
use serde::{Serialize, Deserialize};
use tokio::fs as async_fs;
use tracing::{info, error};

// 项目内部
use crate::state::AppState;
use crate::error::AccountError;

// 3. 类型定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
}

// 4. 常量
const MAX_RETRY_COUNT: u32 = 3;
const BACKUP_DIR: &str = "backups";

// 5. 公开函数
pub async fn switch_account(account_id: &str) -> Result<()> {
    // 实现
}

// 6. 私有函数
fn validate_account_id(id: &str) -> bool {
    // 实现
}

// 7. 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_account_id() {
        assert!(validate_account_id("valid-id"));
    }
}
```

### 目录组织

#### 前端目录结构

```
src/
├── components/          # UI 组件
│   ├── base-ui/        # 基础组件（按钮、输入框等）
│   │   ├── BaseButton.tsx
│   │   ├── BaseInput.tsx
│   │   └── index.ts    # 统一导出
│   ├── business/       # 业务组件
│   │   ├── UserCard.tsx
│   │   ├── AccountList.tsx
│   │   └── index.ts
│   └── ui/             # 通用 UI 组件
│       └── tooltip.tsx
├── hooks/              # 自定义 Hooks
│   ├── useAntigravityAccount.ts
│   ├── useCurrentUser.ts
│   └── index.ts
├── services/           # 服务层
│   ├── antigravity-service.ts
│   ├── backup-service.ts
│   └── index.ts
├── modules/            # 业务模块（状态管理）
│   ├── use-antigravity-account.ts
│   └── db-monitoring-store.ts
├── utils/              # 工具函数
│   ├── logger.ts
│   ├── date-formatter.ts
│   └── index.ts
├── types/              # 类型定义
│   ├── account.types.ts
│   ├── backup.types.ts
│   └── index.ts
├── App.tsx             # 应用主组件
└── main.tsx           # 应用入口
```

#### 后端目录结构

```
src-tauri/src/
├── commands/           # Tauri 命令
│   ├── account_commands.rs
│   ├── backup_commands.rs
│   ├── process_commands.rs
│   └── mod.rs
├── antigravity/        # 业务逻辑
│   ├── backup.rs
│   ├── restore.rs
│   ├── starter.rs
│   └── mod.rs
├── platform/           # 平台适配
│   ├── windows.rs
│   ├── macos.rs
│   ├── linux.rs
│   └── mod.rs
├── utils/              # 工具模块
│   ├── path_utils.rs
│   ├── log_sanitizer.rs
│   └── mod.rs
├── state.rs            # 全局状态
├── error.rs            # 错误定义
└── main.rs            # 应用入口
```

### 模块导出

#### TypeScript 索引文件

```typescript
// components/base-ui/index.ts
export { BaseButton } from './BaseButton';
export { BaseInput } from './BaseInput';
export { BaseDialog } from './BaseDialog';

// 或使用命名空间
export * as BaseUI from './BaseButton';
```

#### Rust 模块文件

```rust
// commands/mod.rs
mod account_commands;
mod backup_commands;
mod process_commands;

// 重新导出
pub use account_commands::*;
pub use backup_commands::*;
pub use process_commands::*;
```

## Linter 和 Formatter 配置

### TypeScript / JavaScript

#### ESLint 配置

创建 `.eslintrc.json`:

```json
{
  "extends": [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended"
  ],
  "parser": "@typescript-eslint/parser",
  "parserOptions": {
    "ecmaVersion": 2021,
    "sourceType": "module",
    "ecmaFeatures": {
      "jsx": true
    }
  },
  "rules": {
    "@typescript-eslint/no-explicit-any": "error",
    "@typescript-eslint/explicit-function-return-type": "off",
    "@typescript-eslint/no-unused-vars": ["error", {
      "argsIgnorePattern": "^_"
    }],
    "react/react-in-jsx-scope": "off",
    "react/prop-types": "off"
  }
}
```

#### Prettier 配置

创建 `.prettierrc`:

```json
{
  "semi": true,
  "trailingComma": "es5",
  "singleQuote": true,
  "printWidth": 100,
  "tabWidth": 2,
  "useTabs": false,
  "arrowParens": "always",
  "endOfLine": "lf"
}
```

#### 运行 Linter

```bash
# 检查代码
npm run lint

# 自动修复
npm run lint -- --fix

# 格式化代码
npm run format
```

在 `package.json` 中添加脚本:

```json
{
  "scripts": {
    "lint": "eslint src --ext .ts,.tsx",
    "lint:fix": "eslint src --ext .ts,.tsx --fix",
    "format": "prettier --write \"src/**/*.{ts,tsx,css}\""
  }
}
```

### Rust

#### Clippy 配置

创建 `.cargo/config.toml`:

```toml
[target.'cfg(all())']
rustflags = ["-D", "warnings"]
```

或在 `Cargo.toml` 中配置:

```toml
[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
```

#### Rustfmt 配置

创建 `rustfmt.toml`:

```toml
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
```

#### 运行 Linter

```bash
# 检查代码
cargo clippy

# 严格检查
cargo clippy -- -D warnings

# 格式化代码
cargo fmt

# 检查格式
cargo fmt -- --check
```

### Git Hooks

使用 Husky 自动运行检查:

```bash
# 安装 Husky
npm install -D husky

# 初始化
npx husky install

# 添加 pre-commit hook
npx husky add .husky/pre-commit "npm run lint && npm run type-check"

# 添加 commit-msg hook
npx husky add .husky/commit-msg 'npx --no -- commitlint --edit "$1"'
```

### VS Code 配置

创建 `.vscode/settings.json`:

```json
{
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "typescript.tsdk": "node_modules/typescript/lib",
  "rust-analyzer.checkOnSave.command": "clippy",
  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

## 代码审查检查清单

在提交代码前，请确保：

### 通用检查

- [ ] 代码遵循项目命名约定
- [ ] 没有使用 `any` 类型（TypeScript）
- [ ] 没有使用 `unwrap()` 或 `expect()`（Rust，除非有充分理由）
- [ ] 错误处理完整
- [ ] 添加了必要的注释
- [ ] 删除了调试代码和 console.log
- [ ] 没有硬编码的值（使用常量）

### TypeScript 检查

- [ ] 通过 `npm run type-check`
- [ ] 通过 `npm run lint`
- [ ] 代码已格式化（Prettier）
- [ ] 组件有明确的 Props 类型
- [ ] 异步函数使用 async/await

### Rust 检查

- [ ] 通过 `cargo clippy`
- [ ] 通过 `cargo fmt --check`
- [ ] 通过 `cargo test`
- [ ] 使用 Result 类型处理错误
- [ ] 文档注释完整

### 测试检查

- [ ] 添加了单元测试（如需要）
- [ ] 测试覆盖关键逻辑
- [ ] 所有测试通过

## 相关文档

### 开发文档
- [开发指南](./development-guide.md) - 开发环境搭建和工作流程
- [贡献指南](./contributing.md) - 如何参与项目贡献
- [系统架构](./architecture.md) - 系统整体架构设计

### 使用文档
- [使用手册](../user-guide/user-guide.md) - 完整的功能说明和操作指南
- [API 参考](../user-guide/api-reference.md) - 所有命令和接口说明

### 进阶文档
- [设计原理](../advanced/design-principles.md) - 核心设计思路和技术选型
- [性能优化](../advanced/performance.md) - 性能分析和优化建议

### 入门文档
- [项目概览](../getting-started/README.md) - 了解项目的基本信息
- [快速开始](../getting-started/quickstart.md) - 5 分钟快速上手教程

### 返回
- [文档首页](../../README.md) - 返回文档导航页

## 参考资源

### TypeScript
- [TypeScript 官方风格指南](https://www.typescriptlang.org/docs/handbook/declaration-files/do-s-and-don-ts.html)
- [Google TypeScript 风格指南](https://google.github.io/styleguide/tsguide.html)
- [Airbnb JavaScript 风格指南](https://github.com/airbnb/javascript)

### Rust
- [Rust API 指南](https://rust-lang.github.io/api-guidelines/)
- [Rust 风格指南](https://doc.rust-lang.org/1.0.0/style/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

**最后更新**: 2025-12-04  
**文档版本**: 1.0.3
