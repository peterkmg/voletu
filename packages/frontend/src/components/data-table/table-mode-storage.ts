import type { TableMode } from './table-mode-toggle'

export function getStoredTableMode(tableId: string | undefined): TableMode {
  if (!tableId)
    return 'virtual'

  try {
    const stored = localStorage.getItem(`table-mode-${tableId}`)
    if (stored === 'paginated' || stored === 'virtual')
      return stored
  }
  catch {
    /* ignore */
  }

  return 'virtual'
}
