import { useCallback, useEffect, useState } from 'react'
import { cn } from '~/lib/utils'
import { TitlebarMenu } from './titlebar-menu'

// Dynamically import Tauri window API only in Tauri context
async function getTauriWindow() {
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    return getCurrentWindow()
  }
  catch {
    return null
  }
}

function WindowControls() {
  const [isTauri, setIsTauri] = useState(false)
  const [isMaximized, setIsMaximized] = useState(false)

  useEffect(() => {
    let cancelled = false
    let unlistenFn: (() => void) | undefined

    getTauriWindow().then(async (w) => {
      if (cancelled || !w)
        return
      setIsTauri(true)
      setIsMaximized(await w.isMaximized())

      // Listen for resize events to track maximize state
      unlistenFn = await w.onResized(async () => {
        if (cancelled)
          return
        setIsMaximized(await w.isMaximized())
      })
    })

    return () => {
      cancelled = true
      unlistenFn?.()
    }
  }, [])

  const handleMinimize = useCallback(async () => {
    const w = await getTauriWindow()
    w?.minimize()
  }, [])

  const handleMaximize = useCallback(async () => {
    const w = await getTauriWindow()
    w?.toggleMaximize()
  }, [])

  const handleClose = useCallback(async () => {
    const w = await getTauriWindow()
    w?.close()
  }, [])

  if (!isTauri)
    return null

  return (
    <div className="flex items-center">
      {/* Minimize */}
      <button
        type="button"
        onClick={handleMinimize}
        className={cn(
          'inline-flex h-8 w-[46px] items-center justify-center',
          'text-muted-foreground hover:bg-accent hover:text-accent-foreground',
          'transition-colors focus-visible:outline-none',
        )}
        aria-label="Minimize"
      >
        <svg width="10" height="1" viewBox="0 0 10 1" fill="currentColor">
          <rect width="10" height="1" />
        </svg>
      </button>

      {/* Maximize / Restore */}
      <button
        type="button"
        onClick={handleMaximize}
        className={cn(
          'inline-flex h-8 w-[46px] items-center justify-center',
          'text-muted-foreground hover:bg-accent hover:text-accent-foreground',
          'transition-colors focus-visible:outline-none',
        )}
        aria-label={isMaximized ? 'Restore' : 'Maximize'}
      >
        {isMaximized
          ? (
              // Restore icon (two overlapping rectangles)
              <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" strokeWidth="1">
                <rect x="2" y="3" width="7" height="7" rx="0.5" />
                <path d="M3 3V1.5C3 1.22 3.22 1 3.5 1H8.5C8.78 1 9 1.22 9 1.5V6.5C9 6.78 8.78 7 8.5 7H7" />
              </svg>
            )
          : (
              // Maximize icon (single rectangle)
              <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" strokeWidth="1">
                <rect x="1" y="1" width="8" height="8" rx="0.5" />
              </svg>
            )}
      </button>

      {/* Close */}
      <button
        type="button"
        onClick={handleClose}
        className={cn(
          'inline-flex h-8 w-[46px] items-center justify-center',
          'text-muted-foreground hover:bg-destructive hover:text-white',
          'transition-colors focus-visible:outline-none',
        )}
        aria-label="Close"
      >
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" strokeWidth="1.2">
          <path d="M1 1L9 9M9 1L1 9" />
        </svg>
      </button>
    </div>
  )
}

export function Titlebar() {
  return (
    <div
      data-tauri-drag-region
      className={cn(
        'flex h-8 shrink-0 select-none items-center justify-between',
        'border-b bg-sidebar',
      )}
    >
      {/* Left: app name + menu bar */}
      <div className="flex h-full items-center" data-tauri-drag-region>
        <span
          className="px-3 text-xs font-medium text-muted-foreground"
          data-tauri-drag-region
        >
          voletu
        </span>
        <TitlebarMenu />
      </div>

      {/* Right: window controls */}
      <WindowControls />
    </div>
  )
}
