<script lang="ts">
  import type { UpdateInfo } from '../lib/types';
  import { installUpdate } from '../lib/tauri';

  export let update: UpdateInfo;

  let installing = false;
  let error = '';

  async function install() {
    installing = true;
    error = '';
    try {
      await installUpdate();
      // app reinicia automaticamente após install
    } catch (e: any) {
      error = e?.message ?? String(e);
      installing = false;
    }
  }
</script>

<div class="update-banner">
  <div class="update-icon">↑</div>
  <div class="update-body">
    <div class="update-title">
      nova versão disponível — <span class="update-version">{update.version}</span>
    </div>
    {#if update.body}
      <div class="update-notes">{update.body}</div>
    {/if}
    {#if error}
      <div class="update-error">{error}</div>
    {/if}
  </div>
  <button class="update-btn" on:click={install} disabled={installing}>
    {installing ? 'instalando...' : 'atualizar agora'}
  </button>
</div>

<style>
  .update-banner {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 200;
    display: flex;
    align-items: center;
    gap: 12px;
    background: rgba(0, 212, 126, 0.08);
    border-top: 1px solid rgba(0, 212, 126, 0.3);
    padding: 10px 16px;
    animation: slideUp 0.2s ease;
  }
  @keyframes slideUp {
    from {
      transform: translateY(100%);
    }
    to {
      transform: translateY(0);
    }
  }
  .update-icon {
    font-size: 16px;
    color: var(--ac);
    flex-shrink: 0;
  }
  .update-body {
    flex: 1;
    min-width: 0;
  }
  .update-title {
    font-size: var(--sm);
    color: var(--t1);
    font-weight: 500;
  }
  .update-version {
    color: var(--ac);
  }
  .update-notes {
    font-size: var(--xs);
    color: var(--t2);
    margin-top: 2px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .update-error {
    font-size: var(--xs);
    color: var(--s-error);
    margin-top: 2px;
  }
  .update-btn {
    background: var(--ac-d);
    border: 1px solid var(--ac);
    border-radius: 3px;
    color: var(--ac);
    font-size: var(--xs);
    padding: 5px 14px;
    flex-shrink: 0;
    letter-spacing: 0.04em;
    transition: background 0.15s;
  }
  .update-btn:hover:not(:disabled) {
    background: rgba(0, 212, 126, 0.18);
  }
  .update-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
