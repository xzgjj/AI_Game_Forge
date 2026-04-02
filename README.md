# GameCraft AI Studio

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-5.0%2B-FF3E00)](https://svelte.dev/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0%2B-FFC131)](https://tauri.app/)

**GameCraft AI Studio** 是一个跨平台桌面应用（Windows/macOS），通过工作台式 AI 协作流程帮助用户快速创建个性化 Unity 游戏。



##  核心特性

- **🎮 多元登录系统**：支持微信扫码、手机验证、邮箱注册、第三方OAuth（GitHub/Google）
- **🧙‍♂️ 项目工作台**：页面化工作流，覆盖新建项目、资源导入、内容画布、协作优化与发布记录
- **🤖 AI协作工作区**：实时生成游戏元素（角色、场景、对话），支持可视化编辑、局部重生成与反馈循环
- **📊 项目与版本管理**：多版本快照、diff、回滚与发布记录，支持主流游戏引擎格式导出
- **🔍 审计追踪**：完整记录所有AI调用、用户操作、生成历史
- **💰 API统计面板**：多厂商AI提供商管理，实时Token消耗统计，成本分析与预算管理

##  用户工作流（5 页）

1. **新建项目**：最少输入完成项目创建和需求冻结
2. **资源生成 / 导入**：参考素材、AI 生成资源、校验和同步
3. **内容画布**：角色、台词、背景和脚本节点的图谱化编辑
4. **协作优化**：diff、局部重生成、验证和回滚
5. **发布 / 版本记录**：构建、发布、审计和历史版本管理

##  UI 设计协作

当前前端仍在从线性流程向工作台式页面过渡，因此 UI 设计和实现需要同步校验实际流转。

设计协作基线见 [doc/figma_ui_spec.md](doc/figma_ui_spec.md)。

当前优先核对的设计范围：
- 登录页的多入口分支与失败态
- 仪表盘的项目管理和主操作入口
- 配置向导的结构化表单与 Unity 状态反馈
- AI 画布的输入、预览和历史记录

##  内部生成阶段（8 阶段）

1. **项目启动与工程骨架**
2. **需求冻结与设计摘要**
3. **系统拆分与内容图谱初始化**
4. **资源目录与清单建立**
5. **角色 / 台词 / 脚本生成**
6. **Unity 最小场景验证**
7. **版本快照与协作迭代**
8. **发布打包与审计归档**



##  系统架构



### 技术栈
- **桌面框架**: Tauri 2.0 + Rust
- **前端框架**: Svelte 5 + TypeScript
- **UI/动画**: Tailwind CSS + Framer Motion + Lottie
- **本地存储**: SQLite + Diesel ORM
- **状态管理**: Svelte Stores



### 逻辑架构
```
┌─────────────────────────────────────────────────────────────┐
│                        表示层 (Presentation)                  │
│  ├─ 项目工作台组件 (ConfigWizard)                            │
│  ├─ AI协作画布 (AICanvas)                                    │
│  ├─ 项目管理器 (ProjectManager)                              │
│  ├─ 用户设置面板 (UserSettings)                              │
│  └─ API统计面板 (APIStatsDashboard)                          │
└─────────────────────────────────────────────────────────────┘
                                │ IPC (强类型接口)
┌─────────────────────────────────────────────────────────────┐
│                     业务逻辑层 (Business Logic)               │
│  ├─ 游戏配置服务 (GameConfigService)                         │
│  ├─ AI协作引擎 (AICollabEngine) ──┬─ OpenAI适配器            │
│  ├─ API管理服务 (APIMgmtService)  ├─ Claude适配器           │
│  ├─ 认证服务 (AuthService)        ├─ 智谱AI适配器            │
│  ├─ 资产管理器 (AssetManager)     ├─ 百度文心适配器          │
│  └─ 审计追踪器 (AuditTracker)     └─ 本地模型适配器          │
└─────────────────────────────────────────────────────────────┘
                                │ 数据访问
┌─────────────────────────────────────────────────────────────┐
│                        数据层 (Data Layer)                   │
│  ├─ 本地数据库 (SQLite)                                      │
│  ├─ 文件系统存储 (游戏资产、配置)                              │
│  └─ 外部API网关 (AI服务、OAuth、短信)                          │
└─────────────────────────────────────────────────────────────┘
```



##  快速开始



### 环境要求
- **Rust 1.70+**: `rustup install stable`
- **Node.js 20+**: 推荐使用 [nvm](https://github.com/nvm-sh/nvm)
- **Tauri CLI**: `cargo install tauri-cli`
- **Git**: 版本控制



### 安装与运行
```bash
# 1. 克隆项目
git clone https://github.com/xzgjj/AI_Game_Forge.git
cd AI_Game_Forge

# 2. 安装依赖
npm install
cd src-tauri && cargo fetch

# 3. 配置环境变量
cp .env.example .env
# 编辑.env文件，填写认证和AI服务配置

# 4. 开发模式运行
npm run tauri dev

# 5. 构建发布版本
npm run tauri build




```

### 开发工作流
```bash
# 前端开发（热重载）
npm run dev

# Rust后端开发
cd src-tauri && cargo watch -x run

# 代码检查
npm run lint
npm run check

# 测试
npm test
cd src-tauri && cargo test
```



## 项目结构

```
gamecraft-ai-studio/
├── src-tauri/                    # Tauri后端（Rust）
│   ├── src/
│   │   ├── ipc/                 # IPC接口定义
│   │   ├── services/            # 业务逻辑服务
│   │   ├── models/              # 数据模型
│   │   ├── database/            # 数据库模块
│   │   ├── providers/           # AI提供商实现
│   │   └── utils/               # 工具函数
│   └── Cargo.toml               # Rust依赖配置
├── src/                         # 前端源码（Svelte + TypeScript）
│   ├── lib/
│   │   ├── components/          # Svelte组件
│   │   ├── stores/              # 状态管理
│   │   ├── services/            # 前端服务
│   │   ├── types/               # TypeScript类型定义
│   │   └── utils/               # 前端工具函数
│   ├── styles/                  # 全局样式
│   └── main.ts                  # 应用入口
├── config/                      # 配置文件
├── docs/                        # 项目文档
└── tests/                       # 测试文件
```



##  配置说明



### AI提供商配置
支持多厂商AI服务：
- **OpenAI**: GPT
- **Anthropic**: Claude 3系列
- **智谱AI**: GLM
- **deepseek**: ERNIE 4.0
- **本地模型**: Ollama + Llama 3（隐私保护）



### 认证配置
- **微信OAuth**: 企业认证应用
- **短信服务**: 阿里云/腾讯云
- **邮箱验证**: SMTP服务
- **第三方OAuth**: GitHub, Google



##  贡献指南



我们欢迎所有形式的贡献！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

1. **报告问题**: 使用 [GitHub Issues](https://github.com/xzgjj/AI_Game_Forge/issues)
2. **功能请求**: 提交详细的提案
3. **代码贡献**: Fork项目并提交Pull Request
4. **文档改进**: 帮助完善文档和示例



### 代码规范
- **Rust**: 遵循Clippy检查，rustfmt格式化
- **TypeScript**: ESLint + Prettier，严格模式
- **提交信息**: Conventional Commits规范
- **测试**: 单元测试 + 集成测试 + E2E测试



##  许可证

本项目采用双重许可证：
- **MIT License**: [LICENSE-MIT](LICENSE-MIT)
- **Apache License 2.0**: [LICENSE-APACHE](LICENSE-APACHE)

您可以选择任一许可证。



##  致谢

- **Tauri团队**: 提供优秀的跨平台桌面应用框架
- **Svelte团队**: 创新的前端框架
- **AI提供商**: OpenAI, Anthropic, 智谱AI, 百度等
- **所有贡献者**: 感谢你们的代码、反馈和支持



##  联系方式

- **GitHub**: [xzgjj/AI_Game_Forge](https://github.com/xzgjj/AI_Game_Forge)
- **问题反馈**: [GitHub Issues](https://github.com/xzgjj/AI_Game_Forge/issues)
- **讨论区**: [GitHub Discussions](https://github.com/xzgjj/AI_Game_Forge/discussions)

---

*让创意成为游戏，让AI成为您的创作伙伴。* 🎮✨
