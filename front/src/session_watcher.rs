use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;

/// Info from a ~/.claude/sessions/{PID}.json file.
#[derive(Debug, Clone)]
pub struct LiveSession {
    pub session_id: String,
    pub pid: i32,
    pub cwd: String,
    pub started_at: u64,
}

/// Get all PIDs of running Claude Code CLI processes (not desktop app).
/// Single PowerShell call instead of one per PID.
fn get_claude_cli_pids() -> HashSet<i32> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Get-Process claude -ErrorAction SilentlyContinue | Where-Object {$_.Path -like '*\\.local\\*'} | Select-Object -ExpandProperty Id",
        ])
        .output();

    let mut pids = HashSet::new();
    if let Ok(o) = output {
        if o.status.success() {
            for line in String::from_utf8_lossy(&o.stdout).lines() {
                if let Ok(pid) = line.trim().parse::<i32>() {
                    pids.insert(pid);
                }
            }
        }
    }
    pids
}

/// Read all ~/.claude/sessions/{PID}.json files for live PIDs and return session mapping.
pub fn discover_live_sessions() -> Vec<LiveSession> {
    let session_dir = match dirs::home_dir() {
        Some(h) => h.join(".claude").join("sessions"),
        None => return vec![],
    };

    let entries = match fs::read_dir(&session_dir) {
        Ok(e) => e,
        Err(_) => return vec![],
    };

    let cli_pids = get_claude_cli_pids();
    let mut sessions = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.extension().is_some_and(|e| e == "json") {
            continue;
        }

        let pid_str = match path.file_stem().map(|s| s.to_string_lossy().to_string()) {
            Some(s) => s,
            None => continue,
        };

        let pid: i32 = match pid_str.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        if !cli_pids.contains(&pid) {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        #[derive(Deserialize)]
        struct SessionFile {
            #[serde(rename = "sessionId")]
            session_id: String,
            #[serde(default)]
            cwd: String,
            #[serde(rename = "startedAt", default)]
            started_at: u64,
        }

        if let Ok(info) = serde_json::from_str::<SessionFile>(&content) {
            sessions.push(LiveSession {
                session_id: info.session_id,
                pid,
                cwd: info.cwd,
                started_at: info.started_at,
            });
        }
    }

    sessions
}

/// Find the JSONL log file for a session by checking ~/.claude/projects/.
/// Returns (jsonl_path, project_name).
pub fn find_session_jsonl(session_id: &str) -> Option<(PathBuf, String)> {
    let projects_dir = dirs::home_dir()?.join(".claude").join("projects");
    let entries = fs::read_dir(&projects_dir).ok()?;

    for entry in entries.flatten() {
        let project_dir = entry.path();
        if !project_dir.is_dir() {
            continue;
        }

        // Check direct match: {sessionId}.jsonl
        let direct = project_dir.join(format!("{}.jsonl", session_id));
        if direct.exists() {
            let project_name = decode_project_path(&project_dir);
            return Some((direct, project_name));
        }

        // Check tail of existing JSONL files for session ID (context reset case)
        let jsonl_files = fs::read_dir(&project_dir).ok()?;
        for jentry in jsonl_files.flatten() {
            let path = jentry.path();
            if !path.extension().is_some_and(|e| e == "jsonl") || path.is_dir() {
                continue;
            }
            if let Ok(content) = fs::read_to_string(&path) {
                let tail_start = content.len().saturating_sub(4096);
                let safe_start = content.ceil_char_boundary(tail_start);
                if content[safe_start..].contains(session_id) {
                    let project_name = decode_project_path(&project_dir);
                    return Some((path, project_name));
                }
            }
        }
    }
    None
}

/// Decode project directory name back to a filesystem path.
/// E.g. "C--Users-foo-project" -> "C:/Users/foo/project"
fn decode_project_path(project_dir: &std::path::Path) -> String {
    let name = project_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    if name.len() >= 3 && name.as_bytes()[0].is_ascii_alphabetic() && &name[1..3] == "--" {
        let drive = &name[0..1];
        let rest = &name[3..];
        format!("{}:/{}", drive, rest.replace('-', "/"))
    } else if let Some(stripped) = name.strip_prefix("--") {
        format!("/{}", stripped.replace('-', "/"))
    } else {
        name
    }
}

/// Get git branch for a working directory.
pub fn git_branch(cwd: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["-C", cwd, "rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() || branch == "HEAD" {
        None
    } else {
        Some(branch)
    }
}

/// Get the git repo name from a working directory.
pub fn git_repo_name(cwd: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["-C", cwd, "rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let toplevel = String::from_utf8_lossy(&output.stdout).trim().to_string();
    std::path::Path::new(&toplevel)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
}
