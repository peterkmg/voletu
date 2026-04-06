import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { CompanyMutateDialog } from '~/features/catalog/companies'
import { ProductGroupMutateDialog } from '~/features/catalog/product-groups'
import { catalogProductCreate, catalogProductHardDelete, catalogProductSoftDelete, catalogProductUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { catalogProductListQueryKey, useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type ProductsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: ProductsProvider, useEntity: useProducts }
  = createEntityProvider<ProductResponse, ProductsDialogType>('Products')

// --- Row Actions ---

const DataTableRowActions = createRowActions<ProductResponse>({ useEntity: useProducts })

// --- Columns ---

function getProductColumns(t: TFunction): ColumnDef<ProductResponse>[] {
  return [
    textColumn<ProductResponse>('commonName', t('catalog:product.columns.commonName')),
    resolvedColumn<ProductResponse>('productGroupId', t('catalog:product.columns.productGroupId'), 'productGroupIdName'),
    resolvedColumn<ProductResponse>('manufacturerId', t('catalog:product.columns.manufacturerId'), 'manufacturerIdName'),
    textColumn<ProductResponse>('addIdentification', t('catalog:product.columns.identification'), { primary: false, sizing: 'capped', maxSize: 200 }),
    { ...dateColumn<ProductResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), sizingCategory: 'capped', requiresRole: 'senior_supervisor' } },
    actionsColumn<ProductResponse>(DataTableRowActions, 2),
  ]
}

// --- Table ---

const productsRoute = getRouteApi('/_authenticated/catalog/products/')
const productsGlobalFilterFn = createGlobalFilter<ProductResponse>('commonName', 'addIdentification')

interface ProductsTableProps {
  data: ProductResponse[]
}

function ProductsTable({ data }: ProductsTableProps) {
  return (
    <EntityTable
      tableId="products"
      data={data}
      getColumns={getProductColumns}
      routeApi={productsRoute}
      globalFilterFn={productsGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
    />
  )
}

// --- Mutate Dialog ---

const productFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  productGroupId: z.string().min(1, 'Product group is required'),
  manufacturerId: z.string().nullable().optional(),
  addIdentification: z.string().nullable().optional(),
})

type ProductFormValues = z.infer<typeof productFormSchema>

interface ProductMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: ProductResponse | null
}

function ProductMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: ProductMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const productGroupsQuery = useCatalogProductGroupList({ embed: 'names' })
  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: productFormSchema,
    defaultValues: {
      commonName: '',
      productGroupId: '',
      manufacturerId: '',
      addIdentification: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      productGroupId: row.productGroupId,
      manufacturerId: row.manufacturerId ?? '',
      addIdentification: row.addIdentification ?? '',
    }),
    transformPayload: values => ({
      ...values,
      manufacturerId: values.manufacturerId || null,
      addIdentification: values.addIdentification || null,
    }),
    createFn: catalogProductCreate,
    updateFn: catalogProductUpdate,
    queryKey: catalogProductListQueryKey(),
    entityLabel: t('catalog:product.singular'),
    formId: 'product-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:product.edit') : t('catalog:product.create')}
      description={isUpdate ? t('catalog:product.edit') : t('catalog:product.create')}
      formId="product-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="product-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<ProductFormValues> name="commonName" label={t('catalog:product.form.commonName')} />
          <EntityPickerField<ProductFormValues>
            name="productGroupId"
            label={t('catalog:product.form.productGroupId')}
            placeholder={t('catalog:product.form.selectProductGroup')}
            queryResult={productGroupsQuery}
            displayField="commonName"
            allowCreate
            createDialog={ProductGroupMutateDialog}
          />
          <EntityPickerField<ProductFormValues>
            name="manufacturerId"
            label={t('catalog:product.form.manufacturerId')}
            placeholder={t('catalog:product.form.selectManufacturer')}
            queryResult={companiesQuery}
            displayField="commonName"
            nullable
            allowCreate
            createDialog={CompanyMutateDialog}
          />
          <TextField<ProductFormValues> name="addIdentification" label={t('catalog:product.form.identification')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const ProductDeleteDialog = createDeleteDialog({
  useEntity: useProducts,
  hardDeleteFn: catalogProductHardDelete,
  softDeleteFn: catalogProductSoftDelete,
  queryKey: catalogProductListQueryKey,
  entityLabel: 'catalog:product.singular',
  i18nNamespaces: ['common', 'catalog'],
})

// --- Entity Dialogs ---

const ProductsDialogs = createEntityDialogs({
  useEntity: useProducts,
  MutateDialog: ProductMutateDialog,
  DeleteDialog: ProductDeleteDialog,
})

// --- Primary Buttons ---

const ProductsPrimaryButtons = createPrimaryButtons({ useEntity: useProducts })

// --- Page ---

export function Products() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogProductList({ embed: 'names' })

  return (
    <EntityPage
      provider={ProductsProvider}
      title={t('catalog:product.title')}
      queryResult={queryResult}
      primaryButtons={ProductsPrimaryButtons}
      table={ProductsTable}
      dialogs={ProductsDialogs}
    />
  )
}
