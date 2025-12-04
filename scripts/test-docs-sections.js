#!/usr/bin/env node

/**
 * 文档章节完整性测试
 * 验证特定文档是否包含必需的章节
 */

import { readFileSync } from 'fs';
import { join } from 'path';

const SECTION_REQUIREMENTS = {
  'README.md': [
    '项目简介',
    '主要功能',
    '技术栈',
  ],
  'docs/zh-CN/getting-started/installation.md': [
    'Windows',
    'macOS',
    '系统要求',
  ],
  'docs/zh-CN/user-guide/user-guide.md': [
    '账户管理',
    '账户切换',
    '导入导出',
  ],
  'docs/zh-CN/user-guide/api-reference.md': [
    '账户管理命令',
    '备份管理命令',
    '进程管理命令',
  ],
  'docs/zh-CN/development/architecture.md': [
    '前端架构',
    '后端架构',
    '数据流',
  ],
  'docs/zh-CN/development/development-guide.md': [
    '开发环境',
    '构建和打包',
    '调试',
  ],
  'docs/zh-CN/development/contributing.md': [
    '开发流程',
    '分支管理',
    'Pull Request',
  ],
  'docs/zh-CN/development/code-style.md': [
    'TypeScript',
    'Rust',
    '命名约定',
  ],
  'docs/zh-CN/advanced/troubleshooting.md': [
    '日志',
    '常见错误',
  ],
};

function testDocumentSections() {
  console.log('🔍 检查文档章节完整性...\n');
  
  let totalTests = 0;
  let passedTests = 0;
  let failedTests = 0;
  const failures = [];
  
  for (const [docPath, requiredSections] of Object.entries(SECTION_REQUIREMENTS)) {
    const fullPath = join(process.cwd(), docPath);
    
    try {
      const content = readFileSync(fullPath, 'utf-8');
      console.log(`\n📄 ${docPath}`);
      
      for (const section of requiredSections) {
        totalTests++;
        const hasSection = content.includes(section);
        
        if (hasSection) {
          console.log(`  ✅ 包含章节: ${section}`);
          passedTests++;
        } else {
          console.log(`  ❌ 缺少章节: ${section}`);
          failedTests++;
          failures.push({ doc: docPath, section });
        }
      }
    } catch (error) {
      console.log(`  ⚠️  无法读取文件: ${error.message}`);
      totalTests += requiredSections.length;
      failedTests += requiredSections.length;
      requiredSections.forEach(section => {
        failures.push({ doc: docPath, section });
      });
    }
  }
  
  console.log(`\n📊 测试结果: ${passedTests}/${totalTests} 通过`);
  
  if (failedTests > 0) {
    console.log('\n❌ 缺失的章节:');
    failures.forEach(({ doc, section }) => {
      console.log(`   - ${doc}: ${section}`);
    });
    process.exit(1);
  } else {
    console.log('\n✅ 所有必需章节都存在！');
    process.exit(0);
  }
}

testDocumentSections();
