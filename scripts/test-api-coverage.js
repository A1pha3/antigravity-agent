#!/usr/bin/env node

/**
 * API 文档覆盖率测试
 * 验证所有 Tauri 命令是否在 API 文档中有说明
 */

import { readFileSync } from 'fs';
import { join } from 'path';
import { globSync } from 'glob';

function extractTauriCommands() {
  const commands = new Set();
  
  // 查找所有 Rust 命令文件
  const commandFiles = globSync('src-tauri/src/commands/**/*.rs', { cwd: process.cwd() });
  
  for (const file of commandFiles) {
    const fullPath = join(process.cwd(), file);
    const content = readFileSync(fullPath, 'utf-8');
    
    // 匹配 #[tauri::command] 注解后的函数名
    const commandRegex = /#\[tauri::command\]\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)/g;
    let match;
    
    while ((match = commandRegex.exec(content)) !== null) {
      commands.add(match[1]);
    }
  }
  
  return Array.from(commands).sort();
}

function checkApiDocumentation(commands) {
  const apiDocPath = join(process.cwd(), 'docs/zh-CN/user-guide/api-reference.md');
  
  try {
    const apiDoc = readFileSync(apiDocPath, 'utf-8');
    
    console.log('🔍 检查 API 文档覆盖率...\n');
    console.log(`📋 发现 ${commands.length} 个 Tauri 命令\n`);
    
    let documented = 0;
    let undocumented = 0;
    const missingCommands = [];
    
    for (const command of commands) {
      // 检查命令是否在文档中被提及
      const isDocumented = apiDoc.includes(command) || 
                          apiDoc.includes(command.replace(/_/g, '-'));
      
      if (isDocumented) {
        console.log(`✅ ${command}`);
        documented++;
      } else {
        console.log(`❌ ${command} - 未在文档中找到`);
        undocumented++;
        missingCommands.push(command);
      }
    }
    
    const coverage = ((documented / commands.length) * 100).toFixed(1);
    console.log(`\n📊 API 文档覆盖率: ${coverage}% (${documented}/${commands.length})`);
    
    if (undocumented > 0) {
      console.log('\n❌ 未记录的命令:');
      missingCommands.forEach(cmd => console.log(`   - ${cmd}`));
      process.exit(1);
    } else {
      console.log('\n✅ 所有 API 命令都已记录！');
      process.exit(0);
    }
  } catch (error) {
    console.error(`❌ 无法读取 API 文档: ${error.message}`);
    process.exit(1);
  }
}

const commands = extractTauriCommands();
checkApiDocumentation(commands);
