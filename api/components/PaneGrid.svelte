<script lang="ts">
  import { splitLayout } from '../lib/stores/layout';
  import type { PaneId } from '../lib/stores/layout';
  import { sessions } from '../lib/stores/sessions';
  import type { Session } from '../lib/stores/sessions';
  import Pane from './Pane.svelte';

  function getSession(sessionId: number | null): Session | null {
    if (sessionId === null) return null;
    return $sessions.find((s) => s.id === sessionId) ?? null;
  }

  // Need 2 columns only when panes exist on BOTH left and right sides
  $: hasLeftCol = $splitLayout.visible.some((p) => p === 'tl' || p === 'bl');
  $: hasRightCol = $splitLayout.visible.some((p) => p === 'tr' || p === 'br');
  // Need 2 rows only when panes exist on BOTH top and bottom sides
  $: hasTopRow = $splitLayout.visible.some((p) => p === 'tl' || p === 'tr');
  $: hasBotRow = $splitLayout.visible.some((p) => p === 'bl' || p === 'br');

  $: cols = hasLeftCol && hasRightCol ? '1fr 1fr' : '1fr';
  $: rows = hasTopRow && hasBotRow ? '1fr 1fr' : '1fr';

  // Reactive map so grid positions update immediately when hasLeftCol/hasRightCol/etc. change
  $: gridAreas = Object.fromEntries(
    (['tl', 'tr', 'bl', 'br'] as PaneId[]).map((p) => {
      const col = hasLeftCol && hasRightCol && (p === 'tr' || p === 'br') ? 2 : 1;
      const row = hasTopRow && hasBotRow && (p === 'bl' || p === 'br') ? 2 : 1;
      return [p, `${row} / ${col} / ${row + 1} / ${col + 1}`];
    })
  ) as Record<PaneId, string>;
</script>

<div class="grid" style="grid-template-columns:{cols};grid-template-rows:{rows}">
  {#each $splitLayout.visible as paneId (paneId)}
    <Pane
      {paneId}
      gridArea={gridAreas[paneId]}
      session={getSession($splitLayout.panes[paneId])}
      focused={$splitLayout.focused === paneId}
      canClose={$splitLayout.visible.length > 1}
      atMaxPanes={$splitLayout.visible.length >= 4}
    />
  {/each}
</div>

<style>
  .grid {
    display: grid;
    flex: 1;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
    gap: 1px;
    background: var(--bd);
  }
</style>
