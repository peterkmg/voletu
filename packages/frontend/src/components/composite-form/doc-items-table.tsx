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

const FORM_ROW_ID = '_formRowId'

type DrawerState
  = | { kind: 'idle' }
    | { kind: 'edit', index: number }
    | { kind: 'create', nonce: number }

export interface DocItemsTableProps<TForm extends FieldValues, TItem extends FieldValues> {
  name: ArrayPath<TForm>
  columns: ColumnSpec<TItem>[]

  rowSchema: unknown
  rowFields: RowFieldSpec<TItem>[]

  emptyRow: TItem

  sectionTitleKey?: string

  emptyStateKey?: string

  density?: TableDensity

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

  const { fields, append, remove, update } = useFieldArray<TForm, ArrayPath<TForm>, typeof FORM_ROW_ID>({
    control,
    name,
    keyName: FORM_ROW_ID,
  })

  const [drawer, setDrawer] = useState<DrawerState>({ kind: 'idle' })

  const compact = density === 'compact'

  const gridTemplate = useMemo(() => {
    const dataCols = columns.map(col => col.gridWidth ?? 'minmax(0, 1fr)').join(' ')
    const actionsCol = '96px'
    return `${dataCols} ${actionsCol}`
  }, [columns])

  const openRow = (index: number) => setDrawer({ kind: 'edit', index })
  const closeRow = () => setDrawer({ kind: 'idle' })

  const handleAdd = () => setDrawer({ kind: 'create', nonce: 0 })

  const handleSaveRow = (data: TItem) => {
    if (drawer.kind === 'edit')
      update(drawer.index, data as unknown as FieldArray<TForm, ArrayPath<TForm>>)
    else if (drawer.kind === 'create')
      append(data as unknown as FieldArray<TForm, ArrayPath<TForm>>)

    setDrawer({ kind: 'idle' })
  }

  const handleSaveAndAdd = (data: TItem) => {
    if (drawer.kind === 'edit') {
      update(drawer.index, data as unknown as FieldArray<TForm, ArrayPath<TForm>>)
      setDrawer({ kind: 'create', nonce: 0 })
      return
    }

    if (drawer.kind === 'create') {
      append(data as unknown as FieldArray<TForm, ArrayPath<TForm>>)

      setDrawer({ kind: 'create', nonce: drawer.nonce + 1 })
    }
  }

  const drawerSelectedIndex = drawer.kind === 'edit' ? drawer.index : null
  const drawerOpen = drawer.kind !== 'idle'
  const drawerKey = drawer.kind === 'create' ? `create-${drawer.nonce}` : drawer.kind === 'edit' ? `edit-${drawer.index}` : 'idle'
  const drawerDefaults = drawer.kind === 'edit'
    ? (fields[drawer.index] as unknown as DefaultValues<TItem>)
    : (emptyRow as unknown as DefaultValues<TItem>)

  return (
    <section
      data-slot="doc-items-table"
      data-density={density}
      className="space-y-4"
    >
      <div className="grid gap-2">
        {sectionTitleKey && (
          <h3
            className={cn(
              'flex items-center gap-2 leading-none',
              compact
                ? 'text-xs font-semibold uppercase text-muted-foreground tracking-wide'
                : 'text-sm font-medium',
            )}
          >
            <span>{t(sectionTitleKey)}</span>
            <span className="text-xs text-muted-foreground font-normal">
              (
              {fields.length}
              )
            </span>
          </h3>
        )}

        <div className="hidden md:block rounded-md border">
          <Table gridTemplate={gridTemplate}>
            <TableHeader>
              <TableRow>
                {columns.map(col => (
                  <TableHead
                    key={col.key}
                    className={cn(
                      'text-xs font-medium',
                      col.widthClass,
                      col.alignClass,
                      compact && 'h-8',
                    )}
                  >
                    {t(col.labelKey)}
                  </TableHead>
                ))}
                <TableHead
                  className={cn(
                    'w-20 text-end text-xs font-medium',
                    compact && 'h-8',
                  )}
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
                    key={row[FORM_ROW_ID]}
                    data-state={drawerSelectedIndex === index ? 'selected' : undefined}
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
                key={row[FORM_ROW_ID]}
                data-state={drawerSelectedIndex === index ? 'selected' : undefined}
                className={cn(
                  'rounded-md border p-3 flex items-start gap-3',
                  drawerSelectedIndex === index && 'border-ring',
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
      </div>

      <Button type="button" variant="outline" size="sm" onClick={handleAdd}>
        <PlusIcon className="size-4 mr-2" />
        {t('itemsTable.add')}
      </Button>

      {drawerOpen && (
        <DocItemRowDrawer
          key={drawerKey}
          rowSchema={rowSchema}
          fields={rowFields}
          defaultValues={drawerDefaults}
          onSave={handleSaveRow}
          onSaveAndAdd={handleSaveAndAdd}
          onCancel={closeRow}
        >
          {drawer.kind === 'edit' ? rowDrawerExtra?.(drawer.index) : null}
        </DocItemRowDrawer>
      )}
    </section>
  )
}
