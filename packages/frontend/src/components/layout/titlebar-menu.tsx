import type { TableDensity } from '~/components/data-table/density-context'
import { Check } from 'lucide-react'
import { useCallback, useState } from 'react'
import { useTranslation } from 'react-i18next'
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

const DENSITY_STORAGE_KEY = 'voletu.table-density'

function useDensityDirect() {
  // eslint-disable-next-line react-naming-convention/use-state -- mirrors DensityProvider naming
  const [density, setDensityState] = useState<TableDensity>(
    () => (localStorage.getItem(DENSITY_STORAGE_KEY) as TableDensity) || 'normal',
  )

  const setDensity = useCallback((d: TableDensity) => {
    setDensityState(d)
    localStorage.setItem(DENSITY_STORAGE_KEY, d)
    // Dispatch a storage event so DensityProvider picks up the change
    window.dispatchEvent(new StorageEvent('storage', {
      key: DENSITY_STORAGE_KEY,
      newValue: d,
    }))
  }, [])

  return { density, setDensity }
}

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

export function TitlebarMenu() {
  const { t } = useTranslation()
  const { theme, setTheme } = useTheme()
  const { density, setDensity } = useDensityDirect()

  const handleToggleSidebar = useCallback(() => {
    // Dispatch Ctrl+B on window to trigger the SidebarProvider's keydown handler
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

  return (
    <div className="flex items-center">
      {/* View menu */}
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <MenuTriggerButton>{t('titlebar.view')}</MenuTriggerButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="min-w-48">
          <DropdownMenuItem onClick={handleToggleSidebar}>
            {t('titlebar.toggleSidebar')}
            <DropdownMenuShortcut>Ctrl+B</DropdownMenuShortcut>
          </DropdownMenuItem>

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

          <DropdownMenuSeparator />

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

      {/* Help menu */}
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <MenuTriggerButton>{t('titlebar.help')}</MenuTriggerButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="min-w-48">
          <DropdownMenuItem>
            {t('titlebar.aboutVoletu')}
          </DropdownMenuItem>
          <DropdownMenuItem>
            {t('titlebar.keyboardShortcuts')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  )
}
