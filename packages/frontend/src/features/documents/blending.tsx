import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BlendingResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPickerField } from '~/components/entity-picker'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { CompanyMutateDialog } from '~/features/catalog/companies'
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
    textColumn<BlendingResponse>('documentNumber', t('documents:blending.columns.documentNumber')),
    dateColumn<BlendingResponse>('date', t('documents:blending.columns.date')),
    resolvedColumn<BlendingResponse>('contractorId', t('documents:items.contractor'), 'contractorIdName'),
    resolvedColumn<BlendingResponse>('targetProductId', t('documents:items.product'), 'targetProductIdName'),
    statusColumn<BlendingResponse>('status', t('documents:blending.columns.status'), documentStatusColors),
    dateColumn<BlendingResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<BlendingResponse>(DataTableRowActions),
  ]
}

// --- Global Filter + Table ---

const route = getRouteApi('/_authenticated/documents/blending/')
const globalFilterFn = createGlobalFilter<BlendingResponse>('documentNumber')

interface BlendingTableProps {
  data: BlendingResponse[]
}

function BlendingTable({ data }: BlendingTableProps) {
  return (
    <EntityTable
      tableId="blending"
      data={data}
      getColumns={getBlendingColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['documents', 'common']}
      bulkActions={t => [
        {
          label: t('common:actions.execute'),
          icon: Play,
          onClick: (rows) => {
            const draftRows = rows.filter(r => r.status === 'DRAFT')
            void draftRows // TODO: wire bulk execute API
          },
        },
        {
          label: t('common:actions.softDelete'),
          icon: Archive,
          variant: 'destructive',
          onClick: (rows) => {
            void rows // TODO: wire bulk soft-delete API
          },
        },
      ]}
    />
  )
}

// --- Mutate Dialog ---

const blendingFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  contractorId: z.string().min(1, 'Contractor is required'),
  targetProductId: z.string().min(1, 'Target product is required'),
})

type BlendingFormValues = z.infer<typeof blendingFormSchema>

interface BlendingMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: BlendingResponse | null
}

function BlendingMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: BlendingMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

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
    mapRowToForm: (row: BlendingResponse) => ({
      documentNumber: row.documentNumber,
      date: row.date ? row.date.slice(0, 16) : '',
      contractorId: row.contractorId,
      targetProductId: row.targetProductId,
    }),
    createFn: blendingDocumentCreate,
    updateFn: blendingDocumentUpdate,
    queryKey: blendingDocumentListQueryKey(),
    entityLabel: t('documents:blending.singular'),
    formId: 'blending-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('common:actions.edit') : t('documents:blending.create')}
      description={isUpdate ? t('common:actions.edit') : t('documents:blending.create')}
      formId="blending-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="blending-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<BlendingFormValues> name="documentNumber" label={t('documents:blending.columns.documentNumber')} />
          <TextField<BlendingFormValues> name="date" label={t('documents:blending.columns.date')} type="datetime-local" />
          <EntityPickerField<BlendingFormValues>
            name="contractorId"
            label={t('documents:items.contractor')}
            queryResult={companiesQuery}
            displayField="commonName"
            allowCreate
            createDialog={CompanyMutateDialog}
          />
          <EntityPickerField<BlendingFormValues>
            name="targetProductId"
            label={t('documents:items.product')}
            queryResult={productsQuery}
            displayField="commonName"
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
  entityLabel: 'documents:blending.singular',
  i18nNamespaces: ['common', 'documents'],
})

// --- Lifecycle Dialog ---

interface BlendingLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: BlendingResponse | null
  variant: 'execute' | 'revert'
}

function BlendingLifecycleDialog({ open, onOpenChange, currentRow, variant }: BlendingLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={variant}
      executeFn={blendingDocumentExecute}
      revertFn={blendingDocumentRevert}
      queryKey={blendingDocumentListQueryKey()}
      entityLabel={t('documents:blending.singular')}
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
  createLabel: 'documents:blending.create',
  i18nNamespaces: ['documents'],
})

// --- Page Component ---

export function BlendingDocuments() {
  const { t } = useTranslation(['documents'])
  const queryResult = useBlendingDocumentList()

  return (
    <EntityPage
      provider={BlendingProvider}
      title={t('documents:blending.title')}
      queryResult={queryResult}
      primaryButtons={BlendingPrimaryButtons}
      table={BlendingTable}
      dialogs={BlendingDialogs}
    />
  )
}
