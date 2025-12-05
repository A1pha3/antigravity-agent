import {create} from "zustand";
import {LanguageServerResponse} from "@/commands/types/language-server-response.types.ts";
import type {AntigravityAccount} from "@/commands/types/account.types.ts";
import {LanguageServerCommands} from "@/commands/LanguageServerCommands.ts";
import {parseUserStatusProto} from "@/utils/protobuf-parser.ts";

type State = {
  // id -> 用户状态
  users: Record<string, LanguageServerResponse.Root>
}

type Actions = {
  fetchData: (antigravityAccount: AntigravityAccount) => Promise<void>
  fetchCurrentUserData: () => Promise<void>
}

/**
 * 从 protobuf 数据构建 LanguageServerResponse
 */
function buildResponseFromProto(
  protoData: string,
  email: string,
  name: string
): LanguageServerResponse.Root | null {
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
        planInfo: {} as LanguageServerResponse.PlanInfo,
      },
      cascadeModelConfigData: {
        clientModelConfigs: parsed.models.map(m => ({
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
              [antigravityAccount.id]: data
            }
          });
          console.log(`从备份数据加载用户 ${antigravityAccount.email} 配额信息成功`);
          return;
        }
      } catch (error) {
        console.warn(`解析备份 protobuf 数据失败:`, error);
      }
    }

    // 回退到 API 调用（需要 CSRF token，可能失败）
    try {
      const data = await LanguageServerCommands.getUserStatus(antigravityAccount.api_key)
      setState({
        users: {
          ...getState().users,
          [antigravityAccount.id]: data
        }
      })
    } catch (error) {
      console.error(`获取用户 ${antigravityAccount.email} 配额信息失败:`, error)
    }
  },

  /**
   * 从 Antigravity 数据库实时获取当前用户的配额数据
   * 这个方法会读取最新的数据，不依赖备份文件
   */
  fetchCurrentUserData: async () => {
    try {
      const authStatus = await LanguageServerCommands.getUserStatusFromDb();
      
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
              [id]: data
            }
          });
          console.log(`从数据库实时加载用户 ${authStatus.email} 配额信息成功`);
        }
      }
    } catch (error) {
      console.error('从数据库获取配额信息失败:', error);
    }
  },
}))
