import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BlendingResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { blendingDocumentCreate, blendingDocumentExecute, blendingDocumentHardDelete, blendingDocumentRevert, blendingDocumentSoftDelete, blendingDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
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

const DataTableRowActions = createRowActions<BlendingResponse>({ useEntity: useBlending, lifecycle: true })

// --- Columns ---

function getBlendingColumns(t: TFunction): ColumnDef<BlendingResponse>[] {
  return [
    selectColumn<BlendingResponse>(),
    textColumn<BlendingResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<BlendingResponse>('date', t('common:table.date')),
    resolvedColumn<BlendingResponse>('contractorId', t('common:table.contractor'), 'contractorIdName'),
    resolvedColumn<BlendingResponse>('targetProductId', t('common:table.product'), 'targetProductIdName'),
    statusColumn<BlendingResponse>('status', t('common:table.status'), documentStatusColors),
    actionsColumn<BlendingResponse>(DataTableRowActions),
  ]
}

// --- Table ---

const blendingRoute = getRouteApi('/_authenticated/internal/blending/')
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
  const productsQuery = useCatalogProductList()

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

const BlendingPrimaryButtons = createPrimaryButtons({
  useEntity: useBlending,
  createLabel: 'common:actions.create',
  i18nNamespaces: ['common'],
})

// --- Page ---

export function BlendingPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useBlendingDocumentList()

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
  return <div className="p-4">Blending Detail — TODO</div>
}
