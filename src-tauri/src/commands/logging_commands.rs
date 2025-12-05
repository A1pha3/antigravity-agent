//! 日志相关命令
//! 提供日志管理功能

use dirs;
use std::fs;
use crate::utils::log_sanitizer::LogSanitizer;

/// 获取日志目录路径
/// 与 state.rs 中的配置目录保持一致
fn get_log_directory() -> std::path::PathBuf {
    if cfg!(windows) {
        // Windows: 优先使用 APPDATA 环境变量
        std::env::var_os("APPDATA")
            .map(|appdata| std::path::PathBuf::from(appdata).join(".antigravity-agent"))
            .or_else(|| {
                // 备用方案：通过用户主目录构建 AppData\Roaming 路径
                dirs::home_dir().map(|home| {
                    home.join("AppData")
                        .join("Roaming")
                        .join(".antigravity-agent")
                })
            })
            .or_else(|| {
                // 最后备用：使用系统标准配置目录
                dirs::config_dir().map(|config| config.join(".antigravity-agent"))
            })
            .unwrap_or_else(|| std::path::PathBuf::from(".antigravity-agent"))
            .join("logs")
    } else {
        // macOS/Linux: 使用标准配置目录
        dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(".antigravity-agent")
            .join("logs")
    }
}

/// 获取日志文件信息
/// 返回日志文件路径、大小等信息，用于前端显示状态
#[tauri::command]
pub async fn get_log_info() -> Result<LogInfo, String> {
    let log_dir = get_log_directory();
    let log_file = log_dir.join("antigravity-agent.log");

    if log_file.exists() {
        let metadata = fs::metadata(&log_file).map_err(|e| format!("获取文件信息失败: {}", e))?;

        let modified = metadata
            .modified()
            .map_err(|e| format!("获取修改时间失败: {}", e))?;

        let modified_str = chrono::DateTime::<chrono::Utc>::from(modified)
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();

        Ok(LogInfo {
            exists: true,
            path: "日志文件路径".to_string(),
            size_bytes: metadata.len(),
            size_human: format_file_size(metadata.len()),
            last_modified: modified_str,
        })
    } else {
        Ok(LogInfo {
            exists: false,
            path: "日志文件路径".to_string(),
            size_bytes: 0,
            size_human: "0 B".to_string(),
            last_modified: "不存在".to_string(),
        })
    }
}

/// 清空日志文件
/// 删除当前日志文件内容，但保留文件本身
#[tauri::command]
pub async fn write_text_file(path: String, content: String) -> Result<String, String> {
    crate::log_async_command!("write_text_file", async {
        use std::fs;
        use std::path::Path;

        let file_path = Path::new(&path);

        // 确保父目录存在
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }

        // 写入文件
        fs::write(&file_path, content).map_err(|e| format!("写入文件失败: {}", e))?;

        tracing::info!("📝 文件写入成功: {}", path);
        Ok("文件写入成功".to_string())
    })
}

/// 解密配置数据 - 接收文件路径
/// 使用 AES-256-GCM 解密，支持向后兼容旧的 XOR 加密格式
#[tauri::command]
pub async fn decrypt_config_data(file_path: String, password: String) -> Result<String, String> {
    crate::log_async_command!("decrypt_config_data", async {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        use tokio::fs as tokio_fs;
        use crate::utils::crypto::{decrypt_with_password, is_encrypted_with_salt};

        // 读取文件内容
        let file_content = tokio_fs::read(&file_path)
            .await
            .map_err(|e| format!("读取文件失败: {}", e))?;

        if file_content.is_empty() {
            return Err("文件内容为空".to_string());
        }

        let file_size = file_content.len();

        // 检测加密格式
        let decrypted_content = if is_encrypted_with_salt(&file_content) {
            // 新格式：AES-256-GCM 加密
            tracing::info!("🔐 检测到 AES-256-GCM 加密格式");
            let decrypted = decrypt_with_password(&file_content, &password)
                .map_err(|e| format!("解密失败: {}（请检查密码是否正确）", e))?;
            String::from_utf8(decrypted)
                .map_err(|e| format!("UTF-8解码失败: {}", e))?
        } else {
            // 尝试旧格式或明文
            let file_string = String::from_utf8(file_content.clone())
                .map_err(|e| format!("文件编码错误: {}", e))?;

            if file_string.trim_start().starts_with('{') {
                // 明文 JSON 格式
                tracing::warn!("⚠️ 检测到明文配置文件，建议使用加密导出");
                file_string
            } else {
                // 旧格式：XOR 加密（向后兼容）
                tracing::warn!("⚠️ 检测到旧版 XOR 加密格式，建议重新导出以使用更安全的加密");
                let encrypted = STANDARD
                    .decode(file_string.trim())
                    .map_err(|e| format!("Base64解码失败: {}", e))?;

                let key_bytes = password.as_bytes();
                let mut decrypted_bytes = vec![0u8; encrypted.len()];

                for (i, &byte) in encrypted.iter().enumerate() {
                    decrypted_bytes[i] = byte ^ key_bytes[i % key_bytes.len()];
                }

                String::from_utf8(decrypted_bytes)
                    .map_err(|e| format!("UTF-8解码失败: {}", e))?
            }
        };

        // 验证是否为有效的JSON
        if serde_json::from_str::<serde_json::Value>(&decrypted_content).is_err() {
            return Err("解密后的数据不是有效的JSON格式，请检查密码是否正确".to_string());
        }

        tracing::info!("🔓 配置文件解密成功，文件大小: {} bytes", file_size);
        Ok(decrypted_content)
    })
}

/// 加密配置数据
/// 使用 AES-256-GCM 加密（Argon2 密钥派生），返回二进制数据的 Base64 编码
#[tauri::command]
pub async fn encrypt_config_data(json_data: String, password: String) -> Result<String, String> {
    crate::log_async_command!("encrypt_config_data", async {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        use crate::utils::crypto::encrypt_with_password;

        // 验证是否为有效的JSON
        if serde_json::from_str::<serde_json::Value>(&json_data).is_err() {
            return Err("输入的数据不是有效的JSON格式".to_string());
        }

        // 使用 AES-256-GCM 加密
        let encrypted = encrypt_with_password(json_data.as_bytes(), &password)
            .map_err(|e| format!("加密失败: {}", e))?;

        // Base64 编码（便于存储和传输）
        let encrypted_base64 = STANDARD.encode(&encrypted);

        tracing::info!("🔐 配置文件加密成功（AES-256-GCM），数据大小: {} bytes", json_data.len());
        Ok(encrypted_base64)
    })
}

#[tauri::command]
pub async fn clear_logs() -> Result<String, String> {
    crate::log_async_command!("clear_logs", async {
        let log_dir = get_log_directory();
        let log_file = log_dir.join("antigravity-agent.log");

        if log_file.exists() {
            // 备份当前日志（可选）
            let backup_path = log_dir.join("antigravity-agent.backup.log");
            if let Ok(_) = fs::copy(&log_file, &backup_path) {
                tracing::info!("📦 日志已备份");
            }

            // 清空日志文件
            fs::write(&log_file, "").map_err(|e| format!("清空日志文件失败: {}", e))?;

            tracing::info!("🗑️ 日志文件已清空");
            Ok("日志文件已清空".to_string())
        } else {
            Err("日志文件不存在".to_string())
        }
    })
}

/// 写入前端日志到统一日志系统
/// 使用智能脱敏处理，与后端日志统一写入文件
#[tauri::command]
pub async fn write_frontend_log(log_entry: serde_json::Value) -> Result<(), String> {
    let sanitizer = LogSanitizer::new();

    // 提取字段
    let level = log_entry["level"].as_str().unwrap_or("info");
    let message = log_entry["message"].as_str().unwrap_or("no message");
    let details = log_entry["details"].as_str();
    let session_id = log_entry["sessionId"].as_str().unwrap_or("unknown");

    // 统一脱敏处理
    let sanitized_message = sanitizer.sanitize(message);
    let sanitized_details = details.map(|d| sanitizer.sanitize(d));

    match level {
        "error" => {
            tracing::error!(
                target = "frontend",
                session_id = session_id,
                details = sanitized_details,
                "🌐 {}", sanitized_message
            );
        }
        "warn" => {
            tracing::warn!(
                target = "frontend",
                session_id = session_id,
                details = sanitized_details,
                "🌐 {}", sanitized_message
            );
        }
        _ => {
            tracing::info!(
                target = "frontend",
                session_id = session_id,
                details = sanitized_details,
                "🌐 {}", sanitized_message
            );
        }
    }

    Ok(())
}

#[derive(serde::Serialize)]
pub struct LogInfo {
    pub exists: bool,
    pub path: String,
    pub size_bytes: u64,
    pub size_human: String,
    pub last_modified: String,
}

/// 格式化文件大小显示
fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
