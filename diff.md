# GameCraft AI Studio 关键代码修改记录



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
