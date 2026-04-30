import type { Table } from '@tanstack/react-table'
import type { ExportFile } from '~/lib/files'

interface CsvExportOptions {
  filename?: string
  now?: Date
}

export function buildCsvExportFile<TData>(
  table: Table<TData>,
  options: CsvExportOptions = {},
): ExportFile {
  const columns = table
    .getVisibleLeafColumns()
    .filter(column => column.id !== 'select' && column.id !== 'actions')

  const headers = columns.map(column =>
    column.columnDef.meta?.label ?? stringHeader(column.columnDef.header) ?? column.id,
  )

  const rows = table.getFilteredRowModel().rows.map(row =>
    columns.map((column) => {
      const exportValue = column.columnDef.meta?.exportValue
      const value = exportValue
        ? exportValue(row.original, row, column.id)
        : row.getValue(column.id)
      return escapeCsvField(value)
    }).join(','),
  )

  return {
    suggestedName: `${options.filename ?? 'export'}-${datePart(options.now ?? new Date())}.csv`,
    mimeType: 'text/csv;charset=utf-8',
    contents: `\uFEFF${[
      headers.map(escapeCsvField).join(','),
      ...rows,
    ].join('\n')}`,
  }
}

function stringHeader(header: unknown): string | undefined {
  return typeof header === 'string' ? header : undefined
}

function datePart(date: Date): string {
  return date.toISOString().slice(0, 10)
}

function escapeCsvField(value: unknown): string {
  if (value == null)
    return ''

  const str = String(value)
  if (/[",\n\r]/.test(str))
    return `"${str.replace(/"/g, '""')}"`

  return str
}
