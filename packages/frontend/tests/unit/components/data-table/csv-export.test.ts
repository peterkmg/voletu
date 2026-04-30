import type { Table } from '@tanstack/react-table'
import { describe, expect, it } from 'vitest'
import { buildCsvExportFile } from '~/components/data-table/csv-export'

interface RowData {
  name: string
  productId: string
  productIdName: string
  amount: number
}

function tableStub(data: RowData[]): Table<RowData> {
  const columns = [
    { id: 'select', columnDef: { header: 'Select' } },
    { id: 'name', columnDef: { meta: { label: 'Name' } } },
    {
      id: 'productId',
      columnDef: {
        meta: {
          label: 'Product',
          exportValue: (row: RowData) => row.productIdName,
        },
      },
    },
    { id: 'amount', columnDef: { meta: { label: 'Amount' } } },
    { id: 'actions', columnDef: { header: 'Actions' } },
  ]

  return {
    getVisibleLeafColumns: () => columns,
    getFilteredRowModel: () => ({
      rows: data.map(row => ({
        original: row,
        getValue: (id: keyof RowData) => row[id],
      })),
    }),
  } as unknown as Table<RowData>
}

describe('buildCsvExportFile', () => {
  it('exports visible data columns with metadata labels and CSV escaping', () => {
    const file = buildCsvExportFile(tableStub([
      {
        name: 'Alpha, Ltd.',
        productId: 'p-1',
        productIdName: 'Diesel "Premium"',
        amount: 12,
      },
      {
        name: 'Beta\nDepot',
        productId: 'p-2',
        productIdName: 'Gasoline',
        amount: 7,
      },
    ]), {
      filename: 'ledger',
      now: new Date('2026-04-30T08:00:00Z'),
    })

    expect(file).toEqual({
      suggestedName: 'ledger-2026-04-30.csv',
      mimeType: 'text/csv;charset=utf-8',
      contents: '\uFEFFName,Product,Amount\n"Alpha, Ltd.","Diesel ""Premium""",12\n"Beta\nDepot",Gasoline,7',
    })
  })
})
