use crate::ipc::IpcError;
use std::path::Path;

/// Find the orbit-mcp binary path. Checks:
/// 1. Tauri sidecar name (orbit-mcp-<triple>) next to exe
/// 2. Plain name (orbit-mcp) next to exe (dev builds)
/// 3. In PATH
fn find_orbit_mcp() -> Option<String> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let ext = if cfg!(windows) { ".exe" } else { "" };
            // Tauri sidecar: orbit-mcp-<triple>[.exe]
            let triple = env!("TARGET_TRIPLE");
            let sidecar = dir.join(format!("orbit-mcp-{triple}{ext}"));
            if sidecar.is_file() {
                return Some(sidecar.to_string_lossy().into_owned());
            }
            // Dev build: orbit-mcp[.exe]
            let plain = dir.join(format!("orbit-mcp{ext}"));
            if plain.is_file() {
                return Some(plain.to_string_lossy().into_owned());
            }
        }
    }
    crate::services::spawn_manager::find_cli_in_path("orbit-mcp")
}

/// Write `.mcp.json` to a project directory, configuring orbit-mcp as an MCP server.
/// Returns the path to the orbit-mcp binary used.
#[tauri::command]
pub fn setup_orchestration(project_path: String) -> Result<String, IpcError> {
    let mcp_bin = find_orbit_mcp()
        .ok_or_else(|| IpcError::Other("orbit-mcp binary not found".to_string()))?;

    let mcp_config = serde_json::json!({
        "mcpServers": {
            "orbit": {
                "command": mcp_bin,
            }
        }
    });

    let config_path = Path::new(&project_path).join(".mcp.json");
    let content =
        serde_json::to_string_pretty(&mcp_config).map_err(|e| IpcError::Other(e.to_string()))?;

    std::fs::write(&config_path, content)
        .map_err(|e| IpcError::Other(format!("failed to write .mcp.json: {e}")))?;

    Ok(mcp_bin)
}

/// Check if orchestration is available (orbit-mcp binary exists).
#[tauri::command]
pub fn check_orchestration() -> serde_json::Value {
    match find_orbit_mcp() {
        Some(path) => serde_json::json!({ "available": true, "path": path }),
        None => serde_json::json!({ "available": false, "path": null }),
    }
}
