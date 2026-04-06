import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BlendingComponentResponse, BlendingFlatRow, BlendingResultResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { Skeleton } from '~/components/ui/skeleton'
import { blendingDocumentCreate, blendingDocumentExecute, blendingDocumentHardDelete, blendingDocumentRevert, blendingDocumentSoftDelete, blendingDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useBlendingCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/useBlendingCompositeGet'
import { flowBlendingFlatQueryQueryKey, useFlowBlendingFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowBlendingFlatQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type BlendingDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider: BlendingProvider, useEntity: useBlending }
  = createEntityProvider<BlendingFlatRow, BlendingDialogType>('Blending')

// --- Row Actions ---

const DataTableRowActions = createRowActions<BlendingFlatRow>({ useEntity: useBlending, lifecycle: true, getDetailPath: row => `/internal/blending/${row.documentId}` })

// --- Columns ---

function getBlendingColumns(t: TFunction): ColumnDef<BlendingFlatRow>[] {
  return [
    // Document-level columns (groupRole: 'doc' — shown only on first row of group)
    { ...textColumn<BlendingFlatRow>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<BlendingFlatRow>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<BlendingFlatRow>('contractorIdName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    { ...textColumn<BlendingFlatRow>('targetProductIdName', t('common:table.product')), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    { ...statusColumn<BlendingFlatRow>('status', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Item-level columns (groupRole: 'item' — shown on every row)
    { ...textColumn<BlendingFlatRow>('itemType', t('common:columns.type')), meta: { label: t('common:columns.type'), sizingCategory: 'capped', groupRole: 'item' as const } },
    { ...textColumn<BlendingFlatRow>('productIdName', t('common:table.product')), id: 'itemProduct', meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<BlendingFlatRow>('storageIdName', t('common:columns.storage')), meta: { label: t('common:columns.storage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<BlendingFlatRow>('amount', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    // Actions (doc-level)
    { ...actionsColumn<BlendingFlatRow>(DataTableRowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

// --- Table ---

const blendingRoute = getRouteApi('/_authenticated/internal/blending/')
const blendingDetailRoute = getRouteApi('/_authenticated/internal/blending/$id')
const blendingGlobalFilterFn = createGlobalFilter<BlendingFlatRow>('documentNumber', 'productIdName')

function BlendingTable({ data }: { data: BlendingFlatRow[] }) {
  return (
    <EntityTable
      tableId="blending"
      data={data}
      getColumns={getBlendingColumns}
      routeApi={blendingRoute}
      globalFilterFn={blendingGlobalFilterFn}
      i18nNamespaces={['common']}
      groupKey="documentId"
    />
  )
}

// --- Mutate Dialog ---

const blendingFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  contractorId: z.string().uuid('Contractor is required'),
  targetProductId: z.string().uuid('Product is required'),
})

type BlendingFormValues = z.infer<typeof blendingFormSchema>

function BlendingMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: BlendingFlatRow | null
}) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()
  const productsQuery = useCatalogProductList({ embed: 'names' })

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: blendingFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      contractorId: '',
      targetProductId: '',
    },
    mapRowToForm: row => ({
      documentNumber: row.documentNumber,
      date: row.date?.split('T')[0] ?? '',
      contractorId: '',
      targetProductId: '',
    }),
    createFn: blendingDocumentCreate,
    updateFn: blendingDocumentUpdate,
    queryKey: flowBlendingFlatQueryQueryKey(),
    entityLabel: t('common:nav.blending'),
    formId: 'blending-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('common:actions.edit') : t('common:actions.create')}
      description={t('common:nav.blending')}
      formId="blending-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form id="blending-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<BlendingFormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<BlendingFormValues> name="date" label={t('common:table.date')} type="date" />
          <EntityPickerField<BlendingFormValues>
            name="contractorId"
            label={t('common:table.contractor')}
            queryResult={companiesQuery}
          />
          <EntityPickerField<BlendingFormValues>
            name="targetProductId"
            label={t('common:table.product')}
            queryResult={productsQuery}
          />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const BlendingDeleteDialog = createDeleteDialog({
  useEntity: useBlending,
  hardDeleteFn: blendingDocumentHardDelete,
  softDeleteFn: blendingDocumentSoftDelete,
  queryKey: flowBlendingFlatQueryQueryKey,
  entityLabel: 'common:nav.blending',
  i18nNamespaces: ['common'],
})

// --- Lifecycle Dialog ---

function BlendingLifecycleDialog({
  open,
  onOpenChange,
  currentRow,
  variant,
}: {
  open: boolean
  onOpenChange: () => void
  currentRow: BlendingFlatRow | null
  variant: 'execute' | 'revert'
}) {
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={variant}
      executeFn={blendingDocumentExecute}
      revertFn={blendingDocumentRevert}
      queryKey={flowBlendingFlatQueryQueryKey()}
      entityLabel="Blending Document"
    />
  )
}

// --- Entity Dialogs ---

const BlendingDialogs = createEntityDialogs({
  useEntity: useBlending,
  MutateDialog: BlendingMutateDialog,
  DeleteDialog: BlendingDeleteDialog,
  LifecycleDialog: BlendingLifecycleDialog,
  lifecyclePropName: 'variant',
})

// --- Primary Buttons ---

const BlendingPrimaryButtons = createPrimaryButtons({ useEntity: useBlending })

// --- Page ---

export function BlendingPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useFlowBlendingFlatQuery()

  return (
    <EntityPage
      provider={BlendingProvider}
      title={t('common:nav.blending')}
      queryResult={queryResult}
      primaryButtons={BlendingPrimaryButtons}
      table={BlendingTable}
      dialogs={BlendingDialogs}
    />
  )
}

export function BlendingDetail() {
  const { id } = blendingDetailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useBlendingCompositeGet(id, { embed: 'names' })

  if (isLoading || !data?.data) {
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>
  }

  const composite = data.data
  const doc = composite.document

  return (
    <DocumentDetailPage
      config={{
        title: t('common:nav.blending'),
        entityLabel: 'Blending Document',
        backTo: '/internal/blending',
        executeFn: blendingDocumentExecute,
        revertFn: blendingDocumentRevert,
        queryKey: flowBlendingFlatQueryQueryKey(),
        statusColorMap: statusColors,
      }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={(
        <div className="grid grid-cols-3 gap-4">
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
            <p>{formatDate(doc.date)}</p>
          </div>
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.contractor')}</span>
            <p>{doc.contractorIdName ?? doc.contractorId}</p>
          </div>
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.product')}</span>
            <p>{doc.targetProductIdName ?? doc.targetProductId}</p>
          </div>
        </div>
      )}
      itemsContent={(
        <>
          <ChildItemsTable
            items={composite.components}
            columns={[
              textColumn<BlendingComponentResponse>('sourceProductIdName', t('common:table.product')),
              textColumn<BlendingComponentResponse>('storageIdName', t('common:columns.storage')),
              numericColumn<BlendingComponentResponse>('amountUsed', t('common:table.quantity')),
            ]}
            isLocked={doc.status === 'EXECUTED'}
            sectionTitle={t('common:sections.componentInputs')}
          />
          <ChildItemsTable
            items={composite.results}
            columns={[
              textColumn<BlendingResultResponse>('storageIdName', t('common:columns.storage')),
              numericColumn<BlendingResultResponse>('producedAmount', t('common:table.quantity')),
            ]}
            isLocked={doc.status === 'EXECUTED'}
            sectionTitle={t('common:sections.resultOutputs')}
          />
        </>
      )}
      metadataContent={
        doc.executedAt
          ? (
              <div className="grid grid-cols-3 gap-4 text-sm">
                <div>
                  <span className="text-muted-foreground">{t('common:metadata.executedAt')}:</span>
                  {' '}
                  {formatDateTime(doc.executedAt)}
                </div>
              </div>
            )
          : null
      }
    />
  )
}
