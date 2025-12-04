# 文档测试指南

本文档说明如何运行和维护 Antigravity Agent 的文档测试套件。

## 概述

文档测试套件确保文档的完整性、准确性和一致性。测试套件包括四个主要测试脚本，涵盖文档存在性、章节完整性、链接有效性和 API 覆盖率。

## 快速开始

### 运行所有测试

```bash
npm run test:docs
```

这将按顺序运行所有四个测试：
1. 文档存在性测试
2. 章节完整性测试
3. 链接有效性测试
4. API 覆盖率测试

### 运行单个测试

```bash
# 文档存在性测试
npm run test:docs:existence

# 章节完整性测试
npm run test:docs:sections

# 链接有效性测试
npm run test:docs:links

# API 覆盖率测试
npm run test:docs:api
```

## 测试详解

### 1. 文档存在性测试

**脚本**: `scripts/test-docs-existence.js`

**目的**: 验证所有必需的文档文件是否存在

**检查内容**:
- 入门文档（4 个文件）
- 使用文档（4 个文件）
- 开发文档（4 个文件）
- 进阶文档（4 个文件）
- 术语表（1 个文件）

**通过条件**: 所有 17 个必需文档文件都存在

### 2. 章节完整性测试

**脚本**: `scripts/test-docs-sections.js`

**目的**: 验证文档是否包含必需的章节

**检查内容**:
- README.md: 项目简介、主要功能、技术栈
- installation.md: Windows、macOS、系统要求
- user-guide.md: 账户管理、账户切换、导入导出
- api-reference.md: 账户管理命令、备份管理命令、进程管理命令
- architecture.md: 前端架构、后端架构、数据流
- development-guide.md: 开发环境、构建和打包、调试
- contributing.md: 开发流程、分支管理、Pull Request
- code-style.md: TypeScript、Rust、命名约定
- troubleshooting.md: 日志、常见错误

**通过条件**: 所有 26 个必需章节都存在

### 3. 链接有效性测试

**脚本**: `scripts/test-docs-links.js`

**目的**: 验证文档中的内部链接是否有效

**检查内容**:
- 所有 Markdown 格式的内部链接
- 相对路径链接的目标文件是否存在
- 跨文档引用的有效性

**排除内容**:
- 外部 URL（http://、https://）
- 纯锚点链接（#section）
- 模板文件（_template.md）

**通过条件**: 所有内部链接都指向存在的文件

### 4. API 覆盖率测试

**脚本**: `scripts/test-api-coverage.js`

**目的**: 验证所有 Tauri 命令是否在 API 文档中有说明

**检查内容**:
- 从 Rust 源代码中提取所有 `#[tauri::command]` 注解的函数
- 验证每个命令是否在 `api-reference.md` 中被提及
- 计算文档覆盖率百分比

**通过条件**: 100% 的 API 命令都有文档说明

## CI/CD 集成

### 自动触发场景

文档测试在以下情况下自动运行：

1. **Pull Request**: 当 PR 修改了以下文件时
   - `docs/**` 目录下的任何文件
   - 测试脚本文件
   - 工作流配置文件

2. **Push 到主分支**: 当代码推送到 `master` 或 `dev` 分支时

3. **定期调度**: 每周一早上 8:00 UTC 自动运行

4. **手动触发**: 可以在 GitHub Actions 页面手动触发

### 工作流配置

文档测试工作流定义在 `.github/workflows/docs-test.yml`，包含以下作业：

- `test-docs-existence`: 文档存在性测试
- `test-docs-sections`: 章节完整性测试
- `test-docs-links`: 链接有效性测试
- `test-api-coverage`: API 覆盖率测试
- `test-docs-quality`: 文档质量检查
- `lint-markdown`: Markdown 格式检查
- `generate-report`: 生成质量报告

### PR 检查

当提交 PR 时，GitHub 会自动运行文档测试。测试结果会显示在 PR 页面，并生成详细的质量报告作为评论。

## 测试失败处理

### 文档存在性测试失败

**原因**: 缺少必需的文档文件

**解决方案**:
1. 检查输出中列出的缺失文件
2. 创建缺失的文档文件
3. 使用文档模板 `docs/zh-CN/_template.md` 作为起点

### 章节完整性测试失败

**原因**: 文档缺少必需的章节

**解决方案**:
1. 检查输出中列出的缺失章节
2. 在相应文档中添加缺失的章节
3. 确保章节标题与测试要求完全匹配

### 链接有效性测试失败

**原因**: 文档中的链接指向不存在的文件

**解决方案**:
1. 检查输出中列出的失效链接
2. 修正链接路径或创建目标文件
3. 验证相对路径是否正确

### API 覆盖率测试失败

**原因**: 代码中的 Tauri 命令未在文档中记录

**解决方案**:
1. 检查输出中列出的未记录命令
2. 在 `api-reference.md` 中添加命令文档
3. 包括命令的功能说明、参数、返回值和示例

## 维护指南

### 添加新文档

当添加新的必需文档时：

1. 更新 `scripts/test-docs-existence.js` 中的 `REQUIRED_DOCS` 数组
2. 如果需要检查特定章节，更新 `scripts/test-docs-sections.js` 中的 `SECTION_REQUIREMENTS` 对象
3. 运行测试验证配置正确

### 修改测试规则

测试规则定义在各个测试脚本中：

- **文档列表**: `scripts/test-docs-existence.js` → `REQUIRED_DOCS`
- **章节要求**: `scripts/test-docs-sections.js` → `SECTION_REQUIREMENTS`
- **链接排除**: `scripts/test-docs-links.js` → `isInternalLink()` 函数
- **API 提取**: `scripts/test-api-coverage.js` → `extractTauriCommands()` 函数

### 更新 CI/CD 配置

CI/CD 配置文件位于 `.github/workflows/docs-test.yml`。修改时注意：

- 保持与主 CI 工作流的一致性
- 测试触发条件要合理，避免过度运行
- 确保所有作业都有适当的依赖关系

## 最佳实践

### 编写文档时

1. **使用模板**: 从 `docs/zh-CN/_template.md` 开始
2. **检查链接**: 编写完成后运行链接测试
3. **验证章节**: 确保包含所有必需章节
4. **本地测试**: 提交前在本地运行所有测试

### 审查 PR 时

1. **检查测试结果**: 确保所有测试通过
2. **查看质量报告**: 关注文档质量指标
3. **验证内容**: 确保文档内容准确完整
4. **检查格式**: 确保符合 Markdown 规范

### 定期维护

1. **每月审查**: 检查文档是否与代码同步
2. **更新链接**: 修复失效的外部链接
3. **改进测试**: 根据实际情况调整测试规则
4. **监控覆盖率**: 确保 API 文档覆盖率保持 100%

## 故障排查

### 测试脚本无法运行

```bash
# 重新安装依赖
npm install

# 检查 Node.js 版本
node --version  # 应该是 18.0 或更高

# 直接运行脚本
node scripts/test-docs-existence.js
```

### CI/CD 测试失败但本地通过

1. 检查文件路径大小写（Linux 区分大小写）
2. 确保所有文件都已提交到 Git
3. 检查 `.gitignore` 是否排除了必需文件
4. 查看 CI 日志获取详细错误信息

### 性能问题

如果测试运行缓慢：

1. 减少链接检查的文件数量
2. 使用缓存加速 CI/CD
3. 考虑并行运行独立的测试

## 相关资源

- [文档质量检查指南](QUALITY-CHECK-GUIDE.md)
- [文档质量检查摘要](QUALITY-CHECK-SUMMARY.md)
- [脚本说明](../scripts/README.md)
- [贡献指南](zh-CN/development/contributing.md)

## 反馈和改进

如果您对文档测试有任何建议或发现问题，欢迎：

- 提交 [Issue](https://github.com/yourusername/antigravity-agent/issues)
- 提交 [Pull Request](https://github.com/yourusername/antigravity-agent/pulls)
- 在团队讨论中提出

---

**最后更新**: 2025-12-04  
**维护者**: Antigravity Agent 团队
