<script lang="ts">
  // Placeholder — panels will be added in later tasks
</script>

<div class="workspace">
  <aside class="sidebar">
    <div class="sidebar-header">
      <span class="title">Agents</span>
      <span class="badge">0</span>
    </div>
    <div class="sidebar-content">
      <p class="placeholder-text">No agents detected</p>
    </div>
    <div class="sidebar-footer">0 tokens total</div>
  </aside>

  <main class="central">
    <div class="central-header">Select an agent</div>
    <div class="central-content"></div>
  </main>

  <aside class="right-panel">
    <div class="panel-tabs">
      <button class="tab active">Diff</button>
      <button class="tab">Files</button>
      <button class="tab">Tasks</button>
      <button class="tab">Stats</button>
    </div>
    <div class="panel-content"></div>
  </aside>
</div>

<style>
  .workspace {
    display: flex;
    height: 100vh;
    width: 100vw;
  }
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
  }
  .sidebar-header .title { font-size: 12px; }
  .sidebar-header .badge {
    background: var(--green-dim);
    color: var(--green);
    padding: 1px 6px;
    border-radius: 8px;
    font-size: 9px;
  }
  .sidebar-content {
    flex: 1;
    overflow-y: auto;
  }
  .sidebar-footer {
    padding: 8px 12px;
    border-top: 1px solid var(--border);
    font-size: 9px;
    color: var(--text-dim);
  }
  .placeholder-text {
    padding: 20px 12px;
    color: var(--text-muted);
    font-size: 11px;
    text-align: center;
  }
  .central {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .central-header {
    padding: 8px 14px;
    border-bottom: 1px solid var(--border);
    font-size: 13px;
    color: var(--text-muted);
  }
  .central-content { flex: 1; }
  .right-panel {
    width: 300px;
    border-left: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }
  .panel-tabs {
    display: flex;
    border-bottom: 1px solid var(--border);
  }
  .tab {
    padding: 8px 12px;
    font-size: 10px;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-muted);
    cursor: pointer;
  }
  .tab.active {
    color: var(--blue);
    border-bottom-color: var(--blue);
  }
  .panel-content { flex: 1; overflow-y: auto; }
</style>
