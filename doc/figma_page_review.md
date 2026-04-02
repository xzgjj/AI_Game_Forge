# Figma Page Review

最后更新：2026-04-02

## 目的
逐个页面对照当前代码实现，检查视觉层、交互层和状态层是否一致，作为 Figma 设计稿的逐页核对基线。

## 页面范围
1. App Shell
2. 登录页
3. 仪表盘
4. 配置向导
5. AI 画布

## 1. App Shell
### 当前代码基线
- [src/lib/App.svelte](/d:/app_project/AI_Game_Forge/src/lib/App.svelte)

### 视觉对照
- 100vw / 100vh 全屏容器
- 深色渐变背景
- 初始化时居中加载态

### 交互对照
- 应用启动后先检查会话，再进入登录或仪表盘
- 加载结束后只显示一个主页面，不出现并排页面

### 需要核对的点
- 是否把“加载态 -> 认证态 -> 主页面”画成连续状态，而不是三张无关页面
- 是否保留全屏、无滚动的桌面端语义

## 2. 登录页
### 当前代码基线
- [src/lib/components/auth/LoginPage.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/auth/LoginPage.svelte)
- [src/lib/components/auth/LoginForm.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/auth/LoginForm.svelte)
- [src/lib/components/auth/WechatQR.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/auth/WechatQR.svelte)
- [src/lib/components/auth/PhoneVerify.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/auth/PhoneVerify.svelte)
- [src/lib/components/auth/EmailRegister.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/auth/EmailRegister.svelte)

### 视觉对照
- 深色渐变背景，中央卡片式登录容器
- 顶部品牌标题 + 副标题
- 邮箱表单、分隔线、第三方登录、底部版本信息
- 错误提示独立成块，不能塞进输入框占位

### 交互对照
- 登录方式是并列分支，而不是层层跳转
- 邮箱登录提交后进入仪表盘
- 微信、手机、邮箱注册都要有返回主登录表单的路径
- GitHub / Google 是同级登录入口，不是隐藏菜单
- 加载态时输入禁用，按钮文案应变化

### 登录分支验收
- `form`：邮箱密码 + 记住我 + 忘记密码 + 第三方登录
- `wechat`：二维码占位 + 授权码输入 + 返回
- `phone`：手机号 + 验证码 + 返回
- `email`：邮箱注册 + 密码 + 验证码 + 返回

### 需要核对的点
- 是否保留多入口，而不是只画单一邮箱表单
- 当前分支是否足够明确，不会让用户误判位置
- 错误态和加载态是否可见

## 3. 仪表盘
### 当前代码基线
- [src/lib/components/dashboard/Dashboard.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/dashboard/Dashboard.svelte)
- [src/lib/components/project/ProjectManager.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/project/ProjectManager.svelte)
- [src/lib/components/settings/UserSettings.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/settings/UserSettings.svelte)
- [src/lib/components/stats/APIStatsDashboard.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/stats/APIStatsDashboard.svelte)

### 视觉对照
- 顶部工具栏 + 主操作按钮
- 标签页导航条
- 总览卡片、项目管理卡片、设置卡片、统计卡片
- 项目管理区需要明显列表层次

### 交互对照
- 顶部按钮可直接进入配置向导 / AI 画布
- 项目列表“打开”应进入配置向导
- 标签切换不应离开仪表盘上下文
- 设置和统计应作为仪表盘的内容区，不是独立路由

### 仪表盘子页验收
- `overview`：主路径说明 + 周目标
- `projects`：项目列表 + 新建项目 + 打开项目
- `settings`：AI 提供商、预算、深色外观
- `stats`：请求数、Token、成本、分提供商表格

### 需要核对的点
- 是否突出主操作入口，而不是只做信息面板
- 项目管理是否可作为继续工作流的入口
- 卡片密度是否过高

## 4. 配置向导
### 当前代码基线
- [src/lib/components/wizard/ConfigWizard.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/wizard/ConfigWizard.svelte)

### 视觉对照
- 顶部标题 + 当前步骤提示
- 顶部步骤进度条
- 左侧表单区，右侧结构化预览区
- 中间状态块展示 Unity 初始化、UPM、校验和保存结果
- 底部上一步 / 下一步 / 完成按钮

### 交互对照
- 8 步步骤导航可直接跳转
- Unity 初始化与校验状态在流程中可见
- 加载最近状态 / 按路径加载是辅助动作，不应抢主流程注意力
- 完成后进入 AI 画布
- 上一步在首步时要回到仪表盘

### 配置步骤验收
- `design`：游戏名称、类型、核心循环、叙事风格、美术风格、目标平台、规模边界
- `architecture`：系统拆分、脚本结构、状态机、数据流
- `unity-init`：Unity 版本、模板、项目名、路径、场景、URP、输入系统
- `core-scripts`：直接生成、协助生成、台词风格、角色画像、BatchMode 校验
- `characters`：可玩角色、NPC、敌人原型、动画风格、配音需求
- `assets`：场景、UI、VFX、音效、资源说明
- `iteration`：迭代目标、数值目标、技能调整、试玩反馈
- `release`：发布平台、版本号、发布说明、QA 清单

### 需要核对的点
- 步骤信息是否一眼可见
- 结构化预览是否与表单形成左右对照
- 是否把流程画成工作台，而不是纯表单页
- Unity 状态块是否足够显性

## 5. AI 画布
### 当前代码基线
- [src/lib/components/canvas/AICanvas.svelte](/d:/app_project/AI_Game_Forge/src/lib/components/canvas/AICanvas.svelte)

### 视觉对照
- 左输入、右预览的双栏布局
- 生成区要突出主输入框和生成按钮
- 历史区应展示最近生成记录

### 交互对照
- 输入后点击生成，右侧内容变化
- 历史记录按时间倒序追加
- 返回仪表盘路径清晰

### 需要核对的点
- 是否保留“轻量生成 + 历史回看”的定位
- 是否避免把它画成大型编辑器
- 空状态、首次生成态、历史堆叠态是否都能解释清楚

## 对照原则
1. 先对照页面职责，再对照视觉样式。
2. 先对照状态变化，再对照按钮位置。
3. 先保证主用户流可走通，再做视觉强化。
4. 如果 Figma 设计和代码不一致，以当前代码流为校验基线，再决定改设计还是改代码。
