import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPickerField } from '~/components/entity-picker'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { SelectField, TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { CompanyMutateDialog } from '~/features/catalog/companies'
import { dispatchDocumentCreate, dispatchDocumentExecute, dispatchDocumentHardDelete, dispatchDocumentRevert, dispatchDocumentSoftDelete, dispatchDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { dispatchDocumentListQueryKey, useDispatchDocumentList } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { dispatchMethodColors, dispatchPurposeColors, documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type DispatchDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider: DispatchProvider, useEntity: useDispatch }
  = createEntityProvider<DispatchResponse, DispatchDialogType>('Dispatch')

// --- Row Actions ---

const DataTableRowActions = createRowActions<DispatchResponse>({ useEntity: useDispatch, lifecycle: true })

// --- Columns ---

function getDispatchColumns(t: TFunction): ColumnDef<DispatchResponse>[] {
  return [
    selectColumn<DispatchResponse>(),
    textColumn<DispatchResponse>('documentNumber', t('documents:dispatch.columns.documentNumber')),
    dateColumn<DispatchResponse>('date', t('documents:dispatch.columns.date')),
    statusColumn<DispatchResponse>('dispatchPurpose', t('documents:dispatch.columns.purpose'), dispatchPurposeColors),
    statusColumn<DispatchResponse>('dispatchMethod', t('documents:dispatch.columns.method'), dispatchMethodColors),
    resolvedColumn<DispatchResponse>('contractorIdName', t('documents:dispatch.columns.contractor', 'Contractor'), 'contractorIdName'),
    resolvedColumn<DispatchResponse>('portIdName', t('documents:dispatch.columns.port', 'Port'), 'portIdName'),
    resolvedColumn<DispatchResponse>('exporterIdName', t('documents:dispatch.columns.exporter', 'Exporter'), 'exporterIdName'),
    resolvedColumn<DispatchResponse>('destinationBaseIdName', t('documents:dispatch.columns.destinationBase', 'Destination Base'), 'destinationBaseIdName'),
    statusColumn<DispatchResponse>('status', t('documents:dispatch.columns.status'), documentStatusColors),
    dateColumn<DispatchResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<DispatchResponse>(DataTableRowActions),
  ]
}

// --- Global Filter + Table ---

const route = getRouteApi('/_authenticated/documents/dispatch/')
const globalFilterFn = createGlobalFilter<DispatchResponse>('documentNumber')

interface DispatchTableProps {
  data: DispatchResponse[]
}

function DispatchTable({ data }: DispatchTableProps) {
  return (
    <EntityTable
      tableId="dispatch"
      data={data}
      getColumns={getDispatchColumns}
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

const dispatchFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  dispatchPurpose: z.enum(['EXTERNAL', 'INTERNAL']),
  dispatchMethod: z.enum(['TRUCK', 'VESSEL_TERMINAL', 'BUNKERING']),
  contractorId: z.string().min(1, 'Contractor is required'),
  receiverEntity: z.string().nullable().optional(),
})

type DispatchFormValues = z.infer<typeof dispatchFormSchema>

const purposeOptions = [
  { value: 'EXTERNAL', label: 'EXTERNAL' },
  { value: 'INTERNAL', label: 'INTERNAL' },
] as const

const methodOptions = [
  { value: 'TRUCK', label: 'TRUCK' },
  { value: 'VESSEL_TERMINAL', label: 'VESSEL_TERMINAL' },
  { value: 'BUNKERING', label: 'BUNKERING' },
] as const

interface DispatchMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: DispatchResponse | null
}

function DispatchMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: DispatchMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog<
    DispatchFormValues,
    DispatchResponse,
    DispatchFormValues & { receiverEntity: string | null }
  >({
    open,
    onOpenChange,
    currentRow,
    schema: dispatchFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      dispatchPurpose: 'EXTERNAL',
      dispatchMethod: 'TRUCK',
      contractorId: '',
      receiverEntity: '',
    },
    mapRowToForm: row => ({
      documentNumber: row.documentNumber,
      date: row.date ? row.date.slice(0, 16) : '',
      dispatchPurpose: row.dispatchPurpose,
      dispatchMethod: row.dispatchMethod,
      contractorId: row.contractorId,
      receiverEntity: row.receiverEntity ?? '',
    }),
    transformPayload: values => ({
      ...values,
      receiverEntity: values.receiverEntity || null,
    }),
    createFn: dispatchDocumentCreate,
    updateFn: dispatchDocumentUpdate,
    queryKey: dispatchDocumentListQueryKey(),
    entityLabel: t('documents:dispatch.singular'),
    formId: 'dispatch-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('common:actions.edit') : t('documents:dispatch.create')}
      description={isUpdate ? t('common:actions.edit') : t('documents:dispatch.create')}
      formId="dispatch-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="dispatch-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<DispatchFormValues> name="documentNumber" label={t('documents:dispatch.columns.documentNumber')} />
          <TextField<DispatchFormValues> name="date" label={t('documents:dispatch.columns.date')} type="datetime-local" />
          <SelectField<DispatchFormValues> name="dispatchPurpose" label={t('documents:dispatch.columns.purpose')} options={purposeOptions} />
          <SelectField<DispatchFormValues> name="dispatchMethod" label={t('documents:dispatch.columns.method')} options={methodOptions} />
          <EntityPickerField<DispatchFormValues>
            name="contractorId"
            label={t('documents:items.contractor')}
            queryResult={companiesQuery}
            displayField="commonName"
            allowCreate
            createDialog={CompanyMutateDialog}
          />
          <TextField<DispatchFormValues> name="receiverEntity" label="Receiver Entity" nullable />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const DispatchDeleteDialog = createDeleteDialog({
  useEntity: useDispatch,
  hardDeleteFn: dispatchDocumentHardDelete,
  softDeleteFn: dispatchDocumentSoftDelete,
  queryKey: dispatchDocumentListQueryKey,
  entityLabel: 'documents:dispatch.singular',
  i18nNamespaces: ['common', 'documents'],
})

// --- Lifecycle Dialog ---

interface DispatchLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: DispatchResponse | null
  variant: 'execute' | 'revert'
}

function DispatchLifecycleDialog({ open, onOpenChange, currentRow, variant }: DispatchLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={variant}
      executeFn={dispatchDocumentExecute}
      revertFn={dispatchDocumentRevert}
      queryKey={dispatchDocumentListQueryKey()}
      entityLabel={t('documents:dispatch.singular')}
    />
  )
}

// --- Entity Dialogs ---

const DispatchDialogs = createEntityDialogs({
  useEntity: useDispatch,
  MutateDialog: DispatchMutateDialog,
  DeleteDialog: DispatchDeleteDialog,
  LifecycleDialog: DispatchLifecycleDialog,
  lifecyclePropName: 'variant',
})

// --- Primary Buttons ---

const DispatchPrimaryButtons = createPrimaryButtons({
  useEntity: useDispatch,
  createLabel: 'documents:dispatch.create',
  i18nNamespaces: ['documents'],
})

// --- Page Component ---

export function DispatchDocuments() {
  const { t } = useTranslation(['documents'])
  const queryResult = useDispatchDocumentList()

  return (
    <EntityPage
      provider={DispatchProvider}
      title={t('documents:dispatch.title')}
      queryResult={queryResult}
      primaryButtons={DispatchPrimaryButtons}
      table={DispatchTable}
      dialogs={DispatchDialogs}
    />
  )
}
