# CLAUDE.md - GameCraft AI Studio 项目准则



## 项目简介

GameCraft AI Studio 是一个跨平台桌面应用，通过AI协作式生成工具，让非专业用户快速创建个性化游戏。



## 构建与运行指令



### 环境准备
- **Node.js 20+** 与 **npm**
- **Rust 1.70+** 与 **Cargo**
- **Tauri CLI**: `cargo install tauri-cli`



### 安装与运行
```bash
# 安装依赖
npm install
cd src-tauri && cargo fetch

# 开发模式运行
npm run tauri dev

# 单独运行前端（热重载）
npm run dev

# 构建发布版本
npm run tauri build
```



## 代码规范



### Rust 代码
- 使用 `cargo fmt` 格式化，遵循 Rust 官方风格
- 使用 `cargo clippy` 进行代码检查，禁止 warnings
- 错误处理：使用 `anyhow` 和 `thiserror`，避免 unwrap 在生产代码中
- 注释：公共API必须添加文档注释（///）

### TypeScript/Svelte 代码
- 使用 ESLint + Prettier，配置见 `.eslintrc` 和 `.prettierrc`
- 类型：严格 TypeScript 模式，避免 `any`
- 组件：Svelte 5 语法，使用 runes 响应式系统
- 命名：camelCase 变量，PascalCase 组件，kebab-case 文件名



### 提交规范

- Conventional Commits 格式：`<类型>[可选范围]: <描述>`
- 类型：feat, fix, docs, style, refactor, test, chore
- 示例：`feat(auth): 添加微信扫码登录功能`



## 常用命令



```bash
# 开发
npm run tauri dev          # 完整Tauri开发模式
npm run dev               # 仅前端热重载
cd src-tauri && cargo watch -x run  # 仅Rust后端热重载

# 代码质量
npm run lint              # ESLint检查
npm run lint:fix          # 自动修复ESLint问题
npm run format            # Prettier格式化
npm run check             # Svelte类型检查
cargo fmt                 # Rust格式化
cargo clippy              # Rust代码检查

# 测试
npm test                  # 前端单元测试
cd src-tauri && cargo test # Rust单元测试

# 构建
npm run tauri build       # 生产构建
```



## 核心约束



1. **AI提供商兼容性**：所有AI提供商必须实现统一的 `Provider` trait，支持流式输出和思考Token处理
2. **数据持久化**：使用 SQLite + Diesel ORM，所有数据变更必须通过迁移文件
3. **安全存储**：敏感信息（API密钥、会话令牌）使用系统安全存储（Tauri plugin-store）
4. **错误处理**：前端展示用户友好错误，后端记录详细日志
5. **性能**：大文件导出、AI生成等耗时操作必须支持进度反馈和取消



## 核心逻辑速览



### 关键模块
- **认证系统** (`src-tauri/src/services/auth_service.rs`)：多方式登录（邮箱、OAuth、微信、手机）
- **AI协作引擎** (`src-tauri/src/services/ai_collab_service.rs`)：协调多个AI提供商，处理思考Token，管理生成历史
- **游戏配置服务** (`src-tauri/src/services/game_config_service.rs`)：游戏类型、美术风格、叙事风格配置
- **API统计面板** (`src-tauri/src/services/api_mgmt_service.rs`)：多厂商Token消耗统计，预算预警
- **项目管理** (`src-tauri/src/services/project.rs`)：版本管理，导出为Unity/Unreal/Godot格式

### 数据流
1. 用户通过前端组件发起请求 → IPC强类型接口 → 后端服务处理
2. AI生成：用户输入 → AI提供商适配器 → 流式返回 → 前端实时显示
3. 审计追踪：所有操作自动记录到 `ai_logs` 表，包含完整上下文



## 安全与代码质量



### 安全要求
- 输入验证：所有用户输入必须在前端和后端双重验证
- API密钥：永不硬编码，通过环境变量或安全存储注入
- SQL注入：使用Diesel查询构造器，禁止原生SQL拼接
- XSS防护：Svelte自动转义，输出内容需清洗

### 代码质量
- 测试覆盖率：关键业务逻辑 >80%
- 依赖更新：定期检查 `npm audit` 和 `cargo audit`
- 性能监控：记录AI调用耗时、内存使用情况
- 日志分级：debug/info/warn/error，生产环境只记录info以上



## AI协作原则



### 修改调整范围
- **允许**：修复bug，优化性能，添加测试，改进文档
- **允许**：实现已规划的功能（参考README路线图）
- **允许**：重构代码以提高可读性，但必须保持接口兼容
- **禁止**：未经讨论更改架构决策（需先创建ADR）
- **禁止**：删除已上线功能，除非有明确迁移方案



### DeepSeek R1 特别注意事项
- **思考Token处理**：DeepSeek R1输出中包含 `<thinking>` 标签，必须在展示前过滤
- **流式响应**：支持思考过程的实时显示，但最终输出需清理中间思考内容
- **成本优化**：思考Token会计入总消耗，需在API统计中准确记录

### 与Claude协作指南
1. **上下文管理**：优先使用现有类型和接口，避免重复定义
2. **增量修改**：每次提交解决一个问题，便于代码审查
3. **错误处理**：提供清晰的错误信息，便于调试
4. **文档更新**：代码变更时同步更新相关注释和文档
5. **性能考虑**：AI生成等耗时操作需添加加载状态和取消支持

---
*最后更新：2026-03-06*
*本文档作为Claude长期协作的准则，请定期审查更新*