const MIN_ZOOM = 0.5;
const MAX_ZOOM = 3.0;
const ZOOM_STEP = 0.1;
const DEFAULT_ZOOM = 1.0;

export function clampZoom(zoom: number): number {
  return Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoom));
}

export function zoomIn(current: number): number {
  return clampZoom(current + ZOOM_STEP);
}

export function zoomOut(current: number): number {
  return clampZoom(current - ZOOM_STEP);
}

export function resetZoom(): number {
  return DEFAULT_ZOOM;
}

export function handleWheelZoom(
  event: WheelEvent,
  currentZoom: number,
): number | null {
  if (event.ctrlKey || event.metaKey) {
    event.preventDefault();
    const delta = event.deltaY > 0 ? -ZOOM_STEP : ZOOM_STEP;
    return clampZoom(currentZoom + delta);
  }
  return null;
}

export { MIN_ZOOM, MAX_ZOOM, ZOOM_STEP, DEFAULT_ZOOM };
