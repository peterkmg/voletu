import type { EntityItem } from './entity-picker-types'

export function entityItemKeywords(item: EntityItem): string[] {
  return [item.label, item.secondary].filter(Boolean) as string[]
}
