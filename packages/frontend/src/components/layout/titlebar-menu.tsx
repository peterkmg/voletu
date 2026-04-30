import type { NavItem } from './types'
import type { TableDensity } from '~/components/data-table/density'
import { useNavigate } from '@tanstack/react-router'
import { Check } from 'lucide-react'
import { useCallback, useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { tableDensityOptions, useDensity } from '~/components/data-table/density'
import { ConfirmDialog } from '~/components/dialogs/confirm-dialog'
import { changeLanguagePreference, languageOptions } from '~/components/layout/actions/language-actions'
import { signOutAction } from '~/components/layout/actions/session-actions'
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
  DropdownMenuLabel,
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
import { toggleDevTools } from '~/lib/devtools'
import { cn } from '~/lib/utils'
import { settingsViewTarget } from '~/router/view-targets'
import { useStartupStore } from '~/stores/startup-store'
import { getSidebarData } from './data/sidebar-data'

let tauriWin: Awaited<
  ReturnType<typeof import('@tauri-apps/api/window').getCurrentWindow>
> | null = null

;(async () => {
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    tauriWin = getCurrentWindow()
  }
  catch { }
})()

function MenuTriggerButton({ children, className, ...props }: React.ComponentProps<'button'>) {
  return (
    <button
      type="button"
      className={cn(
        'h-full px-2 text-xs text-muted-foreground',
        'hover:bg-accent hover:text-accent-foreground',
        'rounded-sm transition-colors',
        'focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring',
        className,
      )}
      {...props}
    >
      {children}
    </button>
  )
}

const shortcutEntries = [
  { key: 'Ctrl+B', action: 'titlebar.toggleSidebar' },
  { key: 'Ctrl+,', action: 'titlebar.settings' },
  { key: 'F11', action: 'titlebar.fullscreen' },
  { key: 'Ctrl+Q', action: 'titlebar.quit' },
] as const

const debugShortcutEntries = [
  { key: 'Ctrl+Shift+D', action: 'debug.toggleDevTools' },
] as const

const titlebarDensityLabelKeys: Record<TableDensity, string> = {
  compact: 'titlebar.densityCompact',
  normal: 'titlebar.densityNormal',
  comfortable: 'titlebar.densityComfortable',
}

function FileMenu() {
  const { t, i18n } = useTranslation()
  const navigate = useNavigate()
  const isTauri = useStartupStore(s => s.startupState !== null)
  const [signOutOpen, setSignOutOpen] = useState(false)

  const handleSignOut = useCallback(() => {
    signOutAction(navigate)
  }, [navigate])

  const handleQuit = useCallback(() => {
    tauriWin?.close()
  }, [])

  const switchLanguage = useCallback((lang: string) => {
    changeLanguagePreference(i18n, lang)
  }, [i18n])

  return (
    <>
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <MenuTriggerButton>{t('titlebar.file')}</MenuTriggerButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="min-w-48">
          <DropdownMenuItem onSelect={() => navigate(settingsViewTarget)}>
            {t('titlebar.settings')}
            <DropdownMenuShortcut>Ctrl+,</DropdownMenuShortcut>
          </DropdownMenuItem>

          <DropdownMenuSub>
            <DropdownMenuSubTrigger>
              {t('titlebar.language')}
            </DropdownMenuSubTrigger>
            <DropdownMenuSubContent>
              {languageOptions.map(lang => (
                <DropdownMenuItem
                  key={lang.code}
                  onSelect={() => switchLanguage(lang.code)}
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
            onSelect={() => setSignOutOpen(true)}
          >
            {t('titlebar.signOut')}
          </DropdownMenuItem>

          {isTauri && (
            <>
              <DropdownMenuSeparator />
              <DropdownMenuItem onSelect={handleQuit}>
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

function ViewMenu() {
  const { t } = useTranslation()
  const { theme, setTheme } = useTheme()
  const { density, setDensity } = useDensity()
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

  const handleDensityChange = useCallback((value: string) => {
    setDensity(value as TableDensity)
  }, [setDensity])

  return (
    <DropdownMenu modal={false}>
      <DropdownMenuTrigger asChild>
        <MenuTriggerButton>{t('titlebar.view')}</MenuTriggerButton>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="min-w-48">
        <DropdownMenuItem onSelect={handleToggleSidebar}>
          {t('titlebar.toggleSidebar')}
          <DropdownMenuShortcut>Ctrl+B</DropdownMenuShortcut>
        </DropdownMenuItem>

        {isTauri && (
          <DropdownMenuItem onSelect={handleFullscreen}>
            {t('titlebar.fullscreen')}
            <DropdownMenuShortcut>F11</DropdownMenuShortcut>
          </DropdownMenuItem>
        )}

        <DropdownMenuSeparator />

        <DropdownMenuLabel>{t('titlebar.density')}</DropdownMenuLabel>
        <DropdownMenuRadioGroup value={density} onValueChange={handleDensityChange}>
          {tableDensityOptions.map(({ value, labelKey }) => (
            <DropdownMenuRadioItem key={value} value={value}>
              {t(titlebarDensityLabelKeys[value] ?? labelKey)}
            </DropdownMenuRadioItem>
          ))}
        </DropdownMenuRadioGroup>

        <DropdownMenuSeparator />

        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            {t('titlebar.themeMenu')}
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem onSelect={() => setTheme('light')}>
              {t('theme.light')}
              <Check
                size={14}
                className={cn('ms-auto', theme !== 'light' && 'hidden')}
              />
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => setTheme('dark')}>
              {t('theme.dark')}
              <Check
                size={14}
                className={cn('ms-auto', theme !== 'dark' && 'hidden')}
              />
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => setTheme('system')}>
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
                    onSelect={() => go(item.url)}
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
                          onSelect={() => go(sub.url)}
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

function HelpMenu() {
  const { t } = useTranslation()
  const isDebugBuild = useStartupStore(s => s.startupState?.isDebugBuild ?? false)
  const [shortcutsOpen, setShortcutsOpen] = useState(false)
  const [aboutOpen, setAboutOpen] = useState(false)

  return (
    <>
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <MenuTriggerButton>{t('titlebar.help')}</MenuTriggerButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="min-w-48">
          <DropdownMenuItem onSelect={() => setShortcutsOpen(true)}>
            {t('titlebar.keyboardShortcuts')}
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem onSelect={() => setAboutOpen(true)}>
            {t('titlebar.aboutVoletu')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

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
            {isDebugBuild && debugShortcutEntries.map(s => (
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

function useGlobalShortcuts() {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const isTauri = useStartupStore(s => s.startupState !== null)
  const isDebugBuild = useStartupStore(s => s.startupState?.isDebugBuild ?? false)

  useEffect(() => {
    function onKeyDown(e: KeyboardEvent) {
      if (e.key === ',' && (e.ctrlKey || e.metaKey)) {
        e.preventDefault()
        navigate(settingsViewTarget)
        return
      }

      if (e.key === 'q' && (e.ctrlKey || e.metaKey) && isTauri) {
        e.preventDefault()
        tauriWin?.close()
        return
      }

      if (e.key === 'F11' && isTauri) {
        e.preventDefault()
        tauriWin?.isFullscreen().then(isFull => tauriWin?.setFullscreen(!isFull))
        return
      }

      if (e.key === 'D' && e.shiftKey && (e.ctrlKey || e.metaKey) && isDebugBuild) {
        e.preventDefault()
        const next = toggleDevTools()
        toast.info(next ? t('debug.devToolsEnabled') : t('debug.devToolsDisabled'))
      }
    }

    window.addEventListener('keydown', onKeyDown)

    return () => window.removeEventListener('keydown', onKeyDown)
  }, [isDebugBuild, isTauri, navigate, t])
}

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
