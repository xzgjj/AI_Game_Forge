# CLAUDE.md - AI Game Forge 协作准则



## 项目简介
AI Game Forge 是一个跨平台桌面应用（Tauri + Svelte + Rust），用于以工作台方式生成 Unity 游戏。

前台交互采用 5 个用户页面：
- 页面1：新建项目
- 页面2：资源生成 / 导入
- 页面3：内容画布
- 页面4：协作优化
- 页面5：发布 / 版本记录

内部仍保留 8 个生成阶段，但它们是实现与校验流水线，不等同于 8 个用户页面。Unity Bridge 负责自动对接 Unity 项目。



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

### 用户页面
1. 页面1：新建项目
2. 页面2：资源生成 / 导入
3. 页面3：内容画布
4. 页面4：协作优化
5. 页面5：发布 / 版本记录

### 内部生成阶段
1. 项目启动与工程骨架
2. 需求冻结与设计摘要
3. 系统拆分与内容图谱初始化
4. 资源目录与清单建立
5. 角色 / 台词 / 脚本生成
6. Unity 最小场景验证
7. 版本快照与协作迭代
8. 发布打包与审计归档

说明：用户页面和内部阶段必须分离理解。页面决定交互，阶段决定实现与校验。执行细节与各页目的以 `implementation_plan.md` 为准。



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
- 每次结构性变更必须更新 `notes.txt` 与 `diff.md`，必要时同步更新 `README.md`。
- 新增页面、版本流转或内部阶段变更必须先讨论并记录。
- 禁止再次把 5 页工作台写回成线性 8 步向导。

---
*最后更新：2026-03-24*
