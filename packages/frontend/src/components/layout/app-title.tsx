import { Link } from '@tanstack/react-router'
import { Layers } from 'lucide-react'
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from '~/components/ui/sidebar'

export function AppTitle() {
  const { setOpenMobile } = useSidebar()
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
            <div className="flex aspect-square size-9 items-center justify-center rounded-lg bg-sidebar-accent text-sidebar-primary">
              <Layers className="size-5" />
            </div>
            <div className="grid flex-1 text-start text-sm leading-tight">
              <span className="truncate font-bold">Voletu</span>
              <span className="truncate text-xs text-muted-foreground">Inventory Management</span>
            </div>
          </Link>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}
