# 文档质量检查脚本

本目录包含用于检查和维护文档质量的脚本。

## check-docs-quality.js

全面的文档质量检查脚本，用于验证文档的完整性、格式规范和链接有效性。

### 功能

1. **文档完整性检查**
   - 验证所有必需的文档文件是否存在
   - 检查每个文档是否包含必需的章节
   - 基于 requirements.md 中定义的验收标准

2. **Markdown 格式检查**
   - 检查标题格式（# 后需要空格）
   - 检查尾随空格
   - 检查多个连续空行
   - 符合 Markdown 最佳实践

3. **链接有效性检查**
   - 验证内部文档链接
   - 识别失效的相对路径链接
   - 统计外部链接和锚点链接

4. **质量报告生成**
   - 生成详细的控制台报告
   - 保存 JSON 格式的详细报告到 `docs-quality-report.json`
   - 提供可操作的改进建议

### 使用方法

#### 通过 npm 脚本运行（推荐）

```bash
npm run check-docs
```

#### 直接运行

```bash
node scripts/check-docs-quality.js
```

### 输出示例

```
=== 文档质量检查 ===

1. 检查文档完整性...
  ✓ docs/zh-CN/getting-started/README.md - 完整
  ✓ docs/zh-CN/getting-started/installation.md - 完整
  ! docs/zh-CN/user-guide/user-guide.md - 缺少章节: 导入导出

2. 检查 Markdown 格式...
  ✓ docs/zh-CN/getting-started/README.md - 格式正确
  ! docs/zh-CN/user-guide/user-guide.md - 5 个格式问题

3. 检查链接有效性...
  ✓ docs/zh-CN/getting-started/README.md - 所有链接有效 (10 个)
  ✗ docs/zh-CN/user-guide/api-reference.md - 2 个失效链接

=== 质量报告 ===

总体统计:
  文档总数: 14
  已创建: 14 (100%)
  完整: 13 (93%)
  不完整: 1
  缺失: 0

状态: 需要改进
建议: 修复上述问题以提高文档质量

详细报告已保存到: docs-quality-report.json
```

### 退出代码

- `0`: 所有检查通过，文档质量优秀
- `1`: 发现需要改进的问题

### 报告文件

脚本会生成 `docs-quality-report.json` 文件，包含：

```json
{
  "totalDocs": 14,
  "existingDocs": 14,
  "missingDocs": [],
  "completeDocs": [...],
  "incompleteDocs": [...],
  "formatIssues": [...],
  "linkIssues": [...],
  "errors": []
}
```

### 集成到 CI/CD

可以在 GitHub Actions 或其他 CI/CD 流程中使用此脚本：

```yaml
- name: Check documentation quality
  run: npm run check-docs
```

脚本会在发现问题时返回非零退出代码，导致 CI 构建失败。

### 自定义检查规则

要修改检查规则，编辑 `check-docs-quality.js` 中的 `requiredDocs` 对象：

```javascript
const requiredDocs = {
  'docs/zh-CN/getting-started/README.md': {
    requiredSections: ['项目概览', '主要功能', '技术栈'],
    description: '项目概览文档'
  },
  // 添加更多文档...
};
```

### 常见问题

**Q: 如何修复"Missing space after heading marker"错误？**

A: 在 Markdown 标题的 `#` 符号后添加空格：
```markdown
# 正确的标题
#错误的标题
```

**Q: 如何修复"Trailing spaces"警告？**

A: 删除行尾的空格。大多数编辑器可以配置为自动删除尾随空格。

**Q: 链接检查失败怎么办？**

A: 检查相对路径是否正确，确保链接的文件存在。

### 维护

- 当添加新文档时，更新 `requiredDocs` 对象
- 定期运行检查以确保文档质量
- 在发布前运行完整检查


---

## 文档测试脚本

### test-docs-existence.js

测试所有必需的文档文件是否存在。

**使用方法:**
```bash
npm run test:docs:existence
```

**测试内容:**
- 验证所有必需的文档文件是否存在
- 检查四个文档层次（入门、使用、开发、进阶）

### test-docs-sections.js

测试文档文件是否包含必需的章节。

**使用方法:**
```bash
npm run test:docs:sections
```

**测试内容:**
- 验证 README.md 包含项目概览、功能和技术栈
- 验证安装指南包含平台特定说明
- 验证使用手册包含所有功能文档
- 验证 API 参考包含所有命令分类
- 验证开发文档包含架构和贡献指南

### test-docs-links.js

测试文档中内部链接的有效性。

**使用方法:**
```bash
npm run test:docs:links
```

**测试内容:**
- 检查所有内部 Markdown 链接
- 验证链接的文件是否存在
- 跳过外部 URL 和纯锚点链接
- 排除模板文件

### test-api-coverage.js

测试 API 文档覆盖率，比较文档中的命令与代码中实际的 Tauri 命令。

**使用方法:**
```bash
npm run test:docs:api
```

**测试内容:**
- 从 Rust 源文件中提取所有 Tauri 命令
- 验证每个命令是否在 api-reference.md 中有文档
- 报告覆盖率百分比

### 运行所有文档测试

一次运行所有文档测试：

```bash
npm run test:docs
```

这将按顺序执行所有四个测试脚本：
1. 文档存在性测试
2. 章节完整性测试
3. 链接有效性测试
4. API 覆盖率测试

## CI/CD 集成

文档测试在以下场景中自动运行：

1. **Pull Request**: 当文档文件被修改时
2. **Push**: 当更改推送到 master 或 dev 分支时
3. **定期调度**: 每周一早上 8:00 UTC
4. **手动触发**: 可通过 GitHub Actions 手动触发

查看 `.github/workflows/docs-test.yml` 了解完整的 CI/CD 配置。

## 退出代码

所有测试脚本遵循相同的退出代码约定：
- `0`: 所有测试通过
- `1`: 发现问题或测试失败
