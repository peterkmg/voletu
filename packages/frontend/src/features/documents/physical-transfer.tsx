import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PhysicalTransferResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, DataTableColumnHeader, dateColumn, DateTimeCell, EntityTable, NumericCell, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { physicalTransferCreate, physicalTransferExecute, physicalTransferRevert } from '~/generated/client'
import { physicalTransferListQueryKey, usePhysicalTransferList } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type PhysicalTransferDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider: PhysicalTransferProvider, useEntity: usePhysicalTransfer }
  = createEntityProvider<PhysicalTransferResponse, PhysicalTransferDialogType>('PhysicalTransfer')

// --- Row Actions ---

const DataTableRowActions = createRowActions<PhysicalTransferResponse>({ useEntity: usePhysicalTransfer, lifecycle: true })

// --- Columns ---

function getPhysicalTransferColumns(t: TFunction): ColumnDef<PhysicalTransferResponse>[] {
  return [
    selectColumn<PhysicalTransferResponse>(),
    textColumn<PhysicalTransferResponse>('documentNumber', t('documents:acceptance.columns.documentNumber')),
    dateColumn<PhysicalTransferResponse>('date', t('documents:acceptance.columns.date')),
    {
      accessorKey: 'startCargoOps',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.startCargoOps', { defaultValue: 'Start Cargo Ops' })}
        />
      ),
      cell: ({ row }) => <DateTimeCell value={row.getValue('startCargoOps')} />,
    },
    {
      accessorKey: 'endCargoOps',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.endCargoOps', { defaultValue: 'End Cargo Ops' })}
        />
      ),
      cell: ({ row }) => <DateTimeCell value={row.getValue('endCargoOps')} />,
    },
    statusColumn<PhysicalTransferResponse>('status', t('common:table.status'), documentStatusColors),
    {
      id: 'itemsCount',
      header: t('documents:items.title'),
      cell: ({ row }) => (
        <NumericCell value={row.original.items.length} />
      ),
      meta: { align: 'right' as const },
    },
    actionsColumn<PhysicalTransferResponse>(DataTableRowActions),
  ]
}

// --- Global Filter + Table ---

const route = getRouteApi('/_authenticated/documents/physical-transfer/')
const globalFilterFn = createGlobalFilter<PhysicalTransferResponse>('documentNumber')

interface PhysicalTransferTableProps {
  data: PhysicalTransferResponse[]
}

function PhysicalTransferTable({ data }: PhysicalTransferTableProps) {
  return (
    <EntityTable
      tableId="physical-transfer"
      data={data}
      getColumns={getPhysicalTransferColumns}
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

const physicalTransferFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  startCargoOps: z.string().min(1, 'Start cargo ops is required'),
  endCargoOps: z.string().min(1, 'End cargo ops is required'),
})

type PhysicalTransferFormValues = z.infer<typeof physicalTransferFormSchema>

interface PhysicalTransferMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

function PhysicalTransferMutateDialog({
  open,
  onOpenChange,
}: PhysicalTransferMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const { form, handleSubmit, handleOpenChange } = useMutateDialog<
    PhysicalTransferFormValues,
    { id: string },
    PhysicalTransferFormValues & { items: never[] }
  >({
    open,
    onOpenChange,
    schema: physicalTransferFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      startCargoOps: '',
      endCargoOps: '',
    },
    transformPayload: values => ({
      ...values,
      items: [],
    }),
    createFn: physicalTransferCreate,
    queryKey: physicalTransferListQueryKey(),
    entityLabel: t('documents:physicalTransfer.singular'),
    formId: 'physical-transfer-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={t('documents:physicalTransfer.create')}
      description={t('documents:physicalTransfer.create')}
      formId="physical-transfer-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="physical-transfer-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<PhysicalTransferFormValues> name="documentNumber" label={t('documents:acceptance.columns.documentNumber')} />
          <TextField<PhysicalTransferFormValues> name="date" label={t('documents:acceptance.columns.date')} type="datetime-local" />
          <TextField<PhysicalTransferFormValues> name="startCargoOps" label={t('common:table.startCargoOps', { defaultValue: 'Start Cargo Ops' })} type="datetime-local" />
          <TextField<PhysicalTransferFormValues> name="endCargoOps" label={t('common:table.endCargoOps', { defaultValue: 'End Cargo Ops' })} type="datetime-local" />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Lifecycle Dialog ---

interface PhysicalTransferLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: PhysicalTransferResponse | null
  action: 'execute' | 'revert'
}

function PhysicalTransferLifecycleDialog({ open, onOpenChange, currentRow, action }: PhysicalTransferLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={action}
      executeFn={physicalTransferExecute}
      revertFn={physicalTransferRevert}
      queryKey={physicalTransferListQueryKey()}
      entityLabel={t('documents:physicalTransfer.singular')}
    />
  )
}

// --- Entity Dialogs ---

const PhysicalTransferDialogs = createEntityDialogs({
  useEntity: usePhysicalTransfer,
  MutateDialog: PhysicalTransferMutateDialog,
  supportsUpdate: false,
  LifecycleDialog: PhysicalTransferLifecycleDialog,
  lifecyclePropName: 'action',
})

// --- Primary Buttons ---

const PhysicalTransferPrimaryButtons = createPrimaryButtons({
  useEntity: usePhysicalTransfer,
  createLabel: 'documents:physicalTransfer.create',
  i18nNamespaces: ['documents'],
})

// --- Page Component ---

export function PhysicalTransfers() {
  const { t } = useTranslation(['documents'])
  const queryResult = usePhysicalTransferList()

  return (
    <EntityPage
      provider={PhysicalTransferProvider}
      title={t('documents:physicalTransfer.title')}
      queryResult={queryResult}
      primaryButtons={PhysicalTransferPrimaryButtons}
      table={PhysicalTransferTable}
      dialogs={PhysicalTransferDialogs}
    />
  )
}
