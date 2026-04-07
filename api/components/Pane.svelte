<script lang="ts">
  import type { PaneId } from '../lib/stores/layout';
  import type { Session } from '../lib/stores/sessions';

  export let paneId: PaneId;
  export let session: Session | null;
  // eslint-disable-next-line svelte/valid-compile
  export let focused: boolean;
  // eslint-disable-next-line svelte/valid-compile
  export let canClose: boolean;
  // eslint-disable-next-line svelte/valid-compile
  export let atMaxPanes: boolean;
</script>

<div class="pane" data-pane-id={paneId}>
  {#if session}
    <span>{session.id}</span>
  {:else}
    <span>+</span>
  {/if}
</div>

<style>
  .pane {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    min-width: 0;
    min-height: 0;
  }
</style>
