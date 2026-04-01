import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { AcceptanceResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { SelectField, TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { acceptanceDocumentCreate, acceptanceDocumentExecute, acceptanceDocumentHardDelete, acceptanceDocumentRevert, acceptanceDocumentSoftDelete, acceptanceDocumentUpdate } from '~/generated/client'
import { acceptanceDocumentListQueryKey, useAcceptanceDocumentList } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { arrivalTypeColors, documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type AcceptanceDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider: AcceptanceProvider, useEntity: useAcceptance }
  = createEntityProvider<AcceptanceResponse, AcceptanceDialogType>('Acceptance')

// --- Row Actions ---

const DataTableRowActions = createRowActions<AcceptanceResponse>({ useEntity: useAcceptance, lifecycle: true })

// --- Columns ---

function getAcceptanceColumns(t: TFunction): ColumnDef<AcceptanceResponse>[] {
  return [
    selectColumn<AcceptanceResponse>(),
    textColumn<AcceptanceResponse>('documentNumber', t('documents:acceptance.columns.documentNumber')),
    dateColumn<AcceptanceResponse>('dateAccepted', t('documents:acceptance.columns.date')),
    statusColumn<AcceptanceResponse>('arrivalType', t('documents:acceptance.columns.arrivalType'), arrivalTypeColors),
    statusColumn<AcceptanceResponse>('status', t('documents:acceptance.columns.status'), documentStatusColors),
    dateColumn<AcceptanceResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<AcceptanceResponse>(DataTableRowActions),
  ]
}

// --- Global Filter + Table ---

const route = getRouteApi('/_authenticated/documents/acceptance/')
const globalFilterFn = createGlobalFilter<AcceptanceResponse>('documentNumber', 'sourceEntity')

interface AcceptanceTableProps {
  data: AcceptanceResponse[]
}

function AcceptanceTable({ data }: AcceptanceTableProps) {
  return (
    <EntityTable
      tableId="acceptance"
      data={data}
      getColumns={getAcceptanceColumns}
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

const arrivalTypes = ['TRUCK', 'RAIL', 'EXTERNAL', 'INITIAL_BALANCE'] as const

const acceptanceFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  dateAccepted: z.string().min(1, 'Date is required'),
  arrivalType: z.enum(arrivalTypes),
  sourceEntity: z.string().nullable().optional(),
})

type AcceptanceFormValues = z.infer<typeof acceptanceFormSchema>

interface AcceptanceMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: AcceptanceResponse | null
}

function AcceptanceMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: AcceptanceMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const arrivalTypeOptions = arrivalTypes.map(type => ({
    value: type,
    label: t(`documents:acceptance.arrivalTypes.${type}`),
  }))

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog<
    AcceptanceFormValues,
    AcceptanceResponse,
    AcceptanceFormValues & { sourceEntity: string | null }
  >({
    open,
    onOpenChange,
    currentRow,
    schema: acceptanceFormSchema,
    defaultValues: {
      documentNumber: '',
      dateAccepted: '',
      arrivalType: 'TRUCK',
      sourceEntity: '',
    },
    mapRowToForm: row => ({
      documentNumber: row.documentNumber,
      dateAccepted: row.dateAccepted ? row.dateAccepted.slice(0, 16) : '',
      arrivalType: row.arrivalType,
      sourceEntity: row.sourceEntity ?? '',
    }),
    transformPayload: values => ({
      ...values,
      sourceEntity: values.sourceEntity || null,
    }),
    createFn: acceptanceDocumentCreate,
    updateFn: acceptanceDocumentUpdate,
    queryKey: acceptanceDocumentListQueryKey(),
    entityLabel: t('documents:acceptance.singular'),
    formId: 'acceptance-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('documents:acceptance.edit') : t('documents:acceptance.create')}
      description={isUpdate ? t('documents:acceptance.edit') : t('documents:acceptance.create')}
      formId="acceptance-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="acceptance-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<AcceptanceFormValues> name="documentNumber" label={t('documents:acceptance.form.documentNumber')} />
          <TextField<AcceptanceFormValues> name="dateAccepted" label={t('documents:acceptance.form.dateAccepted')} type="datetime-local" />
          <SelectField<AcceptanceFormValues> name="arrivalType" label={t('documents:acceptance.form.arrivalType')} options={arrivalTypeOptions} />
          <TextField<AcceptanceFormValues> name="sourceEntity" label={t('documents:acceptance.form.sourceEntity')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const AcceptanceDeleteDialog = createDeleteDialog({
  useEntity: useAcceptance,
  hardDeleteFn: acceptanceDocumentHardDelete,
  softDeleteFn: acceptanceDocumentSoftDelete,
  queryKey: acceptanceDocumentListQueryKey,
  entityLabel: 'documents:acceptance.singular',
  i18nNamespaces: ['common', 'documents'],
})

// --- Lifecycle Dialog ---

interface AcceptanceLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: AcceptanceResponse | null
  action: 'execute' | 'revert'
}

function AcceptanceLifecycleDialog({ open, onOpenChange, currentRow, action }: AcceptanceLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={action}
      executeFn={acceptanceDocumentExecute}
      revertFn={acceptanceDocumentRevert}
      queryKey={acceptanceDocumentListQueryKey()}
      entityLabel={t('documents:acceptance.singular')}
    />
  )
}

// --- Entity Dialogs ---

const AcceptanceDialogs = createEntityDialogs({
  useEntity: useAcceptance,
  MutateDialog: AcceptanceMutateDialog,
  DeleteDialog: AcceptanceDeleteDialog,
  LifecycleDialog: AcceptanceLifecycleDialog,
  lifecyclePropName: 'action',
})

// --- Primary Buttons ---

const AcceptancePrimaryButtons = createPrimaryButtons({
  useEntity: useAcceptance,
  createLabel: 'documents:acceptance.create',
  i18nNamespaces: ['documents'],
})

// --- Page Component ---

export function AcceptanceDocuments() {
  const { t } = useTranslation(['documents'])
  const queryResult = useAcceptanceDocumentList()

  return (
    <EntityPage
      provider={AcceptanceProvider}
      title={t('documents:acceptance.title')}
      queryResult={queryResult}
      primaryButtons={AcceptancePrimaryButtons}
      table={AcceptanceTable}
      dialogs={AcceptanceDialogs}
    />
  )
}
