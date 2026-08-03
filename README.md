# QuickMD

> A fast, lightweight, cross-platform Markdown viewer

QuickMD is a desktop application that opens instantly, renders Markdown beautifully, and stays out of your way. Designed to be the default app for `.md` files on Windows, Linux, and macOS.

Built with **Tauri v2** + **Svelte 5** — 7MB binary, under 100MB RAM.

## Features

- 🚀 **Instant startup** — Cold launch under 300ms
- 💾 **Lightweight** — ~7MB binary, under 100MB RAM idle
- 📝 **GitHub-Flavored Markdown** — Tables, task lists, code blocks with syntax highlighting, footnotes, emoji
- 🎨 **Themes** — Light, dark, and system-following with smooth transitions
- 🔍 **Search** — Ctrl+F with match highlighting
- 🔄 **Auto-reload** — Preview updates when the file changes on disk
- 🔍 **Zoom** — Ctrl+MouseWheel, Ctrl+/-, Ctrl+0 with smooth transform scaling
- 🖥️ **System tray** — Close hides to tray, tray menu (Open, Settings, About, Exit)
- ⚙️ **Settings** — Persistent settings with real-time preview updates
- ⌨️ **Keyboard shortcuts** — Full keyboard navigation
- 📁 **File associations** — Make QuickMD the default for .md, .markdown, .mdown
- 🚪 **Autostart hidden** — Start minimized to tray on system login

## Installation

### Windows
Download the `.exe` installer from [Releases](https://github.com/quickmd/quickmd/releases).

### macOS (coming soon)
Download the `.dmg` from [Releases](https://github.com/quickmd/quickmd/releases).

### Linux (coming soon)
Download the `.AppImage`, `.deb`, or `.rpm` from [Releases](https://github.com/quickmd/quickmd/releases).

## Development

### Prerequisites

- [Rust](https://rustup.rs) (stable, 1.77+)
- [Node.js](https://nodejs.org) (v18+)
- Platform-specific Tauri v2 dependencies ([see docs](https://v2.tauri.app/start/prerequisites/))
- Windows: Microsoft C++ Build Tools + WebView2 Runtime

### Setup

```bash
git clone https://github.com/quickmd/quickmd.git
cd quickmd
npm install
```

### Run

```bash
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

Output: `src-tauri/target/release/bundle/nsis/QuickMD_0.1.0_x64-setup.exe`

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Shell | Tauri v2 (Rust) |
| UI | Svelte 5 + TypeScript |
| Markdown | markdown-it + highlight.js |
| CSS | Custom properties theme system |
| Build | Vite + cargo |

## Roadmap

- [ ] Mermaid diagram rendering (lazy-loaded)
- [ ] KaTeX math rendering (lazy-loaded)
- [ ] Table of Contents sidebar
- [ ] PDF export (print-to-PDF)
- [ ] Multiple tabs
- [ ] Linux + macOS CI builds
- [ ] Auto-updater with code signing
- [ ] Presentation mode
- [ ] Custom CSS themes
- [ ] Plugin system

## License

MIT © QuickMD Contributors
