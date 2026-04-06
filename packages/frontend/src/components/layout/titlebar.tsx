import { useCallback, useEffect, useState } from 'react'
import { cn } from '~/lib/utils'
import { TitlebarMenu } from './titlebar-menu'

// Lazily resolved + cached Tauri window handle
let cachedWindow: Awaited<
  ReturnType<typeof import('@tauri-apps/api/window').getCurrentWindow>
> | null = null

async function getTauriWindow() {
  if (cachedWindow)
    return cachedWindow
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    cachedWindow = getCurrentWindow()
    return cachedWindow
  }
  catch {
    return null
  }
}

// Eagerly kick off resolution
getTauriWindow()

const controlBase = cn(
  'inline-flex h-full w-12 items-center justify-center',
  'text-foreground/60 transition-colors duration-150',
  'focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring',
)

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

  const handleMinimize = useCallback(() => {
    cachedWindow?.minimize()
  }, [])

  const handleMaximize = useCallback(() => {
    cachedWindow?.toggleMaximize()
  }, [])

  const handleClose = useCallback(() => {
    cachedWindow?.close()
  }, [])

  if (!isTauri)
    return null

  return (
    <div className="flex h-full items-center">
      {/* Minimize */}
      <button
        type="button"
        onClick={handleMinimize}
        className={cn(controlBase, 'hover:bg-foreground/8')}
        aria-label="Minimize"
      >
        <svg width="10" height="1" viewBox="0 0 10 1" className="fill-current">
          <rect width="10" height="1" rx="0.5" />
        </svg>
      </button>

      {/* Maximize / Restore */}
      <button
        type="button"
        onClick={handleMaximize}
        className={cn(controlBase, 'hover:bg-foreground/8')}
        aria-label={isMaximized ? 'Restore' : 'Maximize'}
      >
        {isMaximized
          ? (
              <svg
                width="10"
                height="10"
                viewBox="0 0 10 10"
                className="fill-none stroke-current"
                strokeWidth="1.1"
              >
                <rect x="0.5" y="2.5" width="7" height="7" rx="1" />
                <path d="M2.5 2.5V1.25C2.5 0.84 2.84 0.5 3.25 0.5h5.5C9.16 0.5 9.5 0.84 9.5 1.25v5.5c0 .41-.34.75-.75.75H7.5" />
              </svg>
            )
          : (
              <svg
                width="10"
                height="10"
                viewBox="0 0 10 10"
                className="fill-none stroke-current"
                strokeWidth="1.1"
              >
                <rect x="0.5" y="0.5" width="9" height="9" rx="1" />
              </svg>
            )}
      </button>

      {/* Close */}
      <button
        type="button"
        onClick={handleClose}
        className={cn(
          controlBase,
          'hover:bg-red-500 hover:text-white',
        )}
        aria-label="Close"
      >
        <svg
          width="10"
          height="10"
          viewBox="0 0 10 10"
          className="fill-none stroke-current"
          strokeWidth="1.3"
          strokeLinecap="round"
        >
          <path d="M1 1l8 8M9 1l-8 8" />
        </svg>
      </button>
    </div>
  )
}

function PageTitleDisplay() {
  const [title, setTitle] = useState(document.title)

  useEffect(() => {
    // Observe document.title changes via MutationObserver on <title> element
    const titleEl = document.querySelector('title')
    if (!titleEl) return

    const observer = new MutationObserver(() => setTitle(document.title))
    observer.observe(titleEl, { childList: true })
    return () => observer.disconnect()
  }, [])

  return (
    <span className="pointer-events-none text-xs text-muted-foreground" data-tauri-drag-region>
      {title}
    </span>
  )
}

export function Titlebar() {
  return (
    <div
      className={cn(
        'flex h-8 shrink-0 select-none items-center',
        'border-b bg-sidebar',
      )}
    >
      {/* Left: app name + menu bar */}
      <div className="flex h-full items-center">
        <span className="px-3 text-xs font-medium tracking-wide text-muted-foreground">
          voletu
        </span>
        <TitlebarMenu />
      </div>

      {/* Center: dynamic page title + drag zone */}
      <div className="flex flex-1 items-center justify-center self-stretch" data-tauri-drag-region>
        <PageTitleDisplay />
      </div>

      {/* Right: window controls */}
      <WindowControls />
    </div>
  )
}
