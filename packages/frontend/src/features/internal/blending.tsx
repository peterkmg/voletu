import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BlendingComponentResponse, BlendingResponse, BlendingResultResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, statusColumn, textColumn } from '~/components/data-table'
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
import { blendingDocumentListQueryKey, useBlendingDocumentList } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type BlendingDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider: BlendingProvider, useEntity: useBlending }
  = createEntityProvider<BlendingResponse, BlendingDialogType>('Blending')

// --- Row Actions ---

const DataTableRowActions = createRowActions<BlendingResponse>({ useEntity: useBlending, lifecycle: true, getDetailPath: row => `/internal/blending/${row.id}` })

// --- Columns ---

function getBlendingColumns(t: TFunction): ColumnDef<BlendingResponse>[] {
  return [
    textColumn<BlendingResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<BlendingResponse>('date', t('common:table.date')),
    resolvedColumn<BlendingResponse>('contractorId', t('common:table.contractor'), 'contractorIdName'),
    resolvedColumn<BlendingResponse>('targetProductId', t('common:table.product'), 'targetProductIdName'),
    statusColumn<BlendingResponse>('status', t('common:table.status'), documentStatusColors),
    { ...dateColumn<BlendingResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), requiresRole: 'senior_supervisor' } },
    { ...dateColumn<BlendingResponse>('updatedAt', t('common:table.updatedAt')), enableHiding: true, meta: { label: t('common:table.updatedAt'), requiresRole: 'senior_supervisor' } },
    actionsColumn<BlendingResponse>(DataTableRowActions),
  ]
}

// --- Table ---

const blendingRoute = getRouteApi('/_authenticated/internal/blending/')
const blendingDetailRoute = getRouteApi('/_authenticated/internal/blending/$id')
const blendingGlobalFilterFn = createGlobalFilter<BlendingResponse>('documentNumber')

function BlendingTable({ data }: { data: BlendingResponse[] }) {
  return (
    <EntityTable
      tableId="blending"
      data={data}
      getColumns={getBlendingColumns}
      routeApi={blendingRoute}
      globalFilterFn={blendingGlobalFilterFn}
      i18nNamespaces={['common']}
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
  currentRow?: BlendingResponse | null
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
      contractorId: row.contractorId,
      targetProductId: row.targetProductId,
    }),
    createFn: blendingDocumentCreate,
    updateFn: blendingDocumentUpdate,
    queryKey: blendingDocumentListQueryKey(),
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
  queryKey: blendingDocumentListQueryKey,
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
  currentRow: BlendingResponse | null
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
      queryKey={blendingDocumentListQueryKey()}
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
  const queryResult = useBlendingDocumentList({ embed: 'names' })

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
  const { data, isLoading } = useBlendingCompositeGet(id)

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
        queryKey: blendingDocumentListQueryKey(),
        statusColorMap: documentStatusColors,
      }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={(
        <div className="grid grid-cols-3 gap-4">
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
            <p>{doc.date}</p>
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
              textColumn<BlendingComponentResponse>('amountUsed', t('common:table.quantity')),
            ]}
            isLocked={doc.status === 'POSTED'}
            sectionTitle={t('common:sections.componentInputs')}
          />
          <ChildItemsTable
            items={composite.results}
            columns={[
              textColumn<BlendingResultResponse>('storageIdName', t('common:columns.storage')),
              textColumn<BlendingResultResponse>('producedAmount', t('common:table.quantity')),
            ]}
            isLocked={doc.status === 'POSTED'}
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
                  {doc.executedAt}
                </div>
              </div>
            )
          : null
      }
    />
  )
}
