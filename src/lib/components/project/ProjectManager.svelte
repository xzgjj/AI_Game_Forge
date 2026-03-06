<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  interface ProjectItem {
    id: string;
    name: string;
    updatedAt: string;
    version: string;
  }

  const dispatch = createEventDispatcher<{ open: { projectId: string } }>();

  let projects: ProjectItem[] = [
    { id: 'p-001', name: '赛博朋克解谜', updatedAt: '2026-03-06 17:10', version: 'v12' },
    { id: 'p-002', name: '像素RPG Demo', updatedAt: '2026-03-05 20:40', version: 'v7' },
  ];

  function createProject(): void {
    const nextId = `p-${projects.length + 1}`;
    projects = [
      {
        id: nextId,
        name: `新项目 ${projects.length + 1}`,
        updatedAt: new Date().toLocaleString(),
        version: 'v1',
      },
      ...projects,
    ];
  }
</script>

<section class="panel">
  <header>
    <h3>项目管理器</h3>
    <button type="button" on:click={createProject}>新建项目</button>
  </header>

  <ul>
    {#each projects as project (project.id)}
      <li>
        <div>
          <strong>{project.name}</strong>
          <small>{project.updatedAt} · {project.version}</small>
        </div>
        <button type="button" on:click={() => dispatch('open', { projectId: project.id })}>打开</button>
      </li>
    {/each}
  </ul>
</section>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 12px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 16px;
    background: rgba(15, 23, 42, 0.5);
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  h3 {
    font-size: 16px;
    color: #f8fafc;
  }

  ul {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 10px;
    padding: 12px;
  }

  strong {
    display: block;
    color: #e2e8f0;
  }

  small {
    color: #94a3b8;
  }

  button {
    padding: 8px 12px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    color: #cbd5e1;
    background: rgba(30, 41, 59, 0.8);
  }
</style>
