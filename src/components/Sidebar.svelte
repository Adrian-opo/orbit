<script lang="ts">
  import { sessions, selectedSessionId } from '../lib/stores/sessions';
  import type { Session } from '../lib/stores/sessions';
  import { theme } from '../lib/stores/preferences';
  import CreateSessionDialog from './CreateSessionDialog.svelte';

  let showCreateDialog = false;

  $: totalTokens = $sessions.reduce((sum, s) => {
    if (!s.tokens) return sum;
    return sum + s.tokens.input + s.tokens.output;
  }, 0);
  $: awaitingCount = $sessions.filter(s => s.status === 'waiting').length;
</script>

{#if showCreateDialog}
  <CreateSessionDialog
    on:created={() => showCreateDialog = false}
    on:cancel={() => showCreateDialog = false}
  />
{/if}

<aside class="sidebar">
  <div class="sidebar-header">
    <span class="title">Sessions</span>
    <div class="header-right">
      <button class="new-session-btn" on:click={() => showCreateDialog = true} title="New Session">+ New Session</button>
      <button class="theme-toggle" on:click={() => theme.toggle()} title="Toggle theme">
        {$theme === 'dark' ? '☀' : '☾'}
      </button>
      <span class="badge">{$sessions.length}</span>
    </div>
  </div>
  <div class="sidebar-content">
    {#if $sessions.length === 0}
      <p class="empty">No sessions detected</p>
    {:else}
      {#each $sessions as session (session.id)}
        <div
          class="session-item"
          class:selected={$selectedSessionId === session.id}
          on:click={() => selectedSessionId.set(session.id)}
          on:keydown={(e) => e.key === 'Enter' && selectedSessionId.set(session.id)}
          role="button"
          tabindex="0"
        >
          <div class="session-name">{session.projectName ?? session.cwd ?? 'Session'}</div>
          <div class="session-meta">
            <span class="status status-{session.status}">{session.status}</span>
            <span class="model">{session.model ?? '—'}</span>
          </div>
          {#if session.tokens}
            <div class="session-tokens">{Math.round((session.tokens.input + session.tokens.output) / 1000)}K tokens</div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
  <div class="sidebar-footer">
    {Math.round(totalTokens / 1000)}K tokens total
    {#if awaitingCount > 0}
      • {awaitingCount} awaiting
    {/if}
  </div>
</aside>

<style>
  .sidebar {
    width: 260px;
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }
  .sidebar-header {
    padding: 10px 12px;
    border-bottom: 1px solid var(--border);
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-weight: 600;
    font-size: 14px;
  }
  .header-right { display: flex; align-items: center; gap: 8px; }
  .new-session-btn {
    background: #3b82f6;
    border: none;
    border-radius: 4px;
    color: white;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    padding: 3px 8px;
    line-height: 1.4;
  }
  .new-session-btn:hover { background: #2563eb; }
  .theme-toggle {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 16px;
    cursor: pointer;
    padding: 2px;
    line-height: 1;
  }
  .theme-toggle:hover { color: var(--text-primary); }
  .badge {
    background: var(--green-dim);
    color: var(--green);
    padding: 1px 6px;
    border-radius: 8px;
    font-size: 11px;
  }
  .sidebar-content { flex: 1; overflow-y: auto; }
  .sidebar-footer {
    padding: 8px 12px;
    border-top: 1px solid var(--border);
    font-size: 11px;
    color: var(--text-dim);
  }
  .empty {
    padding: 20px 12px;
    color: var(--text-muted);
    font-size: 13px;
    text-align: center;
  }
  .session-item {
    padding: 10px 12px;
    border-bottom: 1px solid var(--border);
    cursor: pointer;
    user-select: none;
  }
  .session-item:hover { background: var(--bg-hover, rgba(255,255,255,0.05)); }
  .session-item.selected { background: var(--bg-selected, rgba(59,130,246,0.15)); }
  .session-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .session-meta {
    display: flex;
    gap: 6px;
    align-items: center;
    margin-top: 3px;
  }
  .status {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    padding: 1px 5px;
    border-radius: 4px;
  }
  .status-running { background: var(--green-dim); color: var(--green); }
  .status-working { background: var(--green-dim); color: var(--green); }
  .status-waiting { background: var(--yellow-dim, rgba(234,179,8,0.15)); color: var(--yellow, #eab308); }
  .status-initializing { background: var(--bg-muted, rgba(255,255,255,0.08)); color: var(--text-muted); }
  .status-completed { background: var(--bg-muted, rgba(255,255,255,0.08)); color: var(--text-dim); }
  .status-stopped { background: var(--bg-muted, rgba(255,255,255,0.08)); color: var(--text-dim); }
  .status-error { background: var(--red-dim, rgba(239,68,68,0.15)); color: var(--red, #ef4444); }
  .model {
    font-size: 11px;
    color: var(--text-muted);
  }
  .session-tokens {
    font-size: 10px;
    color: var(--text-dim);
    margin-top: 2px;
  }
</style>
