use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager};

use crate::agent_tree;
use crate::journal_reader::{self, JournalState};
use crate::models::*;
use crate::session_watcher;

pub struct AppState {
    pub journal_states: Mutex<HashMap<String, JournalState>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            journal_states: Mutex::new(HashMap::new()),
        }
    }
}

/// Start the background polling loop that emits agent updates every 2 seconds.
pub fn start_polling(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            let agents = poll_once(&app);
            let _ = app.emit("agents-update", &agents);
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });
}

fn poll_once(app: &AppHandle) -> Vec<AgentState> {
    let live_sessions = session_watcher::discover_live_sessions();
    let state = app.state::<AppState>();
    let mut journal_states = state.journal_states.lock().unwrap();

    let mut agents = Vec::new();

    for live in &live_sessions {
        let (jsonl_path, project_name) = match session_watcher::find_session_jsonl(&live.session_id) {
            Some(p) => p,
            None => continue,
        };

        let prev = journal_states.get(&live.session_id);
        let prev_file_size = prev.map(|s| s.file_size).unwrap_or(0);

        let journal = journal_reader::parse_journal(&jsonl_path, prev_file_size, prev);

        let branch = session_watcher::git_branch(&live.cwd);
        let project = session_watcher::git_repo_name(&live.cwd)
            .unwrap_or(project_name);

        let model_display = journal.model.as_deref()
            .map(model_display_name)
            .unwrap_or("—")
            .to_string();

        let context_percent = {
            let total = journal.input_tokens + journal.output_tokens;
            let window = journal.model.as_deref()
                .map(context_window)
                .unwrap_or(200_000);
            if window > 0 { (total as f64 / window as f64) * 100.0 } else { 0.0 }
        };

        let subagents = agent_tree::read_subagents(&live.session_id);

        agents.push(AgentState {
            session_id: live.session_id.clone(),
            project,
            cwd: live.cwd.clone(),
            git_branch: branch,
            status: journal.status.clone(),
            model: journal.model.clone(),
            model_display,
            tokens: TokenUsage {
                input: journal.input_tokens,
                output: journal.output_tokens,
                cache_read: journal.cache_read,
                cache_write: journal.cache_write,
            },
            context_percent,
            subagents,
            mini_log: journal.mini_log.clone(),
            pending_approval: journal.pending_approval.clone(),
            pid: Some(live.pid),
            started_at: live.started_at,
        });

        journal_states.insert(live.session_id.clone(), journal);
    }

    // Remove stale sessions
    let live_ids: std::collections::HashSet<_> = live_sessions.iter().map(|s| s.session_id.clone()).collect();
    journal_states.retain(|id, _| live_ids.contains(id));

    agents
}
