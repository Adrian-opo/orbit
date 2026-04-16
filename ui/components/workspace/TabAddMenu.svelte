<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

  export let x: number;
  export let y: number;

  const dispatch = createEventDispatcher<{
    select: { action: 'terminal' | 'session' | 'open' };
    close: void;
  }>();

  function select(action: 'terminal' | 'session' | 'open') {
    dispatch('select', { action });
    dispatch('close');
  }

  onMount(() => {
    setTimeout(() => {
      window.addEventListener('click', handleOutsideClick, { once: true });
    }, 0);

    return () => {
      window.removeEventListener('click', handleOutsideClick);
    };
  });

  function handleOutsideClick() {
    dispatch('close');
  }
</script>

<div class="menu" style="left: {x}px; top: {y}px;" role="menu">
  <button class="menu-item" role="menuitem" on:click={() => select('terminal')}>
    <span class="item-icon">{'>'}</span>
    New terminal
  </button>
  <button class="menu-item" role="menuitem" on:click={() => select('session')}>
    <span class="item-icon">●</span>
    New session
  </button>
  <div class="menu-divider"></div>
  <button class="menu-item" role="menuitem" on:click={() => select('open')}>
    <span class="item-icon">⊕</span>
    Open session...
  </button>
</div>

<style>
  .menu {
    position: fixed;
    background: var(--bg2);
    border: 1px solid var(--bd1);
    border-radius: var(--radius-sm);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    min-width: 160px;
    padding: var(--sp-2) 0;
    z-index: 1000;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: var(--sp-3);
    width: 100%;
    padding: var(--sp-2) var(--sp-4);
    border: none;
    background: transparent;
    color: var(--t1);
    font-size: var(--sm);
    cursor: pointer;
    text-align: left;
    transition: background 0.1s;
  }

  .menu-item:hover {
    background: var(--bg3);
    color: var(--t0);
  }

  .item-icon {
    font-size: var(--xs);
    color: var(--t2);
    width: 14px;
    text-align: center;
    flex-shrink: 0;
  }

  .menu-divider {
    height: 1px;
    background: var(--bd);
    margin: var(--sp-2) 0;
  }
</style>
