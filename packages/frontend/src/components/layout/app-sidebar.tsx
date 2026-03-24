import { useNavigate } from '@tanstack/react-router'
import { Settings } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
  SidebarRail,
} from '~/components/ui/sidebar'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'
import { AppTitle } from './app-title'
import { getSidebarData } from './data/sidebar-data'
import { NavGroup } from './nav-group'
import { NavUser } from './nav-user'
import { filterNavByRole } from './sidebar-utils'

function SidebarInitAction() {
  const { t } = useTranslation('system')
  const status = useNodeStore(s => s.status)
  const user = useAuthStore(s => s.auth.user)
  const navigate = useNavigate()

  if (status.isInitialized || user?.role !== 'ADMIN')
    return null

  return (
    <SidebarMenu className="px-2 py-1">
      <SidebarMenuItem>
        <Button
          variant="outline"
          size="sm"
          className="w-full border-amber-300 bg-amber-50 text-amber-800 hover:bg-amber-100 dark:border-amber-700 dark:bg-amber-950 dark:text-amber-200 dark:hover:bg-amber-900"
          onClick={() => navigate({ to: '/system/init' })}
        >
          <Settings className="mr-2 size-4" />
          <span className="group-data-[collapsible=icon]:hidden">
            {t('node.initialize')}
          </span>
        </Button>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}

export function AppSidebar() {
  const { t } = useTranslation()
  const sidebarData = getSidebarData(t)
  const user = useAuthStore(s => s.auth.user)
  const filteredGroups = filterNavByRole(sidebarData.navGroups, user?.role)

  return (
    <Sidebar collapsible="icon">
      <SidebarHeader>
        <AppTitle />
      </SidebarHeader>
      <SidebarInitAction />
      <SidebarContent>
        {filteredGroups.map(props => (
          <NavGroup key={props.title} {...props} />
        ))}
      </SidebarContent>
      <SidebarFooter>
        <NavUser />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
