<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher<{ back: undefined }>();

  let prompt = '';
  let generatedContent = '等待生成...';
  let versions: string[] = [];

  function generate(): void {
    if (!prompt.trim()) {
      generatedContent = '请输入生成指令后再试。';
      return;
    }

    generatedContent = `已生成：${prompt.trim()}（预览）`;
    versions = [generatedContent, ...versions].slice(0, 8);
  }
</script>

<section class="canvas">
  <header>
    <h2>AI 协作画布</h2>
    <button type="button" on:click={() => dispatch('back')}>返回仪表盘</button>
  </header>

  <div class="layout">
    <article>
      <h3>输入区</h3>
      <textarea bind:value={prompt} rows="5" placeholder="描述角色、场景、动作和台词..."></textarea>
      <button type="button" class="primary" on:click={generate}>生成内容</button>
    </article>

    <article>
      <h3>预览区</h3>
      <div class="preview">{generatedContent}</div>
      <h4>版本历史</h4>
      <ul>
        {#if versions.length === 0}
          <li>暂无历史版本</li>
        {:else}
          {#each versions as item, idx (`${item}-${idx}`)}
            <li>{item}</li>
          {/each}
        {/if}
      </ul>
    </article>
  </div>
</section>

<style>
  .canvas {
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

  .layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    height: 100%;
  }

  article {
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 14px;
    background: rgba(15, 23, 42, 0.55);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  h3 {
    font-size: 16px;
  }

  h4 {
    font-size: 14px;
    color: #94a3b8;
  }

  textarea,
  .preview {
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: rgba(30, 41, 59, 0.8);
    color: #f8fafc;
    padding: 10px;
    min-height: 120px;
  }

  ul {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 220px;
    overflow: auto;
  }

  li {
    padding: 8px;
    border-radius: 8px;
    background: rgba(30, 41, 59, 0.7);
    color: #cbd5e1;
    font-size: 13px;
  }

  button {
    padding: 9px 12px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #cbd5e1;
    background: rgba(15, 23, 42, 0.7);
  }

  button.primary {
    align-self: flex-start;
    background: #6d28d9;
    color: white;
    border-color: transparent;
  }
</style>
