<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { WizardState, WizardStepId, WizardStepMeta } from '$lib/types/wizard.types';
  import type { UnityBatchValidateReport, UnityBridgeResult, UnityValidationReport } from '$lib/types/unity.types';
  import {
    unityBatchValidate,
    unityInitProject,
    unityInjectUpm,
    unityValidateProject,
  } from '$lib/services/unity.service';
  import { loadLatestWizardState, loadWizardStateByProject, saveWizardState } from '$lib/services/wizard.service';

  const dispatch = createEventDispatcher<{
    complete: WizardState;
    back: undefined;
  }>();

  const steps: WizardStepMeta[] = [
    { id: 'design', title: '游戏设计', subtitle: '明确玩法与方向' },
    { id: 'architecture', title: 'AI 设计架构', subtitle: '系统拆分与脚本结构' },
    { id: 'unity-init', title: 'Unity 项目初始化', subtitle: '模板与工程配置' },
    { id: 'core-scripts', title: 'AI 生成核心脚本', subtitle: '控制/AI/碰撞/状态机' },
    { id: 'characters', title: 'AI 生成人物', subtitle: '角色与 NPC 配置' },
    { id: 'assets', title: 'AI 生成资源', subtitle: '场景/UI/音效/VFX' },
    { id: 'iteration', title: '玩法迭代', subtitle: '平衡与技能调整' },
    { id: 'release', title: '打包发布', subtitle: '版本与发布检查' },
  ];

  const maxStepIndex = steps.length - 1;

  let stepIndex = 0;
  let validationError = '';
  let bridgeBusy = false;
  let bridgeMessage = '';
  let bridgeError = '';
  let initResult: UnityBridgeResult | null = null;
  let upmResult: UnityBridgeResult | null = null;
  let validationReport: UnityValidationReport | null = null;
  let batchReport: UnityBatchValidateReport | null = null;
  let persistMessage = '';
  let loadMessage = '';
  let loadError = '';
  let loadBusy = false;

  // Step 1
  let gameTitle = '';
  let genre = 'RPG';
  let coreLoop = '';
  let narrativeTone = 'Light';
  let artStyle = 'Pixel';
  let targetPlatforms: string[] = ['PC'];
  let scopeNotes = '';

  // Step 2
  let systemModules = '';
  let scriptStructure = '';
  let stateMachineNotes = '';
  let dataFlowNotes = '';

  // Step 3
  let unityVersion = 'Unity 6 LTS';
  let templatePreset = '3D URP';
  let projectName = '';
  let projectPath = '';
  let sceneName = 'Main';
  let useURP = true;
  let useInputSystem = true;
  let unityEditorPath = '';
  let enableBatchValidation = false;

  // Step 4
  let directGenerate: string[] = ['玩家控制', '敌人 AI', '碰撞检测', '状态机'];
  let assistedGenerate: string[] = ['核心循环', '数值设计', '技能系统'];
  let dialogueStyle = '短句、情绪化';
  let portraitStyle = '统一风格立绘';
  let scriptNotes = '';

  // Step 5
  let playableCount = 1;
  let enemyArchetypes = '';
  let npcCount = 3;
  let animationStyle = '关键帧 + 少量混合';
  let voiceRequirement = '仅文本';

  // Step 6
  let environmentStyle = '低饱和度幻想';
  let uiTheme = '简洁科技';
  let vfxStyle = '粒子轻量';
  let audioStyle = '环境氛围 + 关键音效';
  let assetNotes = '';

  // Step 7
  let iterationGoals = '';
  let balanceTargets = '';
  let skillAdjustments = '';
  let playtestNotes = '';

  // Step 8
  let buildTargets: string[] = ['Windows'];
  let versionTag = 'v0.1.0';
  let releaseNotes = '';
  let qaChecklist = '';

  let currentStep: WizardStepMeta = steps[0];
  let wizardState: WizardState = buildWizardState();

  $: currentStep = steps[stepIndex];
  $: wizardState = buildWizardState();

  function buildWizardState(): WizardState {
    return {
      stepIndex,
      steps,
      design: {
        gameTitle,
        genre,
        coreLoop,
        narrativeTone,
        artStyle,
        targetPlatforms,
        scopeNotes,
      },
      architecture: {
        systemModules,
        scriptStructure,
        stateMachineNotes,
        dataFlowNotes,
      },
      unityInit: {
        unityVersion,
        templatePreset,
        projectName,
        projectPath,
        sceneName,
        useURP,
        useInputSystem,
        unityEditorPath,
        enableBatchValidation,
      },
      coreScripts: {
        directGenerate,
        assistedGenerate,
        dialogueStyle,
        portraitStyle,
        scriptNotes,
      },
      characters: {
        playableCount,
        enemyArchetypes,
        npcCount,
        animationStyle,
        voiceRequirement,
      },
      assets: {
        environmentStyle,
        uiTheme,
        vfxStyle,
        audioStyle,
        assetNotes,
      },
      iteration: {
        iterationGoals,
        balanceTargets,
        skillAdjustments,
        playtestNotes,
      },
      release: {
        buildTargets,
        versionTag,
        releaseNotes,
        qaChecklist,
      },
      updatedAt: new Date().toISOString(),
    };
  }

  function toggleListItem(list: string[], value: string): string[] {
    if (list.includes(value)) {
      return list.filter((item) => item !== value);
    }
    return [...list, value];
  }

  function validateStep(id: WizardStepId): string | null {
    switch (id) {
      case 'design':
        if (!gameTitle.trim()) return '请填写游戏名称。';
        if (!coreLoop.trim()) return '请填写核心循环（玩法核心描述）。';
        return null;
      case 'architecture':
        if (!systemModules.trim()) return '请填写系统拆分清单。';
        if (!scriptStructure.trim()) return '请填写脚本结构说明。';
        return null;
      case 'unity-init':
        if (!projectName.trim()) return '请填写 Unity 项目名称。';
        if (!projectPath.trim()) return '请填写 Unity 项目路径。';
        return null;
      case 'core-scripts':
        if (directGenerate.length === 0) return '至少选择一项核心脚本生成内容。';
        return null;
      case 'characters':
        if (!enemyArchetypes.trim()) return '请填写敌人原型描述。';
        return null;
      case 'assets':
        if (!environmentStyle.trim()) return '请填写场景风格。';
        return null;
      case 'iteration':
        if (!iterationGoals.trim()) return '请填写本次迭代目标。';
        return null;
      case 'release':
        if (buildTargets.length === 0) return '请选择至少一个发布平台。';
        return null;
      default:
        return null;
    }
  }

  async function runUnityInit(): Promise<boolean> {
    bridgeBusy = true;
    bridgeMessage = '正在初始化 Unity 项目...';
    bridgeError = '';

    try {
      initResult = await unityInitProject({
        projectRoot: projectPath,
        unityVersion,
        templatePreset,
        sceneName,
        useUrp: useURP,
        useInputSystem,
      });
      await saveWizardState(projectPath, wizardState);
      persistMessage = '已保存当前 WizardState 到 SQLite。';
      bridgeMessage = 'Unity 初始化完成';
      return true;
    } catch (error) {
      bridgeError = error instanceof Error ? error.message : 'Unity 初始化失败';
      return false;
    } finally {
      bridgeBusy = false;
    }
  }

  async function runUnityUpmAndValidate(): Promise<boolean> {
    bridgeBusy = true;
    bridgeMessage = '正在注入 UPM 包并校验...';
    bridgeError = '';

    try {
      upmResult = await unityInjectUpm({
        projectRoot: projectPath,
      });
      validationReport = await unityValidateProject({
        projectRoot: projectPath,
        requirePackage: true,
      });

      if (!validationReport.ok) {
        bridgeError = 'Unity 校验未通过';
        return false;
      }

      if (enableBatchValidation) {
        if (!unityEditorPath.trim()) {
          bridgeError = '请填写 Unity Editor 路径以执行 BatchMode 校验';
          return false;
        }
        batchReport = await unityBatchValidate({
          projectRoot: projectPath,
          editorPath: unityEditorPath,
        });
        if (!batchReport.ok) {
          bridgeError = 'Unity BatchMode 编译校验失败';
          return false;
        }
      }

      await saveWizardState(projectPath, wizardState);
      persistMessage = '已保存当前 WizardState 到 SQLite。';

      bridgeMessage = 'UPM 注入与校验完成';
      return true;
    } catch (error) {
      bridgeError = error instanceof Error ? error.message : 'UPM 注入失败';
      return false;
    } finally {
      bridgeBusy = false;
    }
  }

  function nextStep(): void {
    const error = validateStep(currentStep.id);
    if (error) {
      validationError = error;
      return;
    }

    validationError = '';

    if (currentStep.id === 'unity-init') {
      void (async () => {
        const ok = await runUnityInit();
        if (!ok) {
          validationError = bridgeError || 'Unity 初始化失败';
          return;
        }
        if (stepIndex < maxStepIndex) {
          stepIndex += 1;
          return;
        }
        dispatch('complete', wizardState);
      })();
      return;
    }

    if (currentStep.id === 'core-scripts') {
      void (async () => {
        const ok = await runUnityUpmAndValidate();
        if (!ok) {
          validationError = bridgeError || 'Unity 校验未通过';
          return;
        }
        if (stepIndex < maxStepIndex) {
          stepIndex += 1;
          return;
        }
        dispatch('complete', wizardState);
      })();
      return;
    }

    if (stepIndex < maxStepIndex) {
      stepIndex += 1;
      return;
    }

    dispatch('complete', wizardState);
  }

  function previousStep(): void {
    validationError = '';

    if (stepIndex === 0) {
      dispatch('back');
      return;
    }

    stepIndex -= 1;
  }

  function applyWizardState(state: WizardState): void {
    stepIndex = Math.min(state.stepIndex ?? 0, maxStepIndex);

    gameTitle = state.design?.gameTitle ?? '';
    genre = state.design?.genre ?? 'RPG';
    coreLoop = state.design?.coreLoop ?? '';
    narrativeTone = state.design?.narrativeTone ?? 'Light';
    artStyle = state.design?.artStyle ?? 'Pixel';
    targetPlatforms = state.design?.targetPlatforms ?? ['PC'];
    scopeNotes = state.design?.scopeNotes ?? '';

    systemModules = state.architecture?.systemModules ?? '';
    scriptStructure = state.architecture?.scriptStructure ?? '';
    stateMachineNotes = state.architecture?.stateMachineNotes ?? '';
    dataFlowNotes = state.architecture?.dataFlowNotes ?? '';

    unityVersion = state.unityInit?.unityVersion ?? 'Unity 6 LTS';
    templatePreset = state.unityInit?.templatePreset ?? '3D URP';
    projectName = state.unityInit?.projectName ?? '';
    projectPath = state.unityInit?.projectPath ?? '';
    sceneName = state.unityInit?.sceneName ?? 'Main';
    useURP = state.unityInit?.useURP ?? true;
    useInputSystem = state.unityInit?.useInputSystem ?? true;
    unityEditorPath = state.unityInit?.unityEditorPath ?? '';
    enableBatchValidation = state.unityInit?.enableBatchValidation ?? false;

    directGenerate = state.coreScripts?.directGenerate ?? ['玩家控制', '敌人 AI', '碰撞检测', '状态机'];
    assistedGenerate = state.coreScripts?.assistedGenerate ?? ['核心循环', '数值设计', '技能系统'];
    dialogueStyle = state.coreScripts?.dialogueStyle ?? '短句、情绪化';
    portraitStyle = state.coreScripts?.portraitStyle ?? '统一风格立绘';
    scriptNotes = state.coreScripts?.scriptNotes ?? '';

    playableCount = state.characters?.playableCount ?? 1;
    enemyArchetypes = state.characters?.enemyArchetypes ?? '';
    npcCount = state.characters?.npcCount ?? 3;
    animationStyle = state.characters?.animationStyle ?? '关键帧 + 少量混合';
    voiceRequirement = state.characters?.voiceRequirement ?? '仅文本';

    environmentStyle = state.assets?.environmentStyle ?? '低饱和度幻想';
    uiTheme = state.assets?.uiTheme ?? '简洁科技';
    vfxStyle = state.assets?.vfxStyle ?? '粒子轻量';
    audioStyle = state.assets?.audioStyle ?? '环境氛围 + 关键音效';
    assetNotes = state.assets?.assetNotes ?? '';

    iterationGoals = state.iteration?.iterationGoals ?? '';
    balanceTargets = state.iteration?.balanceTargets ?? '';
    skillAdjustments = state.iteration?.skillAdjustments ?? '';
    playtestNotes = state.iteration?.playtestNotes ?? '';

    buildTargets = state.release?.buildTargets ?? ['Windows'];
    versionTag = state.release?.versionTag ?? 'v0.1.0';
    releaseNotes = state.release?.releaseNotes ?? '';
    qaChecklist = state.release?.qaChecklist ?? '';
  }

  async function loadLatestState(): Promise<void> {
    loadBusy = true;
    loadError = '';
    loadMessage = '';

    try {
      const record = await loadLatestWizardState();
      if (!record) {
        loadMessage = '没有找到历史 WizardState 记录。';
        return;
      }

      applyWizardState(record.wizardState);
      loadMessage = `已加载 ${record.updatedAt} 的 WizardState。`;
    } catch (error) {
      loadError = error instanceof Error ? error.message : '加载失败';
    } finally {
      loadBusy = false;
    }
  }

  async function loadByProjectPath(): Promise<void> {
    loadBusy = true;
    loadError = '';
    loadMessage = '';

    if (!projectPath.trim()) {
      loadError = '请先填写项目路径再加载。';
      loadBusy = false;
      return;
    }

    try {
      const record = await loadWizardStateByProject(projectPath.trim());
      if (!record) {
        loadMessage = '未找到该路径对应的 WizardState。';
        return;
      }

      applyWizardState(record.wizardState);
      loadMessage = `已加载 ${record.updatedAt} 的 WizardState（按路径）。`;
    } catch (error) {
      loadError = error instanceof Error ? error.message : '加载失败';
    } finally {
      loadBusy = false;
    }
  }
</script>

<section class="wizard">
  <header class="wizard-header">
    <div>
      <h2>配置向导</h2>
      <p>Step {stepIndex + 1} / {steps.length} · {currentStep.title}</p>
    </div>
    <div class="header-actions">
      <button type="button" on:click={loadLatestState} disabled={loadBusy}>
        {loadBusy ? '加载中...' : '加载上次 WizardState'}
      </button>
      <button type="button" on:click={loadByProjectPath} disabled={loadBusy}>
        按路径加载
      </button>
    </div>
    <div class="progress">
      {#each steps as step, index}
        <button
          type="button"
          class="step-chip {index === stepIndex ? 'active' : ''} {index < stepIndex ? 'done' : ''}"
          on:click={() => (stepIndex = index)}
        >
          <span class="chip-index">{index + 1}</span>
          <span class="chip-title">{step.title}</span>
        </button>
      {/each}
    </div>
  </header>

  <div class="wizard-body">
    <div class="form-panel">
      <div class="step-title">
        <h3>{currentStep.title}</h3>
        <p>{currentStep.subtitle}</p>
      </div>

      {#if validationError}
        <div class="alert">{validationError}</div>
      {/if}
      {#if bridgeError}
        <div class="alert">{bridgeError}</div>
      {/if}
      {#if loadError}
        <div class="alert">{loadError}</div>
      {/if}
      {#if loadMessage}
        <div class="info">{loadMessage}</div>
      {/if}

      {#if currentStep.id === 'design'}
        <div class="form-grid">
          <label>
            游戏名称
            <input bind:value={gameTitle} placeholder="例如：迷雾回声" />
          </label>
          <label>
            游戏类型
            <select bind:value={genre}>
              <option value="RPG">RPG</option>
              <option value="Adventure">Adventure</option>
              <option value="Puzzle">Puzzle</option>
              <option value="Strategy">Strategy</option>
              <option value="Other">Other</option>
            </select>
          </label>
          <label class="full">
            核心循环
            <textarea bind:value={coreLoop} rows="3" placeholder="探索 -> 战斗 -> 收集 -> 强化 -> 探索" />
          </label>
          <label>
            叙事风格
            <select bind:value={narrativeTone}>
              <option value="Light">Light</option>
              <option value="Epic">Epic</option>
              <option value="Mystery">Mystery</option>
              <option value="SciFi">SciFi</option>
              <option value="Other">Other</option>
            </select>
          </label>
          <label>
            美术风格
            <select bind:value={artStyle}>
              <option value="Pixel">Pixel</option>
              <option value="HandDrawn">HandDrawn</option>
              <option value="Cartoon3D">Cartoon3D</option>
              <option value="Realistic">Realistic</option>
              <option value="Other">Other</option>
            </select>
          </label>
          <label class="full">
            目标平台
            <div class="pill-group">
              {#each ['PC', 'Mobile', 'Console', 'Web'] as platform}
                <button
                  type="button"
                  class:active={targetPlatforms.includes(platform)}
                  on:click={() => (targetPlatforms = toggleListItem(targetPlatforms, platform))}
                >
                  {platform}
                </button>
              {/each}
            </div>
          </label>
          <label class="full">
            规模与边界
            <textarea bind:value={scopeNotes} rows="3" placeholder="例如：可玩时长 20 分钟，单场景" />
          </label>
        </div>
      {:else if currentStep.id === 'architecture'}
        <div class="form-grid">
          <label class="full">
            系统拆分清单
            <textarea bind:value={systemModules} rows="3" placeholder="角色控制、敌人AI、碰撞、状态机、关卡系统..." />
          </label>
          <label class="full">
            Script 结构设计
            <textarea bind:value={scriptStructure} rows="3" placeholder="例如：Core/AI/Combat/UI/Systems" />
          </label>
          <label class="full">
            状态机说明
            <textarea bind:value={stateMachineNotes} rows="3" placeholder="角色/敌人状态切换规则" />
          </label>
          <label class="full">
            数据流说明
            <textarea bind:value={dataFlowNotes} rows="3" placeholder="输入 -> 状态 -> 事件 -> UI" />
          </label>
        </div>
      {:else if currentStep.id === 'unity-init'}
        <div class="form-grid">
          <label>
            Unity 版本
            <input bind:value={unityVersion} placeholder="Unity 6 LTS" />
          </label>
          <label>
            模板预设
            <select bind:value={templatePreset}>
              <option value="3D URP">3D URP</option>
              <option value="2D URP">2D URP</option>
              <option value="3D Built-in">3D Built-in</option>
              <option value="2D Built-in">2D Built-in</option>
            </select>
          </label>
          <label>
            项目名称
            <input bind:value={projectName} placeholder="AI_Game_Demo" />
          </label>
          <label>
            项目路径
            <input bind:value={projectPath} placeholder="D:\\UnityProjects\\AI_Game_Demo" />
          </label>
          <label>
            初始场景名
            <input bind:value={sceneName} placeholder="Main" />
          </label>
          <label>
            渲染管线
            <div class="toggle-row">
              <button type="button" class:active={useURP} on:click={() => (useURP = !useURP)}>
                {useURP ? 'URP' : 'Built-in'}
              </button>
              <span>切换渲染管线</span>
            </div>
          </label>
          <label>
            新输入系统
            <div class="toggle-row">
              <button type="button" class:active={useInputSystem} on:click={() => (useInputSystem = !useInputSystem)}>
                {useInputSystem ? '启用' : '关闭'}
              </button>
              <span>Input System</span>
            </div>
          </label>
        </div>
        <div class="bridge-panel">
          <h4>Unity 初始化状态</h4>
          {#if bridgeBusy}
            <p>{bridgeMessage}</p>
          {:else if initResult}
            <p>已创建 {initResult.created.length} 项</p>
            {#if initResult.warnings.length > 0}
              <p>警告：{initResult.warnings.join('；')}</p>
            {/if}
            {#if persistMessage}
              <p>{persistMessage}</p>
            {/if}
          {:else}
            <p>点击下一步将执行 Unity 项目初始化。</p>
          {/if}
        </div>
      {:else if currentStep.id === 'core-scripts'}
        <div class="form-grid">
          <label class="full">
            AI 直接生成
            <div class="pill-group">
              {#each ['玩家控制', '敌人 AI', '碰撞检测', '状态机'] as item}
                <button
                  type="button"
                  class:active={directGenerate.includes(item)}
                  on:click={() => (directGenerate = toggleListItem(directGenerate, item))}
                >
                  {item}
                </button>
              {/each}
            </div>
          </label>
          <label class="full">
            AI 协助生成
            <div class="pill-group">
              {#each ['核心循环', '数值设计', '技能系统'] as item}
                <button
                  type="button"
                  class:active={assistedGenerate.includes(item)}
                  on:click={() => (assistedGenerate = toggleListItem(assistedGenerate, item))}
                >
                  {item}
                </button>
              {/each}
            </div>
          </label>
          <label>
            台词风格
            <input bind:value={dialogueStyle} />
          </label>
          <label>
            角色画像风格
            <input bind:value={portraitStyle} />
          </label>
          <label class="full">
            脚本补充说明
            <textarea bind:value={scriptNotes} rows="3" placeholder="输入限制、代码规范、接口习惯" />
          </label>
          <label class="full">
            Unity Editor 路径（BatchMode 校验）
            <input bind:value={unityEditorPath} placeholder="例如：C:\\Program Files\\Unity\\Hub\\Editor\\2022.3.20f1\\Editor\\Unity.exe" />
          </label>
          <label class="full">
            BatchMode 编译校验
            <div class="toggle-row">
              <button type="button" class:active={enableBatchValidation} on:click={() => (enableBatchValidation = !enableBatchValidation)}>
                {enableBatchValidation ? '启用' : '关闭'}
              </button>
              <span>执行 Unity BatchMode 进行编译校验</span>
            </div>
          </label>
        </div>
        <div class="bridge-panel">
          <h4>UPM 注入与校验</h4>
          {#if bridgeBusy}
            <p>{bridgeMessage}</p>
          {:else if validationReport}
            <p>{validationReport.ok ? '校验通过' : '校验失败'}</p>
            {#if validationReport.errors.length > 0}
              <p>错误：{validationReport.errors.join('；')}</p>
            {/if}
            {#if validationReport.warnings.length > 0}
              <p>警告：{validationReport.warnings.join('；')}</p>
            {/if}
            {#if batchReport}
              <p>{batchReport.ok ? 'BatchMode 校验通过' : 'BatchMode 校验失败'}</p>
              {#if batchReport.log_tail}
                <p>日志片段：{batchReport.log_tail}</p>
              {/if}
            {/if}
            {#if persistMessage}
              <p>{persistMessage}</p>
            {/if}
          {:else if upmResult}
            <p>已创建 {upmResult.created.length} 项，等待校验。</p>
          {:else}
            <p>点击下一步将注入 UPM 并执行校验。</p>
          {/if}
        </div>
      {:else if currentStep.id === 'characters'}
        <div class="form-grid">
          <label>
            可玩角色数
            <input type="number" min="1" bind:value={playableCount} />
          </label>
          <label>
            NPC 数量
            <input type="number" min="0" bind:value={npcCount} />
          </label>
          <label class="full">
            敌人原型描述
            <textarea bind:value={enemyArchetypes} rows="3" placeholder="巡逻兵、远程法师、重装怪等" />
          </label>
          <label>
            动画风格
            <input bind:value={animationStyle} />
          </label>
          <label>
            配音需求
            <select bind:value={voiceRequirement}>
              <option value="仅文本">仅文本</option>
              <option value="关键角色配音">关键角色配音</option>
              <option value="全量配音">全量配音</option>
            </select>
          </label>
        </div>
      {:else if currentStep.id === 'assets'}
        <div class="form-grid">
          <label>
            场景风格
            <input bind:value={environmentStyle} />
          </label>
          <label>
            UI 主题
            <input bind:value={uiTheme} />
          </label>
          <label>
            VFX 风格
            <input bind:value={vfxStyle} />
          </label>
          <label>
            音效风格
            <input bind:value={audioStyle} />
          </label>
          <label class="full">
            资源补充说明
            <textarea bind:value={assetNotes} rows="3" placeholder="资源命名、分辨率、色彩约束" />
          </label>
        </div>
      {:else if currentStep.id === 'iteration'}
        <div class="form-grid">
          <label class="full">
            本次迭代目标
            <textarea bind:value={iterationGoals} rows="3" placeholder="例如：优化战斗节奏、提高掉落率" />
          </label>
          <label class="full">
            数值平衡目标
            <textarea bind:value={balanceTargets} rows="3" placeholder="例如：Boss 战 3 分钟内结束" />
          </label>
          <label class="full">
            技能调整
            <textarea bind:value={skillAdjustments} rows="3" placeholder="技能冷却、伤害系数调整" />
          </label>
          <label class="full">
            试玩反馈
            <textarea bind:value={playtestNotes} rows="3" placeholder="记录玩家反馈" />
          </label>
        </div>
      {:else if currentStep.id === 'release'}
        <div class="form-grid">
          <label class="full">
            发布平台
            <div class="pill-group">
              {#each ['Windows', 'macOS', 'Linux', 'WebGL'] as platform}
                <button
                  type="button"
                  class:active={buildTargets.includes(platform)}
                  on:click={() => (buildTargets = toggleListItem(buildTargets, platform))}
                >
                  {platform}
                </button>
              {/each}
            </div>
          </label>
          <label>
            版本号
            <input bind:value={versionTag} />
          </label>
          <label class="full">
            发布说明
            <textarea bind:value={releaseNotes} rows="3" placeholder="版本亮点、已知问题" />
          </label>
          <label class="full">
            QA 检查清单
            <textarea bind:value={qaChecklist} rows="3" placeholder="启动、加载、战斗、保存、崩溃监控" />
          </label>
        </div>
      {/if}
    </div>

    <aside class="summary-panel">
      <div class="summary-header">
        <h4>结构化产物预览</h4>
        <span>{new Date(wizardState.updatedAt).toLocaleString()}</span>
      </div>
      <pre>{JSON.stringify(wizardState, null, 2)}</pre>
    </aside>
  </div>

  <footer class="wizard-footer">
    <button type="button" class="ghost" on:click={previousStep} disabled={bridgeBusy}>上一步</button>
    <button type="button" class="primary" on:click={nextStep} disabled={bridgeBusy}>
      {#if stepIndex === maxStepIndex}
        完成并进入画布
      {:else}
        下一步
      {/if}
    </button>
  </footer>
</section>

<style>
  .wizard {
    display: flex;
    flex-direction: column;
    gap: 18px;
    padding: 24px;
    color: #e2e8f0;
    height: 100%;
    background: radial-gradient(circle at top left, rgba(88, 105, 255, 0.25), transparent 50%),
      radial-gradient(circle at top right, rgba(16, 185, 129, 0.2), transparent 45%);
  }

  .wizard-header {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .header-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .header-actions button {
    padding: 8px 12px;
    border-radius: 10px;
    border: 1px solid rgba(148, 163, 184, 0.2);
    background: rgba(15, 23, 42, 0.7);
    color: #cbd5e1;
  }

  h2 {
    font-size: 22px;
    margin: 0;
  }

  p {
    margin: 6px 0 0;
    color: #94a3b8;
    font-size: 14px;
  }

  .progress {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 8px;
  }

  .step-chip {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 10px 12px;
    border-radius: 12px;
    border: 1px solid rgba(148, 163, 184, 0.2);
    background: rgba(15, 23, 42, 0.65);
    color: #cbd5e1;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .step-chip.active {
    border-color: rgba(99, 102, 241, 0.7);
    box-shadow: 0 0 0 1px rgba(99, 102, 241, 0.6) inset;
    color: #f8fafc;
  }

  .step-chip.done {
    border-color: rgba(34, 197, 94, 0.5);
  }

  .chip-index {
    font-size: 12px;
    color: #94a3b8;
  }

  .chip-title {
    font-size: 13px;
  }

  .wizard-body {
    display: grid;
    grid-template-columns: minmax(0, 2fr) minmax(0, 1fr);
    gap: 16px;
    flex: 1;
    overflow: hidden;
  }

  .form-panel,
  .summary-panel {
    background: rgba(15, 23, 42, 0.75);
    border-radius: 16px;
    padding: 18px;
    border: 1px solid rgba(148, 163, 184, 0.15);
  }

  .form-panel {
    display: flex;
    flex-direction: column;
    gap: 16px;
    overflow: auto;
  }

  .summary-panel {
    display: flex;
    flex-direction: column;
    gap: 12px;
    overflow: auto;
  }

  .summary-panel pre {
    white-space: pre-wrap;
    word-break: break-word;
    font-size: 12px;
    color: #cbd5e1;
  }

  .summary-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .summary-header span {
    color: #94a3b8;
    font-size: 12px;
  }

  .step-title h3 {
    margin: 0;
    font-size: 18px;
  }

  .step-title p {
    margin: 6px 0 0;
  }

  .alert {
    padding: 10px 12px;
    border-radius: 10px;
    background: rgba(248, 113, 113, 0.15);
    color: #fecaca;
    border: 1px solid rgba(248, 113, 113, 0.4);
  }

  .info {
    padding: 10px 12px;
    border-radius: 10px;
    background: rgba(56, 189, 248, 0.12);
    color: #bae6fd;
    border: 1px solid rgba(56, 189, 248, 0.3);
  }

  .bridge-panel {
    margin-top: 12px;
    padding: 12px;
    border-radius: 12px;
    background: rgba(15, 23, 42, 0.6);
    border: 1px solid rgba(148, 163, 184, 0.15);
  }

  .bridge-panel h4 {
    margin: 0 0 6px;
    font-size: 14px;
    color: #e2e8f0;
  }

  .bridge-panel p {
    margin: 6px 0 0;
    color: #94a3b8;
    font-size: 13px;
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 6px;
    color: #cbd5e1;
    font-size: 14px;
  }

  label.full {
    grid-column: 1 / -1;
  }

  input,
  select,
  textarea {
    border-radius: 10px;
    border: 1px solid rgba(148, 163, 184, 0.2);
    background: rgba(30, 41, 59, 0.9);
    color: #f8fafc;
    padding: 10px 12px;
    font-size: 14px;
  }

  .pill-group {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .pill-group button {
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.2);
    background: rgba(30, 41, 59, 0.8);
    color: #cbd5e1;
    padding: 6px 12px;
    font-size: 12px;
  }

  .pill-group button.active {
    border-color: rgba(99, 102, 241, 0.8);
    color: #f8fafc;
    box-shadow: 0 0 0 1px rgba(99, 102, 241, 0.5) inset;
  }

  .toggle-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toggle-row button {
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.2);
    background: rgba(30, 41, 59, 0.8);
    color: #cbd5e1;
    padding: 6px 12px;
    font-size: 12px;
  }

  .toggle-row button.active {
    border-color: rgba(14, 165, 233, 0.8);
    color: #f8fafc;
  }

  .wizard-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }

  .wizard-footer button {
    padding: 10px 16px;
    border-radius: 10px;
    border: 1px solid rgba(148, 163, 184, 0.2);
    color: #cbd5e1;
    background: rgba(15, 23, 42, 0.7);
  }

  .wizard-footer button.primary {
    background: linear-gradient(135deg, #4f46e5, #22c55e);
    color: #fff;
    border-color: transparent;
  }

  .wizard-footer button.ghost {
    background: transparent;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  @media (max-width: 900px) {
    .wizard-body {
      grid-template-columns: 1fr;
    }
  }
</style>
