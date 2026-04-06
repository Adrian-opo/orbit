# Orbit

**Orbit** is a desktop application for managing multiple [Claude Code](https://github.com/anthropics/claude-code) sessions simultaneously. It provides a clean terminal-style interface to create, monitor, and interact with Claude agents running in parallel across different projects.

---

## Overview

```
┌─ orbit ──────────────────────────────────────────────────────────────┐
│  sidebar        │  session feed              │  meta panel           │
│  ─────────────  │  ──────────────────────    │  ───────────────────  │
│  ● api-server   │  you · 14:32               │  tokens               │
│    running      │    fix the auth bug        │  24.8K                │
│    sonnet · 14K │                            │  input    19.2K       │
│  ● dashboard    │  claude · 14:33            │  output    5.6K       │
│    waiting      │    I'll look at auth.ts…   │                       │
│    opus · 89K   │                            │  cost  $0.48          │
│  ● utils-lib    │  run · bash                │                       │
│    idle         │    $ git status            │  context  14.2%       │
└─────────────────┴────────────────────────────┴───────────────────────┘
```

---

## Features

- **Multi-session management** — spawn and control multiple Claude Code processes in parallel
- **Real-time journal** — streaming JSON output displayed as structured entries (thinking, tool calls, responses)
- **Session persistence** — SQLite database stores history; sessions resume after app restart via `--resume`
- **Project isolation** — each session runs in its own working directory
- **Context menu** — right-click sessions to rename, stop, or delete
- **Permission mode** — optional per-session tool approval
- **Slash commands** — `/` autocomplete from installed Claude Code plugins
- **@ file picker** — `@filename` inline file references
- **Cost tracking** — per-session token usage and estimated cost
- **Mock mode** — run frontend in any browser without the Rust backend

---

## Requirements

### Runtime
- **Windows 10 1903+** (or macOS / Linux)
- **[Claude Code CLI](https://github.com/anthropics/claude-code)** installed and authenticated:
  ```bash
  npm install -g @anthropic-ai/claude-code
  claude login
  ```

### Development
- **Node.js** ≥ 20
- **Rust** stable ([rustup](https://rustup.rs))
- **npm** ≥ 10

---

## Getting Started

```bash
git clone https://github.com/xinnaider/orbit.git
cd orbit
npm install
npm run tauri:dev
```

### Mock mode (browser only, no Rust required)

```bash
npm run dev:mock
# Open http://localhost:1420
```

---

## Project Structure

```
orbit/
├── api/                    # Svelte/TypeScript frontend (UI layer)
│   ├── components/         # UI components
│   ├── lib/
│   │   ├── assets/         # SVG icons
│   │   ├── mock/           # Mock Tauri API for browser testing
│   │   ├── stores/         # Svelte stores (sessions, journal)
│   │   ├── cost.ts         # Token cost estimation
│   │   ├── status.ts       # Session status helpers
│   │   └── tauri.ts        # IPC bindings + event listeners
│   └── routes/             # SvelteKit routes
├── front/                  # Rust/Tauri native layer
│   ├── src/
│   │   ├── ipc/            # Tauri command handlers
│   │   ├── services/
│   │   │   ├── database.rs      # SQLite persistence (rusqlite)
│   │   │   ├── session_manager.rs # Session lifecycle + events
│   │   │   └── spawn_manager.rs  # Claude process spawning
│   │   ├── journal_reader.rs    # JSONL stream parser
│   │   └── models.rs            # Shared data types
│   └── tauri.conf.json     # Tauri configuration
└── .github/workflows/
    └── build.yml           # Windows .exe build + SHA-256
```

---

## Architecture

Two-phase session lifecycle:

1. **Init** (instant) — creates DB record, returns `Session` to frontend immediately
2. **Spawn** (background) — launches `claude.exe --output-format stream-json -p "prompt"` with piped stdout

Follow-up messages spawn a new process with `--resume <claude_session_id> -p "message"`. The `claude_session_id` is extracted from the first system message and persisted to SQLite.

```
Frontend             Rust backend         Claude CLI
────────             ────────────         ──────────
create_session() ──► init_session()
             ◄─ Session ──
                     do_spawn() ────────► claude.exe -p "prompt"
                     ◄── JSON stream ────
session:output ◄────
session:state  ◄────

send_message() ────► send_message()
                     do_spawn() ────────► claude.exe --resume <id> -p "msg"
                     ◄── JSON stream ────
session:output ◄────
```

### Key IPC commands

| Command | Description |
|---|---|
| `create_session` | Creates DB record + spawns Claude in background |
| `send_session_message` | Spawns follow-up Claude process with `--resume` |
| `list_sessions` | Returns all sessions enriched with runtime state |
| `get_session_journal` | Returns in-memory journal entries |
| `stop_session` | Marks session stopped |
| `check_claude` | Checks if Claude CLI is installed and returns path |
| `diagnose_spawn` | Runs path diagnostics for troubleshooting |

---

## Tests

```bash
npm test          # Frontend (TypeScript / Vitest)
npm run test:rust  # Backend (Rust / cargo test)
```

---

## Build

```bash
npm run tauri:build
```

### CI — GitHub Actions

Every push to `master` triggers a Windows build. Tag to release:

```bash
git tag v1.0.0 && git push origin v1.0.0
```

Artifacts include the `.exe` installer and `sha256sum.txt`.

---

## License

MIT © josefernando
