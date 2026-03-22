import { Link } from '@tanstack/react-router'
import { Layers } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from '~/components/ui/sidebar'
import { useNodeStore } from '~/stores/node-store'
import { cn } from '~/lib/utils'

export function AppTitle() {
  const { t } = useTranslation('system')
  const { setOpenMobile, state } = useSidebar()
  const status = useNodeStore(s => s.status)
  const isCollapsed = state === 'collapsed'

  const isPeripheral = status.nodeType === 'PERIPHERAL'
  const isOnline = status.workerState === 'OnlineIdle'
    || status.workerState === 'Syncing'

  const subtitle = status.isInitialized && status.nodeName
    ? `${status.nodeType} · ${status.nodeName}`
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
              {isPeripheral && status.isInitialized && isCollapsed && (
                <span
                  className={cn(
                    'absolute -right-0.5 -top-0.5 size-2.5 rounded-full border-2 border-sidebar',
                    isOnline ? 'bg-green-500' : 'bg-red-500',
                  )}
                />
              )}
            </div>
            <div className="grid flex-1 text-start text-sm leading-tight">
              <span className="truncate font-bold">Voletu</span>
              <span className="truncate text-xs text-muted-foreground">{subtitle}</span>
              {isPeripheral && status.isInitialized && !isCollapsed && (
                <span className="flex items-center gap-1">
                  <span
                    className={cn(
                      'size-2 rounded-full',
                      isOnline ? 'bg-green-500' : 'bg-red-500',
                    )}
                  />
                  <span className="text-[10px] text-muted-foreground">
                    {isOnline
                      ? t('node.status.online')
                      : t('node.status.offline')}
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
