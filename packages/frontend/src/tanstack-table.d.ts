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
    /** Controls how CSS Grid distributes width to this column.
     *  - 'fixed': exact px (select, actions) — never grows or shrinks
     *  - 'capped': grows from minSize to maxSize then stops (dates, status, numbers)
     *  - 'flex': absorbs remaining space via 1fr (names, descriptions)
     *  When omitted, getGridTemplate falls back to the legacy minSize/maxSize heuristic. */
    sizingCategory?: 'fixed' | 'capped' | 'flex'
    /** For grouped tables: 'doc' columns show only on group's first row;
     *  'item' columns show on every row. Unset = always show. */
    groupRole?: 'doc' | 'item'
  }
}
