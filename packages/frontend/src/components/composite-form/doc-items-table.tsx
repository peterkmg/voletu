import type { ReactNode } from 'react'
import type { ArrayPath, DefaultValues, FieldArray, FieldValues } from 'react-hook-form'
import type { ColumnSpec, RowFieldSpec, TableDensity } from './types'
import { PencilIcon, PlusIcon, Trash2Icon } from 'lucide-react'
import { useMemo, useState } from 'react'
import {

  useFieldArray,
  useFormContext,
} from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '~/components/ui/table'
import { cn } from '~/lib/utils'
import { DocItemRowDrawer } from './doc-item-row-drawer'

export interface DocItemsTableProps<TForm extends FieldValues, TItem extends FieldValues> {
  name: ArrayPath<TForm>
  columns: ColumnSpec<TItem>[]
  /** Use `unknown` to mirror DocFormProvider's loosened typing. */
  rowSchema: unknown
  rowFields: RowFieldSpec<TItem>[]
  /** Default values for a new (empty) row. */
  emptyRow: TItem
  /** i18n key for the section heading rendered above the table. */
  sectionTitleKey?: string
  /** i18n key for the empty-state caption. Default: forms.itemsTable.empty. */
  emptyStateKey?: string
  /** Compact density for nested inner tables (rail-receipt). */
  density?: TableDensity
  /** Optional content rendered inside the row drawer below the field grid (used for nested tables). */
  rowDrawerExtra?: (rowIndex: number) => ReactNode
}

export function DocItemsTable<TForm extends FieldValues, TItem extends FieldValues>({
  name,
  columns,
  rowSchema,
  rowFields,
  emptyRow,
  sectionTitleKey,
  emptyStateKey = 'itemsTable.empty',
  density = 'default',
  rowDrawerExtra,
}: DocItemsTableProps<TForm, TItem>) {
  const { t } = useTranslation('forms')
  const { control } = useFormContext<TForm>()
  const { fields, append, remove, update } = useFieldArray<TForm>({ control, name })
  const [editingIndex, setEditingIndex] = useState<number | null>(null)

  const compact = density === 'compact'

  // CSS Grid track sizes for the bespoke `<Table>` (see ui/table.tsx). Each data
  // column defaults to an equal flex share; the trailing actions column is fixed
  // at 96px to comfortably fit the two ghost icon buttons (edit + delete) plus
  // padding. Per-column overrides come from `ColumnSpec.gridWidth`.
  const gridTemplate = useMemo(() => {
    const dataCols = columns.map(col => col.gridWidth ?? 'minmax(0, 1fr)').join(' ')
    const actionsCol = '96px'
    return `${dataCols} ${actionsCol}`
  }, [columns])

  const openRow = (index: number) => setEditingIndex(index)
  const closeRow = () => setEditingIndex(null)

  const handleAdd = () => {
    append(emptyRow as unknown as FieldArray<TForm, ArrayPath<TForm>>)
    setEditingIndex(fields.length) // open the just-appended row
  }

  const handleSaveRow = (data: TItem) => {
    if (editingIndex === null)
      return
    update(editingIndex, data as unknown as FieldArray<TForm, ArrayPath<TForm>>)
    setEditingIndex(null)
  }

  const handleSaveAndAdd = (data: TItem) => {
    if (editingIndex === null)
      return
    update(editingIndex, data as unknown as FieldArray<TForm, ArrayPath<TForm>>)
    append(emptyRow as unknown as FieldArray<TForm, ArrayPath<TForm>>)
    setEditingIndex(fields.length) // open the new row (post-append index)
  }

  return (
    <section
      data-slot="doc-items-table"
      data-density={density}
      className="space-y-2"
    >
      {sectionTitleKey && (
        <div className="flex items-center justify-between mb-2">
          <h3
            className={cn(
              'font-semibold',
              compact ? 'text-xs uppercase text-muted-foreground tracking-wide' : 'text-base',
            )}
          >
            {t(sectionTitleKey)}
            <span className="ml-2 text-sm text-muted-foreground font-normal">
              (
              {fields.length}
              )
            </span>
          </h3>
        </div>
      )}

      <div className="hidden md:block rounded-md border">
        <Table gridTemplate={gridTemplate}>
          <TableHeader>
            <TableRow>
              {columns.map(col => (
                <TableHead
                  key={col.key}
                  className={cn(
                    col.widthClass,
                    col.alignClass,
                    compact && 'h-8 text-xs',
                  )}
                >
                  {t(col.labelKey)}
                </TableHead>
              ))}
              <TableHead
                className={cn('w-20 text-end', compact && 'h-8 text-xs')}
              >
                {t('itemsTable.actions')}
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {fields.length === 0 && (
              <TableRow>
                <TableCell
                  colSpan={columns.length + 1}
                  className="text-center text-muted-foreground py-6"
                >
                  {t(emptyStateKey)}
                </TableCell>
              </TableRow>
            )}
            {fields.map((row, index) => {
              const item = row as unknown as TItem
              return (
                <TableRow
                  key={row.id}
                  data-state={editingIndex === index ? 'selected' : undefined}
                  className={cn(compact && '[&_td]:p-1 [&_td]:text-xs')}
                >
                  {columns.map(col => (
                    <TableCell
                      key={col.key}
                      className={cn(col.widthClass, col.alignClass)}
                    >
                      {col.render
                        ? col.render(item[col.key], item)
                        : String(item[col.key] ?? '')}
                    </TableCell>
                  ))}
                  <TableCell className="text-end">
                    <Button
                      type="button"
                      variant="ghost"
                      size="icon"
                      onClick={() => openRow(index)}
                      aria-label={t('editRow')}
                    >
                      <PencilIcon className="size-4" />
                    </Button>
                    <Button
                      type="button"
                      variant="ghost"
                      size="icon"
                      onClick={() => remove(index)}
                      aria-label={t('deleteRow')}
                    >
                      <Trash2Icon className="size-4 text-destructive" />
                    </Button>
                  </TableCell>
                </TableRow>
              )
            })}
          </TableBody>
        </Table>
      </div>

      {/* Mobile (< md): card list */}
      <div className="md:hidden space-y-2">
        {fields.length === 0 && (
          <p className="text-center text-sm text-muted-foreground py-6">
            {t(emptyStateKey)}
          </p>
        )}
        {fields.map((row, index) => {
          const item = row as unknown as TItem
          return (
            <div
              key={row.id}
              data-state={editingIndex === index ? 'selected' : undefined}
              className={cn(
                'rounded-md border p-3 flex items-start gap-3',
                editingIndex === index && 'border-ring',
              )}
            >
              <div className="flex-1 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-sm">
                {columns.map(col => (
                  <div key={col.key} className="contents">
                    <span className="text-muted-foreground">{t(col.labelKey)}</span>
                    <span className="text-end break-words">
                      {col.render
                        ? col.render(item[col.key], item)
                        : String(item[col.key] ?? '')}
                    </span>
                  </div>
                ))}
              </div>
              <div className="flex flex-col gap-1 shrink-0">
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  onClick={() => openRow(index)}
                  aria-label={t('editRow')}
                >
                  <PencilIcon className="size-4" />
                </Button>
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  onClick={() => remove(index)}
                  aria-label={t('deleteRow')}
                >
                  <Trash2Icon className="size-4 text-destructive" />
                </Button>
              </div>
            </div>
          )
        })}
      </div>

      <Button type="button" variant="outline" size="sm" onClick={handleAdd}>
        <PlusIcon className="size-4 mr-2" />
        {t('itemsTable.add')}
      </Button>

      {editingIndex !== null && fields[editingIndex] && (
        <DocItemRowDrawer
          rowSchema={rowSchema}
          fields={rowFields}
          defaultValues={fields[editingIndex] as unknown as DefaultValues<TItem>}
          onSave={handleSaveRow}
          onSaveAndAdd={handleSaveAndAdd}
          onCancel={closeRow}
        >
          {rowDrawerExtra?.(editingIndex)}
        </DocItemRowDrawer>
      )}
    </section>
  )
}
