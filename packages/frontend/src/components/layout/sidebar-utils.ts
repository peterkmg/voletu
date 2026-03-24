import type { NavGroup } from './types'

export function filterNavByRole(groups: NavGroup[], userRole: string | undefined): NavGroup[] {
  if (!userRole)
    return []

  return groups
    .filter(group => !group.roles || group.roles.includes(userRole))
    .map(group => ({
      ...group,
      items: group.items
        .filter(item => !item.roles || item.roles.includes(userRole))
        .map((item) => {
          if ('items' in item && item.items) {
            return {
              ...item,
              items: item.items.filter(
                sub => !sub.roles || sub.roles.includes(userRole),
              ),
            }
          }
          return item
        })
        .filter(item => !('items' in item && item.items) || (item as any).items.length > 0),
    }))
    .filter(group => group.items.length > 0)
}
