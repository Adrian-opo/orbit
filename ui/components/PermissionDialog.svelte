<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Modal from './Modal.svelte';

  export let sessionId: number;
  export let toolName: string;
  export let description: string;

  const dispatch = createEventDispatcher<{
    allow: { sessionId: number };
    deny: { sessionId: number };
  }>();
</script>

<Modal title="Permission Request" on:close={() => dispatch('deny', { sessionId })}>
  <div class="perm-body">
    <div class="perm-tool">{toolName}</div>
    <div class="perm-desc">{description}</div>
  </div>
  <div class="perm-actions">
    <button class="btn deny" on:click={() => dispatch('deny', { sessionId })}>Deny</button>
    <button class="btn allow" on:click={() => dispatch('allow', { sessionId })}>Allow</button>
  </div>
</Modal>

<style>
  .perm-body {
    display: flex;
    flex-direction: column;
    gap: var(--sp-4);
    padding: var(--sp-4) 0;
  }

  .perm-tool {
    font-size: var(--md);
    font-weight: 600;
    color: var(--s-input);
  }

  .perm-desc {
    font-size: var(--sm);
    color: var(--t1);
    line-height: 1.5;
  }

  .perm-actions {
    display: flex;
    gap: var(--sp-4);
    justify-content: flex-end;
    padding-top: var(--sp-4);
  }

  .btn {
    border: 1px solid var(--bd1);
    border-radius: var(--radius-sm);
    padding: var(--sp-3) var(--sp-7);
    font-size: var(--sm);
    cursor: pointer;
    font-family: var(--mono);
  }

  .deny {
    background: none;
    color: var(--t1);
  }

  .deny:hover {
    border-color: var(--s-error);
    color: var(--s-error);
  }

  .allow {
    background: rgba(0, 212, 126, 0.1);
    color: var(--ac);
    border-color: var(--ac);
  }

  .allow:hover {
    background: rgba(0, 212, 126, 0.2);
  }
</style>
