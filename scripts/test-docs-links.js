#!/usr/bin/env node

/**
 * 文档链接有效性测试
 * 验证文档中的内部链接是否有效
 */

import { readFileSync, existsSync } from 'fs';
import { join, dirname, resolve } from 'path';
import { globSync } from 'glob';

function extractMarkdownLinks(content) {
  // 匹配 [text](link) 格式的链接
  const linkRegex = /\[([^\]]+)\]\(([^)]+)\)/g;
  const links = [];
  let match;
  
  while ((match = linkRegex.exec(content)) !== null) {
    links.push({
      text: match[1],
      url: match[2],
    });
  }
  
  return links;
}

function isInternalLink(url) {
  // 排除外部链接、锚点链接和特殊协议
  return !url.startsWith('http://') &&
         !url.startsWith('https://') &&
         !url.startsWith('#') &&
         !url.startsWith('mailto:');
}

function testDocumentLinks() {
  console.log('🔍 检查文档链接有效性...\n');
  
  const docFiles = globSync('docs/**/*.md', { cwd: process.cwd() });
  
  let totalLinks = 0;
  let validLinks = 0;
  let invalidLinks = 0;
  const brokenLinks = [];
  
  for (const docPath of docFiles) {
    // 跳过模板文件
    if (docPath.includes('_template.md')) {
      continue;
    }
    const fullPath = join(process.cwd(), docPath);
    const content = readFileSync(fullPath, 'utf-8');
    const links = extractMarkdownLinks(content);
    
    const internalLinks = links.filter(link => isInternalLink(link.url));
    
    if (internalLinks.length === 0) continue;
    
    console.log(`\n📄 ${docPath}`);
    
    for (const link of internalLinks) {
      totalLinks++;
      
      // 移除锚点部分
      const urlWithoutAnchor = link.url.split('#')[0];
      if (!urlWithoutAnchor) {
        // 纯锚点链接，跳过
        validLinks++;
        continue;
      }
      
      // 解析相对路径
      const docDir = dirname(fullPath);
      const targetPath = resolve(docDir, urlWithoutAnchor);
      
      const exists = existsSync(targetPath);
      
      if (exists) {
        console.log(`  ✅ ${link.url}`);
        validLinks++;
      } else {
        console.log(`  ❌ ${link.url} -> ${targetPath}`);
        invalidLinks++;
        brokenLinks.push({ doc: docPath, link: link.url, target: targetPath });
      }
    }
  }
  
  console.log(`\n📊 测试结果: ${validLinks}/${totalLinks} 链接有效`);
  
  if (invalidLinks > 0) {
    console.log('\n❌ 失效的链接:');
    brokenLinks.forEach(({ doc, link, target }) => {
      console.log(`   - ${doc}: ${link}`);
      console.log(`     目标: ${target}`);
    });
    process.exit(1);
  } else {
    console.log('\n✅ 所有内部链接都有效！');
    process.exit(0);
  }
}

testDocumentLinks();
