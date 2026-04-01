import { useTranslation } from 'react-i18next'
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail,
} from '~/components/ui/sidebar'
import { useAuthStore } from '~/stores/auth-store'
import { AppTitle } from './app-title'
import { getSidebarData } from './data/sidebar-data'
import { NavGroup } from './nav-group'
import { NavUser } from './nav-user'
import { filterNavByRole } from './sidebar-utils'

export function AppSidebar() {
  const { t } = useTranslation()
  const sidebarData = getSidebarData(t)
  const user = useAuthStore(s => s.user)
  const filteredGroups = filterNavByRole(sidebarData.navGroups, user?.role)

  return (
    <Sidebar collapsible="icon" variant="inset">
      <SidebarHeader>
        <AppTitle />
      </SidebarHeader>
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
