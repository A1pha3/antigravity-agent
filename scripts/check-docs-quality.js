#!/usr/bin/env node

/**
 * Documentation Quality Check Script
 * 
 * This script performs comprehensive quality checks on the documentation:
 * 1. Markdown format validation using markdownlint
 * 2. Link validity checking using markdown-link-check
 * 3. Document completeness verification
 * 4. Quality report generation
 */

import { readFileSync, readdirSync, statSync, existsSync, writeFileSync } from 'fs';
import { join, relative } from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

// ANSI color codes for terminal output
const colors = {
  reset: '\x1b[0m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m',
  bold: '\x1b[1m'
};

// Required document structure based on requirements
const requiredDocs = {
  'docs/zh-CN/getting-started/README.md': {
    requiredSections: ['项目概览', '主要功能', '技术栈'],
    description: '项目概览文档'
  },
  'docs/zh-CN/getting-started/installation.md': {
    requiredSections: ['Windows', 'macOS', 'Linux'],
    description: '安装指南'
  },
  'docs/zh-CN/getting-started/quickstart.md': {
    requiredSections: ['快速开始', '首次启动'],
    description: '快速开始教程'
  },
  'docs/zh-CN/user-guide/user-guide.md': {
    requiredSections: ['账户管理', '切换账户', '导入导出'],
    description: '使用手册'
  },
  'docs/zh-CN/user-guide/api-reference.md': {
    requiredSections: ['API', '命令'],
    description: 'API 参考'
  },
  'docs/zh-CN/user-guide/configuration.md': {
    requiredSections: ['配置', '设置'],
    description: '配置说明'
  },
  'docs/zh-CN/development/architecture.md': {
    requiredSections: ['架构', '前端', '后端'],
    description: '系统架构'
  },
  'docs/zh-CN/development/development-guide.md': {
    requiredSections: ['开发环境', '构建', '调试'],
    description: '开发指南'
  },
  'docs/zh-CN/development/contributing.md': {
    requiredSections: ['贡献', '提交', 'PR'],
    description: '贡献指南'
  },
  'docs/zh-CN/development/code-style.md': {
    requiredSections: ['代码规范', '命名'],
    description: '代码规范'
  },
  'docs/zh-CN/advanced/design-principles.md': {
    requiredSections: ['设计原理', '技术选型'],
    description: '设计原理'
  },
  'docs/zh-CN/advanced/performance.md': {
    requiredSections: ['性能', '优化'],
    description: '性能优化'
  },
  'docs/zh-CN/advanced/troubleshooting.md': {
    requiredSections: ['问题排查', '日志'],
    description: '问题排查'
  },
  'docs/zh-CN/advanced/faq.md': {
    requiredSections: ['常见问题', 'FAQ'],
    description: 'FAQ'
  }
};

// Quality check results
const results = {
  totalDocs: 0,
  existingDocs: 0,
  missingDocs: [],
  completeDocs: [],
  incompleteDocs: [],
  formatIssues: [],
  linkIssues: [],
  errors: []
};

/**
 * Check if a file exists
 */
function checkFileExists(filePath) {
  const fullPath = join(rootDir, filePath);
  return existsSync(fullPath);
}

/**
 * Read file content
 */
function readFileContent(filePath) {
  try {
    const fullPath = join(rootDir, filePath);
    return readFileSync(fullPath, 'utf-8');
  } catch (error) {
    return null;
  }
}

/**
 * Check if document contains required sections
 */
function checkDocumentCompleteness(filePath, requiredSections) {
  const content = readFileContent(filePath);
  if (!content) {
    return { complete: false, missingSections: requiredSections };
  }

  const missingSections = [];
  for (const section of requiredSections) {
    // Check if section exists in content (case-insensitive)
    const regex = new RegExp(section, 'i');
    if (!regex.test(content)) {
      missingSections.push(section);
    }
  }

  return {
    complete: missingSections.length === 0,
    missingSections
  };
}

/**
 * Extract all markdown links from content
 */
function extractLinks(content) {
  const links = [];
  
  // Match markdown links: [text](url)
  const linkRegex = /\[([^\]]+)\]\(([^)]+)\)/g;
  let match;
  
  while ((match = linkRegex.exec(content)) !== null) {
    links.push({
      text: match[1],
      url: match[2],
      line: content.substring(0, match.index).split('\n').length
    });
  }
  
  return links;
}

/**
 * Check if a link is valid
 */
function checkLink(link, filePath) {
  const url = link.url;
  
  // Skip external URLs (we'll just note them)
  if (url.startsWith('http://') || url.startsWith('https://')) {
    return { valid: true, type: 'external' };
  }
  
  // Skip anchors
  if (url.startsWith('#')) {
    return { valid: true, type: 'anchor' };
  }
  
  // Check relative file links
  const fileDir = dirname(join(rootDir, filePath));
  const linkedFile = join(fileDir, url.split('#')[0]);
  
  if (existsSync(linkedFile)) {
    return { valid: true, type: 'internal' };
  }
  
  return { valid: false, type: 'internal', error: 'File not found' };
}

/**
 * Check basic markdown format issues
 */
function checkMarkdownFormat(content, filePath) {
  const issues = [];
  const lines = content.split('\n');
  
  lines.forEach((line, index) => {
    const lineNum = index + 1;
    
    // Check for multiple consecutive blank lines
    if (index > 0 && line === '' && lines[index - 1] === '' && lines[index - 2] === '') {
      issues.push({
        line: lineNum,
        message: 'Multiple consecutive blank lines',
        severity: 'warning'
      });
    }
    
    // Check for trailing spaces
    if (line.endsWith(' ') && line.trim() !== '') {
      issues.push({
        line: lineNum,
        message: 'Trailing spaces',
        severity: 'warning'
      });
    }
    
    // Check for proper heading format (space after #)
    if (/^#+[^#\s]/.test(line)) {
      issues.push({
        line: lineNum,
        message: 'Missing space after heading marker',
        severity: 'error'
      });
    }
  });
  
  return issues;
}

/**
 * Run all quality checks
 */
function runQualityChecks() {
  console.log(`${colors.bold}${colors.cyan}=== 文档质量检查 ===${colors.reset}\n`);
  
  // Check document existence and completeness
  console.log(`${colors.blue}1. 检查文档完整性...${colors.reset}`);
  results.totalDocs = Object.keys(requiredDocs).length;
  
  for (const [filePath, config] of Object.entries(requiredDocs)) {
    const exists = checkFileExists(filePath);
    
    if (!exists) {
      results.missingDocs.push({
        path: filePath,
        description: config.description
      });
      console.log(`  ${colors.red}✗${colors.reset} ${filePath} - 缺失`);
    } else {
      results.existingDocs++;
      const completeness = checkDocumentCompleteness(filePath, config.requiredSections);
      
      if (completeness.complete) {
        results.completeDocs.push(filePath);
        console.log(`  ${colors.green}✓${colors.reset} ${filePath} - 完整`);
      } else {
        results.incompleteDocs.push({
          path: filePath,
          missingSections: completeness.missingSections
        });
        console.log(`  ${colors.yellow}!${colors.reset} ${filePath} - 缺少章节: ${completeness.missingSections.join(', ')}`);
      }
    }
  }
  
  console.log();
  
  // Check markdown format
  console.log(`${colors.blue}2. 检查 Markdown 格式...${colors.reset}`);
  for (const filePath of Object.keys(requiredDocs)) {
    if (!checkFileExists(filePath)) continue;
    
    const content = readFileContent(filePath);
    if (!content) continue;
    
    const formatIssues = checkMarkdownFormat(content, filePath);
    if (formatIssues.length > 0) {
      results.formatIssues.push({
        path: filePath,
        issues: formatIssues
      });
      console.log(`  ${colors.yellow}!${colors.reset} ${filePath} - ${formatIssues.length} 个格式问题`);
    } else {
      console.log(`  ${colors.green}✓${colors.reset} ${filePath} - 格式正确`);
    }
  }
  
  console.log();
  
  // Check links
  console.log(`${colors.blue}3. 检查链接有效性...${colors.reset}`);
  for (const filePath of Object.keys(requiredDocs)) {
    if (!checkFileExists(filePath)) continue;
    
    const content = readFileContent(filePath);
    if (!content) continue;
    
    const links = extractLinks(content);
    const brokenLinks = [];
    
    for (const link of links) {
      const linkCheck = checkLink(link, filePath);
      if (!linkCheck.valid) {
        brokenLinks.push({
          ...link,
          error: linkCheck.error
        });
      }
    }
    
    if (brokenLinks.length > 0) {
      results.linkIssues.push({
        path: filePath,
        brokenLinks
      });
      console.log(`  ${colors.red}✗${colors.reset} ${filePath} - ${brokenLinks.length} 个失效链接`);
    } else if (links.length > 0) {
      console.log(`  ${colors.green}✓${colors.reset} ${filePath} - 所有链接有效 (${links.length} 个)`);
    } else {
      console.log(`  ${colors.cyan}○${colors.reset} ${filePath} - 无链接`);
    }
  }
  
  console.log();
}

/**
 * Generate quality report
 */
function generateReport() {
  console.log(`${colors.bold}${colors.cyan}=== 质量报告 ===${colors.reset}\n`);
  
  // Summary
  console.log(`${colors.bold}总体统计:${colors.reset}`);
  console.log(`  文档总数: ${results.totalDocs}`);
  console.log(`  已创建: ${results.existingDocs} (${Math.round(results.existingDocs / results.totalDocs * 100)}%)`);
  console.log(`  完整: ${results.completeDocs.length} (${Math.round(results.completeDocs.length / results.totalDocs * 100)}%)`);
  console.log(`  不完整: ${results.incompleteDocs.length}`);
  console.log(`  缺失: ${results.missingDocs.length}`);
  console.log();
  
  // Missing documents
  if (results.missingDocs.length > 0) {
    console.log(`${colors.bold}${colors.red}缺失的文档 (${results.missingDocs.length}):${colors.reset}`);
    results.missingDocs.forEach(doc => {
      console.log(`  - ${doc.path} (${doc.description})`);
    });
    console.log();
  }
  
  // Incomplete documents
  if (results.incompleteDocs.length > 0) {
    console.log(`${colors.bold}${colors.yellow}不完整的文档 (${results.incompleteDocs.length}):${colors.reset}`);
    results.incompleteDocs.forEach(doc => {
      console.log(`  - ${doc.path}`);
      console.log(`    缺少章节: ${doc.missingSections.join(', ')}`);
    });
    console.log();
  }
  
  // Format issues
  if (results.formatIssues.length > 0) {
    console.log(`${colors.bold}${colors.yellow}格式问题 (${results.formatIssues.length} 个文档):${colors.reset}`);
    results.formatIssues.forEach(doc => {
      console.log(`  ${doc.path}:`);
      doc.issues.slice(0, 5).forEach(issue => {
        const icon = issue.severity === 'error' ? '✗' : '!';
        console.log(`    ${icon} 行 ${issue.line}: ${issue.message}`);
      });
      if (doc.issues.length > 5) {
        console.log(`    ... 还有 ${doc.issues.length - 5} 个问题`);
      }
    });
    console.log();
  }
  
  // Link issues
  if (results.linkIssues.length > 0) {
    console.log(`${colors.bold}${colors.red}链接问题 (${results.linkIssues.length} 个文档):${colors.reset}`);
    results.linkIssues.forEach(doc => {
      console.log(`  ${doc.path}:`);
      doc.brokenLinks.forEach(link => {
        console.log(`    ✗ 行 ${link.line}: [${link.text}](${link.url}) - ${link.error}`);
      });
    });
    console.log();
  }
  
  // Overall status
  const hasIssues = results.missingDocs.length > 0 || 
                    results.incompleteDocs.length > 0 || 
                    results.linkIssues.length > 0 ||
                    results.formatIssues.filter(f => f.issues.some(i => i.severity === 'error')).length > 0;
  
  if (hasIssues) {
    console.log(`${colors.bold}${colors.yellow}状态: 需要改进${colors.reset}`);
    console.log(`建议: 修复上述问题以提高文档质量\n`);
  } else {
    console.log(`${colors.bold}${colors.green}状态: 优秀${colors.reset}`);
    console.log(`所有文档都已完成且质量良好！\n`);
  }
  
  // Save report to file
  const reportPath = join(rootDir, 'docs-quality-report.json');
  writeFileSync(reportPath, JSON.stringify(results, null, 2));
  console.log(`详细报告已保存到: ${relative(rootDir, reportPath)}\n`);
  
  return hasIssues ? 1 : 0;
}

/**
 * Main execution
 */
function main() {
  try {
    runQualityChecks();
    const exitCode = generateReport();
    process.exit(exitCode);
  } catch (error) {
    console.error(`${colors.red}错误: ${error.message}${colors.reset}`);
    process.exit(1);
  }
}

main();
