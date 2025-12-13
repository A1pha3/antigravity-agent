// Antigravity 用户数据备份模块
// 负责将 Antigravity 应用数据备份到加密文件
// 使用 AES-256-GCM 加密，基于机器特征派生密钥

use rusqlite::{Connection, OptionalExtension};
use serde_json::Value;
use std::fs;
use std::path::Path;

use crate::constants::database;
use crate::path_utils::AppPaths;
use crate::utils::crypto::{encrypt_machine_data, decrypt_machine_data, secure_write_file, secure_create_dir, is_encrypted};

/// 智能备份 Antigravity 账户（终极版 - 保存完整 Marker）
///
/// 备份策略：
/// 1. 保存所有关键字段的原始字符串值
/// 2. 保存完整的 __$__targetStorageMarker 对象（作为恢复时的参考）
/// 3. 保存 __$__isNewStorageMarker 状态标记
///
/// # 参数
/// - `email`: 用户邮箱
///
/// # 返回
/// - `Ok((backup_name, is_overwrite))`: 备份文件名和是否为覆盖操作
/// - `Err(message)`: 错误信息
pub fn smart_backup_antigravity_account(email: &str) -> Result<(String, bool), String> {
    tracing::info!("🔧 执行智能备份（加密模式），邮箱: {}", email);

    let config_dir = AppPaths::backup_dir().ok_or("无法获取备份目录")?;
    
    // 使用安全方式创建目录（设置 0700 权限）
    secure_create_dir(&config_dir).map_err(|e| e.to_string())?;

    // 简单的覆盖逻辑：每个邮箱只保留一个备份
    // 使用 .enc 扩展名表示加密文件
    let backup_name = email.to_string();
    let encrypted_file = config_dir.join(format!("{}.enc", backup_name));
    let legacy_file = config_dir.join(format!("{}.json", backup_name));
    let is_overwrite = encrypted_file.exists() || legacy_file.exists();

    let app_data = AppPaths::antigravity_data_dir()
        .map(|path| path.join("state.vscdb"))
        .ok_or("未找到数据库路径")?;

    if !app_data.exists() {
        return Err(format!("数据库文件不存在: {}", app_data.display()));
    }

    let conn = Connection::open(&app_data).map_err(|e| e.to_string())?;

    // 使用常量定义所有需要备份的关键字段
    let keys_to_backup = database::ALL_KEYS;

    let mut data_map = serde_json::Map::new();

    // 1. 提取数据（保持原始字符串格式）
    for key in keys_to_backup {
        let val: Option<String> = conn
            .query_row("SELECT value FROM ItemTable WHERE key = ?", [key], |row| {
                row.get(0)
            })
            .optional()
            .unwrap_or(None);

        if let Some(v) = val {
            data_map.insert(key.to_string(), Value::String(v));
        } else {
            tracing::debug!(target: "backup::database", key = %key, "字段不存在，跳过");
        }
    }

    // 2. 提取并解析 Marker（作为恢复时的参考书）
    let marker_json: Option<String> = conn
        .query_row(
            &format!(
                "SELECT value FROM ItemTable WHERE key = '{}'",
                database::TARGET_STORAGE_MARKER
            ),
            [],
            |row| row.get(0),
        )
        .optional()
        .unwrap_or(None);

    if let Some(m) = marker_json {
        // 将 Marker 解析为对象存入备份
        match serde_json::from_str::<Value>(&m) {
            Ok(parsed_marker) => {
                tracing::debug!(target: "backup::database", "备份完整 Marker（作为恢复参考）");
                data_map.insert(database::TARGET_STORAGE_MARKER.to_string(), parsed_marker);
            },
            Err(e) => {
                tracing::warn!(target: "backup::database", error = %e, "Marker JSON 解析失败，跳过该字段");
            }
        }
    }

    // 3. 添加元信息
    data_map.insert(
        "account_email".to_string(),
        Value::String(email.to_string()),
    );
    data_map.insert(
        "backup_time".to_string(),
        Value::String(chrono::Local::now().to_rfc3339()),
    );

    // 4. 加密并写入备份文件
    let backup_file = config_dir.join(format!("{}.enc", backup_name));
    let file_content = serde_json::to_string_pretty(&data_map).map_err(|e| e.to_string())?;
    
    // 派生机器密钥并加密 (自动使用 Argon2 V2)
    let encrypted_content = encrypt_machine_data(file_content.as_bytes())
        .map_err(|e| format!("加密失败: {}", e))?;
    
    // 使用安全方式写入文件（设置 0600 权限）
    secure_write_file(&backup_file, &encrypted_content).map_err(|e| e.to_string())?;
    
    // 删除旧的明文备份文件（如果存在）
    let legacy_file = config_dir.join(format!("{}.json", backup_name));
    if legacy_file.exists() {
        if let Err(e) = fs::remove_file(&legacy_file) {
            tracing::warn!(target: "backup::database", error = %e, "删除旧明文备份失败");
        } else {
            tracing::info!(target: "backup::database", "已删除旧明文备份文件");
        }
    }

    let action = if is_overwrite { "覆盖" } else { "创建" };
    tracing::info!(target: "backup::database", action = %action, file = %backup_file.display(), "加密备份成功");
    Ok((backup_name, is_overwrite))
}

/// 读取备份文件（支持加密和明文格式）
/// 
/// 自动检测文件格式：
/// - .enc 文件：使用机器密钥解密（自动支持 V1 SHA-256 和 V2 Argon2）
/// - .json 文件：直接读取（兼容旧版本）
pub fn read_backup_file(backup_path: &Path) -> Result<Value, String> {
    let content = fs::read(backup_path)
        .map_err(|e| format!("读取备份文件失败: {}", e))?;
    
    // 检查是否为加密文件
    if is_encrypted(&content) {
        tracing::debug!(target: "backup::read", "检测到加密备份文件，正在解密");
        
        let decrypted = decrypt_machine_data(&content)
            .map_err(|e| format!("解密失败: {}", e))?;
            
        let json_str = String::from_utf8(decrypted)
            .map_err(|e| format!("UTF-8 解码失败: {}", e))?;
        serde_json::from_str(&json_str)
            .map_err(|e| format!("JSON 解析失败: {}", e))
    } else {
        // 明文 JSON 文件（兼容旧版本）
        tracing::warn!(target: "backup::read", "检测到明文备份文件，建议重新备份以加密");
        let json_str = String::from_utf8(content)
            .map_err(|e| format!("UTF-8 解码失败: {}", e))?;
        serde_json::from_str(&json_str)
            .map_err(|e| format!("JSON 解析失败: {}", e))
    }
}
