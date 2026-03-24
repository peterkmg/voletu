import type { NavItem } from './types'
import type { TableDensity } from '~/components/data-table/density-context'
import { useNavigate } from '@tanstack/react-router'
import { Check } from 'lucide-react'
import { useCallback, useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { ConfirmDialog } from '~/components/confirm-dialog'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '~/components/ui/dialog'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { useTheme } from '~/context/theme-provider'
import { cn } from '~/lib/utils'
import { useAuthStore } from '~/stores/auth-store'
import { useStartupStore } from '~/stores/startup-store'
import { getSidebarData } from './data/sidebar-data'

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

const DENSITY_STORAGE_KEY = 'voletu.table-density'

const languages = [
  { code: 'en', label: 'English' },
  { code: 'ru', label: 'Русский' },
] as const

function useDensityDirect() {
  // eslint-disable-next-line react-naming-convention/use-state -- mirrors DensityProvider naming
  const [density, setDensityState] = useState<TableDensity>(
    () => (localStorage.getItem(DENSITY_STORAGE_KEY) as TableDensity) || 'normal',
  )

  const setDensity = useCallback((d: TableDensity) => {
    setDensityState(d)
    localStorage.setItem(DENSITY_STORAGE_KEY, d)
    window.dispatchEvent(new StorageEvent('storage', {
      key: DENSITY_STORAGE_KEY,
      newValue: d,
    }))
  }, [])

  return { density, setDensity }
}

/** Cached Tauri window handle for fullscreen / quit. */
let tauriWin: Awaited<
  ReturnType<typeof import('@tauri-apps/api/window').getCurrentWindow>
> | null = null

;(async () => {
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    tauriWin = getCurrentWindow()
  }
  catch { /* not in Tauri */ }
})()

function MenuTriggerButton({ children }: { children: React.ReactNode }) {
  return (
    <button
      type="button"
      className={cn(
        'h-full px-2 text-xs text-muted-foreground',
        'hover:bg-accent hover:text-accent-foreground',
        'rounded-sm transition-colors',
        'focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring',
      )}
    >
      {children}
    </button>
  )
}

// ---------------------------------------------------------------------------
// Keyboard shortcuts data
// ---------------------------------------------------------------------------

const shortcutEntries = [
  { key: 'Ctrl+B', action: 'titlebar.toggleSidebar' },
  { key: 'Ctrl+,', action: 'titlebar.settings' },
  { key: 'F11', action: 'titlebar.fullscreen' },
  { key: 'Ctrl+Q', action: 'titlebar.quit' },
] as const

// ---------------------------------------------------------------------------
// File menu
// ---------------------------------------------------------------------------

function FileMenu() {
  const { t, i18n } = useTranslation()
  const navigate = useNavigate()
  const isTauri = useStartupStore(s => s.startupState !== null)
  const [signOutOpen, setSignOutOpen] = useState(false)

  const handleSignOut = useCallback(() => {
    useAuthStore.getState().auth.clearSession()
    navigate({ to: '/sign-in' })
  }, [navigate])

  const handleQuit = useCallback(() => {
    tauriWin?.close()
  }, [])

  const switchLanguage = useCallback((lang: string) => {
    i18n.changeLanguage(lang)
    localStorage.setItem('voletu.language', lang)
  }, [i18n])

  return (
    <>
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <MenuTriggerButton>{t('titlebar.file')}</MenuTriggerButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="min-w-48">
          <DropdownMenuItem onClick={() => navigate({ to: '/settings' })}>
            {t('titlebar.settings')}
            <DropdownMenuShortcut>Ctrl+,</DropdownMenuShortcut>
          </DropdownMenuItem>

          <DropdownMenuSub>
            <DropdownMenuSubTrigger>
              {t('titlebar.language')}
            </DropdownMenuSubTrigger>
            <DropdownMenuSubContent>
              {languages.map(lang => (
                <DropdownMenuItem
                  key={lang.code}
                  onClick={() => switchLanguage(lang.code)}
                >
                  {lang.label}
                  <Check
                    size={14}
                    className={cn('ms-auto', i18n.language !== lang.code && 'hidden')}
                  />
                </DropdownMenuItem>
              ))}
            </DropdownMenuSubContent>
          </DropdownMenuSub>

          <DropdownMenuSeparator />

          <DropdownMenuItem
            variant="destructive"
            onClick={() => setSignOutOpen(true)}
          >
            {t('titlebar.signOut')}
          </DropdownMenuItem>

          {isTauri && (
            <>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={handleQuit}>
                {t('titlebar.quit')}
                <DropdownMenuShortcut>Ctrl+Q</DropdownMenuShortcut>
              </DropdownMenuItem>
            </>
          )}
        </DropdownMenuContent>
      </DropdownMenu>

      <ConfirmDialog
        open={signOutOpen}
        onOpenChange={setSignOutOpen}
        title={t('titlebar.signOut')}
        description={t('titlebar.signOutConfirm')}
        confirmLabel={t('titlebar.signOut')}
        cancelLabel={t('actions.cancel')}
        variant="destructive"
        onConfirm={handleSignOut}
      />
    </>
  )
}

// ---------------------------------------------------------------------------
// View menu
// ---------------------------------------------------------------------------

function ViewMenu() {
  const { t } = useTranslation()
  const { theme, setTheme } = useTheme()
  const { density, setDensity } = useDensityDirect()
  const isTauri = useStartupStore(s => s.startupState !== null)

  const handleToggleSidebar = useCallback(() => {
    window.dispatchEvent(
      new KeyboardEvent('keydown', {
        key: 'b',
        code: 'KeyB',
        ctrlKey: true,
        bubbles: true,
        cancelable: true,
      }),
    )
  }, [])

  const handleFullscreen = useCallback(async () => {
    if (!tauriWin)
      return
    const isFull = await tauriWin.isFullscreen()
    await tauriWin.setFullscreen(!isFull)
  }, [])

  return (
    <DropdownMenu modal={false}>
      <DropdownMenuTrigger asChild>
        <MenuTriggerButton>{t('titlebar.view')}</MenuTriggerButton>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="min-w-48">
        <DropdownMenuItem onClick={handleToggleSidebar}>
          {t('titlebar.toggleSidebar')}
          <DropdownMenuShortcut>Ctrl+B</DropdownMenuShortcut>
        </DropdownMenuItem>

        {isTauri && (
          <DropdownMenuItem onClick={handleFullscreen}>
            {t('titlebar.fullscreen')}
            <DropdownMenuShortcut>F11</DropdownMenuShortcut>
          </DropdownMenuItem>
        )}

        <DropdownMenuSeparator />

        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            {t('titlebar.density')}
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuRadioGroup
              value={density}
              onValueChange={v => setDensity(v as TableDensity)}
            >
              <DropdownMenuRadioItem value="compact">
                {t('titlebar.densityCompact')}
              </DropdownMenuRadioItem>
              <DropdownMenuRadioItem value="normal">
                {t('titlebar.densityNormal')}
              </DropdownMenuRadioItem>
              <DropdownMenuRadioItem value="comfortable">
                {t('titlebar.densityComfortable')}
              </DropdownMenuRadioItem>
            </DropdownMenuRadioGroup>
          </DropdownMenuSubContent>
        </DropdownMenuSub>

        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            {t('titlebar.themeMenu')}
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem onClick={() => setTheme('light')}>
              {t('theme.light')}
              <Check
                size={14}
                className={cn('ms-auto', theme !== 'light' && 'hidden')}
              />
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => setTheme('dark')}>
              {t('theme.dark')}
              <Check
                size={14}
                className={cn('ms-auto', theme !== 'dark' && 'hidden')}
              />
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => setTheme('system')}>
              {t('theme.system')}
              <Check
                size={14}
                className={cn('ms-auto', theme !== 'system' && 'hidden')}
              />
            </DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

// ---------------------------------------------------------------------------
// Go menu — generated from sidebar navigation data
// ---------------------------------------------------------------------------

function GoMenu() {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const sidebarData = getSidebarData(t)

  const go = useCallback(
    (url: string) => navigate({ to: url }),
    [navigate],
  )

  return (
    <DropdownMenu modal={false}>
      <DropdownMenuTrigger asChild>
        <MenuTriggerButton>{t('titlebar.go')}</MenuTriggerButton>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="min-w-48">
        {sidebarData.navGroups.map((group, gi) => (
          <div key={group.title}>
            {gi > 0 && <DropdownMenuSeparator />}
            {group.items.map((item: NavItem) => {
              if ('url' in item && item.url) {
                return (
                  <DropdownMenuItem
                    key={item.url}
                    onClick={() => go(item.url)}
                  >
                    {item.title}
                  </DropdownMenuItem>
                )
              }
              if ('items' in item && item.items) {
                return (
                  <DropdownMenuSub key={item.title}>
                    <DropdownMenuSubTrigger>
                      {item.title}
                    </DropdownMenuSubTrigger>
                    <DropdownMenuSubContent>
                      {item.items.map(sub => (
                        <DropdownMenuItem
                          key={sub.url}
                          onClick={() => go(sub.url)}
                        >
                          {sub.title}
                        </DropdownMenuItem>
                      ))}
                    </DropdownMenuSubContent>
                  </DropdownMenuSub>
                )
              }
              return null
            })}
          </div>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

// ---------------------------------------------------------------------------
// Help menu
// ---------------------------------------------------------------------------

function HelpMenu() {
  const { t } = useTranslation()
  const [shortcutsOpen, setShortcutsOpen] = useState(false)
  const [aboutOpen, setAboutOpen] = useState(false)

  return (
    <>
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <MenuTriggerButton>{t('titlebar.help')}</MenuTriggerButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="min-w-48">
          <DropdownMenuItem onClick={() => setShortcutsOpen(true)}>
            {t('titlebar.keyboardShortcuts')}
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem onClick={() => setAboutOpen(true)}>
            {t('titlebar.aboutVoletu')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      {/* Keyboard Shortcuts dialog */}
      <Dialog open={shortcutsOpen} onOpenChange={setShortcutsOpen}>
        <DialogContent className="sm:max-w-sm">
          <DialogHeader>
            <DialogTitle>{t('titlebar.keyboardShortcuts')}</DialogTitle>
            <DialogDescription className="sr-only">
              {t('titlebar.keyboardShortcuts')}
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-2 py-2">
            {shortcutEntries.map(s => (
              <div
                key={s.key}
                className="flex items-center justify-between text-sm"
              >
                <span className="text-muted-foreground">{t(s.action)}</span>
                <kbd className="rounded border bg-muted px-2 py-0.5 font-mono text-xs">
                  {s.key}
                </kbd>
              </div>
            ))}
          </div>
        </DialogContent>
      </Dialog>

      {/* About dialog */}
      <Dialog open={aboutOpen} onOpenChange={setAboutOpen}>
        <DialogContent className="sm:max-w-xs">
          <DialogHeader>
            <DialogTitle>{t('titlebar.aboutVoletu')}</DialogTitle>
            <DialogDescription>
              {t('titlebar.aboutDescription')}
            </DialogDescription>
          </DialogHeader>
          <p className="text-sm text-muted-foreground">
            {t('titlebar.aboutVersion', { version: '0.1.0' })}
          </p>
        </DialogContent>
      </Dialog>
    </>
  )
}

// ---------------------------------------------------------------------------
// Global keyboard shortcuts (Ctrl+, / Ctrl+Q / F11)
// ---------------------------------------------------------------------------

function useGlobalShortcuts() {
  const navigate = useNavigate()
  const isTauri = useStartupStore(s => s.startupState !== null)

  useEffect(() => {
    function onKeyDown(e: KeyboardEvent) {
      // Ctrl+, → Settings
      if (e.key === ',' && (e.ctrlKey || e.metaKey)) {
        e.preventDefault()
        navigate({ to: '/settings' })
        return
      }

      // Ctrl+Q → Quit (Tauri only)
      if (e.key === 'q' && (e.ctrlKey || e.metaKey) && isTauri) {
        e.preventDefault()
        tauriWin?.close()
        return
      }

      // F11 → Toggle fullscreen (Tauri only)
      if (e.key === 'F11' && isTauri) {
        e.preventDefault()
        tauriWin?.isFullscreen().then(isFull => tauriWin?.setFullscreen(!isFull))
      }
    }

    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
  }, [isTauri, navigate])
}

// ---------------------------------------------------------------------------
// Exported composite
// ---------------------------------------------------------------------------

export function TitlebarMenu() {
  useGlobalShortcuts()

  return (
    <div className="flex items-center">
      <FileMenu />
      <ViewMenu />
      <GoMenu />
      <HelpMenu />
    </div>
  )
}
