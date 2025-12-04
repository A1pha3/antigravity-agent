#!/usr/bin/env node

/**
 * 文档存在性测试
 * 验证所有必需的文档文件是否存在
 */

import { existsSync } from 'fs';
import { join } from 'path';

const REQUIRED_DOCS = [
  // 入门文档
  'docs/README.md',
  'docs/zh-CN/getting-started/README.md',
  'docs/zh-CN/getting-started/installation.md',
  'docs/zh-CN/getting-started/quickstart.md',
  
  // 使用文档
  'docs/zh-CN/user-guide/user-guide.md',
  'docs/zh-CN/user-guide/api-reference.md',
  'docs/zh-CN/user-guide/configuration.md',
  'docs/zh-CN/user-guide/examples.md',
  
  // 开发文档
  'docs/zh-CN/development/architecture.md',
  'docs/zh-CN/development/development-guide.md',
  'docs/zh-CN/development/contributing.md',
  'docs/zh-CN/development/code-style.md',
  
  // 进阶文档
  'docs/zh-CN/advanced/design-principles.md',
  'docs/zh-CN/advanced/performance.md',
  'docs/zh-CN/advanced/troubleshooting.md',
  'docs/zh-CN/advanced/faq.md',
  
  // 术语表
  'docs/zh-CN/GLOSSARY.md',
];

function testDocumentExistence() {
  console.log('🔍 检查文档存在性...\n');
  
  let passed = 0;
  let failed = 0;
  const missingDocs = [];
  
  for (const docPath of REQUIRED_DOCS) {
    const fullPath = join(process.cwd(), docPath);
    const exists = existsSync(fullPath);
    
    if (exists) {
      console.log(`✅ ${docPath}`);
      passed++;
    } else {
      console.log(`❌ ${docPath} - 文件不存在`);
      failed++;
      missingDocs.push(docPath);
    }
  }
  
  console.log(`\n📊 测试结果: ${passed} 通过, ${failed} 失败`);
  
  if (failed > 0) {
    console.log('\n❌ 缺失的文档:');
    missingDocs.forEach(doc => console.log(`   - ${doc}`));
    process.exit(1);
  } else {
    console.log('\n✅ 所有必需文档都存在！');
    process.exit(0);
  }
}

testDocumentExistence();
