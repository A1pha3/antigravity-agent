/**
 * 语言服务器相关的前端命令封装
 */

import { invoke } from '@tauri-apps/api/core';
import {LanguageServerResponse} from "@/commands/types/language-server-response.types.ts";

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
 * 语言服务器命令
 */
export class LanguageServerCommands {
  /**
   * 获取用户状态信息（通过 API，需要 CSRF token）
   * @param apiKey API密钥
   * @returns 用户状态信息（JSON数据，具体格式取决于Antigravity API响应）
   */
  static async getUserStatus(apiKey: string): Promise<LanguageServerResponse.Root> {
    return await invoke<LanguageServerResponse.Root>('language_server_get_user_status', { apiKey });
  }

  /**
   * 直接从 Antigravity 数据库读取用户状态（不需要 CSRF token）
   * @returns 认证状态信息，包含 userStatusProtoBinaryBase64
   */
  static async getUserStatusFromDb(): Promise<AuthStatusFromDb> {
    return await invoke<AuthStatusFromDb>('get_user_status_from_db');
  }
}
