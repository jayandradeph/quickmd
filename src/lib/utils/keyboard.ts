/**
 * Keyboard shortcut definitions for QuickMD.
 * All shortcuts use the format expected by KeyboardEvent.
 */

export interface KeyboardShortcut {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  action: string;
  description: string;
}

export const SHORTCUTS: KeyboardShortcut[] = [
  { key: "o", ctrl: true, action: "open", description: "Open Markdown file" },
  { key: "r", ctrl: true, action: "reload", description: "Reload current file" },
  { key: "f", ctrl: true, action: "search", description: "Search in document" },
  { key: "p", ctrl: true, action: "print", description: "Print document" },
  { key: "p", ctrl: true, shift: true, action: "export-pdf", description: "Export to PDF" },
  { key: "=", ctrl: true, action: "zoom-in", description: "Zoom in" },
  { key: "-", ctrl: true, action: "zoom-out", description: "Zoom out" },
  { key: "0", ctrl: true, action: "zoom-reset", description: "Reset zoom" },
  { key: "F11", action: "fullscreen", description: "Toggle fullscreen" },
  { key: "Escape", action: "escape", description: "Close overlay / clear search" },
];

/**
 * Match a keyboard event against a shortcut definition.
 */
export function matchShortcut(
  event: KeyboardEvent,
  shortcut: KeyboardShortcut,
): boolean {
  const mod = event.ctrlKey || event.metaKey;
  if (shortcut.key !== event.key) return false;
  if ((shortcut.ctrl ?? false) !== mod) return false;
  if ((shortcut.shift ?? false) !== event.shiftKey) return false;
  if ((shortcut.alt ?? false) !== event.altKey) return false;
  return true;
}
