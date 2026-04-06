use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};

use crate::models::{Session, SessionId, JournalEntry};
use crate::services::session_manager::SessionManager;
use crate::services::spawn_manager::{find_claude, spawn_test};

pub struct SessionState(pub Arc<Mutex<SessionManager>>);

/// Create a session: returns immediately after creating the DB record (status = "initializing").
/// The actual Claude process spawns in a background thread — non-blocking.
/// Frontend should listen to "session:running" (ready) or "session:error" (spawn failed).
#[tauri::command]
pub fn create_session(
    project_path: String,
    prompt: String,
    model: Option<String>,
    permission_mode: Option<String>,
    session_name: Option<String>,
    state: State<SessionState>,
    app: AppHandle,
) -> Result<Session, String> {
    let mode = permission_mode.unwrap_or_else(|| "ignore".to_string());

    // Phase 1: fast — create DB record, return session immediately
    let session = {
        let mut m = state.0.lock().unwrap();
        m.init_session(
            &project_path,
            session_name.as_deref(),
            &mode,
            model.as_deref(),
        )?
    };

    // Phase 2: slow — spawn PTY in background (non-blocking)
    let manager = Arc::clone(&state.0);
    let session_clone = session.clone();
    std::thread::spawn(move || {
        SessionManager::do_spawn(manager, app, session_clone, prompt);
    });

    Ok(session)
}

#[tauri::command]
pub fn list_sessions(state: State<SessionState>) -> Vec<Session> {
    state.0.lock().unwrap().get_sessions()
}

#[tauri::command]
pub fn stop_session(session_id: SessionId, state: State<SessionState>) -> Result<(), String> {
    state.0.lock().unwrap().stop_session(session_id)
}

#[tauri::command]
pub fn send_session_message(
    session_id: SessionId,
    message: String,
    state: State<SessionState>,
) -> Result<(), String> {
    state.0.lock().unwrap().send_message(session_id, &message)
}

#[tauri::command]
pub fn get_session_journal(session_id: SessionId, state: State<SessionState>) -> Vec<JournalEntry> {
    state.0.lock().unwrap().get_journal(session_id)
}

/// Diagnostic: check if claude CLI is available and return its path or an error message.
#[tauri::command]
pub fn check_claude() -> serde_json::Value {
    match find_claude() {
        Some(path) => serde_json::json!({ "found": true, "path": path }),
        None => {
            let path_var = std::env::var("PATH").unwrap_or_default();
            serde_json::json!({
                "found": false,
                "path": null,
                "searchedPath": path_var,
                "hint": "Install with: npm install -g @anthropic-ai/claude-code"
            })
        }
    }
}

/// Diagnostic: test PTY spawning with a simple echo command.
/// Returns { pty_works, claude_found, claude_path, echo_output, error }
#[tauri::command]
pub fn diagnose_spawn() -> serde_json::Value {
    let claude_path = find_claude();
    let tmp = std::env::temp_dir();

    // Test 1: PTY works at all with a simple echo
    #[cfg(windows)]
    let echo_result = spawn_test("cmd", &["/c", "echo", "PTY_OK"], &tmp);
    #[cfg(not(windows))]
    let echo_result = spawn_test("echo", &["PTY_OK"], &tmp);

    let pty_works = echo_result.as_ref().map(|s| s.contains("PTY_OK")).unwrap_or(false);

    // Test 2: where/which claude in augmented PATH
    #[cfg(windows)]
    let which_result = spawn_test("cmd", &["/c", "where", "claude"], &tmp);
    #[cfg(not(windows))]
    let which_result = spawn_test("which", &["claude"], &tmp);

    serde_json::json!({
        "ptyWorks": pty_works,
        "echoOutput": echo_result.unwrap_or_else(|e| format!("FAIL: {e}")),
        "claudeFound": claude_path.is_some(),
        "claudePath": claude_path,
        "whichOutput": which_result.unwrap_or_else(|e| format!("FAIL: {e}")),
        "processPath": std::env::var("PATH").unwrap_or_default().chars().take(300).collect::<String>(),
    })
}

/// Rename a session.
#[tauri::command]
pub fn rename_session(
    session_id: SessionId,
    name: String,
    state: State<SessionState>,
) -> Result<(), String> {
    state.0.lock().unwrap().rename_session(session_id, &name)
}

/// Delete a session (removes from DB, stops if running).
#[tauri::command]
pub fn delete_session(
    session_id: SessionId,
    state: State<SessionState>,
) -> Result<(), String> {
    state.0.lock().unwrap().delete_session(session_id)
}
