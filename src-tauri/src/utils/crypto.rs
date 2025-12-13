//! 安全加密模块
//! 使用 AES-256-GCM 加密敏感数据，Argon2 进行密钥派生

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::SaltString,
    Argon2, PasswordHasher, Params, Algorithm, Version
};
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use zeroize::Zeroizing;

/// 加密错误类型
#[derive(Debug)]
pub enum CryptoError {
    EncryptionFailed(String),
    DecryptionFailed(String),
    KeyDerivationFailed(String),
    IoError(String),
    InvalidData(String),
    WeakPassword(String),
    MachineIdError(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::EncryptionFailed(msg) => write!(f, "加密失败: {}", msg),
            CryptoError::DecryptionFailed(msg) => write!(f, "解密失败: {}", msg),
            CryptoError::KeyDerivationFailed(msg) => write!(f, "密钥派生失败: {}", msg),
            CryptoError::IoError(msg) => write!(f, "IO错误: {}", msg),
            CryptoError::InvalidData(msg) => write!(f, "无效数据: {}", msg),
            CryptoError::WeakPassword(msg) => write!(f, "密码强度不足: {}", msg),
            CryptoError::MachineIdError(msg) => write!(f, "机器ID获取失败: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

/// 加密文件头魔数（V1: SHA-256）
const ENCRYPTED_FILE_MAGIC_V1: &[u8] = b"AGCRYPT1";
/// 加密文件头魔数（V2: Argon2id）
const ENCRYPTED_FILE_MAGIC_V2: &[u8] = b"AGCRYPT2";
/// Nonce 长度（12 字节）
const NONCE_SIZE: usize = 12;
/// Salt 长度（16 字节）
const SALT_SIZE: usize = 16;

/// 验证密码强度
/// 要求：长度 >= 12，包含数字、小写、大写、特殊字符
pub fn validate_password_strength(password: &str) -> Result<(), CryptoError> {
    if password.len() < 12 {
        return Err(CryptoError::WeakPassword("密码长度必须至少为 12 个字符".to_string()));
    }

    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

    if !has_digit || !has_lower || !has_upper || !has_special {
        return Err(CryptoError::WeakPassword("密码必须包含数字、大小写字母和特殊字符".to_string()));
    }

    Ok(())
}

/// 获取机器唯一标识（V2：强制错误处理）
fn get_machine_id() -> Result<String, CryptoError> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .ok()
            .and_then(|output| {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .find(|line| line.contains("IOPlatformUUID"))
                    .and_then(|line| {
                        line.split('"')
                            .nth(3)
                            .map(|s| s.to_string())
                    })
            })
            .ok_or_else(|| CryptoError::MachineIdError("无法获取 macOS IOPlatformUUID".to_string()))
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("reg")
            .args(["query", "HKLM\\SOFTWARE\\Microsoft\\Cryptography", "/v", "MachineGuid"])
            .output()
            .ok()
            .and_then(|output| {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .find(|line| line.contains("MachineGuid"))
                    .and_then(|line| {
                        line.split_whitespace().last().map(|s| s.to_string())
                    })
            })
            .ok_or_else(|| CryptoError::MachineIdError("无法获取 Windows MachineGuid".to_string()))
    }
    
    #[cfg(target_os = "linux")]
    {
        fs::read_to_string("/etc/machine-id")
            .map(|s| s.trim().to_string())
            .map_err(|e| CryptoError::MachineIdError(format!("无法读取 /etc/machine-id: {}", e)))
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err(CryptoError::MachineIdError("不支持的操作系统".to_string()))
    }
}

/// 旧版机器密钥派生 (V1: SHA-256) - 仅用于兼容性解密
fn derive_machine_key_v1() -> Result<Zeroizing<[u8; 32]>, CryptoError> {
    // 为了兼容旧逻辑，我们需要模拟原本的 get_machine_id 的行为（包括默认值回退）
    // 但是这里我们已经重构了 get_machine_id。
    // 如果我们想完美兼容旧数据的解密，我们必须完全复刻旧逻辑，包括 fallback。
    // 下面是旧版 get_machine_id 的逻辑复刻：
    let machine_id = match get_machine_id() {
        Ok(id) => id,
        Err(_) => {
            #[cfg(target_os = "macos")] { "default-mac-id".to_string() }
            #[cfg(target_os = "windows")] { "default-win-id".to_string() }
            #[cfg(target_os = "linux")] { "default-linux-id".to_string() }
            #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))] { "default-unknown-id".to_string() }
        }
    };

    let username = whoami::username();
    let app_salt = "antigravity-agent-v1";
    
    let combined = format!("{}:{}:{}", machine_id, username, app_salt);
    
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    
    Ok(Zeroizing::new(key))
}

/// 新版机器密钥派生 (V2: Argon2id)
fn derive_machine_key_v2() -> Result<Zeroizing<[u8; 32]>, CryptoError> {
    let machine_id = get_machine_id()?; // 必须成功，不回退
    let username = whoami::username();
    let app_salt = "antigravity-agent-v2-argon2"; // 更新 Salt
    
    let input_material = format!("{}:{}", machine_id, username);
    let salt = format!("{}:{}", app_salt, machine_id); // Salt 混入 machine_id 增加熵

    // Argon2 配置: 19MB 内存, 2 passes, 1 lane
    let params = Params::new(19456, 2, 1, Some(32)).map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; 32];
    argon2.hash_password_into(input_material.as_bytes(), salt.as_bytes(), &mut key)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;
        
    Ok(Zeroizing::new(key))
}

/// 自动加密机器数据 (使用 V2)
pub fn encrypt_machine_data(plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let key = derive_machine_key_v2()?;
    
    let cipher = Aes256Gcm::new_from_slice(&*key)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
    
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
    
    let mut output = Vec::with_capacity(ENCRYPTED_FILE_MAGIC_V2.len() + NONCE_SIZE + ciphertext.len());
    output.extend_from_slice(ENCRYPTED_FILE_MAGIC_V2);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    
    Ok(output)
}

/// 自动解密机器数据 (支持 V1 和 V2)
pub fn decrypt_machine_data(encrypted: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if encrypted.len() < ENCRYPTED_FILE_MAGIC_V1.len() {
        return Err(CryptoError::InvalidData("数据太短".to_string()));
    }

    // 检查是否是 V2
    if encrypted.starts_with(ENCRYPTED_FILE_MAGIC_V2) {
        let key = derive_machine_key_v2()?;
        return decrypt_data_internal(encrypted, &key, ENCRYPTED_FILE_MAGIC_V2.len());
    }

    // 检查是否是 V1
    if encrypted.starts_with(ENCRYPTED_FILE_MAGIC_V1) {
        let key = derive_machine_key_v1()?;
        return decrypt_data_internal(encrypted, &key, ENCRYPTED_FILE_MAGIC_V1.len());
    }

    Err(CryptoError::InvalidData("未知的文件格式或版本".to_string()))
}

/// 内部解密逻辑
fn decrypt_data_internal(encrypted: &[u8], key: &[u8; 32], magic_len: usize) -> Result<Vec<u8>, CryptoError> {
    if encrypted.len() < magic_len + NONCE_SIZE + 16 {
        return Err(CryptoError::InvalidData("数据太短".to_string()));
    }
    
    let nonce_start = magic_len;
    let nonce_end = nonce_start + NONCE_SIZE;
    let nonce = Nonce::from_slice(&encrypted[nonce_start..nonce_end]);
    let ciphertext = &encrypted[nonce_end..];
    
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
    
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
}

/// 保持向后兼容的公开 API (如果需要)
/// 注意：这个函数现在被标记为过时，建议使用 encrypt_machine_data
pub fn derive_machine_key() -> Result<[u8; 32], CryptoError> {
    let key = derive_machine_key_v1()?;
    Ok(*key) // 返回拷贝，注意这里是不安全的（内存残留），但为了兼容性保留
}

// ... encrypt_data 和 decrypt_data 保留原样或重定向 ...
// 为了兼容性，encrypt_data 仍使用旧逻辑？不，最好让它成为底层函数。
// 让我们保留 encrypt_data 作为底层函数，但只接受 Key。

/// 底层加密函数 (接受 Key)
pub fn encrypt_data(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
    // 默认使用 V1 Magic ? 不，这个函数是通用的。
    // 但是为了兼容旧代码调用，旧代码期望 AGCRYPT1。
    // 如果我们想让旧代码透明升级，我们可以让这里使用 V2。
    // 但是调用者自己派生了 Key。如果调用者用 V1 Key，我们却写 V2 Magic，那就乱了。
    // 所以：encrypt_data 应该只负责加密，不负责 Magic？
    // 原来的实现是负责 Magic 的。
    // 既然我们提供了 encrypt_machine_data，那个函数负责 Magic。
    // 这个函数如果被直接调用，我们假设它是 V1 行为（为了兼容）。
    
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
    
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
    
    // 使用 V1 Magic 保持兼容
    let mut output = Vec::with_capacity(ENCRYPTED_FILE_MAGIC_V1.len() + NONCE_SIZE + ciphertext.len());
    output.extend_from_slice(ENCRYPTED_FILE_MAGIC_V1);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    
    Ok(output)
}

/// 底层解密函数 (接受 Key)
pub fn decrypt_data(encrypted: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
    // 尝试匹配 V1
    if encrypted.starts_with(ENCRYPTED_FILE_MAGIC_V1) {
         return decrypt_data_internal(encrypted, key, ENCRYPTED_FILE_MAGIC_V1.len());
    }
    // 尝试匹配 V2 (如果调用者传入了 Key，但数据是 V2 格式)
    if encrypted.starts_with(ENCRYPTED_FILE_MAGIC_V2) {
         return decrypt_data_internal(encrypted, key, ENCRYPTED_FILE_MAGIC_V2.len());
    }
    
    Err(CryptoError::InvalidData("未知的文件格式".to_string()))
}

/// 从密码派生加密密钥（用于导入导出功能）
pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<Zeroizing<[u8; 32]>, CryptoError> {
    let argon2 = Argon2::default();
    
    // 将 salt 转换为 SaltString
    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;
    
    // 从 hash 中提取 32 字节密钥
    let hash_bytes = password_hash.hash.ok_or_else(|| {
        CryptoError::KeyDerivationFailed("无法获取哈希值".to_string())
    })?;
    
    let mut key = [0u8; 32];
    let hash_slice = hash_bytes.as_bytes();
    let copy_len = std::cmp::min(32, hash_slice.len());
    key[..copy_len].copy_from_slice(&hash_slice[..copy_len]);
    
    Ok(Zeroizing::new(key))
}

/// 使用密码加密数据（用于导入导出）
pub fn encrypt_with_password(plaintext: &[u8], password: &str) -> Result<Vec<u8>, CryptoError> {
    // 验证密码强度
    validate_password_strength(password)?;

    // 生成随机 salt
    let mut salt = [0u8; SALT_SIZE];
    OsRng.fill_bytes(&mut salt);
    
    // 派生密钥
    let key = derive_key_from_password(password, &salt)?;
    
    // 加密
    let encrypted = encrypt_data(plaintext, &key)?;
    
    // 组合输出: salt + encrypted
    let mut output = Vec::with_capacity(SALT_SIZE + encrypted.len());
    output.extend_from_slice(&salt);
    output.extend_from_slice(&encrypted);
    
    Ok(output)
}

/// 使用密码解密数据（用于导入导出）
pub fn decrypt_with_password(encrypted: &[u8], password: &str) -> Result<Vec<u8>, CryptoError> {
    // 注意：解密时不验证密码强度，以免以前的弱密码数据无法解密
    if encrypted.len() < SALT_SIZE + ENCRYPTED_FILE_MAGIC_V1.len() + NONCE_SIZE + 16 {
        return Err(CryptoError::InvalidData("数据太短".to_string()));
    }
    
    let salt = &encrypted[..SALT_SIZE];
    let ciphertext = &encrypted[SALT_SIZE..];
    
    let key = derive_key_from_password(password, salt)?;
    decrypt_data(ciphertext, &key)
}

/// 检查数据是否已加密
pub fn is_encrypted(data: &[u8]) -> bool {
    (data.len() >= ENCRYPTED_FILE_MAGIC_V1.len() 
        && &data[..ENCRYPTED_FILE_MAGIC_V1.len()] == ENCRYPTED_FILE_MAGIC_V1) ||
    (data.len() >= ENCRYPTED_FILE_MAGIC_V2.len() 
        && &data[..ENCRYPTED_FILE_MAGIC_V2.len()] == ENCRYPTED_FILE_MAGIC_V2)
}

/// 检查文件是否已加密（带 salt 前缀）
pub fn is_encrypted_with_salt(data: &[u8]) -> bool {
    (data.len() >= SALT_SIZE + ENCRYPTED_FILE_MAGIC_V1.len()
        && &data[SALT_SIZE..SALT_SIZE + ENCRYPTED_FILE_MAGIC_V1.len()] == ENCRYPTED_FILE_MAGIC_V1) ||
    (data.len() >= SALT_SIZE + ENCRYPTED_FILE_MAGIC_V2.len()
        && &data[SALT_SIZE..SALT_SIZE + ENCRYPTED_FILE_MAGIC_V2.len()] == ENCRYPTED_FILE_MAGIC_V2)
}

/// 安全写入文件（设置严格权限）
pub fn secure_write_file(path: &Path, data: &[u8]) -> Result<(), CryptoError> {
    // 先写入文件
    fs::write(path, data).map_err(|e| CryptoError::IoError(e.to_string()))?;
    
    // 设置严格权限（Unix）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(path, permissions)
            .map_err(|e| CryptoError::IoError(format!("设置文件权限失败: {}", e)))?;
    }

    // 设置严格权限（Windows）
    #[cfg(target_os = "windows")]
    {
        // 使用 icacls 设置权限
        // /inheritance:r - 移除继承的权限
        // /grant:r "%USERNAME%":F - 授予当前用户完全控制权限
        // 注意：我们使用 whoami crate 获取用户名，而不是依赖 %USERNAME% 环境变量
        let username = whoami::username();
        
        // Windows 用户名可能包含域/机器名，icacls 需要正确的格式
        // 简单起见，我们尝试使用当前用户的 SID 或者直接让 icacls 处理当前用户
        // 实际上 %USERNAME% 在 Command 中不会自动展开，我们需要手动构建
        
        let output = std::process::Command::new("icacls")
            .arg(path)
            .arg("/inheritance:r")
            .arg("/grant:r")
            .arg(format!("{}:F", username))
            .output()
            .map_err(|e| CryptoError::IoError(format!("执行 icacls 失败: {}", e)))?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CryptoError::IoError(format!("icacls 设置权限失败: {}", stderr)));
        }
    }
    
    Ok(())
}

/// 安全创建目录（设置严格权限）
pub fn secure_create_dir(path: &Path) -> Result<(), CryptoError> {
    fs::create_dir_all(path).map_err(|e| CryptoError::IoError(e.to_string()))?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o700);
        fs::set_permissions(path, permissions)
            .map_err(|e| CryptoError::IoError(format!("设置目录权限失败: {}", e)))?;
    }
    
    #[cfg(target_os = "windows")]
    {
        let username = whoami::username();
        
        // 目录权限需要继承选项 (OI)(CI)
        let output = std::process::Command::new("icacls")
            .arg(path)
            .arg("/inheritance:r")
            .arg("/grant:r")
            .arg(format!("{}:(OI)(CI)F", username))
            .output()
            .map_err(|e| CryptoError::IoError(format!("执行 icacls 失败: {}", e)))?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CryptoError::IoError(format!("icacls 设置权限失败: {}", stderr)));
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_machine_key_derivation_v1() {
        let key1 = derive_machine_key_v1().unwrap();
        let key2 = derive_machine_key_v1().unwrap();
        assert_eq!(*key1, *key2, "同一机器应生成相同密钥 (V1)");
    }

    #[test]
    fn test_machine_key_derivation_v2() {
        // V2 依赖机器 ID，如果测试环境无法获取机器 ID，此测试会失败。
        // 我们假设在开发机上能获取。
        if let Ok(key1) = derive_machine_key_v2() {
            let key2 = derive_machine_key_v2().unwrap();
            assert_eq!(*key1, *key2, "同一机器应生成相同密钥 (V2)");
        } else {
            // 在某些 CI 环境可能无法获取机器 ID，跳过
            println!("无法获取机器 ID，跳过 V2 密钥一致性测试");
        }
    }
    
    #[test]
    fn test_encrypt_decrypt_v1_legacy() {
        let key = derive_machine_key().unwrap(); // Legacy V1
        let plaintext = b"Hello, World! V1 Legacy";
        
        let encrypted = encrypt_data(plaintext, &key).unwrap();
        assert!(encrypted.starts_with(ENCRYPTED_FILE_MAGIC_V1));

        let decrypted = decrypt_data(&encrypted, &key).unwrap();
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_machine_auto() {
        let plaintext = b"Hello, World! Machine Auto V2";
        
        // 如果无法获取机器 ID，此测试会 panic。
        // 在本地开发环境这是预期的。
        if get_machine_id().is_err() {
            return;
        }

        let encrypted = encrypt_machine_data(plaintext).unwrap();
        assert!(encrypted.starts_with(ENCRYPTED_FILE_MAGIC_V2));

        let decrypted = decrypt_machine_data(&encrypted).unwrap();
        assert_eq!(plaintext.to_vec(), decrypted);
    }
    
    #[test]
    fn test_password_encrypt_decrypt() {
        let password = "Test-Password-123!"; // 强密码
        let plaintext = b"Sensitive data here";
        
        let encrypted = encrypt_with_password(plaintext, password).unwrap();
        let decrypted = decrypt_with_password(&encrypted, password).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }
    
    #[test]
    fn test_weak_password() {
        let weak_pass = "123456";
        let result = validate_password_strength(weak_pass);
        assert!(result.is_err());
        
        let result_enc = encrypt_with_password(b"data", weak_pass);
        assert!(matches!(result_enc, Err(CryptoError::WeakPassword(_))));
    }

    #[test]
    fn test_wrong_password() {
        let plaintext = b"Sensitive data";
        let password = "Correct-Password-1!";
        let encrypted = encrypt_with_password(plaintext, password).unwrap();
        
        let result = decrypt_with_password(&encrypted, "Wrong-Password-1!");
        assert!(result.is_err());
    }

    #[test]
    fn test_v1_v2_compatibility() {
        // 模拟一个 V1 加密的数据
        let key_v1 = derive_machine_key_v1().unwrap();
        let plaintext = b"Data from old version";
        let encrypted_v1 = encrypt_data(plaintext, &key_v1).unwrap();

        // 使用 decrypt_machine_data 解密 V1 数据
        // 注意：decrypt_machine_data 内部会自动调用 derive_machine_key_v1
        // 前提是 derive_machine_key_v1 在测试环境能工作
        if derive_machine_key_v1().is_ok() {
            let decrypted = decrypt_machine_data(&encrypted_v1).unwrap();
            assert_eq!(plaintext.to_vec(), decrypted);
        }
    }
}

