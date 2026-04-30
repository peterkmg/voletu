import type { Row } from '@tanstack/react-table'
import '@tanstack/react-table'

declare module '@tanstack/react-table' {

  interface ColumnMeta<TData, TValue> {
    className?: string
    tdClassName?: string
    thClassName?: string
    align?: 'left' | 'center' | 'right'

    label?: string

    exportValue?: (row: TData, tableRow: Row<TData>, columnId: string) => unknown
    filterType?: 'text' | 'date' | 'number' | 'enum'
    enableHeaderFilter?: boolean

    requiresRole?: string

    sizingCategory?: 'fixed' | 'capped' | 'flex'

    groupRole?: 'doc' | 'item'
  }
}
