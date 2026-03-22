import type { LucideIcon } from 'lucide-react'

interface BaseNavItem {
  title: string
  badge?: string
  icon?: LucideIcon
  roles?: string[]
}

type NavLink = BaseNavItem & {
  url: string
  items?: never
}

type NavCollapsible = BaseNavItem & {
  items: (BaseNavItem & { url: string })[]
  url?: never
}

type NavItem = NavLink | NavCollapsible

interface NavGroup {
  title: string
  items: NavItem[]
  roles?: string[]
}

interface SidebarData {
  navGroups: NavGroup[]
}

export type { NavCollapsible, NavGroup, NavItem, NavLink, SidebarData }
