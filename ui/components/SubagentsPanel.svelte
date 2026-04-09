<script lang="ts">
  import type { SubagentInfo, JournalEntry } from '../lib/types';
  import { getSubagentJournal } from '../lib/tauri';
  import Feed from './Feed.svelte';

  export let sessionId: string;
  export let subagents: SubagentInfo[];

  $: running = subagents.filter((a) => a.status === 'running');
  $: done = subagents.filter((a) => a.status === 'done');

  let modalAgent: SubagentInfo | null = null;
  let modalEntries: JournalEntry[] = [];
  let loading = false;

  async function openLog(agent: SubagentInfo) {
    modalAgent = agent;
    loading = true;
    modalEntries = [];
    try {
      modalEntries = await getSubagentJournal(sessionId, agent.id);
    } catch (e) {
      console.error('Failed to load subagent journal', e);
    }
    loading = false;
  }

  function closeModal() {
    modalAgent = null;
    modalEntries = [];
  }

  function handleBackdropClick(e: MouseEvent) {
    if ((e.target as HTMLElement).classList.contains('modal-backdrop')) {
      closeModal();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') closeModal();
  }

  function statusColor(status: string): string {
    return status === 'done' ? 'var(--s-working)' : 'var(--s-input)';
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="subagents">
  {#if subagents.length === 0}
    <p class="empty">No sub-agents spawned</p>
  {:else}
    {#if running.length > 0}
      <div class="section">
        <div class="section-header">
          <span class="dot running"></span> Running ({running.length})
        </div>
        {#each running as agent}
          <button class="agent-row running-row" onclick={() => openLog(agent)}>
            <div class="agent-top">
              <span class="agent-type">{agent.agentType}</span>
              <span class="badge badge-running">running</span>
            </div>
            {#if agent.description}
              <div class="agent-desc">{agent.description}</div>
            {/if}
            <div class="view-hint">Click to view log</div>
          </button>
        {/each}
      </div>
    {/if}

    {#if done.length > 0}
      <div class="section">
        <div class="section-header">
          <span class="dot done"></span> Completed ({done.length})
        </div>
        {#each done as agent}
          <button class="agent-row done-row" onclick={() => openLog(agent)}>
            <div class="agent-top">
              <span class="agent-type">{agent.agentType}</span>
              <span class="badge badge-done">done</span>
            </div>
            {#if agent.description}
              <div class="agent-desc">{agent.description}</div>
            {/if}
            <div class="view-hint">Click to view log</div>
          </button>
        {/each}
      </div>
    {/if}
  {/if}
</div>

{#if modalAgent}
  <div
    class="modal-backdrop"
    onclick={handleBackdropClick}
    onkeydown={(e) => e.key === 'Escape' && closeModal()}
    role="dialog"
    tabindex="-1"
  >
    <div class="modal">
      <div class="modal-header">
        <div class="header-left">
          <span class="dot-status" style="color:{statusColor(modalAgent.status)}">●</span>
          <span class="modal-type">{modalAgent.agentType}</span>
          <span
            class="modal-status"
            class:status-done={modalAgent.status === 'done'}
            class:status-running={modalAgent.status === 'running'}
          >
            {modalAgent.status}
          </span>
        </div>
        <div class="header-right">
          <button class="modal-close" onclick={closeModal}>×</button>
        </div>
      </div>
      {#if modalAgent.description}
        <div class="desc-strip">
          <span class="desc-icon">⊙</span>
          <span class="desc-text">{modalAgent.description}</span>
        </div>
      {/if}
      <div class="modal-body">
        {#if loading}
          <p class="loading">Loading log...</p>
        {:else if modalEntries.length === 0}
          <p class="loading">No log entries found</p>
        {:else}
          <Feed entries={modalEntries} status={modalAgent.status} />
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .subagents {
    padding: 8px;
  }
  .empty {
    color: var(--t2);
    font-size: 12px;
    text-align: center;
    padding: 20px;
  }
  .section {
    margin-bottom: 12px;
  }
  .section-header {
    font-size: 10px;
    font-weight: 600;
    color: var(--t2);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 4px;
    display: flex;
    align-items: center;
    gap: 5px;
  }
  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    display: inline-block;
    flex-shrink: 0;
  }
  .dot.running {
    background: var(--s-input);
  }
  .dot.done {
    background: var(--s-working);
  }

  .agent-row {
    display: block;
    width: 100%;
    text-align: left;
    padding: 7px 9px;
    border-radius: 5px;
    margin-bottom: 2px;
    background: var(--bg2);
    border: 1px solid transparent;
    cursor: pointer;
    font-family: var(--mono);
    color: inherit;
    transition: border-color 0.12s;
  }
  .agent-row:hover {
    border-color: var(--bd1);
    background: var(--bg3);
  }
  .agent-row.running-row {
    border-left: 2px solid var(--s-input);
  }
  .agent-row.done-row {
    border-left: 2px solid var(--s-working);
  }

  .agent-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .agent-type {
    font-size: 12px;
    font-weight: 500;
    color: var(--t0);
  }
  .badge {
    font-size: 10px;
    padding: 1px 6px;
    border-radius: 3px;
    font-weight: 500;
  }
  .badge-running {
    background: rgba(232, 160, 48, 0.12);
    color: var(--s-input);
  }
  .badge-done {
    background: rgba(0, 212, 126, 0.1);
    color: var(--s-working);
  }

  .agent-desc {
    font-size: 11px;
    color: var(--t1);
    margin-top: 2px;
    line-height: 1.4;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .view-hint {
    font-size: 10px;
    color: var(--t3);
    margin-top: 3px;
  }

  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  .modal {
    width: 70vw;
    max-width: 900px;
    height: 75vh;
    background: var(--bg);
    border: 1px solid var(--bd1);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 14px;
    border-bottom: 1px solid var(--bd);
    flex-shrink: 0;
    background: var(--bg1);
  }
  .header-left {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
    flex: 1;
    overflow: hidden;
  }
  .dot-status {
    font-size: 8px;
    line-height: 1;
    flex-shrink: 0;
  }
  .modal-type {
    font-size: 12px;
    font-weight: 500;
    color: var(--t0);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .modal-status {
    font-size: 10px;
    color: var(--t2);
    letter-spacing: 0.04em;
    flex-shrink: 0;
  }
  .modal-status.status-done {
    color: var(--s-working);
  }
  .modal-status.status-running {
    color: var(--s-input);
  }

  .header-right {
    flex-shrink: 0;
    padding-left: 12px;
  }
  .modal-close {
    background: var(--bg3);
    border: 1px solid var(--bd1);
    color: var(--t2);
    width: 18px;
    height: 18px;
    border-radius: 3px;
    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-family: var(--mono);
    line-height: 1;
    transition:
      border-color 0.15s,
      color 0.15s;
  }
  .modal-close:hover {
    border-color: var(--s-error);
    color: var(--s-error);
  }

  .desc-strip {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 2px 14px;
    border-bottom: 1px solid var(--bd);
    background: var(--bg1);
    flex-shrink: 0;
    min-width: 0;
    overflow: hidden;
  }
  .desc-icon {
    font-size: 10px;
    color: var(--t3);
    flex-shrink: 0;
  }
  .desc-text {
    font-size: 10px;
    color: var(--t3);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .modal-body {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .loading {
    color: var(--t2);
    font-size: 12px;
    text-align: center;
    padding: 30px;
  }
</style>
