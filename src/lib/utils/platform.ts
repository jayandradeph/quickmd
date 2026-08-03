/**
 * Platform detection utilities.
 */

export function isWindows(): boolean {
  return navigator.platform.toLowerCase().includes("win");
}

export function isMacOS(): boolean {
  return navigator.platform.toLowerCase().includes("mac");
}

export function isLinux(): boolean {
  return navigator.platform.toLowerCase().includes("linux");
}

export function getModifierKey(): string {
  return isMacOS() ? "⌘" : "Ctrl";
}

export function getPlatform(): "windows" | "macos" | "linux" | "unknown" {
  if (isWindows()) return "windows";
  if (isMacOS()) return "macos";
  if (isLinux()) return "linux";
  return "unknown";
}
