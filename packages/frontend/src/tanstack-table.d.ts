import '@tanstack/react-table'

declare module '@tanstack/react-table' {

  interface ColumnMeta<TData, TValue> {
    className?: string
    tdClassName?: string
    thClassName?: string
    align?: 'left' | 'center' | 'right'
    /** Localized display name for view-options dropdown. */
    label?: string
    filterType?: 'text' | 'date' | 'number' | 'enum'
    enableHeaderFilter?: boolean
    /** Role required to see this column in view-options dropdown. */
    requiresRole?: string
  }
}
