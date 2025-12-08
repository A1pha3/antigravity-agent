/**
 * 数据库直接读取命令
 * 用于绕过 CSRF token 限制，直接从 Antigravity 数据库读取数据
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * 从数据库读取的认证状态
 */
export interface AuthStatusFromDb {
  name?: string;
  email?: string;
  apiKey?: string;
  userStatusProtoBinaryBase64?: string;
}

/**
 * 数据库直接读取命令
 */
export class DbCommands {
  /**
   * 直接从 Antigravity 数据库读取用户状态（不需要 CSRF token）
   * @returns 认证状态信息，包含 userStatusProtoBinaryBase64
   */
  static async getUserStatusFromDb(): Promise<AuthStatusFromDb> {
    return await invoke<AuthStatusFromDb>('get_user_status_from_db');
  }
}
