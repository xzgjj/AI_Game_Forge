<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher<{
    complete: undefined;
    back: undefined;
  }>();

  const steps = ['游戏类型', '美术风格', '叙事风格', '目标平台', '高级设置', '确认提交'];

  let stepIndex = 0;
  let gameType = 'RPG';
  let artStyle = 'Pixel';
  let narrativeTone = 'Light';
  let targetPlatform = 'PC';
  let advancedNotes = '';

  const maxStepIndex = steps.length - 1;

  function nextStep(): void {
    if (stepIndex < maxStepIndex) {
      stepIndex += 1;
      return;
    }

    dispatch('complete');
  }

  function previousStep(): void {
    if (stepIndex === 0) {
      dispatch('back');
      return;
    }

    stepIndex -= 1;
  }
</script>

<section class="wizard">
  <header>
    <h2>配置向导</h2>
    <span>Step {stepIndex + 1} / {steps.length}: {steps[stepIndex]}</span>
  </header>

  <div class="form-grid">
    <label>
      游戏类型
      <select bind:value={gameType}>
        <option value="RPG">RPG</option>
        <option value="Adventure">Adventure</option>
        <option value="Puzzle">Puzzle</option>
      </select>
    </label>

    <label>
      美术风格
      <select bind:value={artStyle}>
        <option value="Pixel">Pixel</option>
        <option value="HandDrawn">HandDrawn</option>
        <option value="Cartoon3D">Cartoon3D</option>
      </select>
    </label>

    <label>
      叙事风格
      <select bind:value={narrativeTone}>
        <option value="Light">Light</option>
        <option value="Epic">Epic</option>
        <option value="Mystery">Mystery</option>
      </select>
    </label>

    <label>
      目标平台
      <select bind:value={targetPlatform}>
        <option value="PC">PC</option>
        <option value="Mobile">Mobile</option>
        <option value="Console">Console</option>
      </select>
    </label>

    <label class="full">
      高级设置
      <textarea bind:value={advancedNotes} rows="3" placeholder="例如：战斗节奏偏快，剧情分支不少于3条"></textarea>
    </label>
  </div>

  <footer>
    <button type="button" on:click={previousStep}>上一步</button>
    <button type="button" class="primary" on:click={nextStep}>
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
    gap: 16px;
    padding: 20px;
    color: #f8fafc;
    height: 100%;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  h2 {
    font-size: 20px;
  }

  span {
    color: #94a3b8;
    font-size: 14px;
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

  select,
  textarea {
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: rgba(30, 41, 59, 0.8);
    color: #f8fafc;
    padding: 8px;
  }

  footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }

  button {
    padding: 10px 14px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #cbd5e1;
    background: rgba(15, 23, 42, 0.7);
  }

  button.primary {
    background: #6d28d9;
    color: #fff;
    border-color: transparent;
  }
</style>
