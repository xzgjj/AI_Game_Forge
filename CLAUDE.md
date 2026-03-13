# CLAUDE.md - AI Game Forge 协作准则

## 项目简介
AI Game Forge 是一个跨平台桌面应用（Tauri + Svelte + Rust），用于分步式生成 Unity 游戏。项目把“游戏生成流程”固定为 8 步，每一步一个页面，并通过 Unity Bridge 自动对接 Unity 项目。

## 构建与运行
### 环境准备
- Node.js 20+
- Rust 1.70+
- Tauri CLI: `cargo install tauri-cli`

### 安装与运行
```bash
npm install
cd src-tauri && cargo fetch

npm run tauri dev
npm run dev
npm run tauri build
```

## 核心流程（不可随意更改）
1. 游戏设计
2. AI 设计架构
3. Unity 项目初始化
4. AI 生成核心脚本（含人物画像/台词生成与分配）
5. AI 生成人物
6. AI 生成资源
7. 玩法迭代
8. 打包发布

说明：每一步为独立页面，顶部显示进度，底部为当前内容与“完成/下一步”。

## AI 能力分配（必须体现）
- AI 可直接生成：玩家控制、敌人 AI、碰撞检测、状态机
- AI 非常适合：游戏系统拆分、Script 结构设计
- AI 可辅助：核心循环、数值设计、技能系统

## Unity 对接约束（必须遵守）
1. Unity 项目必须可自动初始化（模板场景 + 目录结构）。
2. 生成脚本以 UPM 包形式注入，包含 manifest 与 GUID 映射。
3. Unity Editor 工具负责导入、校验、回滚。
4. 生成流程必须可追溯、可回滚。

## 数据与安全
- 数据持久化：SQLite + Diesel ORM，所有变更必须通过迁移文件。
- 敏感信息：使用系统安全存储（Tauri plugin-store），禁止硬编码。
- 审计追踪：AI 调用、人工修改与发布记录必须可追溯。

## 代码规范
### Rust
- 使用 `cargo fmt` 与 `cargo clippy`
- 生产代码禁止 `unwrap`
- 公共 API 必须文档注释

### TypeScript / Svelte
- 使用 ESLint + Prettier
- 严格 TypeScript 模式，避免 `any`
- 组件命名：PascalCase，文件名 kebab-case

## 协作要求
- 文档必须与代码一致，禁止“文档声称已实现但代码为空”。
- 每次结构性变更必须更新 `notes.txt` 与 `diff.md`。
- 新增流程或步骤变更必须先讨论并记录。

---
*最后更新：2026-03-13*
