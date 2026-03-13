# GameCraft AI Studio 关键代码修改记录

## 2026-03-14 修复 cargo test 编译并添加占位图标

### 涉及文件
1. `src-tauri/src/main.rs`
2. `src-tauri/src/database/mod.rs`
3. `src-tauri/src/database/migrations/mod.rs`
4. `src-tauri/src/models/auth_session.rs`
5. `src-tauri/src/services/auth_service.rs`
6. `src-tauri/src/services/api_mgmt_service.rs`
7. `src-tauri/src/utils/*.rs`
8. `src-tauri/icons/icon.ico`

### 核心 Diff 摘要
- 修复 Tauri API 适配与数据库调用签名，保证 `cargo test` 通过。
- 补齐缺失的 `utils` 模块文件占位实现。
- 生成 `icons/icon.ico` 占位图标。

### 修改意图
1. 消除编译错误，恢复测试流程。
2. 提供基础占位实现，待后续替换为正式逻辑。

### 对项目的影响
1. `cargo test` 已可执行（仍有警告）。
2. 图标暂为占位，待替换为正式品牌资源。

## 2026-03-14 修复 Cargo lib 入口以支持测试

### 涉及文件
1. `src-tauri/src/lib.rs`

### 核心 Diff 摘要
- 新增 `src-tauri/src/lib.rs`，补齐 Rust 库入口，支持 `cargo test`。

### 修改意图
1. 解决 Cargo 无法解析库入口的问题。

### 对项目的影响
1. Rust 测试与迁移验证可恢复执行。

## 2026-03-14 修复 Cargo 依赖配置以便测试

### 涉及文件
1. `src-tauri/Cargo.toml`

### 核心 Diff 摘要
- 增加 `ollama-rs` 可选依赖，修复 feature 指向缺失依赖的问题。
- 调整 `tauri` features，移除无效的 `api-all` 等配置。

### 修改意图
1. 让 `cargo test` 能正常解析依赖与特性。

### 对项目的影响
1. 测试配置更贴合当前 Tauri 2 特性集合。

## 2026-03-14 WizardState SQLite 持久化 + 加载入口

### 涉及文件
1. `src-tauri/migrations/202603140001_add_wizard_states/up.sql`
2. `src-tauri/migrations/202603140001_add_wizard_states/down.sql`
3. `src-tauri/src/ipc/wizard.rs`
4. `src-tauri/src/ipc/mod.rs`
5. `src-tauri/src/main.rs`
6. `src/lib/services/wizard.service.ts`
7. `src/lib/components/wizard/ConfigWizard.svelte`
8. `src/lib/types/wizard.types.ts`

### 核心 Diff 摘要
- 新增 `wizard_states` 表迁移并使用 SQLite 存储 WizardState。
- 增加 `load_latest_wizard_state` IPC 与前端“加载上次 WizardState”入口。
- Step3/Step4 保存 WizardState 到 SQLite。

### 修改意图
1. 落地结构化产物数据库持久化。
2. 提供恢复入口，支持断点续作。

### 对项目的影响
1. WizardState 可跨会话恢复，流程更可用。
2. 依赖数据库迁移成功执行。

## 2026-03-14 WizardState 按项目路径加载

### 涉及文件
1. `src-tauri/src/ipc/wizard.rs`
2. `src-tauri/src/main.rs`
3. `src/lib/services/wizard.service.ts`
4. `src/lib/components/wizard/ConfigWizard.svelte`

### 核心 Diff 摘要
- 新增 `load_wizard_state_by_project` IPC 命令。\n- 前端增加“按路径加载”入口，按 `projectPath` 精确恢复 WizardState。

### 修改意图
1. 支持多项目并行恢复，减少误加载。\n2. 提升可用性与可控性。

### 对项目的影响
1. 持久化检索更精确，可按项目路径恢复。

## 2026-03-13 配置向导升级为 8 步与 WizardState 结构化产物

### 涉及文件
1. `src/lib/components/wizard/ConfigWizard.svelte`
2. `src/lib/types/wizard.types.ts`

### 核心 Diff 摘要
- ConfigWizard 从 6 步扩展为 8 步，并增加步骤校验与进度导航。
- 新增 WizardState 结构化产物类型，覆盖 8 步核心字段。
- 提供结构化产物实时预览，便于后续持久化与审计接入。

### 修改意图
1. 与 8 步流程标准对齐，确保 UI 与业务一致。
2. 为 Unity Bridge 与 AI 生成管线提供结构化输入输出基础。

### 对项目的影响
1. 8 步流程在前端可视化落地，减少需求偏差。
2. WizardState 成为后续存储、审计与导出 Unity 的核心数据结构。

## 2026-03-13 Unity Bridge 最小闭环（模板 + UPM 注入 + 校验）

### 涉及文件
1. `src-tauri/src/services/unity_bridge_service.rs`
2. `src-tauri/src/ipc/unity_bridge.rs`
3. `src-tauri/src/ipc/mod.rs`
4. `src-tauri/src/services/mod.rs`
5. `src-tauri/src/main.rs`

### 核心 Diff 摘要
- 新增 Unity Bridge 服务：初始化 Unity 项目目录、生成基础场景与 manifest。
- 新增 UPM 注入：创建本地包并写入 `Packages/manifest.json`。
- 新增基础校验：检查关键文件并对 C# 脚本做静态检查。
- 暴露 IPC 命令：`unity_init_project`、`unity_inject_upm`、`unity_validate_project`。

### 修改意图
1. 建立 Unity 对接最小闭环，保证“项目模板 + 脚本包 + 校验”可执行。
2. 为后续 AI 生成脚本的自动导入与验证打基础。

### 对项目的影响
1. Unity Bridge 从空白变为可调用的最小实现。
2. 校验仍为静态检查，需后续接入 Unity 实际编译验证。

## 2026-03-13 Step3/Step4 前端接入 Unity Bridge

### 涉及文件
1. `src/lib/components/wizard/ConfigWizard.svelte`
2. `src/lib/services/unity.service.ts`
3. `src/lib/types/unity.types.ts`

### 核心 Diff 摘要
- Step3 绑定 Unity 初始化调用，Step4 绑定 UPM 注入与校验调用。
- 新增 Unity IPC 调用封装与类型定义（前端层）。
- 在向导中展示 Unity 对接状态与错误提示。

### 修改意图
1. 打通 Step3/Step4 的前端调用链路，确保流程可执行。
2. 为后续脚本生成与导入提供最小验证闭环。

### 对项目的影响
1. 8 步流程在 UI 层具备 Unity 对接能力。
2. 仍为静态校验，后续可升级至 BatchMode 编译验证。

## 2026-03-13 Step3/Step4 持久化 + BatchMode 编译校验

### 涉及文件
1. `src-tauri/src/services/unity_bridge_service.rs`
2. `src-tauri/src/ipc/unity_bridge.rs`
3. `src-tauri/src/ipc/wizard.rs`
4. `src-tauri/src/ipc/mod.rs`
5. `src-tauri/src/main.rs`
6. `src/lib/services/unity.service.ts`
7. `src/lib/services/wizard.service.ts`
8. `src/lib/types/unity.types.ts`
9. `src/lib/types/wizard.types.ts`
10. `src/lib/components/wizard/ConfigWizard.svelte`

### 核心 Diff 摘要
- 增加 Unity BatchMode 编译校验能力与 Editor 验证脚本注入。\n- Step3/Step4 保存 WizardState 到 `.aigameforge/wizard_state.json`。\n- 前端接入 BatchMode 校验开关与 Unity Editor 路径配置。

### 修改意图
1. 为核心流程增加真实编译校验能力（可选）。\n2. 在关键步骤落地结构化产物持久化。

### 对项目的影响
1. Step3/Step4 可落地本地持久化。\n2. BatchMode 校验依赖本机 Unity Editor 路径。

## 2026-03-13 统一 8 步流程与 Unity 对接文档重写

### 涉及文件
1. `doc/AI_Game_Forge_Report.md`
2. `CLAUDE.md`
3. `notes.txt`
4. `implementation_plan.md`
5. `AGENT_EXECUTION_PROTOCOL.md`

### 核心 Diff 摘要
- 重写项目报告，固定 8 步流程并明确 Unity 对接策略。
- 更新协作准则，强调文档与实现一致性、AI 生成范围与 Unity 约束。
- 更新开发笔记，记录流程不对齐与 Unity 对接缺失的问题。
- 新增实施计划与代理执行协议，落地执行规则与验收标准。

### 修改意图
1. 纠正流程定义偏差，明确“每一步一个页面”的 UI 结构。
2. 强化 Unity 自动对接与脚本生成的硬性要求。
3. 建立统一的执行与审计规范，避免文档虚高。

### 对项目的影响
1. 目标流程与实现方向被明确锁定，减少后续偏航风险。
2. 文档与执行规范统一，便于团队协作与审计。

## 2026-03-06 创建项目协作准则和开发笔记系统

### 涉及文件
1. `CLAUDE.md` - 新建文件
2. `notes.txt` - 新建文件
3. `.gitignore` - 修改文件

### 核心 Diff 摘要

**1. `.gitignore` 修改**
```diff
- CLAUDE.md
- notes.txt
+ # CLAUDE.md - 由AI维护的项目准则，建议保留本地
+ # notes.txt - 开发笔记，现在由git跟踪以保持同步
```

**2. `CLAUDE.md` 新增内容摘要**
- 项目简介：AI协作式游戏生成工具
- 构建与运行指令：Node.js 20+, Rust 1.70+, Tauri CLI
- 代码规范：Rust/TypeScript/Svelte编码标准
- 核心约束：AI提供商兼容性、DeepSeek R1思考Token处理
- AI协作原则：明确允许/禁止的修改范围

**3. `notes.txt` 新增内容摘要**
- 当前进度：框架搭建完成，认证/AI引擎待实现
- 关键决策：创建CLAUDE.md规范AI协作
- 待办事项：分优先级列出后续开发任务
- 上下文断点：会话重启需知的关键信息

### 修改意图
1. **建立AI协作规范**：为Claude等AI助手提供明确的项目准则，避免架构偏离
2. **记录开发进度**：创建增量更新机制，跟踪项目状态和决策
3. **同步机制**：通过Git跟踪notes.txt，确保开发笔记与代码库同步
4. **DeepSeek R1特别处理**：明确思考Token的过滤要求和统计方式

### 对项目的影响
1. **协作标准化**：为AI协作提供清晰的边界和规范
2. **上下文管理**：解决长时间开发中的会话重启问题
3. **进度可视化**：明确当前状态和后续方向
4. **知识传承**：记录关键决策原因，便于团队理解

---

*本文档记录项目的关键代码修改，按时间倒序排列*

## 2026-03-06 业务逻辑层最小可用实现与测试

### 涉及文件
1. `src-tauri/src/services/auth_service.rs`
2. `src-tauri/src/services/game_config_service.rs`
3. `src-tauri/src/services/ai_collab_service.rs`
4. `src-tauri/src/services/api_mgmt_service.rs`
5. `src-tauri/src/services/provider_manager.rs`
6. `src-tauri/src/services/user_service.rs`
7. `src-tauri/src/services/audit_service.rs`
8. `src-tauri/src/services/mod.rs`
9. `src-tauri/src/lib.rs`
10. `src-tauri/src/utils/mod.rs`
11. `src-tauri/tests/business_logic/smoke.rs`
12. `.gitignore`
13. `README.md`
14. `doc/说明.md`

### 核心 Diff 摘要
- 认证服务：实现微信/手机/邮箱/OAuth 登录、会话验证、刷新、登出和失败次数限制。
- 游戏配置服务：实现草稿创建、故事字段更新、角色/地点/机制写入与版本递增。
- AI 协作服务：实现提供商选择、历史记录、再生成、DeepSeek `<thinking>` 清洗。
- API 管理服务：实现平衡路由、统计聚合、预算校验与预算告警。
- 用户/审计服务：实现用户资料与偏好更新、消费累积、审计事件记录与筛选。
- 服务初始化：增加 `ServiceContainer`，统一托管核心业务服务。
- 测试：增加业务层 smoke 测试和服务内单测（受当前环境影响未执行 Rust 测试）。

### 修改意图
1. 让业务逻辑层从“空壳 TODO”进入“可调用、可测试、可扩展”的最小可用状态。
2. 对齐模块功能清单中的认证、AI管理、配置、审计和用户管理核心路径。
3. 为后续 IPC 层联调和数据层持久化接入提供稳定服务接口。

### 对项目的影响
1. 业务服务已具备基础可运行闭环，前端可逐步切换到真实调用。
2. 提供商路由与预算告警可支持多厂商 token/成本治理。
3. 由于本机缺少 Rust 工具链，Rust 测试仍需在安装 `cargo` 后补跑验证。

## 2026-03-06 数据层（Data Layer）实现与最小测试

### 涉及文件
1. `src-tauri/src/database/mod.rs`
2. `src-tauri/src/database/migrations/mod.rs`
3. `src-tauri/src/database/schema.rs`
4. `src-tauri/src/database/repository.rs`
5. `src-tauri/src/lib.rs`
6. `src-tauri/migrations/00000000000001_initial/up.sql`
7. `src-tauri/migrations/00000000000001_initial/down.sql`
8. `src-tauri/tests/data_layer/repository_smoke.rs`

### 核心 Diff 摘要
- 新增首版 SQLite 迁移脚本：创建 users/projects/ai_logs/game_specs/api_stats/auth_sessions 表与关键索引。
- 增强数据库管理器：修正连接初始化、迁移调用、完整性检查与统计信息输出。
- 提供运行时 schema：补齐 Diesel table 定义并允许跨表查询。
- 实现仓储层最小可用 CRUD：用户、项目、AI日志、游戏配置、API统计、认证会话仓储及管理器。
- 增加数据层最小测试：仓储 smoke 测试覆盖创建/查询/活跃会话路径。

### 修改意图
1. 让 Data Layer 从占位文件进入可用状态，支持后续业务层/IPC 层联调。
2. 先以最小实现保障接口稳定，再在后续迭代替换为完整 Diesel 查询实现。
3. 为数据库迁移和结构演进建立统一入口，便于版本化维护。

### 对项目的影响
1. 数据层具备基础结构与迁移脚手架，启动期可完成 schema 建立。
2. 仓储层可直接支持业务层单元测试与离线验证。
3. 当前环境缺少 Rust 工具链，Rust 测试尚未执行，需要补装 cargo 后复测。

## 2026-03-06 基础设施层（Infrastructure Layer）IPC落地与演示构建

### 涉及文件
1. `src-tauri/src/main.rs`
2. `src-tauri/src/ipc/auth.rs`
3. `src-tauri/src/ipc/api_stats.rs`
4. `src-tauri/src/ipc/ai_engine.rs`
5. `src-tauri/src/ipc/game_config.rs`
6. `src-tauri/src/ipc/project.rs`
7. `src-tauri/src/services/mod.rs`
8. `src-tauri/src/services/project_service.rs`
9. `src-tauri/tests/infrastructure_layer/smoke.rs`
10. `src-tauri/tauri.conf.json`

### 核心 Diff 摘要
- IPC 命令从占位逻辑切换为真实服务调用：认证、AI生成、API统计、游戏配置、项目管理全链路可调用。
- 新增 `ProjectService`：实现项目创建、保存版本、加载、导出、列表筛选、软删除/恢复。
- 主进程 `main.rs` 重构：兼容当前 Tauri 2 写法并补全命令注册，统一在 setup 初始化数据库与服务容器。
- 新增最小 `tauri.conf.json`：补齐项目识别配置，修复 `tauri dev` 无配置直接报错问题。
- 增加演示提供商 `demo`：在无外部密钥场景下可走通 AI 生成与统计面板最小闭环。
- 新增基础设施层最小 smoke 测试文件（待 Rust 工具链环境补跑）。

### 修改意图
1. 让基础设施层从“接口定义”进入“端到端可调用”，支撑前端真实 invoke 与演示联调。
2. 在未接入真实多厂商密钥前，提供可稳定复现的 demo provider，避免演示阻塞。
3. 为后续可视化验证和问题定位建立统一入口（主进程命令挂载 + 项目管理服务）。

### 对项目的影响
1. 前端页面可直接调用完整 IPC（认证、配置、AI、项目、统计），不再依赖纯占位接口。
2. 已完成前端构建链验证：`npm run check` / `npm run test -- --run` / `npm run build` 通过（在提权环境）。
3. Rust 侧仍受本机环境限制：`cargo` 未安装，`cargo test` 与 `tauri dev/build` 仍不可执行（当前会报 `cargo metadata ... program not found`）。
