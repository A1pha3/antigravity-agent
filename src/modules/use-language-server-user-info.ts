/**
 * 语言服务器用户信息状态管理
 * 用于管理和获取用户配额信息
 */
import { create } from 'zustand';
import type { AntigravityAccount } from '@/commands/types/account.types';
import { DbCommands } from '@/commands/DbCommands';
import { parseUserStatusProto } from '@/utils/protobuf-parser';

/**
 * 配额信息
 */
export interface QuotaInfo {
  remainingFraction: number;
  resetTime?: string;
}

/**
 * 模型配置
 */
export interface ModelConfig {
  label: string;
  quotaInfo: QuotaInfo;
  allowedTiers?: string[];
  isRecommended?: boolean;
  modelOrAlias?: { model: string };
}

/**
 * 用户状态响应
 */
export interface UserStatusResponse {
  userStatus: {
    acceptedLatestTermsOfService: boolean;
    disableTelemetry: boolean;
    email: string;
    name: string;
    planStatus: {
      availableFlowCredits: number;
      availablePromptCredits: number;
      planInfo: Record<string, unknown>;
    };
    cascadeModelConfigData: {
      clientModelConfigs: ModelConfig[];
      clientModelSorts: unknown[];
      defaultOverrideModelConfig: { modelOrAlias: { model: string } };
    };
  };
}


type State = {
  // id -> 用户状态
  users: Record<string, UserStatusResponse>;
};

type Actions = {
  fetchData: (antigravityAccount: AntigravityAccount) => Promise<void>;
  fetchCurrentUserData: () => Promise<void>;
};

/**
 * 从 protobuf 数据构建 UserStatusResponse
 */
function buildResponseFromProto(
  protoData: string,
  email: string,
  name: string
): UserStatusResponse | null {
  const parsed = parseUserStatusProto(protoData);
  if (parsed.models.length === 0) return null;

  return {
    userStatus: {
      acceptedLatestTermsOfService: true,
      disableTelemetry: false,
      email,
      name,
      planStatus: {
        availableFlowCredits: 0,
        availablePromptCredits: 0,
        planInfo: {},
      },
      cascadeModelConfigData: {
        clientModelConfigs: parsed.models.map((m) => ({
          label: m.label,
          quotaInfo: {
            remainingFraction: m.quotaInfo.remainingFraction,
            resetTime: m.quotaInfo.resetTime || '',
          },
          allowedTiers: [],
          isRecommended: false,
          modelOrAlias: { model: m.label },
        })),
        clientModelSorts: [],
        defaultOverrideModelConfig: { modelOrAlias: { model: '' } },
      },
    },
  };
}

export const useLanguageServerUserInfo = create<State & Actions>((setState, getState) => ({
  users: {},

  /**
   * 获取指定账户的配额数据（从备份文件）
   */
  fetchData: async (antigravityAccount: AntigravityAccount) => {
    // 优先使用本地 protobuf 数据（不需要 CSRF token）
    if (antigravityAccount.user_status_proto) {
      try {
        const data = buildResponseFromProto(
          antigravityAccount.user_status_proto,
          antigravityAccount.email,
          antigravityAccount.name
        );
        if (data) {
          setState({
            users: {
              ...getState().users,
              [antigravityAccount.id]: data,
            },
          });
          console.log(`从备份数据加载用户 ${antigravityAccount.email} 配额信息成功`);
          return;
        }
      } catch (error) {
        console.warn(`解析备份 protobuf 数据失败:`, error);
      }
    }

    // 如果没有本地数据，记录警告
    console.warn(`用户 ${antigravityAccount.email} 没有本地配额数据`);
  },

  /**
   * 从 Antigravity 数据库实时获取当前用户的配额数据
   * 这个方法会读取最新的数据，不依赖备份文件
   */
  fetchCurrentUserData: async () => {
    try {
      const authStatus = await DbCommands.getUserStatusFromDb();

      if (authStatus.userStatusProtoBinaryBase64 && authStatus.email) {
        const data = buildResponseFromProto(
          authStatus.userStatusProtoBinaryBase64,
          authStatus.email,
          authStatus.name || authStatus.email
        );

        if (data) {
          const id = `account_${authStatus.email}`;
          setState({
            users: {
              ...getState().users,
              [id]: data,
            },
          });
          console.log(`从数据库实时加载用户 ${authStatus.email} 配额信息成功`);
        }
      }
    } catch (error) {
      console.error('从数据库获取配额信息失败:', error);
    }
  },
}));
