# 安全审计与加固设计文档

## 概述

本设计文档详细说明了 Antigravity Agent 项目的安全审计和加固方案。该方案采用纵深防御策略，从多个层面加强系统安全性：

- **加密层**: 增强密钥派生和加密算法配置
- **存储层**: 改进文件权限和安全删除机制
- **数据库层**: 强化 SQL 注入防护和数据验证
- **内存层**: 实施敏感数据清零和安全内存管理
- **网络层**: 加强 TLS 配置和证书验证
- **应用层**: 完善输入验证、错误处理和审计日志
- **依赖层**: 建立持续的漏洞扫描和更新流程

设计目标：
1. 修复现有的安全漏洞和弱点
2. 实施安全最佳实践
3. 建立持续的安全监控和审计机制
4. 确保符合行业安全标准

## 代码审查发现的具体修复方案

### V-001: 机器密钥派生增强

**当前代码** (`src-tauri/src/utils/crypto.rs:45-56`):
```rust
pub fn derive_machine_key() -> Result<[u8; 32], CryptoError> {
    let machine_id = get_machine_id();
    let username = whoami::username();
    let app_salt = "antigravity-agent-v1";
    let combined = format!("{}:{}:{}", machine_id, username, app_salt);
    
    // 仅使用 SHA-256，缺少 KDF 增强
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    // ...
}
```

**修复方案**:
```rust
pub fn derive_machine_key() -> Result<[u8; 32], CryptoError> {
    let machine_id = get_machine_id()?; // 返回 Result，不使用默认值
    let username = whoami::username();
    let app_salt = "antigravity-agent-v2"; // 版本升级
    let combined = format!("{}:{}:{}", machine_id, username, app_salt);
    
    // 第一步：SHA-256 预处理
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let pre_key = hasher.finalize();
    
    // 第二步：Argon2id 密钥派生
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(19456, 2, 1, Some(32)).unwrap()
    );
    
    let salt = SaltString::encode_b64(&pre_key[..16]).unwrap();
    let password_hash = argon2.hash_password(&pre_key, &salt)?;
    
    let mut key = [0u8; 32];
    key.copy_from_slice(password_hash.hash.unwrap().as_bytes());
    Ok(key)
}
```

### V-002: 移除默认机器 ID

**当前代码** (`src-tauri/src/utils/crypto.rs:170-195`):
```rust
fn get_machine_id() -> String {
    // ...
    .unwrap_or_else(|| "default-mac-id".to_string())  // 危险！
}
```

**修复方案**:
```rust
fn get_machine_id() -> Result<String, CryptoError> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .ok()
            .and_then(|output| {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.lines()
                    .find(|line| line.contains("IOPlatformUUID"))
                    .and_then(|line| line.split('"').nth(3).map(|s| s.to_string()))
            })
            .ok_or_else(|| CryptoError::KeyDerivationFailed(
                "无法获取机器 ID，请确保系统权限正确".to_string()
            ))
    }
    // 其他平台类似处理...
}
```

### V-003: 密码强度验证

**新增函数**:
```rust
pub fn validate_password_strength(password: &str) -> Result<(), CryptoError> {
    if password.len() < 12 {
        return Err(CryptoError::InvalidData("密码长度至少 12 个字符".to_string()));
    }
    
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    if !has_upper || !has_lower || !has_digit || !has_special {
        return Err(CryptoError::InvalidData(
            "密码必须包含大小写字母、数字和特殊字符".to_string()
        ));
    }
    
    Ok(())
}
```

### V-004: Windows ACL 设置

**修复方案** (`src-tauri/src/utils/crypto.rs`):
```rust
pub fn secure_write_file(path: &Path, data: &[u8]) -> Result<(), CryptoError> {
    fs::write(path, data).map_err(|e| CryptoError::IoError(e.to_string()))?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(path, permissions)?;
    }
    
    #[cfg(windows)]
    {
        use windows_acl::acl::ACL;
        // 获取当前用户 SID
        let current_user = whoami::username();
        // 设置 DACL 仅允许当前用户访问
        let mut acl = ACL::from_file_path(path, false)?;
        acl.all_entries().iter().for_each(|e| acl.remove(e.sid, None, None));
        acl.allow(current_user, true, windows_acl::acl::AceFlags::OBJECT_INHERIT_ACE)?;
        acl.apply()?;
    }
    
    Ok(())
}
```

### V-005: 敏感数据清零

**新增依赖** (`Cargo.toml`):
```toml
zeroize = { version = "1.7", features = ["derive"] }
```

**新增模块** (`src-tauri/src/security/secure_memory.rs`):
```rust
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureKey([u8; 32]);

impl SecureKey {
    pub fn new(key: [u8; 32]) -> Self {
        Self(key)
    }
    
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
```

### V-006: SQL 参数化查询

**当前代码** (`src-tauri/src/antigravity/backup.rs:85`):
```rust
conn.query_row(
    &format!("SELECT value FROM ItemTable WHERE key = '{}'", database::TARGET_STORAGE_MARKER),
    [],
    |row| row.get(0),
)
```

**修复方案**:
```rust
conn.query_row(
    "SELECT value FROM ItemTable WHERE key = ?",
    [database::TARGET_STORAGE_MARKER],
    |row| row.get(0),
)
```

## 架构

### 安全架构分层

```
┌─────────────────────────────────────────────────────────┐
│                    应用层安全                              │
│  - 输入验证  - 输出编码  - 错误处理  - 审计日志            │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│                    业务逻辑层安全                          │
│  - 访问控制  - 权限验证  - 业务规则验证                    │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│                    数据层安全                              │
│  - 加密存储  - 数据完整性  - 安全删除                      │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│                    系统层安全                              │
│  - 文件权限  - 进程隔离  - 内存保护                        │
└─────────────────────────────────────────────────────────┘
```

### 安全组件关系图

```
┌──────────────────┐
│  Security Config │ ←─── 集中管理安全配置
└────────┬─────────┘
         │
    ┌────┴────┬────────┬────────┬────────┐
    ↓         ↓        ↓        ↓        ↓
┌────────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐
│ Crypto │ │ Input│ │ Audit│ │ Access│ │ Secure│
│ Manager│ │Validator│ │Logger│ │Control│ │Storage│
└────────┘ └──────┘ └──────┘ └──────┘ └──────┘
```

## 组件和接口

### 1. 增强的加密管理器 (EnhancedCryptoManager)

**职责**: 提供强化的加密密钥派生和管理功能

**接口**:
```rust
pub struct EnhancedCryptoManager;

impl EnhancedCryptoManager {
    /// 使用 Argon2 增强的机器密钥派生
    pub fn derive_machine_key_secure() -> Result<[u8; 32], CryptoError>;
    
    /// 验证密码强度
    pub fn validate_password_strength(password: &str) -> Result<(), PasswordError>;
    
    /// 安全清零内存中的敏感数据
    pub fn secure_zero(data: &mut [u8]);
    
    /// 生成密码学安全的随机数
    pub fn generate_secure_random(size: usize) -> Vec<u8>;
}
```

**安全增强**:
- 使用 Argon2id 替代单纯的 SHA-256 进行密钥派生
- 实施密码强度验证（最小长度、复杂度要求）
- 使用 `zeroize` crate 确保敏感数据使用后被清零
- 添加密钥派生失败时的安全回退机制

### 2. 安全存储管理器 (SecureStorageManager)

**职责**: 管理文件和目录的安全创建、访问和删除

**接口**:
```rust
pub struct SecureStorageManager;

impl SecureStorageManager {
    /// 安全写入文件（跨平台权限设置）
    pub fn secure_write(path: &Path, data: &[u8]) -> Result<(), StorageError>;
    
    /// 安全删除文件（覆写后删除）
    pub fn secure_delete(path: &Path) -> Result<(), StorageError>;
    
    /// 验证文件权限
    pub fn verify_permissions(path: &Path) -> Result<bool, StorageError>;
    
    /// 创建安全的临时文件
    pub fn create_temp_file() -> Result<TempFile, StorageError>;
}
```

**安全增强**:
- Unix: 设置 0600/0700 权限
- Windows: 使用 DACL 限制访问
- 实施安全删除（多次覆写）
- 原子文件操作防止 TOCTOU

### 3. 输入验证器 (InputValidator)

**职责**: 验证和清理所有外部输入

**接口**:
```rust
pub struct InputValidator;

impl InputValidator {
    /// 验证文件路径（防止路径遍历）
    pub fn validate_path(path: &str) -> Result<PathBuf, ValidationError>;
    
    /// 验证邮箱地址
    pub fn validate_email(email: &str) -> Result<String, ValidationError>;
    
    /// 验证账户名称
    pub fn validate_account_name(name: &str) -> Result<String, ValidationError>;
    
    /// 清理 SQL 输入（额外防护层）
    pub fn sanitize_sql_input(input: &str) -> String;
    
    /// 验证 JSON 结构
    pub fn validate_json_structure(json: &Value, schema: &Schema) -> Result<(), ValidationError>;
}
```

**安全增强**:
- 路径规范化和白名单验证
- 正则表达式验证（防止 ReDoS）
- 长度和字符集限制
- JSON schema 验证

### 4. 审计日志管理器 (AuditLogger)

**职责**: 记录安全相关事件，同时保护敏感信息

**接口**:
```rust
pub struct AuditLogger;

impl AuditLogger {
    /// 记录安全事件
    pub fn log_security_event(event: SecurityEvent);
    
    /// 记录认证事件
    pub fn log_auth_event(user: &str, action: &str, success: bool);
    
    /// 记录数据访问
    pub fn log_data_access(resource: &str, operation: &str);
    
    /// 过滤敏感信息
    fn sanitize_log_data(data: &str) -> String;
}

pub struct SecurityEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub severity: Severity,
    pub user_id: Option<String>,
    pub details: HashMap<String, String>,
}
```

**安全增强**:
- 自动过滤敏感数据（密钥、密码、令牌）
- 结构化日志格式（JSON）
- 日志完整性保护（HMAC）
- 日志轮转和大小限制


### 5. 访问控制管理器 (AccessControlManager)

**职责**: 实施细粒度的访问控制和权限验证

**接口**:
```rust
pub struct AccessControlManager;

impl AccessControlManager {
    /// 验证文件访问权限
    pub fn check_file_access(path: &Path, operation: Operation) -> Result<(), AccessError>;
    
    /// 验证进程访问权限
    pub fn check_process_access(pid: u32) -> Result<(), AccessError>;
    
    /// 验证用户操作权限
    pub fn check_user_permission(user: &str, resource: &str, action: &str) -> Result<(), AccessError>;
}

pub enum Operation {
    Read,
    Write,
    Execute,
    Delete,
}
```

**安全增强**:
- 最小权限原则
- 基于角色的访问控制（RBAC）
- 操作前权限验证
- 审计所有权限检查

### 6. 安全数据库访问层 (SecureDatabaseAccess)

**职责**: 提供安全的数据库访问接口

**接口**:
```rust
pub struct SecureDatabaseAccess {
    conn: Connection,
}

impl SecureDatabaseAccess {
    /// 执行参数化查询
    pub fn query_safe<T>(&self, sql: &str, params: &[&dyn ToSql]) -> Result<Vec<T>, DbError>;
    
    /// 验证数据库路径
    pub fn validate_db_path(path: &Path) -> Result<(), DbError>;
    
    /// 验证查询结果
    pub fn validate_result<T>(result: &T) -> Result<(), DbError>;
    
    /// 加密敏感字段
    pub fn encrypt_sensitive_field(value: &str) -> Result<String, DbError>;
}
```

**安全增强**:
- 强制使用参数化查询
- 路径验证防止注入
- 结果验证和类型检查
- 敏感字段自动加密

### 7. 内存安全管理器 (MemorySafetyManager)

**职责**: 管理敏感数据的内存生命周期

**接口**:
```rust
pub struct SecureMemory<T> {
    data: T,
}

impl<T> SecureMemory<T> {
    /// 创建安全内存区域
    pub fn new(data: T) -> Self;
    
    /// 获取数据引用
    pub fn get(&self) -> &T;
    
    /// 获取可变引用
    pub fn get_mut(&mut self) -> &mut T;
}

impl<T> Drop for SecureMemory<T> {
    /// 自动清零内存
    fn drop(&mut self);
}
```

**安全增强**:
- 使用 `zeroize` 自动清零
- RAII 模式确保清理
- 防止敏感数据泄露到 swap
- 内存锁定（mlock）支持

### 8. 网络安全管理器 (NetworkSecurityManager)

**职责**: 管理网络通信的安全配置

**接口**:
```rust
pub struct NetworkSecurityManager;

impl NetworkSecurityManager {
    /// 创建安全的 HTTP 客户端
    pub fn create_secure_client() -> Result<Client, NetworkError>;
    
    /// 验证 TLS 证书
    pub fn verify_certificate(cert: &Certificate) -> Result<(), NetworkError>;
    
    /// 验证 CSRF Token
    pub fn verify_csrf_token(token: &str) -> Result<(), NetworkError>;
}
```

**安全增强**:
- 强制 TLS 1.2+
- 证书固定（Certificate Pinning）
- 超时和重试限制
- CSRF Token 验证

## 数据模型

### 安全配置模型

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// 加密配置
    pub crypto: CryptoConfig,
    /// 访问控制配置
    pub access_control: AccessControlConfig,
    /// 审计配置
    pub audit: AuditConfig,
    /// 网络安全配置
    pub network: NetworkSecurityConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoConfig {
    /// Argon2 内存成本（KB）
    pub argon2_memory_cost: u32,
    /// Argon2 时间成本（迭代次数）
    pub argon2_time_cost: u32,
    /// Argon2 并行度
    pub argon2_parallelism: u32,
    /// 最小密码长度
    pub min_password_length: usize,
    /// 密钥轮转周期（天）
    pub key_rotation_days: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// 是否启用审计日志
    pub enabled: bool,
    /// 日志级别
    pub log_level: LogLevel,
    /// 日志保留天数
    pub retention_days: u32,
    /// 最大日志文件大小（MB）
    pub max_file_size_mb: u32,
}
```

### 安全事件模型

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// 事件 ID
    pub id: Uuid,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 事件类型
    pub event_type: EventType,
    /// 严重程度
    pub severity: Severity,
    /// 用户标识
    pub user_id: Option<String>,
    /// 资源标识
    pub resource: Option<String>,
    /// 操作类型
    pub action: Option<String>,
    /// 操作结果
    pub result: EventResult,
    /// 附加信息
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    Authentication,
    Authorization,
    DataAccess,
    DataModification,
    ConfigChange,
    SecurityViolation,
    SystemError,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventResult {
    Success,
    Failure,
    Blocked,
}
```

### 漏洞跟踪模型

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct VulnerabilityReport {
    /// 漏洞 ID
    pub id: String,
    /// 发现时间
    pub discovered_at: DateTime<Utc>,
    /// 漏洞类型
    pub vulnerability_type: VulnerabilityType,
    /// 严重程度
    pub severity: Severity,
    /// 受影响组件
    pub affected_component: String,
    /// 描述
    pub description: String,
    /// 修复状态
    pub status: VulnerabilityStatus,
    /// 修复时间
    pub fixed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VulnerabilityType {
    Injection,
    BrokenAuthentication,
    SensitiveDataExposure,
    XmlExternalEntities,
    BrokenAccessControl,
    SecurityMisconfiguration,
    CrossSiteScripting,
    InsecureDeserialization,
    UsingComponentsWithKnownVulnerabilities,
    InsufficientLogging,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VulnerabilityStatus {
    Open,
    InProgress,
    Fixed,
    Mitigated,
    Accepted,
}
```


## 正确性属性

*属性是一个特征或行为，应该在系统的所有有效执行中保持为真——本质上是关于系统应该做什么的正式陈述。属性作为人类可读规范和机器可验证正确性保证之间的桥梁。*

### 加密和密钥管理属性

**属性 1: 密钥长度充足性**
*对于任意*密钥派生操作，生成的密钥长度应该至少为 256 位（32 字节）
**验证: 需求 1.1**

**属性 2: Nonce 唯一性**
*对于任意*多次加密操作序列，所有生成的 Nonce 应该是唯一的，不存在重复
**验证: 需求 2.2**

**属性 3: 密码强度验证**
*对于任意*密码输入，如果密码长度小于 12 或不包含大小写字母、数字和特殊字符，系统应该拒绝该密码
**验证: 需求 1.5**

**属性 4: 敏感数据清零**
*对于任意*敏感数据（密钥、密码、令牌），在使用完毕后，其内存区域应该被清零（全部为 0）
**验证: 需求 2.8, 4.3**

### 存储和权限属性

**属性 5: Unix 文件权限正确性**
*对于任意*在 Unix 系统上创建的敏感文件，其权限应该是 0600（仅所有者可读写）
**验证: 需求 2.3**

**属性 6: Unix 目录权限正确性**
*对于任意*在 Unix 系统上创建的敏感目录，其权限应该是 0700（仅所有者可访问）
**验证: 需求 2.4**

**属性 7: 安全删除有效性**
*对于任意*被安全删除的敏感文件，删除后应该无法通过常规方法恢复原始内容
**验证: 需求 2.7**

### 输入验证属性

**属性 8: 路径遍历防护**
*对于任意*文件路径输入，如果路径包含路径遍历模式（如 `../`, `..\\`），系统应该拒绝该路径
**验证: 需求 5.1**

**属性 9: 邮箱格式验证**
*对于任意*邮箱地址输入，如果不符合标准邮箱格式（user@domain.tld），系统应该拒绝该输入
**验证: 需求 5.3**

**属性 10: JSON 结构验证**
*对于任意*JSON 输入，如果不符合预定义的 schema，系统应该拒绝该输入
**验证: 需求 5.4**

**属性 11: 账户名称字符集限制**
*对于任意*账户名称输入，如果包含非法字符（如控制字符、路径分隔符），系统应该拒绝该输入
**验证: 需求 5.2**

### 数据库安全属性

**属性 12: SQL 参数化强制**
*对于任意*SQL 查询操作，查询应该使用参数化形式，不应该包含字符串拼接的用户输入
**验证: 需求 3.1**

**属性 13: 数据库路径验证**
*对于任意*数据库路径输入，路径应该在允许的目录白名单内，不应该指向系统敏感位置
**验证: 需求 3.2**

**属性 14: 备份数据加密**
*对于任意*数据库备份文件，其中的敏感字段（API 密钥、令牌）应该是加密的，不应该以明文形式存在
**验证: 需求 3.4**

**属性 15: 备份完整性验证**
*对于任意*备份文件恢复操作，系统应该验证备份文件的完整性（通过 HMAC 或认证加密的 tag）
**验证: 需求 3.5**

### 日志和审计属性

**属性 16: 日志敏感信息过滤**
*对于任意*日志记录操作，日志内容不应该包含敏感数据模式（如 API 密钥格式、密码、令牌）
**验证: 需求 4.5, 7.1**

**属性 17: 错误消息安全性**
*对于任意*错误处理，错误消息不应该包含敏感信息（完整文件路径、SQL 查询、密钥片段）
**验证: 需求 4.6, 9.1-9.4**

**属性 18: 审计日志完整性**
*对于任意*安全相关操作（认证、授权、数据访问），系统应该记录审计日志，包含时间戳、用户标识和操作结果
**验证: 需求 7.2**

### 内存安全属性

**属性 19: 缓存过期机制**
*对于任意*缓存的敏感信息，缓存应该在指定时间后自动失效并被清理
**验证: 需求 4.4**

**属性 20: 进程内存访问最小化**
*对于任意*进程内存扫描操作，访问的内存区域大小应该不超过必要的最小范围
**验证: 需求 4.1**

### 网络安全属性

**属性 21: TLS 版本强制**
*对于任意*网络连接，使用的 TLS 版本应该是 1.2 或更高
**验证: 需求 10.1**

**属性 22: 证书验证完整性**
*对于任意*TLS 连接，系统应该验证服务器证书的有效性、过期时间和证书链
**验证: 需求 10.2**

### 并发和竞态条件属性

**属性 23: 文件操作原子性**
*对于任意*文件检查和访问操作序列，应该使用原子操作，防止 TOCTOU（Time-of-Check-Time-of-Use）攻击
**验证: 需求 14.1**

**属性 24: 权限验证原子性**
*对于任意*权限验证和操作执行序列，验证和执行应该是原子的，不应该被中间状态打断
**验证: 需求 14.2**

### 数据完整性属性

**属性 25: 加密认证性**
*对于任意*加密数据，解密时应该验证认证标签（GCM tag），确保数据未被篡改
**验证: 需求 15.1**

**属性 26: 数据格式验证**
*对于任意*从外部源读取的数据，系统应该验证数据格式和版本兼容性
**验证: 需求 15.2**


## 错误处理

### 错误类型层次

```rust
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("加密错误: {0}")]
    Crypto(#[from] CryptoError),
    
    #[error("存储错误: {0}")]
    Storage(#[from] StorageError),
    
    #[error("验证错误: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("访问控制错误: {0}")]
    AccessControl(#[from] AccessError),
    
    #[error("数据库错误: {0}")]
    Database(#[from] DbError),
    
    #[error("网络错误: {0}")]
    Network(#[from] NetworkError),
    
    #[error("配置错误: {0}")]
    Configuration(String),
    
    #[error("内部错误")]
    Internal,
}
```

### 错误处理原则

1. **不泄露敏感信息**: 错误消息应该是通用的，不包含路径、查询、密钥等敏感细节
2. **详细日志记录**: 在安全日志中记录详细错误信息，用于调试和审计
3. **统一错误响应**: 对前端返回标准化的错误代码和消息
4. **快速失败**: 遇到安全错误时立即失败，不尝试降级或绕过
5. **审计记录**: 所有安全相关错误都应该记录到审计日志

### 错误处理示例

```rust
// 好的错误处理
pub fn decrypt_backup(path: &Path) -> Result<BackupData, SecurityError> {
    // 验证路径
    let validated_path = InputValidator::validate_path(path)
        .map_err(|e| {
            AuditLogger::log_security_event(SecurityEvent {
                event_type: EventType::SecurityViolation,
                severity: Severity::High,
                details: format!("Invalid backup path: {}", e),
                ..Default::default()
            });
            SecurityError::Validation(ValidationError::InvalidPath)
        })?;
    
    // 读取文件
    let encrypted_data = SecureStorageManager::read_file(&validated_path)
        .map_err(|e| {
            tracing::error!("Failed to read backup file: {}", e);
            // 不向用户泄露完整路径
            SecurityError::Storage(StorageError::ReadFailed)
        })?;
    
    // 解密
    let decrypted = decrypt_data(&encrypted_data)
        .map_err(|e| {
            AuditLogger::log_security_event(SecurityEvent {
                event_type: EventType::SecurityViolation,
                severity: Severity::Critical,
                details: "Backup decryption failed".to_string(),
                ..Default::default()
            });
            // 不泄露密钥或算法细节
            SecurityError::Crypto(CryptoError::DecryptionFailed)
        })?;
    
    Ok(decrypted)
}
```

## 测试策略

### 单元测试

**目标**: 验证单个安全组件的正确性

**测试内容**:
- 加密/解密功能的正确性
- 输入验证逻辑
- 权限检查函数
- 错误处理路径

**示例**:
```rust
#[test]
fn test_password_strength_validation() {
    // 弱密码应该被拒绝
    assert!(InputValidator::validate_password("weak").is_err());
    assert!(InputValidator::validate_password("12345678").is_err());
    
    // 强密码应该被接受
    assert!(InputValidator::validate_password("StrongP@ssw0rd123").is_ok());
}

#[test]
fn test_path_traversal_prevention() {
    // 路径遍历尝试应该被拒绝
    assert!(InputValidator::validate_path("../../../etc/passwd").is_err());
    assert!(InputValidator::validate_path("..\\..\\windows\\system32").is_err());
    
    // 合法路径应该被接受
    assert!(InputValidator::validate_path("backups/user@example.com.enc").is_ok());
}
```

### 属性测试

**目标**: 验证安全属性在大量随机输入下保持正确

**使用库**: `proptest` 或 `quickcheck`

**测试内容**:
- 密钥长度始终为 256 位
- Nonce 唯一性
- 敏感数据清零
- 文件权限正确性
- 输入验证的完整性

**示例**:
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_key_length_is_256_bits(seed in any::<u64>()) {
        let key = derive_machine_key_secure().unwrap();
        assert_eq!(key.len(), 32); // 256 bits = 32 bytes
    }
    
    #[test]
    fn prop_nonce_uniqueness(count in 1..1000usize) {
        let mut nonces = HashSet::new();
        for _ in 0..count {
            let nonce = generate_nonce();
            assert!(!nonces.contains(&nonce), "Nonce collision detected");
            nonces.insert(nonce);
        }
    }
    
    #[test]
    fn prop_path_traversal_rejected(malicious_path in ".*\\.\\..*") {
        assert!(InputValidator::validate_path(&malicious_path).is_err());
    }
    
    #[test]
    fn prop_sensitive_data_zeroed(data in prop::collection::vec(any::<u8>(), 1..1024)) {
        let mut secure_mem = SecureMemory::new(data.clone());
        drop(secure_mem);
        // 验证内存已被清零（需要 unsafe 代码或内存检查工具）
    }
}
```

### 集成测试

**目标**: 验证安全组件之间的交互

**测试内容**:
- 完整的备份和恢复流程
- 数据库访问和加密
- 网络通信和证书验证
- 进程管理和权限检查

**示例**:
```rust
#[tokio::test]
async fn test_secure_backup_restore_flow() {
    // 创建测试账户
    let account = create_test_account();
    
    // 备份
    let backup_result = backup_account(&account).await;
    assert!(backup_result.is_ok());
    
    // 验证备份文件权限
    let backup_path = get_backup_path(&account.email);
    assert_eq!(get_file_permissions(&backup_path), 0o600);
    
    // 验证备份文件已加密
    let raw_content = fs::read(&backup_path).unwrap();
    assert!(is_encrypted(&raw_content));
    
    // 恢复
    let restored = restore_account(&account.email).await;
    assert!(restored.is_ok());
    assert_eq!(restored.unwrap().email, account.email);
}
```

### 安全测试

**目标**: 主动寻找安全漏洞

**测试类型**:

1. **模糊测试 (Fuzzing)**
   - 使用 `cargo-fuzz` 或 `afl.rs`
   - 测试输入解析器、加密函数、文件处理

2. **静态分析**
   - 使用 `cargo-clippy` 检查常见安全问题
   - 使用 `cargo-audit` 检查依赖漏洞
   - 使用 `cargo-geiger` 检查 unsafe 代码

3. **动态分析**
   - 使用 `valgrind` 或 `miri` 检查内存安全
   - 使用 `strace`/`dtrace` 监控系统调用
   - 使用内存清理检查工具验证敏感数据清零

4. **渗透测试**
   - SQL 注入测试
   - 路径遍历测试
   - 权限提升测试
   - 信息泄露测试

### 持续安全监控

**CI/CD 集成**:
```yaml
# .github/workflows/security.yml
name: Security Audit

on: [push, pull_request]

jobs:
  security-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        
      - name: Run cargo-audit
        run: cargo audit
        
      - name: Run cargo-clippy
        run: cargo clippy -- -D warnings
        
      - name: Run security tests
        run: cargo test --features security-tests
        
      - name: Check for unsafe code
        run: cargo geiger
```

## 实施优先级

### 高优先级（立即修复）

1. **密钥派生增强** (需求 1.2)
   - 添加 Argon2 到机器密钥派生
   - 影响: 所有加密操作的安全性

2. **机器 ID 失败处理** (需求 1.3)
   - 移除默认值回退
   - 影响: 防止弱密钥生成

3. **SQL 注入防护审计** (需求 3.1)
   - 审查所有 SQL 查询
   - 影响: 数据库安全

4. **敏感数据日志过滤** (需求 4.5, 7.1)
   - 实施日志过滤器
   - 影响: 防止信息泄露

5. **Windows 文件权限** (需求 2.5)
   - 实施 Windows ACL 设置
   - 影响: Windows 用户的数据安全

### 中优先级（1-2 周内完成）

6. **输入验证框架** (需求 5.1-5.7)
   - 实施统一的输入验证
   - 影响: 防止注入攻击

7. **安全删除功能** (需求 2.7)
   - 实施文件覆写删除
   - 影响: 防止数据恢复

8. **审计日志系统** (需求 7.2-7.7)
   - 实施结构化审计日志
   - 影响: 安全监控和合规

9. **内存安全管理** (需求 2.8, 4.3)
   - 实施 SecureMemory 类型
   - 影响: 防止内存泄露

10. **网络安全配置** (需求 10.1-10.3)
    - 强化 TLS 配置
    - 影响: 网络通信安全

### 低优先级（持续改进）

11. **依赖项扫描自动化** (需求 8.1-8.7)
    - 集成 cargo-audit 到 CI
    - 影响: 供应链安全

12. **安全测试套件** (需求 18.1-18.7)
    - 建立完整的安全测试
    - 影响: 持续安全保证

13. **配置安全** (需求 13.1-13.7)
    - 加密敏感配置
    - 影响: 配置安全

14. **隐私保护** (需求 17.1-17.7)
    - 实施隐私保护措施
    - 影响: 用户隐私和合规

## 性能考虑

### 加密性能

- **Argon2 参数调优**: 平衡安全性和性能
  - 推荐: 内存成本 19MB, 时间成本 2, 并行度 1
  - 预期: 密钥派生时间 < 100ms

- **AES-GCM 硬件加速**: 利用 CPU AES-NI 指令
  - 预期: 加密/解密速度 > 1GB/s

### 输入验证性能

- **正则表达式编译缓存**: 避免重复编译
- **路径规范化**: 使用高效的路径处理库
- **预期**: 输入验证延迟 < 1ms

### 日志性能

- **异步日志**: 使用非阻塞日志写入
- **批量写入**: 减少 I/O 操作
- **预期**: 日志记录延迟 < 10μs

## 兼容性

### 平台兼容性

- **Windows**: 完整支持，包括 ACL 设置
- **macOS**: 完整支持，包括 Unix 权限
- **Linux**: 完整支持，包括 Unix 权限

### 向后兼容性

- **旧备份文件**: 支持读取明文 .json 备份，提示用户升级
- **配置迁移**: 自动迁移旧配置到新格式
- **API 兼容**: 保持现有 Tauri 命令接口不变

## 文档和培训

### 开发者文档

- **安全编码指南**: 详细的安全最佳实践
- **API 文档**: 所有安全组件的使用说明
- **安全审查清单**: 代码审查时的安全检查项

### 用户文档

- **安全特性说明**: 向用户解释安全措施
- **最佳实践指南**: 用户如何安全使用应用
- **故障排除**: 常见安全相关问题的解决方案

### 安全响应流程

- **漏洞报告**: 建立安全漏洞报告渠道
- **响应时间**: 高危漏洞 48 小时内响应
- **补丁发布**: 安全补丁优先发布
- **公开披露**: 修复后 90 天公开漏洞详情
