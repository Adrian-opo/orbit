use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};

use crate::models::{Session, SessionId, JournalEntry};
use crate::services::session_manager::SessionManager;

pub struct SessionState(pub Arc<Mutex<SessionManager>>);

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
    SessionManager::create_session(
        Arc::clone(&state.0),
        app,
        project_path,
        prompt,
        model,
        mode,
        session_name,
    )
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
