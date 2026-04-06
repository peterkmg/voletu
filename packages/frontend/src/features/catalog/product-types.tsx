import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductTypeResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogProductTypeCreate, catalogProductTypeHardDelete, catalogProductTypeSoftDelete, catalogProductTypeUpdate } from '~/generated/client'
import { catalogProductTypeListQueryKey, useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type ProductTypesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: ProductTypesProvider, useEntity: useProductTypes }
  = createEntityProvider<ProductTypeResponse, ProductTypesDialogType>('ProductTypes')

// --- Row Actions ---

const DataTableRowActions = createRowActions<ProductTypeResponse>({ useEntity: useProductTypes })

// --- Columns ---

function getProductTypeColumns(t: TFunction): ColumnDef<ProductTypeResponse>[] {
  return [
    textColumn<ProductTypeResponse>('commonName', t('catalog:productType.columns.commonName'), { sizing: 'capped', maxSize: 250 }),
    textColumn<ProductTypeResponse>('longName', t('catalog:productType.columns.longName'), { primary: false }),
    { ...dateColumn<ProductTypeResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), sizingCategory: 'capped', requiresRole: 'senior_supervisor' } },
    actionsColumn<ProductTypeResponse>(DataTableRowActions, 2),
  ]
}

// --- Table ---

const productTypesRoute = getRouteApi('/_authenticated/catalog/product-types/')
const productTypesGlobalFilterFn = createGlobalFilter<ProductTypeResponse>('commonName', 'longName')

interface ProductTypesTableProps {
  data: ProductTypeResponse[]
  actions?: React.ReactNode
}

function ProductTypesTable({ data, actions }: ProductTypesTableProps) {
  return (
    <EntityTable
      tableId="product-types"
      data={data}
      getColumns={getProductTypeColumns}
      routeApi={productTypesRoute}
      globalFilterFn={productTypesGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
      actions={actions}
    />
  )
}

// --- Mutate Dialog ---

const productTypeFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  longName: z.string().nullable().optional(),
})

type ProductTypeFormValues = z.infer<typeof productTypeFormSchema>

interface ProductTypeMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductTypeResponse | null
  onCreated?: (id: string) => void
}

export function ProductTypeMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: ProductTypeMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: productTypeFormSchema,
    defaultValues: {
      commonName: '',
      longName: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      longName: row.longName ?? '',
    }),
    transformPayload: values => ({
      ...values,
      longName: values.longName || null,
    }),
    createFn: catalogProductTypeCreate,
    updateFn: catalogProductTypeUpdate,
    queryKey: catalogProductTypeListQueryKey(),
    entityLabel: t('catalog:productType.singular'),
    formId: 'product-type-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:productType.edit') : t('catalog:productType.create')}
      description={isUpdate ? t('catalog:productType.edit') : t('catalog:productType.create')}
      formId="product-type-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="product-type-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<ProductTypeFormValues> name="commonName" label={t('catalog:productType.form.commonName')} />
          <TextField<ProductTypeFormValues> name="longName" label={t('catalog:productType.form.longName')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const ProductTypeDeleteDialog = createDeleteDialog({
  useEntity: useProductTypes,
  hardDeleteFn: catalogProductTypeHardDelete,
  softDeleteFn: catalogProductTypeSoftDelete,
  queryKey: catalogProductTypeListQueryKey,
  entityLabel: 'catalog:productType.singular',
  i18nNamespaces: ['common', 'catalog'],
})

// --- Entity Dialogs ---

const ProductTypesDialogs = createEntityDialogs({
  useEntity: useProductTypes,
  MutateDialog: ProductTypeMutateDialog,
  DeleteDialog: ProductTypeDeleteDialog,
})

// --- Primary Buttons ---

const ProductTypesPrimaryButtons = createPrimaryButtons({ useEntity: useProductTypes })

// --- Page ---

export function ProductTypes() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogProductTypeList()

  return (
    <EntityPage
      provider={ProductTypesProvider}
      title={t('catalog:productType.title')}
      queryResult={queryResult}
      primaryButtons={ProductTypesPrimaryButtons}
      table={ProductTypesTable}
      dialogs={ProductTypesDialogs}
    />
  )
}
