import type { SyncUiState } from '~/views/system/sync/sync-ui-state'
import { Link } from '@tanstack/react-router'
import { Layers } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from '~/components/ui/sidebar'
import { cn } from '~/lib/utils'
import { useNodeStore } from '~/stores/node-store'
import { deriveSyncUiState } from '~/views/system/sync/sync-ui-state'

const syncStateConfig: Record<SyncUiState, { dot: string, label: string }> = {
  central: { dot: 'bg-muted-foreground/50', label: 'node.syncState.central' },
  online: { dot: 'bg-green-500', label: 'node.syncState.online' },
  syncing: { dot: 'bg-green-500', label: 'node.syncState.syncing' },
  setupIncomplete: { dot: 'bg-yellow-500', label: 'node.syncState.setupNeeded' },
  offline: { dot: 'bg-red-500', label: 'node.syncState.offline' },
}

export function AppTitle() {
  const { t } = useTranslation('system')
  const { setOpenMobile, state } = useSidebar()
  const status = useNodeStore(s => s.status)
  const basesLoaded = useNodeStore(s => s.basesLoaded)
  const isCollapsed = state === 'collapsed'

  const syncState = deriveSyncUiState(status, basesLoaded)
  const { dot: dotColor, label: labelKey } = syncStateConfig[syncState]

  const subtitle = status.isInitialized && status.nodeName
    ? `${t('node.label')}: ${status.nodeName}`
    : 'Inventory Management'

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <SidebarMenuButton
          size="lg"
          className="gap-0 py-0 hover:bg-transparent active:bg-transparent"
          asChild
        >
          <Link
            to="/"
            onClick={() => setOpenMobile(false)}
            className="flex items-center gap-2"
          >
            <div className="relative flex aspect-square size-9 items-center justify-center rounded-lg bg-sidebar-accent text-sidebar-primary">
              <Layers className="size-5" />
              {status.isInitialized && isCollapsed && (
                <span
                  className={cn(
                    'absolute -right-0.5 -top-0.5 size-2.5 rounded-full border-2 border-sidebar',
                    dotColor,
                  )}
                />
              )}
            </div>
            <div className="grid flex-1 text-start text-sm leading-tight">
              <span className="truncate font-bold">Voletu</span>
              <span className="truncate text-xs text-muted-foreground">{subtitle}</span>
              {status.isInitialized && !isCollapsed && (
                <span className="flex items-center gap-1.5">
                  <span className={cn('size-2 rounded-full', dotColor)} />
                  <span className="text-xs text-muted-foreground">
                    {t(labelKey)}
                  </span>
                </span>
              )}
            </div>
          </Link>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}
