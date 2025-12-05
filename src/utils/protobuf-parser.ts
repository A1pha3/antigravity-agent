/**
 * 简单的 Protobuf 解析器
 * 用于解析 Antigravity 的 userStatusProtoBinaryBase64 数据
 * 
 * 数据结构分析：
 * - 模型名称后面跟着配置数据
 * - 配额信息在 `7a 0d 0d` 标记后面，是一个 float32 值
 */

export interface QuotaInfo {
  remainingFraction: number;
  resetTime?: string;
}

export interface ModelConfig {
  label: string;
  quotaInfo: QuotaInfo;
}

export interface ParsedUserStatus {
  name?: string;
  email?: string;
  models: ModelConfig[];
}

// 已知的模型名称列表
const KNOWN_MODELS = [
  'Gemini 3 Pro (High)',
  'Gemini 3 Pro (Low)',
  'Claude Sonnet 4.5',
  'Claude Sonnet 4.5 (Thinking)',
  'Claude Opus 4.5 (Thinking)',
  'GPT-OSS 120B (Medium)',
  'GPT-4o',
  'GPT-4',
  'Claude 3.5 Sonnet',
  'Claude 3 Opus',
  'Gemini Pro',
  'Gemini Ultra',
];

/**
 * 从 base64 编码的 protobuf 数据中解析用户状态
 */
export function parseUserStatusProto(base64Data: string): ParsedUserStatus {
  try {
    // 解码 base64
    const binaryString = atob(base64Data);
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }

    const result: ParsedUserStatus = {
      models: [],
    };

    // 将字节转换为文本用于搜索模型名称
    const text = new TextDecoder().decode(bytes);

    // 查找所有模型名称及其配额
    for (const modelName of KNOWN_MODELS) {
      const idx = text.indexOf(modelName);
      if (idx >= 0) {
        // 在模型名称后面查找配额信息
        // 配额标记是 `7a 0d 0d`，后面跟着 float32
        const searchStart = idx + modelName.length;
        const searchEnd = Math.min(searchStart + 50, bytes.length - 4);
        
        let remainingFraction = 1.0; // 默认值
        
        for (let i = searchStart; i < searchEnd; i++) {
          // 查找 `7a 0d 0d` 标记 (field 15, wire type 5, followed by 0d 0d)
          if (bytes[i] === 0x7a && bytes[i + 1] === 0x0d && bytes[i + 2] === 0x0d) {
            // 读取 float32 (little-endian)
            const view = new DataView(bytes.buffer, i + 3, 4);
            remainingFraction = view.getFloat32(0, true);
            break;
          }
        }

        result.models.push({
          label: modelName,
          quotaInfo: { remainingFraction },
        });
      }
    }

    // 如果没有找到已知模型，尝试动态解析
    if (result.models.length === 0) {
      result.models = parseDynamicModels(bytes);
    }

    return result;
  } catch (error) {
    console.error('解析 protobuf 数据失败:', error);
    return { models: [] };
  }
}

/**
 * 动态解析模型配置（当已知模型列表不匹配时）
 */
function parseDynamicModels(bytes: Uint8Array): ModelConfig[] {
  const models: ModelConfig[] = [];
  const text = new TextDecoder().decode(bytes);
  
  // 使用正则表达式查找可能的模型名称
  // 模型名称通常是 "XXX (YYY)" 格式
  const modelPattern = /([A-Za-z0-9\s\.\-]+\s*\([A-Za-z0-9\s]+\))/g;
  let match;
  
  while ((match = modelPattern.exec(text)) !== null) {
    const modelName = match[1].trim();
    
    // 过滤掉太短或不像模型名称的字符串
    if (modelName.length < 5 || modelName.length > 50) continue;
    if (!/[A-Z]/.test(modelName)) continue; // 必须包含大写字母
    
    const idx = match.index;
    const searchStart = idx + modelName.length;
    const searchEnd = Math.min(searchStart + 50, bytes.length - 4);
    
    let remainingFraction = 1.0;
    
    for (let i = searchStart; i < searchEnd; i++) {
      if (bytes[i] === 0x7a && bytes[i + 1] === 0x0d && bytes[i + 2] === 0x0d) {
        const view = new DataView(bytes.buffer, i + 3, 4);
        remainingFraction = view.getFloat32(0, true);
        break;
      }
    }

    // 避免重复
    if (!models.some(m => m.label === modelName)) {
      models.push({
        label: modelName,
        quotaInfo: { remainingFraction },
      });
    }
  }

  return models;
}
