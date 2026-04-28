import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { WarehouseResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, textColumn } from '~/components/data-table'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogWarehouseCreate, catalogWarehouseHardDelete, catalogWarehouseSoftDelete, catalogWarehouseUpdate } from '~/generated/client'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { catalogWarehouseListQueryKey, useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { defineCrudViews } from '~/lib/define-crud-views'
import { BaseMutateDialog } from '~/views/catalog/bases'

// --- Columns ---

function getWarehouseColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<WarehouseResponse> }>,
): ColumnDef<WarehouseResponse>[] {
  return [
    textColumn<WarehouseResponse>('commonName', t('entities:commonName')),
    resolvedColumn<WarehouseResponse>('baseId', t('entities:base'), 'baseIdName'),
    { ...dateColumn<WarehouseResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), sizingCategory: 'capped', requiresRole: 'senior_supervisor' } },
    actionsColumn<WarehouseResponse>(RowActions, 2),
  ]
}

// --- Table ---

const warehousesRoute = getRouteApi('/_authenticated/catalog/warehouses/')
const warehousesGlobalFilterFn = createGlobalFilter<WarehouseResponse>('commonName', 'longName')

interface WarehousesTableProps {
  data: WarehouseResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<WarehouseResponse> }>
}

function WarehousesTable({ data, actions, RowActions }: WarehousesTableProps) {
  return (
    <EntityTable
      tableId="warehouses"
      data={data}
      getColumns={t => getWarehouseColumns(t, RowActions)}
      routeApi={warehousesRoute}
      globalFilterFn={warehousesGlobalFilterFn}
      i18nNamespaces={['catalog', 'entities', 'common']}
      actions={actions}
    />
  )
}

function useWarehousesTitle() {
  return useTranslation(['catalog']).t('catalog:warehouse.title')
}

// --- Mutate Dialog ---

const warehouseFormSchema = z.object({
  commonName: z.string().min(1),
  baseId: z.string().min(1),
})

type WarehouseFormValues = z.infer<typeof warehouseFormSchema>

interface WarehouseMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: WarehouseResponse | null
  onCreated?: (id: string) => void
}

export function WarehouseMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: WarehouseMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'entities', 'common', 'forms'])

  const basesQuery = useCatalogBaseList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: warehouseFormSchema,
    defaultValues: {
      commonName: '',
      baseId: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      baseId: row.baseId,
    }),
    createFn: catalogWarehouseCreate,
    updateFn: catalogWarehouseUpdate,
    queryKey: catalogWarehouseListQueryKey(),
    entityLabel: t('catalog:warehouse.singular'),
    formId: 'warehouse-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:warehouse.edit') : t('catalog:warehouse.create')}
      description={isUpdate ? t('catalog:warehouse.edit') : t('catalog:warehouse.create')}
      formId="warehouse-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="warehouse-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<WarehouseFormValues> name="commonName" label={t('entities:commonName')} />
          <EntityPickerField<WarehouseFormValues>
            name="baseId"
            label={t('entities:base')}
            placeholder={t('forms:picker.selectEntity', { entity: t('entities:base').toLowerCase() })}
            queryResult={basesQuery}
            displayField="commonName"
            allowCreate
            createDialog={BaseMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}

const warehousesViewDefinition = defineCrudViews<WarehouseResponse>({
  displayName: 'Warehouses',
  useTitle: useWarehousesTitle,
  useQuery: () => useCatalogWarehouseList({ embed: 'names' }),
  Table: WarehousesTable,
  MutateDialog: WarehouseMutateDialog,
  deleteDialog: {
    hardDeleteFn: catalogWarehouseHardDelete,
    softDeleteFn: catalogWarehouseSoftDelete,
    queryKey: catalogWarehouseListQueryKey,
    entityLabel: 'catalog:warehouse.singular',
    i18nNamespaces: ['common', 'catalog'],
  },
})

export function Warehouses() {
  return <warehousesViewDefinition.View />
}
