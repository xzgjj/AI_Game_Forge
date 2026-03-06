# 贡献指南

感谢您对GameCraft AI Studio项目的关注！我们欢迎所有形式的贡献，包括但不限于代码、文档、测试、问题报告和功能建议。

## 🚀 开始贡献

### 1. 设置开发环境

请参阅 [README.md](README.md) 中的"快速开始"部分，确保您的开发环境配置正确。

### 2. 选择任务

- **新手任务**: 查看 [Good First Issues](https://github.com/xzgjj/AI_Game_Forge/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
- **功能开发**: 查看 [Feature Requests](https://github.com/xzgjj/AI_Game_Forge/issues?q=is%3Aissue+is%3Aopen+label%3Afeature)
- **Bug修复**: 查看 [Bug Reports](https://github.com/xzgjj/AI_Game_Forge/issues?q=is%3Aissue+is%3Aopen+label%3Abug)

### 3. Fork仓库

1. 点击GitHub页面右上角的"Fork"按钮
2. 克隆您的Fork到本地：
   ```bash
   git clone https://github.com/your-username/AI_Game_Forge.git
   cd AI_Game_Forge
   ```

### 4. 创建分支

为您的更改创建一个新分支：
```bash
git checkout -b feat/your-feature-name
# 或
git checkout -b fix/issue-description
```

分支命名约定：
- `feat/`: 新功能
- `fix/`: Bug修复
- `docs/`: 文档更新
- `test/`: 测试相关
- `refactor/`: 代码重构
- `chore/`: 构建/工具更新

## 📝 开发流程

### 代码规范

#### Rust代码
```bash
# 运行Clippy检查
cargo clippy -- -D warnings

# 格式化代码
cargo fmt

# 运行测试
cargo test
```

#### TypeScript/Svelte代码
```bash
# 运行ESLint检查
npm run lint

# 格式化代码
npm run format

# 类型检查
npm run check
```

### 提交规范

使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<类型>[可选范围]: <描述>

[可选正文]

[可选脚注]
```

类型：
- `feat`: 新功能
- `fix`: Bug修复
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建/工具更新

示例：
```
feat(auth): 添加微信扫码登录功能

- 实现微信OAuth2.0认证流程
- 添加微信登录组件
- 更新认证服务接口

Closes #123
```

### 测试要求

1. **单元测试**: 所有新功能必须包含单元测试
2. **集成测试**: 重要的业务逻辑需要集成测试
3. **E2E测试**: 用户流程需要端到端测试

运行测试：
```bash
# 前端测试
npm test

# Rust测试
cd src-tauri && cargo test

# 所有测试
./scripts/run-all-tests.sh
```

## 🔧 代码审查流程

### 1. 推送更改

```bash
git push origin your-branch-name
```

### 2. 创建Pull Request

1. 访问您的Fork仓库
2. 点击"Compare & pull request"
3. 填写PR模板：
   - **标题**: 简洁描述更改内容
   - **描述**: 详细说明更改内容、动机和测试情况
   - **关联Issue**: 使用 `Closes #123` 或 `Fixes #123`
   - **检查清单**: 确保完成所有必填项

### 3. PR模板

```markdown
## 更改描述
<!-- 详细描述这次PR做了什么 -->

## 相关Issue
<!-- 关联的Issue编号，例如：Closes #123 -->

## 测试情况
<!-- 描述如何进行测试的，测试结果如何 -->

## 检查清单
- [ ] 代码通过所有测试
- [ ] 代码符合代码规范
- [ ] 添加了必要的测试
- [ ] 更新了相关文档
- [ ] 提交信息符合规范
```

### 4. 代码审查

- 至少需要一名核心贡献者批准
- 所有CI检查必须通过
- 解决所有审查意见

### 5. 合并

- 使用"Squash and merge"选项
- 确保合并信息清晰
- 删除已合并的分支

## 🐛 报告问题

### Bug报告

使用 [GitHub Issues](https://github.com/xzgjj/AI_Game_Forge/issues) 报告Bug，请包含：

1. **标题**: 简洁描述问题
2. **描述**: 详细说明问题
3. **重现步骤**: 如何重现问题
4. **期望行为**: 期望的结果
5. **实际行为**: 实际的结果
6. **环境信息**:
   - 操作系统
   - 应用版本
   - 相关配置
7. **日志/截图**: 如果有的话

### 功能请求

1. **问题描述**: 您想解决什么问题
2. **解决方案**: 您建议的解决方案
3. **替代方案**: 考虑过的其他方案
4. **附加信息**: 任何其他相关信息

## 📚 文档贡献

### 文档类型

1. **API文档**: 代码注释生成的文档
2. **用户指南**: 如何使用应用
3. **开发指南**: 如何开发贡献
4. **架构文档**: 系统设计和决策

### 文档标准

- 使用清晰、简洁的语言
- 包含代码示例
- 保持文档与代码同步
- 使用Markdown格式

## 🏗️ 架构决策

### 提出新架构

1. 创建 [Architecture Decision Record (ADR)](docs/adr/)
2. 包含：
   - 问题描述
   - 考虑的方案
   - 决策结果
   - 后果
3. 提交PR讨论

### 修改现有架构

1. 更新相关ADR
2. 确保向后兼容
3. 提供迁移指南

## 🎯 新手任务

如果您是第一次贡献，可以从以下任务开始：

### 前端任务
- 添加新的UI组件
- 改进现有组件的样式
- 添加单元测试
- 优化性能

### 后端任务
- 添加数据模型
- 实现简单的API端点
- 编写数据库迁移
- 添加错误处理

### 文档任务
- 改进现有文档
- 添加代码示例
- 翻译文档
- 创建教程

## 🤝 行为准则

我们遵守 [贡献者公约](CODE_OF_CONDUCT.md)。请确保：
- 使用友好和尊重的语言
- 接受不同的观点和经验
- 提供有建设性的批评
- 关注社区的最佳利益

## 📞 获取帮助

- **问题讨论**: [GitHub Discussions](https://github.com/xzgjj/AI_Game_Forge/discussions)
- **即时聊天**: [Discord/Slack频道]（待定）
- **邮件列表**: [开发组邮件]（待定）

## 🙏 致谢

感谢所有贡献者！您的每一份贡献都让这个项目变得更好。

---

*一起打造最好的AI游戏创作工具！* 🎮✨

