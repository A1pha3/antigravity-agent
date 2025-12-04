# 文档质量检查使用指南

## 快速开始

### 运行质量检查

```bash
npm run check-docs
```

这将检查：
- ✅ 文档完整性（所有必需文档和章节）
- ✅ Markdown 格式规范
- ✅ 链接有效性

### 查看详细报告

检查完成后，查看生成的报告：

```bash
cat docs-quality-report.json
```

## 理解检查结果

### 符号说明

- ✓ (绿色) - 通过检查
- ! (黄色) - 警告，建议修复
- ✗ (红色) - 错误，需要修复
- ○ (青色) - 信息提示

### 常见问题及修复方法

#### 1. Missing space after heading marker

**问题**: 标题标记后缺少空格

```markdown
❌ ##错误的标题
✅ ## 正确的标题
```

**修复方法**:
- 手动在 `#` 后添加空格
- 或使用查找替换: `##` → `## `

#### 2. Trailing spaces

**问题**: 行尾有多余空格

**修复方法**:
- 配置编辑器自动删除尾随空格
- VS Code: 设置 `"files.trimTrailingWhitespace": true`
- 或手动删除行尾空格

#### 3. Multiple consecutive blank lines

**问题**: 多个连续空行

**修复方法**:
- 删除多余的空行，最多保留一个空行

#### 4. Broken links

**问题**: 链接指向不存在的文件

**修复方法**:
- 检查文件路径是否正确
- 确保链接的文件存在
- 使用相对路径而非绝对路径

## 编辑器配置

### VS Code

在 `.vscode/settings.json` 中添加：

```json
{
  "files.trimTrailingWhitespace": true,
  "files.insertFinalNewline": true,
  "files.trimFinalNewlines": true,
  "[markdown]": {
    "editor.formatOnSave": true,
    "editor.wordWrap": "on"
  }
}
```

### EditorConfig

项目已包含 `.editorconfig` 文件，确保编辑器支持 EditorConfig。

## 最佳实践

### 编写新文档时

1. 使用文档模板（`docs/zh-CN/_template.md`）
2. 确保标题格式正确（`#` 后有空格）
3. 使用相对路径链接其他文档
4. 编写完成后运行质量检查

### 更新现有文档时

1. 保持格式一致性
2. 更新相关的交叉引用
3. 验证所有链接仍然有效
4. 运行质量检查确认无问题

### 提交前检查

```bash
# 运行质量检查
npm run check-docs

# 如果有错误，修复后再次检查
npm run check-docs
```

## 自动化检查

### Git Pre-commit Hook

可以添加 pre-commit hook 自动检查：

```bash
#!/bin/sh
npm run check-docs
```

### CI/CD 集成

在 `.github/workflows/docs-check.yml` 中：

```yaml
name: Documentation Quality Check

on:
  pull_request:
    paths:
      - 'docs/**'
  push:
    branches:
      - main
    paths:
      - 'docs/**'

jobs:
  check-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: npm install
      - run: npm run check-docs
```

## 质量标准

### 优秀 (Excellent)

- ✅ 所有文档存在且完整
- ✅ 无格式错误
- ✅ 所有链接有效
- ✅ 无警告

### 良好 (Good)

- ✅ 所有文档存在且完整
- ✅ 无格式错误
- ✅ 所有链接有效
- ⚠️ 少量警告（如尾随空格）

### 需要改进 (Needs Improvement)

- ⚠️ 有格式错误
- ⚠️ 有失效链接
- ⚠️ 文档不完整

### 不合格 (Unacceptable)

- ❌ 缺少必需文档
- ❌ 大量格式错误
- ❌ 多个失效链接

## 获取帮助

### 查看脚本文档

```bash
cat scripts/README.md
```

### 查看质量总结

```bash
cat docs/QUALITY-CHECK-SUMMARY.md
```

### 常见问题

**Q: 质量检查失败了怎么办？**

A: 查看控制台输出，找到具体的错误和警告，按照提示修复。

**Q: 如何添加新的必需文档？**

A: 编辑 `scripts/check-docs-quality.js` 中的 `requiredDocs` 对象。

**Q: 可以忽略某些警告吗？**

A: 警告不会导致检查失败，但建议修复以保持文档质量。

**Q: 如何检查单个文档？**

A: 当前脚本检查所有文档，可以修改脚本添加单文档检查功能。

## 维护

定期（建议每月）运行质量检查，确保文档保持高质量：

```bash
npm run check-docs
```

在发布新版本前，务必运行完整检查并修复所有问题。
