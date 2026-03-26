<script lang="ts">
  import type { SubagentInfo, AgentStatus } from '../lib/types';

  export let status: AgentStatus;
  export let subagents: SubagentInfo[];

  $: running = subagents.filter(a => a.status === 'running');
  $: done = subagents.filter(a => a.status === 'done');
  // Show running agents + last 2 done (most recent)
  $: visible = [...done.slice(-2), ...running];
  $: hiddenCount = subagents.length - visible.length;
</script>

{#if subagents.length > 0}
<div class="tree mono">
  <span class="main">● main <span class="status-{status}">({status})</span></span>
  {#if hiddenCount > 0}
    <span class="sep"> → </span>
    <span class="hidden-count">{hiddenCount} done</span>
  {/if}
  {#each visible as agent, i}
    <span class="sep"> → </span>
    <span>{i < visible.length - 1 ? '├' : '└'} {agent.agentType}
      <span class="agent-status" class:done={agent.status === 'done'} class:running={agent.status === 'running'}>
        {agent.status === 'done' ? '✓' : '●'}
      </span>
    </span>
  {/each}
</div>
{/if}

<style>
  .tree {
    padding: 5px 14px;
    border-bottom: 1px solid var(--border-subtle);
    background: var(--bg-subtle);
    font-size: 12px;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .main { color: var(--text-primary); }
  .sep { color: var(--text-dim); }
  .hidden-count { color: var(--text-dim); font-size: 11px; }
  .status-working { color: var(--green); }
  .status-input { color: var(--amber); }
  .status-idle { color: var(--text-muted); }
  .agent-status.done { color: var(--green); }
  .agent-status.running { color: var(--amber); }
</style>
