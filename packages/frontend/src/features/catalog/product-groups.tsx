import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductGroupResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { ProductTypeMutateDialog } from '~/features/catalog/product-types'
import { catalogProductGroupCreate, catalogProductGroupHardDelete, catalogProductGroupSoftDelete, catalogProductGroupUpdate } from '~/generated/client'
import { catalogProductGroupListQueryKey, useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type ProductGroupsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: ProductGroupsProvider, useEntity: useProductGroups }
  = createEntityProvider<ProductGroupResponse, ProductGroupsDialogType>('ProductGroups')

// --- Row Actions ---

const DataTableRowActions = createRowActions<ProductGroupResponse>({ useEntity: useProductGroups })

// --- Columns ---

function getProductGroupColumns(t: TFunction): ColumnDef<ProductGroupResponse>[] {
  return [
    textColumn<ProductGroupResponse>('commonName', t('catalog:productGroup.columns.commonName')),
    resolvedColumn<ProductGroupResponse>('productTypeId', t('catalog:productGroup.columns.productType'), 'productTypeIdName'),
    dateColumn<ProductGroupResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<ProductGroupResponse>(DataTableRowActions),
  ]
}

// --- Table ---

const productGroupsRoute = getRouteApi('/_authenticated/catalog/product-groups/')
const productGroupsGlobalFilterFn = createGlobalFilter<ProductGroupResponse>('commonName')

interface ProductGroupsTableProps {
  data: ProductGroupResponse[]
}

function ProductGroupsTable({ data }: ProductGroupsTableProps) {
  return (
    <EntityTable
      tableId="product-groups"
      data={data}
      getColumns={getProductGroupColumns}
      routeApi={productGroupsRoute}
      globalFilterFn={productGroupsGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
    />
  )
}

// --- Mutate Dialog ---

const productGroupFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  productTypeId: z.string().min(1, 'Product type is required'),
})

type ProductGroupFormValues = z.infer<typeof productGroupFormSchema>

interface ProductGroupMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductGroupResponse | null
  onCreated?: (id: string) => void
}

export function ProductGroupMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: ProductGroupMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const productTypesQuery = useCatalogProductTypeList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: productGroupFormSchema,
    defaultValues: {
      commonName: '',
      productTypeId: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      productTypeId: row.productTypeId,
    }),
    createFn: catalogProductGroupCreate,
    updateFn: catalogProductGroupUpdate,
    queryKey: catalogProductGroupListQueryKey(),
    entityLabel: t('catalog:productGroup.singular'),
    formId: 'product-group-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:productGroup.edit') : t('catalog:productGroup.create')}
      description={isUpdate ? t('catalog:productGroup.edit') : t('catalog:productGroup.create')}
      formId="product-group-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="product-group-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<ProductGroupFormValues> name="commonName" label={t('catalog:productGroup.form.commonName')} />
          <EntityPickerField<ProductGroupFormValues>
            name="productTypeId"
            label={t('catalog:productGroup.form.productType')}
            placeholder={t('catalog:productGroup.form.selectProductType')}
            queryResult={productTypesQuery}
            displayField="commonName"
            allowCreate
            createDialog={ProductTypeMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const ProductGroupDeleteDialog = createDeleteDialog({
  useEntity: useProductGroups,
  hardDeleteFn: catalogProductGroupHardDelete,
  softDeleteFn: catalogProductGroupSoftDelete,
  queryKey: catalogProductGroupListQueryKey,
  entityLabel: 'catalog:productGroup.singular',
  i18nNamespaces: ['common', 'catalog'],
})

// --- Entity Dialogs ---

const ProductGroupsDialogs = createEntityDialogs({
  useEntity: useProductGroups,
  MutateDialog: ProductGroupMutateDialog,
  DeleteDialog: ProductGroupDeleteDialog,
})

// --- Primary Buttons ---

const ProductGroupsPrimaryButtons = createPrimaryButtons({ useEntity: useProductGroups })

// --- Page ---

export function ProductGroups() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogProductGroupList()

  return (
    <EntityPage
      provider={ProductGroupsProvider}
      title={t('catalog:productGroup.title')}
      queryResult={queryResult}
      primaryButtons={ProductGroupsPrimaryButtons}
      table={ProductGroupsTable}
      dialogs={ProductGroupsDialogs}
    />
  )
}
