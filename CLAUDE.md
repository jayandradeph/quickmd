# CLAUDE.md — QuickMD

A fast, lightweight, cross-platform Markdown viewer built with Tauri v2 + Svelte 5.

## Tech Stack

- **Backend:** Rust + Tauri v2 (native shell, system tray, file watching, platform integration)
- **Frontend:** Svelte 5 + TypeScript + Vite
- **Markdown:** markdown-it (GFM) + highlight.js (syntax highlighting) + DOMPurify (sanitization)
- **CSS:** Custom CSS custom properties theme system (light/dark/system)

## Commands

```bash
# Development
npm install                  # Install frontend dependencies
npm run dev                  # Start Vite dev server (port 1420)
npm run tauri dev            # Start full Tauri dev (frontend + Rust backend)
npm run tauri build          # Production build with NSIS installer

# Quality
npm run check                # Svelte type checking
npm run typecheck            # TypeScript type checking
cargo check                  # Rust type checking (from src-tauri/)
cargo test                   # Rust tests
cargo clippy                 # Rust linting
```

## Architecture

```
Frontend (Svelte 5)            Backend (Rust)
───────────────────────────    ──────────────────────────
src/App.svelte                 src-tauri/src/lib.rs       (plugin registration, setup, close-to-tray)
src/lib/components/            src-tauri/src/commands/     (IPC command handlers)
  viewer/MarkdownPreview.svelte  file.rs, settings.rs, export.rs
  settings/SettingsDialog.svelte  platform.rs, watcher.rs
src/lib/services/markdown.ts   src-tauri/src/services/     (business logic)
                                  tray.rs (hardcoded icon, no file dependency)
                                  settings_manager.rs (JSON persistence, no disk reload)
                                  file_association.rs (OpenWithProgids + RegisteredApplications)
                                  file_watcher.rs, recent_files.rs, startup.rs, updater.rs

IPC: invoke() on frontend ↔ #[tauri::command] on backend
Events: Rust emit ("tray-open-file", "tray-open-settings") ↔ Frontend listen()
```

## Key Features (working)

- **Markdown rendering:** Custom CSS stylesheet (no github-markdown-css dependency), GFM via markdown-it, syntax highlighting via highlight.js
- **Dark mode:** Full dark theme via CSS custom properties with `!important` overrides for text visibility
- **Zoom:** `transform: scale()` on preview wrapper, Ctrl+MouseWheel, Ctrl+/-, Ctrl+0
- **Settings:** Persistent via Rust JSON store + localStorage backup. All settings applied in real-time
- **Toolbar:** Toggleable via settings. Emergency floating ⚙️ button when toolbar is hidden
- **Close-to-tray:** Window X hides to tray (doesn't exit). Right-click tray → Exit to quit
- **Autostart hidden mode:** `--hidden` flag launches directly to tray without showing window
- **File associations:** Windows registry with OpenWithProgids, RegisteredApplications, and Capabilities
- **CLI file open:** `quickmd.exe file.md` stores path in AppState, frontend retrieves via `get_pending_file_path` on mount
- **Tray icon:** Hardcoded 32x32 RGBA icon in Rust (no file dependency, never panics)
- **Encoding detection:** UTF-8, UTF-16 BE/LE with BOM, chardetng fallback

## Settings Schema

Stored in platform config directory (`com.quickmd.QuickMD/settings.json`) with localStorage mirror:
- `theme` (light/dark/system), `zoomLevel`, `fontSize`, `fontFamily`, `fontFamily`
- `lineSpacing`, `autoReload`, `showToolbar`, `showStatusBar`
- `recentFilesMax`, `enableAnimations`, `checkForUpdates`
- `enableTrayIcon`, `enableAutostart`, `isDefaultApp`
- `hardwareAcceleration`, `telemetry`

## Capabilities

`src-tauri/capabilities/default.json`: dialog, fs, store, shell, process, updater, window-state, autostart, global-shortcut, opener

## Known Limitations

- Windows-only built (Linux/macOS CI not configured yet)
- NSIS installer only (MSI dropped — requires WiX Toolset)
- PDF export is HTML-only (print-to-PDF via browser not yet wired)
- Mermaid/KaTeX not yet integrated
- No TOC sidebar or presentation mode
- Updater configured but endpoint placeholder (needs signing keys)
