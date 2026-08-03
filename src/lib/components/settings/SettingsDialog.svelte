<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    isOpen: boolean;
    theme: string;
    zoom: number;
    onclose?: () => void;
    onsettingschanged?: () => void;
  }

  let {
    isOpen = false,
    theme = "system",
    zoom = 1,
    onclose,
    onsettingschanged,
  }: Props = $props();

  // Local state (loaded from backend on open)
  let fontSize = $state(16);
  let fontFamily = $state("system");
  let lineSpacing = $state(1.6);
  let autoReload = $state(true);
  let showToolbar = $state(true);
  let showStatusBar = $state(true);
  let recentFilesMax = $state(15);
  let enableAnimations = $state(true);
  let checkForUpdates = $state(true);
  let enableTrayIcon = $state(false);
  let enableAutostart = $state(false);
  let isDefaultApp = $state(false);
  let hardwareAcceleration = $state(true);
  let telemetry = $state(false);
  let activePanel = $state("appearance");
  let saveMessage = $state("");

  // ---- Load settings from backend ----
  async function loadSettings() {
    try {
      const settings = await invoke<Record<string, any>>("get_settings");
      if (settings.theme) theme = settings.theme;
      if (settings.fontSize) fontSize = settings.fontSize;
      if (settings.fontFamily) fontFamily = settings.fontFamily;
      if (settings.lineSpacing) lineSpacing = settings.lineSpacing;
      if (settings.autoReload !== undefined) autoReload = settings.autoReload;
      if (settings.showToolbar !== undefined) showToolbar = settings.showToolbar;
      if (settings.showStatusBar !== undefined) showStatusBar = settings.showStatusBar;
      if (settings.recentFilesMax) recentFilesMax = settings.recentFilesMax;
      if (settings.enableAnimations !== undefined) enableAnimations = settings.enableAnimations;
      if (settings.checkForUpdates !== undefined) checkForUpdates = settings.checkForUpdates;
      if (settings.enableTrayIcon !== undefined) enableTrayIcon = settings.enableTrayIcon;
      if (settings.enableAutostart !== undefined) enableAutostart = settings.enableAutostart;
      if (settings.isDefaultApp !== undefined) isDefaultApp = settings.isDefaultApp;
      if (settings.hardwareAcceleration !== undefined) hardwareAcceleration = settings.hardwareAcceleration;
      if (settings.telemetry !== undefined) telemetry = settings.telemetry;
    } catch (e) {
      console.error("Failed to load settings:", e);
    }
  }

  // ---- Save + notify parent ----
  async function saveSetting(key: string, value: any) {
    try {
      await invoke("set_setting", { key, value });
      showSaveMessage("Saved");
      // Notify parent to re-apply settings
      onsettingschanged?.();
      return true;
    } catch (e) {
      console.error(`Failed to save ${key}:`, e);
      showSaveMessage("Save failed");
      return false;
    }
  }

  function showSaveMessage(msg: string) {
    saveMessage = msg;
    setTimeout(() => saveMessage = "", 1800);
  }

  // ---- Checkbox handlers (no bind:checked — explicit toggle only) ----
  async function toggleSetting(key: string, currentVal: boolean, setter: (v: boolean) => void) {
    const newVal = !currentVal;
    setter(newVal);
    await saveSetting(key, newVal);
  }

  // ---- Theme change: save first, then the parent's onsettingschanged applies it ----
  async function handleThemeChange(newTheme: string) {
    theme = newTheme;
    await saveSetting("theme", newTheme);
  }

  // ---- File association (needs both save + registry) ----
  async function toggleDefaultApp() {
    const newVal = !isDefaultApp;
    isDefaultApp = newVal;
    await saveSetting("isDefaultApp", newVal);
    try {
      if (newVal) {
        await invoke("register_file_association");
      } else {
        await invoke("remove_file_association");
      }
    } catch (e) {
      console.error("File association error:", e);
    }
  }

  // ---- Autostart toggle ----
  async function toggleAutostart() {
    const newVal = !enableAutostart;
    enableAutostart = newVal;
    await saveSetting("enableAutostart", newVal);
    try {
      await invoke("set_autostart", { enabled: newVal });
    } catch (e) {
      console.error("Autostart error:", e);
    }
  }

  // ---- Panels ----
  function handleClose() {
    onclose?.();
  }

  $effect(() => {
    if (isOpen) loadSettings();
  });

  const fontOptions = [
    { value: "system", label: "System Default" },
    { value: "serif", label: "Serif (Georgia)" },
    { value: "sans-serif", label: "Sans-Serif" },
    { value: "mono", label: "Monospace" },
  ];
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="settings-overlay" onclick={handleClose} role="dialog" aria-label="Settings">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="settings-panel" onclick={(e) => e.stopPropagation()}>
      <nav class="settings-nav">
        <h2 class="settings-title">Settings</h2>
        <button class="settings-nav-item" class:active={activePanel === "appearance"}
          onclick={() => activePanel = "appearance"}>
          🎨 Appearance
        </button>
        <button class="settings-nav-item" class:active={activePanel === "behavior"}
          onclick={() => activePanel = "behavior"}>
          ⚡ Behavior
        </button>
        <button class="settings-nav-item" class:active={activePanel === "system"}
          onclick={() => activePanel = "system"}>
          💻 System
        </button>
      </nav>

      <div class="settings-content">
        <button class="settings-close" onclick={handleClose} aria-label="Close settings">✕</button>

        {#if activePanel === "appearance"}
          <h3>Appearance</h3>

          <div class="setting-group">
            <label class="setting-label">Theme</label>
            <div class="theme-options">
              <button class="theme-btn" class:active={theme === "light"} onclick={() => handleThemeChange("light")}>
                ☀️ Light
              </button>
              <button class="theme-btn" class:active={theme === "dark"} onclick={() => handleThemeChange("dark")}>
                🌙 Dark
              </button>
              <button class="theme-btn" class:active={theme === "system"} onclick={() => handleThemeChange("system")}>
                💻 System
              </button>
            </div>
          </div>

          <div class="setting-group">
            <label class="setting-label">Font Size ({fontSize}px)
              <input type="range" min="12" max="28" value={fontSize}
                oninput={(e) => { fontSize = Number((e.target as HTMLInputElement).value); saveSetting("fontSize", fontSize); }} />
            </label>
          </div>

          <div class="setting-group">
            <label class="setting-label">Font Family</label>
            <select class="setting-select" value={fontFamily}
              onchange={(e) => { fontFamily = (e.target as HTMLSelectElement).value; saveSetting("fontFamily", fontFamily); }}>
              {#each fontOptions as opt}
                <option value={opt.value}>{opt.label}</option>
              {/each}
            </select>
          </div>

          <div class="setting-group">
            <label class="setting-label">Line Spacing ({lineSpacing.toFixed(1)})
              <input type="range" min="1.0" max="2.5" step="0.1" value={lineSpacing}
                oninput={(e) => { lineSpacing = Number((e.target as HTMLInputElement).value); saveSetting("lineSpacing", lineSpacing); }} />
            </label>
          </div>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={enableAnimations}
                onchange={() => toggleSetting("enableAnimations", enableAnimations, (v) => enableAnimations = v)} />
              <span>Animations</span>
            </label>
          </div>

        {:else if activePanel === "behavior"}
          <h3>Behavior</h3>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={autoReload}
                onchange={() => toggleSetting("autoReload", autoReload, (v) => autoReload = v)} />
              <span>Auto-reload on file change</span>
            </label>
          </div>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={showToolbar}
                onchange={() => toggleSetting("showToolbar", showToolbar, (v) => showToolbar = v)} />
              <span>Show toolbar</span>
            </label>
          </div>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={showStatusBar}
                onchange={() => toggleSetting("showStatusBar", showStatusBar, (v) => showStatusBar = v)} />
              <span>Show status bar</span>
            </label>
          </div>

          <div class="setting-group">
            <label class="setting-label">Recent Files Limit
              <input type="number" class="num-input" min="0" max="50" value={recentFilesMax}
                onchange={(e) => { recentFilesMax = Number((e.target as HTMLInputElement).value); saveSetting("recentFilesMax", recentFilesMax); }} />
            </label>
          </div>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={hardwareAcceleration}
                onchange={() => toggleSetting("hardwareAcceleration", hardwareAcceleration, (v) => hardwareAcceleration = v)} />
              <span>Hardware acceleration</span>
            </label>
          </div>

        {:else if activePanel === "system"}
          <h3>System Integration</h3>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={isDefaultApp}
                onchange={toggleDefaultApp} />
              <span>Make QuickMD the default Markdown viewer</span>
            </label>
            <p class="setting-hint">Associate .md, .markdown, and .mdown files with QuickMD.</p>
          </div>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={enableAutostart}
                onchange={toggleAutostart} />
              <span>Launch on system startup (minimized to tray)</span>
            </label>
            <p class="setting-hint">Automatically start QuickMD in the system tray when you log in.</p>
          </div>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={enableTrayIcon}
                onchange={() => toggleSetting("enableTrayIcon", enableTrayIcon, (v) => enableTrayIcon = v)} />
              <span>Close to system tray</span>
            </label>
            <p class="setting-hint">When closing the window, QuickMD stays running in the tray.</p>
          </div>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={checkForUpdates}
                onchange={() => toggleSetting("checkForUpdates", checkForUpdates, (v) => checkForUpdates = v)} />
              <span>Check for updates</span>
            </label>
          </div>

          <div class="setting-group">
            <label class="setting-toggle">
              <input type="checkbox" checked={telemetry}
                onchange={() => toggleSetting("telemetry", telemetry, (v) => telemetry = v)} />
              <span>Send anonymous usage data</span>
            </label>
          </div>

          <hr />

          <div class="setting-group">
            <button class="secondary-btn" onclick={async () => {
              await invoke("reset_settings");
              await loadSettings();
              onsettingschanged?.();
              showSaveMessage("Reset to defaults");
            }}>Reset All Settings</button>
          </div>

          <div class="setting-group">
            <p class="about-text">QuickMD v0.1.0</p>
            <p class="about-text about-muted">Built with Tauri + Svelte</p>
          </div>
        {/if}

        {#if saveMessage}
          <div class="save-toast">{saveMessage}</div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .settings-overlay {
    position: fixed; inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 100;
    display: flex; justify-content: center; align-items: center;
  }
  .settings-panel {
    display: flex;
    width: 680px; max-width: 90vw;
    height: 500px; max-height: 80vh;
    background: var(--color-surface);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-lg);
    overflow: hidden;
  }
  .settings-nav {
    width: 170px; flex-shrink: 0;
    background: var(--color-bg-alt);
    padding: var(--space-5) var(--space-3);
    display: flex; flex-direction: column; gap: var(--space-1);
    border-right: 1px solid var(--color-border-light);
  }
  .settings-title {
    font-size: var(--font-size-sm); font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.05em;
    color: var(--color-text-muted);
    margin-bottom: var(--space-3); padding: 0 var(--space-2);
  }
  .settings-nav-item {
    display: flex; align-items: center; gap: var(--space-2);
    padding: var(--space-2) var(--space-2);
    border: none; border-radius: var(--radius-sm);
    background: transparent; color: var(--color-text);
    font-size: var(--font-size-sm); cursor: pointer;
    text-align: left; transition: background var(--transition-fast); width: 100%;
  }
  .settings-nav-item:hover { background: var(--color-bg-hover); }
  .settings-nav-item.active {
    background: var(--color-accent-subtle); color: var(--color-accent); font-weight: 600;
  }
  .settings-content {
    flex: 1; padding: var(--space-6); overflow-y: auto; position: relative;
  }
  .settings-close {
    position: absolute; top: var(--space-3); right: var(--space-3);
    width: 28px; height: 28px; border: none; border-radius: var(--radius-sm);
    background: transparent; color: var(--color-text-muted);
    font-size: 16px; cursor: pointer;
    display: flex; align-items: center; justify-content: center;
  }
  .settings-close:hover { background: var(--color-bg-hover); }
  .settings-content h3 {
    font-size: var(--font-size-xl); margin-bottom: var(--space-5);
    color: var(--color-text);
  }
  .setting-group { margin-bottom: var(--space-5); }
  .setting-label {
    display: flex; align-items: center; gap: var(--space-3);
    font-size: var(--font-size-sm); font-weight: 500; color: var(--color-text);
  }
  .setting-label input[type="range"] { width: 120px; margin-left: auto; }
  .setting-label input[type="number"] {
    width: 60px; padding: 2px 6px;
    border: 1px solid var(--color-border); border-radius: var(--radius-sm);
    background: var(--color-bg); color: var(--color-text); font-size: var(--font-size-sm);
  }
  .num-input { margin-left: auto; }
  .setting-select {
    display: block; margin-top: var(--space-1);
    padding: var(--space-1) var(--space-2);
    border: 1px solid var(--color-border); border-radius: var(--radius-sm);
    background: var(--color-bg); color: var(--color-text);
    font-size: var(--font-size-sm); width: 100%;
  }
  .setting-toggle {
    display: flex; align-items: center; gap: var(--space-2);
    font-size: var(--font-size-sm); font-weight: 500; cursor: pointer; color: var(--color-text);
  }
  .setting-toggle input[type="checkbox"] {
    width: 16px; height: 16px; accent-color: var(--color-accent);
  }
  .setting-hint {
    font-size: var(--font-size-xs); color: var(--color-text-muted);
    margin-top: var(--space-1); margin-left: 24px;
  }
  .theme-options { display: flex; gap: var(--space-2); margin-top: var(--space-2); }
  .theme-btn {
    display: flex; flex-direction: column; align-items: center; gap: var(--space-1);
    padding: var(--space-3) var(--space-4); border: 2px solid var(--color-border);
    border-radius: var(--radius-md); background: var(--color-bg);
    color: var(--color-text); font-size: var(--font-size-xs);
    cursor: pointer; transition: all var(--transition-fast); flex: 1;
  }
  .theme-btn:hover { border-color: var(--color-accent); }
  .theme-btn.active {
    border-color: var(--color-accent); background: var(--color-accent-subtle); color: var(--color-accent);
  }
  .secondary-btn {
    padding: var(--space-2) var(--space-4);
    border: 1px solid var(--color-border); border-radius: var(--radius-md);
    background: var(--color-bg); color: var(--color-text);
    font-size: var(--font-size-sm); cursor: pointer; transition: all var(--transition-fast);
  }
  .secondary-btn:hover { border-color: var(--color-accent); }
  .about-text { font-size: var(--font-size-xs); color: var(--color-text); margin: 0; }
  .about-muted { color: var(--color-text-muted); }
  hr { border: none; border-top: 1px solid var(--color-border-light); margin: var(--space-5) 0; }
  .save-toast {
    position: absolute; bottom: var(--space-4); right: var(--space-4);
    padding: var(--space-2) var(--space-4);
    background: var(--color-toast-bg); color: var(--color-toast-text);
    border-radius: var(--radius-md); font-size: var(--font-size-xs);
    animation: fadeIn 0.2s ease;
  }
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(4px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
