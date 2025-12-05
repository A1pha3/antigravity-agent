//! 安全加密模块
//! 使用 AES-256-GCM 加密敏感数据，Argon2 进行密钥派生

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

/// 加密错误类型
#[derive(Debug)]
pub enum CryptoError {
    EncryptionFailed(String),
    DecryptionFailed(String),
    KeyDerivationFailed(String),
    IoError(String),
    InvalidData(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::EncryptionFailed(msg) => write!(f, "加密失败: {}", msg),
            CryptoError::DecryptionFailed(msg) => write!(f, "解密失败: {}", msg),
            CryptoError::KeyDerivationFailed(msg) => write!(f, "密钥派生失败: {}", msg),
            CryptoError::IoError(msg) => write!(f, "IO错误: {}", msg),
            CryptoError::InvalidData(msg) => write!(f, "无效数据: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

/// 加密文件头魔数（用于识别加密文件）
const ENCRYPTED_FILE_MAGIC: &[u8] = b"AGCRYPT1";
/// Nonce 长度（12 字节）
const NONCE_SIZE: usize = 12;
/// Salt 长度（16 字节）
const SALT_SIZE: usize = 16;

/// 从机器特征派生加密密钥（无需用户输入密码）
/// 使用机器 ID + 用户名 + 固定盐值生成唯一密钥
pub fn derive_machine_key() -> Result<[u8; 32], CryptoError> {
    let machine_id = get_machine_id();
    let username = whoami::username();
    let app_salt = "antigravity-agent-v1";
    
    let combined = format!("{}:{}:{}", machine_id, username, app_salt);
    
    // 使用 SHA-256 生成 32 字节密钥
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    
    Ok(key)
}

/// 从密码派生加密密钥（用于导入导出功能）
pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<[u8; 32], CryptoError> {
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
    
    Ok(key)
}

/// 使用 AES-256-GCM 加密数据
pub fn encrypt_data(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
    
    // 生成随机 nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // 加密
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
    
    // 组合输出: magic + nonce + ciphertext
    let mut output = Vec::with_capacity(ENCRYPTED_FILE_MAGIC.len() + NONCE_SIZE + ciphertext.len());
    output.extend_from_slice(ENCRYPTED_FILE_MAGIC);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    
    Ok(output)
}

/// 使用 AES-256-GCM 解密数据
pub fn decrypt_data(encrypted: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
    // 验证魔数
    if encrypted.len() < ENCRYPTED_FILE_MAGIC.len() + NONCE_SIZE + 16 {
        return Err(CryptoError::InvalidData("数据太短".to_string()));
    }
    
    if &encrypted[..ENCRYPTED_FILE_MAGIC.len()] != ENCRYPTED_FILE_MAGIC {
        return Err(CryptoError::InvalidData("不是加密文件".to_string()));
    }
    
    let nonce_start = ENCRYPTED_FILE_MAGIC.len();
    let nonce_end = nonce_start + NONCE_SIZE;
    let nonce = Nonce::from_slice(&encrypted[nonce_start..nonce_end]);
    
    let ciphertext = &encrypted[nonce_end..];
    
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
    
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
}

/// 使用密码加密数据（用于导入导出）
pub fn encrypt_with_password(plaintext: &[u8], password: &str) -> Result<Vec<u8>, CryptoError> {
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
    if encrypted.len() < SALT_SIZE + ENCRYPTED_FILE_MAGIC.len() + NONCE_SIZE + 16 {
        return Err(CryptoError::InvalidData("数据太短".to_string()));
    }
    
    let salt = &encrypted[..SALT_SIZE];
    let ciphertext = &encrypted[SALT_SIZE..];
    
    let key = derive_key_from_password(password, salt)?;
    decrypt_data(ciphertext, &key)
}

/// 检查数据是否已加密
pub fn is_encrypted(data: &[u8]) -> bool {
    data.len() >= ENCRYPTED_FILE_MAGIC.len() 
        && &data[..ENCRYPTED_FILE_MAGIC.len()] == ENCRYPTED_FILE_MAGIC
}

/// 检查文件是否已加密（带 salt 前缀）
pub fn is_encrypted_with_salt(data: &[u8]) -> bool {
    data.len() >= SALT_SIZE + ENCRYPTED_FILE_MAGIC.len()
        && &data[SALT_SIZE..SALT_SIZE + ENCRYPTED_FILE_MAGIC.len()] == ENCRYPTED_FILE_MAGIC
}

/// 获取机器唯一标识
fn get_machine_id() -> String {
    #[cfg(target_os = "macos")]
    {
        // macOS: 使用 IOPlatformUUID
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
            .unwrap_or_else(|| "default-mac-id".to_string())
    }
    
    #[cfg(target_os = "windows")]
    {
        // Windows: 使用 MachineGuid
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
            .unwrap_or_else(|| "default-win-id".to_string())
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux: 使用 /etc/machine-id
        fs::read_to_string("/etc/machine-id")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "default-linux-id".to_string())
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        "default-unknown-id".to_string()
    }
}

/// 安全写入文件（设置严格权限）
pub fn secure_write_file(path: &Path, data: &[u8]) -> Result<(), CryptoError> {
    // 先写入文件
    fs::write(path, data).map_err(|e| CryptoError::IoError(e.to_string()))?;
    
    // 设置严格权限（仅 Unix）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(path, permissions)
            .map_err(|e| CryptoError::IoError(format!("设置文件权限失败: {}", e)))?;
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
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_machine_key_derivation() {
        let key1 = derive_machine_key().unwrap();
        let key2 = derive_machine_key().unwrap();
        assert_eq!(key1, key2, "同一机器应生成相同密钥");
    }
    
    #[test]
    fn test_encrypt_decrypt() {
        let key = derive_machine_key().unwrap();
        let plaintext = b"Hello, World! This is a test message.";
        
        let encrypted = encrypt_data(plaintext, &key).unwrap();
        let decrypted = decrypt_data(&encrypted, &key).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }
    
    #[test]
    fn test_password_encrypt_decrypt() {
        let password = "test-password-123";
        let plaintext = b"Sensitive data here";
        
        let encrypted = encrypt_with_password(plaintext, password).unwrap();
        let decrypted = decrypt_with_password(&encrypted, password).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }
    
    #[test]
    fn test_wrong_password() {
        let plaintext = b"Sensitive data";
        let encrypted = encrypt_with_password(plaintext, "correct-password").unwrap();
        
        let result = decrypt_with_password(&encrypted, "wrong-password");
        assert!(result.is_err());
    }
}
