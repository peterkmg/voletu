import type React from 'react'
import { useCallback, useEffect, useState } from 'react'

interface TauriWindowHandle {
  startDragging: () => Promise<void>
  toggleMaximize: () => Promise<void>
}

/** Module-level cache — resolved once, shared by every hook instance. */
let win: TauriWindowHandle | null | undefined // undefined = pending

async function resolve(): Promise<TauriWindowHandle | null> {
  if (win !== undefined) return win
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    win = getCurrentWindow()
  } catch {
    win = null
  }
  return win
}

// Kick off resolution at module load (no-op in a plain browser).
resolve()

/** Elements that must NOT initiate a window drag. */
const NO_DRAG = 'a,button,input,select,textarea,[role="button"],[data-no-drag]'

/**
 * Returns props to spread onto elements that should act as window-drag regions.
 *
 * - Interactive children (`<a>`, `<button>`, etc.) are automatically excluded.
 * - Elements (and their children) with `data-no-drag` are excluded.
 * - Double-clicking toggles maximise (via `e.detail === 2`).
 * - Returns `{}` when the app is not running inside Tauri.
 */
export function useDragWindow() {
  const [ready, setReady] = useState(win != null)

  useEffect(() => {
    if (win === undefined) {
      resolve().then(w => {
        if (w) setReady(true)
      })
    }
  }, [])

  // Single mousedown handler — uses e.detail to distinguish
  // single-click (drag) from double-click (toggle maximise).
  const onMouseDown = useCallback((e: React.MouseEvent) => {
    if (!win || e.buttons !== 1) return
    if ((e.target as HTMLElement).closest(NO_DRAG)) return
    if (e.detail === 2) {
      win.toggleMaximize()
    } else {
      win.startDragging()
    }
  }, [])

  if (!ready) return {}

  return {
    onMouseDown,
    'data-tauri-drag-region': true,
  } as const
}
