<script lang="ts">
  import MarkdownPreview from "./lib/components/viewer/MarkdownPreview.svelte";
  import SettingsDialog from "./lib/components/settings/SettingsDialog.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  const STORAGE_KEY = "quickmd-settings";

  // ---- State ----
  let currentContent = $state("");
  let currentFilePath = $state("");
  let currentFileName = $state("");
  let isLoading = $state(false);
  let errorMessage = $state("");
  let zoom = $state(1);
  let theme = $state<"light" | "dark" | "system">("system");
  let showSearch = $state(false);
  let showSettings = $state(false);
  let searchQuery = $state("");
  let recentFiles = $state<Array<{ path: string; name: string; opened_at: string }>>([]);
  let showToolbar = $state(true);
  let showStatusBar = $state(true);

  // ---- Theme Management ----
  function applyTheme(newTheme: "light" | "dark" | "system") {
    theme = newTheme;
    const html = document.documentElement;
    html.classList.remove("light", "dark");
    if (newTheme === "system") {
      html.classList.toggle("dark", window.matchMedia("(prefers-color-scheme: dark)").matches);
    } else {
      html.classList.toggle("dark", newTheme === "dark");
    }
  }

  function toggleTheme() {
    const next: Record<string, "light" | "dark" | "system"> = {
      light: "dark", dark: "system", system: "light",
    };
    const newTheme = next[theme];
    applyTheme(newTheme);
    saveLocalSetting("theme", newTheme);
    invoke("set_setting", { key: "theme", value: newTheme }).catch(() => {});
  }

  // ---- Settings (localStorage first, Rust backend second) ----
  function getLocalSettings(): Record<string, any> | null {
    try {
      const raw = localStorage.getItem(STORAGE_KEY);
      return raw ? JSON.parse(raw) : null;
    } catch { return null; }
  }

  function saveLocalSetting(key: string, value: any) {
    try {
      const settings = getLocalSettings() || {};
      settings[key] = value;
      localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
    } catch { /* ignore */ }
  }

  async function loadAndApplySettings() {
    // Priority: 1. localStorage  2. Rust backend  3. defaults
    const local = getLocalSettings();

    let settings: Record<string, any> | null = null;
    try {
      settings = await invoke<Record<string, any>>("get_settings");
    } catch { /* Rust backend unavailable, use local */ }

    // Merge: Rust settings have priority, local fills gaps
    const merged: Record<string, any> = { ...(local || {}), ...(settings || {}) };

    if (merged.theme) applyTheme(merged.theme as "light" | "dark" | "system");
    if (merged.zoomLevel !== undefined) zoom = merged.zoomLevel;
    if (merged.showToolbar !== undefined) showToolbar = merged.showToolbar;
    if (merged.showStatusBar !== undefined) showStatusBar = merged.showStatusBar;

    if (merged.fontSize) document.documentElement.style.setProperty("--font-size-base", merged.fontSize + "px");
    if (merged.fontFamily && merged.fontFamily !== "system") {
      const fm: Record<string, string> = {
        serif: "Georgia, serif", "sans-serif": "-apple-system, sans-serif", mono: "'JetBrains Mono', monospace",
      };
      document.documentElement.style.setProperty("--font-sans", fm[merged.fontFamily] || "-apple-system, sans-serif");
    }
    if (merged.lineSpacing) document.documentElement.style.setProperty("--line-height", String(merged.lineSpacing));

    // Persist merged settings to localStorage
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(merged)); } catch { /* ignore */ }
  }

  // Called when SettingsDialog saves anything — reload and re-apply
  function onSettingsChanged() {
    loadAndApplySettings();
  }

  // ---- File Operations ----
  async function openFile(path?: string) {
    try {
      let filePath = path;
      if (!filePath) {
        const result = await open({
          multiple: false,
          filters: [
            { name: "Markdown", extensions: ["md", "markdown", "mdown"] },
            { name: "All Files", extensions: ["*"] },
          ],
        });
        if (!result) return;
        filePath = result as string;
      }

      isLoading = true;
      errorMessage = "";

      const fileInfo = await invoke<{
        path: string; name: string; content: string;
        size: number; modified_at: string; encoding: string;
      }>("open_file", { path: filePath });

      currentContent = fileInfo.content;
      currentFilePath = fileInfo.path;
      currentFileName = fileInfo.name;
      document.title = `${fileInfo.name} — QuickMD`;

      await loadRecentFiles();
    } catch (err) {
      errorMessage = String(err);
    } finally {
      isLoading = false;
    }
  }

  async function reloadFile() {
    if (currentFilePath) await openFile(currentFilePath);
  }

  async function loadRecentFiles() {
    try { recentFiles = await invoke("get_recent_files"); } catch { recentFiles = []; }
  }

  // ---- Search ----
  function toggleSearch() {
    showSearch = !showSearch;
    if (!showSearch) searchQuery = "";
  }

  // ---- Zoom ----
  function zoomIn() { zoom = Math.min(3, Math.round((zoom + 0.1) * 10) / 10); }
  function zoomOut() { zoom = Math.max(0.5, Math.round((zoom - 0.1) * 10) / 10); }
  function resetZoom() { zoom = 1; }

  // ---- Event Handlers ----
  function handleDrop(ev: DragEvent) {
    ev.preventDefault();
    const files = ev.dataTransfer?.files;
    const file = files?.[0] as (File & { path?: string }) | undefined;
    if (file?.name.match(/\.(md|markdown|mdown)$/i) && file.path) openFile(file.path);
  }

  function handleDragover(ev: DragEvent) {
    ev.preventDefault();
    if (ev.dataTransfer) ev.dataTransfer.dropEffect = "copy";
  }

  function handleWheel(ev: WheelEvent) {
    if (ev.ctrlKey || ev.metaKey) {
      ev.preventDefault();
      if (ev.deltaY > 0) zoomOut(); else zoomIn();
    }
  }

  function handleKeydown(ev: KeyboardEvent) {
    const mod = ev.ctrlKey || ev.metaKey;
    if (mod && ev.key === "o") { ev.preventDefault(); openFile(); }
    else if (mod && ev.key === "f") { ev.preventDefault(); toggleSearch(); }
    else if (mod && ev.key === "r") { ev.preventDefault(); reloadFile(); }
    else if (mod && ev.key === "=") { ev.preventDefault(); zoomIn(); }
    else if (mod && ev.key === "-") { ev.preventDefault(); zoomOut(); }
    else if (mod && ev.key === "0") { ev.preventDefault(); resetZoom(); }
    else if (ev.key === "F11") { ev.preventDefault(); document.documentElement.requestFullscreen(); }
    else if (ev.key === "Escape") {
      if (showSearch) { showSearch = false; searchQuery = ""; }
    }
  }

  function themeLabel() {
    return theme === "light" ? "☀️ Light" : theme === "dark" ? "🌙 Dark" : "💻 System";
  }

  // ---- Lifecycle ----
  onMount(() => {
    let unlisten1: (() => void) | undefined;

    void (async () => {
      await loadAndApplySettings();

      // Check for pending file from CLI / "Open With"
      try {
        const pendingPath = await invoke<string | null>("get_pending_file_path");
        if (pendingPath) await openFile(pendingPath);
      } catch { /* no pending file */ }

      window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
        if (theme === "system") applyTheme("system");
      });

      unlisten1 = await listen<string>("open-file-request", (e) => openFile(e.payload));
      await listen("tray-open-file", () => openFile());
      await listen("tray-open-settings", () => showSettings = true);
      await loadRecentFiles();
    })();

    return () => { unlisten1?.(); };
  });
</script>

<svelte:window onkeydown={handleKeydown} onwheel={handleWheel} />

<div class="app-container" ondrop={handleDrop} ondragover={handleDragover}
  role="application" aria-label="QuickMD Markdown Viewer">

  <!-- Toolbar -->
  {#if showToolbar}
    <header class="toolbar">
      <div class="toolbar-left">
        <button class="toolbar-btn" onclick={() => openFile()} title="Open (Ctrl+O)">
          <span class="btn-icon">📂</span> <span class="btn-label">Open</span>
        </button>
        <button class="toolbar-btn" onclick={reloadFile} disabled={!currentFilePath} title="Reload (Ctrl+R)">
          <span class="btn-icon">↻</span> <span class="btn-label">Reload</span>
        </button>
        <button class="toolbar-btn" onclick={toggleTheme} title="Toggle Theme">
          <span class="btn-icon">{theme === "light" ? "☀️" : theme === "dark" ? "🌙" : "💻"}</span>
          <span class="btn-label">{themeLabel()}</span>
        </button>
      </div>
      <div class="toolbar-center">
        {#if currentFileName}<span class="file-name">{currentFileName}</span>{/if}
      </div>
      <div class="toolbar-right">
        <button class="toolbar-btn" onclick={toggleSearch} title="Search (Ctrl+F)">🔍</button>
        <button class="toolbar-btn" onclick={zoomOut} title="Zoom Out (Ctrl+-)">➖</button>
        <span class="zoom-label">{Math.round(zoom * 100)}%</span>
        <button class="toolbar-btn" onclick={zoomIn} title="Zoom In (Ctrl+=)">➕</button>
        <button class="toolbar-btn" onclick={resetZoom} title="Reset Zoom (Ctrl+0)">↺</button>
        <button class="toolbar-btn" onclick={() => showSettings = true} title="Settings">⚙️</button>
      </div>
    </header>
  {/if}

  <!-- Search Bar -->
  {#if showSearch}
    <div class="search-bar">
      <span class="search-icon">🔍</span>
      <!-- svelte-ignore a11y_autofocus -- autofocus is intentional: Ctrl+F must let the user type immediately -->
      <input type="text" placeholder="Search in document..." bind:value={searchQuery}
        class="search-input" autofocus />
      <button class="search-close" onclick={toggleSearch} title="Close (Escape)">✕</button>
    </div>
  {/if}

  <!-- Settings Dialog -->
  <SettingsDialog
    isOpen={showSettings}
    {theme} {zoom}
    onclose={() => showSettings = false}
    onsettingschanged={onSettingsChanged}
  />

  <!-- Main Content -->
  <main class="main-content">
    {#if errorMessage}
      <div class="error-state">
        <span class="state-icon">⚠️</span>
        <p>{errorMessage}</p>
        <button class="primary-btn" onclick={() => openFile()}>Try Again</button>
      </div>
    {:else if isLoading}
      <div class="loading-state">
        <div class="spinner"></div>
        <p>Loading...</p>
      </div>
    {:else if currentContent}
      <MarkdownPreview content={currentContent} filePath={currentFilePath} {zoom} />
    {:else}
      <div class="welcome-state">
        <div class="welcome-icon">📝</div>
        <h1>QuickMD</h1>
        <p class="welcome-subtitle">Fast, lightweight Markdown viewer</p>
        <div class="welcome-actions">
          <button class="primary-btn" onclick={() => openFile()}>📂 Open Markdown File</button>
          <p class="welcome-hint">Or drag and drop a .md file here</p>
        </div>
        {#if recentFiles.length > 0}
          <div class="recent-files">
            <h3>Recent Files</h3>
            {#each recentFiles.slice(0, 10) as file}
              <button class="recent-file-item" onclick={() => openFile(file.path)}>
                <span class="recent-file-name">📄 {file.name}</span>
                <span class="recent-file-path">{file.path}</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </main>

  <!-- Status Bar -->
  {#if showStatusBar && currentFilePath}
    <footer class="statusbar">
      <span class="statusbar-item">📄 {currentFilePath}</span>
      <span class="statusbar-spacer"></span>
      <span class="statusbar-item">{currentContent.split(/\s+/).filter(Boolean).length.toLocaleString()} words</span>
      <span class="statusbar-item">{Math.max(1, Math.ceil(currentContent.split(/\s+/).filter(Boolean).length / 200))} min read</span>
    </footer>
  {/if}

  <!-- Floating settings button — ALWAYS visible, even when toolbar is hidden -->
  {#if !showToolbar}
    <button class="settings-float" onclick={() => showSettings = true} title="Settings">⚙️</button>
  {/if}
</div>

<style>
  .app-container {
    display: flex; flex-direction: column; height: 100vh;
    background: var(--color-bg); color: var(--color-text); position: relative;
  }
  .toolbar {
    display: flex; align-items: center; justify-content: space-between;
    height: var(--toolbar-height); padding: 0 var(--space-2);
    background: var(--color-surface); border-bottom: 1px solid var(--color-border-light);
    -webkit-app-region: drag; user-select: none; flex-shrink: 0;
  }
  .toolbar-left, .toolbar-right {
    display: flex; align-items: center; gap: 2px; -webkit-app-region: no-drag;
  }
  .toolbar-center { display: flex; align-items: center; -webkit-app-region: no-drag; }
  .file-name {
    font-size: var(--font-size-sm); font-weight: 600; color: var(--color-text-secondary);
    max-width: 300px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
  }
  .toolbar-btn {
    display: flex; align-items: center; gap: 2px;
    padding: var(--space-1) var(--space-2); border: none; border-radius: var(--radius-sm);
    background: transparent; color: var(--color-text); font-size: var(--font-size-sm);
    cursor: pointer; transition: background var(--transition-fast); white-space: nowrap;
  }
  .toolbar-btn:hover { background: var(--color-bg-hover); }
  .toolbar-btn:disabled { opacity: 0.4; cursor: default; }
  .btn-icon { font-size: 16px; line-height: 1; }
  .btn-label { font-size: var(--font-size-sm); }
  .zoom-label { font-size: var(--font-size-xs); color: var(--color-text-muted); min-width: 36px; text-align: center; }
  .search-bar {
    display: flex; align-items: center; padding: var(--space-2) var(--space-4);
    background: var(--color-surface-raised); border-bottom: 1px solid var(--color-border-light);
    gap: var(--space-2); flex-shrink: 0;
  }
  .search-input {
    flex: 1; padding: var(--space-1) var(--space-3);
    border: 1px solid var(--color-border); border-radius: var(--radius-md);
    background: var(--color-bg); color: var(--color-text); font-size: var(--font-size-sm); outline: none;
  }
  .search-input:focus { border-color: var(--color-border-focus); box-shadow: 0 0 0 2px var(--color-accent-subtle); }
  .search-close { padding: 2px 8px; border: none; background: none; color: var(--color-text-muted); cursor: pointer; font-size: var(--font-size-lg); }
  .main-content { flex: 1; display: flex; overflow: hidden; }

  /* Floating settings — always visible */
  .settings-float {
    position: fixed; bottom: 12px; right: 12px; z-index: 50;
    width: 40px; height: 40px; border: 1px solid var(--color-border);
    border-radius: 50%; background: var(--color-surface);
    color: var(--color-text); font-size: 20px; cursor: pointer;
    display: flex; align-items: center; justify-content: center;
    box-shadow: var(--shadow-md); transition: all var(--transition-fast);
  }
  .settings-float:hover { background: var(--color-bg-hover); transform: scale(1.05); }

  .welcome-state {
    flex: 1; display: flex; flex-direction: column; align-items: center;
    justify-content: center; gap: var(--space-4); padding: var(--space-8);
  }
  .welcome-icon { font-size: 64px; }
  .welcome-state h1 { font-size: var(--font-size-3xl); font-weight: 700; color: var(--color-text); }
  .welcome-subtitle { color: var(--color-text-secondary); font-size: var(--font-size-lg); }
  .welcome-actions { display: flex; flex-direction: column; align-items: center; gap: var(--space-3); margin-top: var(--space-4); }
  .welcome-hint { font-size: var(--font-size-sm); color: var(--color-text-muted); }
  .primary-btn {
    display: flex; align-items: center; gap: var(--space-2);
    padding: var(--space-2) var(--space-5); background: var(--color-accent);
    color: white; border: none; border-radius: var(--radius-md);
    font-size: var(--font-size-base); font-weight: 500; cursor: pointer;
    transition: background var(--transition-fast);
  }
  .primary-btn:hover { background: var(--color-accent-hover); }
  .error-state, .loading-state {
    flex: 1; display: flex; flex-direction: column; align-items: center;
    justify-content: center; gap: var(--space-4); color: var(--color-text-secondary);
  }
  .spinner {
    width: 32px; height: 32px; border: 3px solid var(--color-border);
    border-top-color: var(--color-accent); border-radius: 50%; animation: spin 0.6s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  .recent-files { margin-top: var(--space-6); width: 100%; max-width: 500px; }
  .recent-files h3 { font-size: var(--font-size-sm); color: var(--color-text-muted); text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: var(--space-2); }
  .recent-file-item {
    display: flex; flex-direction: column; align-items: flex-start; width: 100%;
    padding: var(--space-2) var(--space-3); border: none; border-radius: var(--radius-sm);
    background: transparent; cursor: pointer; text-align: left; transition: background var(--transition-fast);
  }
  .recent-file-item:hover { background: var(--color-bg-hover); }
  .recent-file-name { font-size: var(--font-size-sm); font-weight: 500; color: var(--color-text); }
  .recent-file-path { font-size: var(--font-size-xs); color: var(--color-text-muted); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 100%; }

  .statusbar {
    display: flex; align-items: center; height: var(--statusbar-height);
    padding: 0 var(--space-3); background: var(--color-surface);
    border-top: 1px solid var(--color-border-light);
    font-size: var(--font-size-xs); color: var(--color-text-muted); flex-shrink: 0; user-select: none;
  }
  .statusbar-item { margin-right: var(--space-4); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .statusbar-spacer { flex: 1; }
</style>
