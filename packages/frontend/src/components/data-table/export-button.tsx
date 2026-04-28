import type { Table } from '@tanstack/react-table'
import { Download } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'

interface ExportButtonProps<TData> {
  table: Table<TData>
  filename?: string
}

function escapeCsvField(value: unknown): string {
  if (value == null)
    return ''
  const str = String(value)
  if (str.includes(',') || str.includes('"') || str.includes('\n')) {
    return `"${str.replace(/"/g, '""')}"`
  }
  return str
}

export function ExportButton<TData>({
  table,
  filename = 'export',
}: ExportButtonProps<TData>) {
  const { t } = useTranslation('tables')

  const handleExport = () => {
    const visibleColumns = table
      .getVisibleLeafColumns()
      .filter(col => col.id !== 'select' && col.id !== 'actions')

    const headers = visibleColumns.map((col) => {
      const def = col.columnDef
      if (typeof def.header === 'string')
        return def.header
      return col.id
    })

    const rows = table.getFilteredRowModel().rows.map(row =>
      visibleColumns.map((col) => {
        const value = row.getValue(col.id)
        return escapeCsvField(value)
      }),
    )

    const csv = [
      headers.map(escapeCsvField).join(','),
      ...rows.map(row => row.join(',')),
    ].join('\n')

    const blob = new Blob([`\uFEFF${csv}`], { type: 'text/csv;charset=utf-8;' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = `${filename}-${new Date().toISOString().slice(0, 10)}.csv`
    link.click()
    URL.revokeObjectURL(url)
  }

  return (
    <Button
      variant="outline"
      size="sm"
      className="h-8"
      onClick={handleExport}
    >
      <Download className="size-4" />
      {t('tables:export')}
    </Button>
  )
}
