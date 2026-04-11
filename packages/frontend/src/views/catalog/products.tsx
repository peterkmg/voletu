import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { ProductResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, textColumn } from '~/components/data-table'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogProductCreate, catalogProductHardDelete, catalogProductSoftDelete, catalogProductUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { catalogProductListQueryKey, useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { defineCrudViews } from '~/lib/define-crud-views'
import { CompanyMutateDialog } from '~/views/catalog/companies'
import { ProductGroupMutateDialog } from '~/views/catalog/product-groups'

// --- Columns ---

function getProductColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<ProductResponse> }>,
): ColumnDef<ProductResponse>[] {
  return [
    textColumn<ProductResponse>('commonName', t('catalog:product.columns.commonName')),
    resolvedColumn<ProductResponse>('productGroupId', t('catalog:product.columns.productGroup'), 'productGroupIdName'),
    resolvedColumn<ProductResponse>('manufacturerId', t('catalog:product.columns.manufacturer'), 'manufacturerIdName'),
    textColumn<ProductResponse>('addIdentification', t('catalog:product.columns.identification'), { primary: false, sizing: 'capped', maxSize: 200 }),
    { ...dateColumn<ProductResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), sizingCategory: 'capped', requiresRole: 'senior_supervisor' } },
    actionsColumn<ProductResponse>(RowActions, 2),
  ]
}

// --- Table ---

const productsRoute = getRouteApi('/_authenticated/catalog/products/')
const productsGlobalFilterFn = createGlobalFilter<ProductResponse>('commonName', 'addIdentification')

interface ProductsTableProps {
  data: ProductResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<ProductResponse> }>
}

function ProductsTable({ data, actions, RowActions }: ProductsTableProps) {
  return (
    <EntityTable
      tableId="products"
      data={data}
      getColumns={t => getProductColumns(t, RowActions)}
      routeApi={productsRoute}
      globalFilterFn={productsGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
      actions={actions}
    />
  )
}

function useProductsTitle() {
  return useTranslation(['catalog']).t('catalog:product.title')
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
  onCreated?: (id: string) => void
}

function ProductMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
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
    onCreated,
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

const productsViewDefinition = defineCrudViews<ProductResponse>({
  displayName: 'Products',
  useTitle: useProductsTitle,
  useQuery: () => useCatalogProductList({ embed: 'names' }),
  Table: ProductsTable,
  MutateDialog: ProductMutateDialog,
  deleteDialog: {
    hardDeleteFn: catalogProductHardDelete,
    softDeleteFn: catalogProductSoftDelete,
    queryKey: catalogProductListQueryKey,
    entityLabel: 'catalog:product.singular',
    i18nNamespaces: ['common', 'catalog'],
  },
})

export function Products() {
  return <productsViewDefinition.View />
}
