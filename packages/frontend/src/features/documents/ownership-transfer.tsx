import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { OwnershipTransferResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, DataTableColumnHeader, dateColumn, EntityTable, IdCell, NumericCell, selectColumn, statusColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { ownershipTransferCreate, ownershipTransferExecute, ownershipTransferRevert } from '~/generated/client'
import { ownershipTransferListQueryKey, useOwnershipTransferList } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type OwnershipTransferDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider: OwnershipTransferProvider, useEntity: useOwnershipTransfer }
  = createEntityProvider<OwnershipTransferResponse, OwnershipTransferDialogType>('OwnershipTransfer')

// --- Row Actions ---

const DataTableRowActions = createRowActions<OwnershipTransferResponse>({ useEntity: useOwnershipTransfer, lifecycle: true })

// --- Columns ---

function getOwnershipTransferColumns(t: TFunction): ColumnDef<OwnershipTransferResponse>[] {
  return [
    selectColumn<OwnershipTransferResponse>(),
    {
      accessorKey: 'id',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:table.id')}
        />
      ),
      cell: ({ row }) => <IdCell value={row.getValue('id')} />,
    },
    dateColumn<OwnershipTransferResponse>('date', t('documents:acceptance.columns.date')),
    statusColumn<OwnershipTransferResponse>('status', t('common:table.status'), documentStatusColors),
    {
      id: 'itemsCount',
      header: t('documents:items.title'),
      cell: ({ row }) => (
        <NumericCell value={row.original.items.length} />
      ),
      meta: { align: 'right' as const },
    },
    actionsColumn<OwnershipTransferResponse>(DataTableRowActions),
  ]
}

// --- Global Filter + Table ---

const route = getRouteApi('/_authenticated/documents/ownership-transfer/')
const globalFilterFn = createGlobalFilter<OwnershipTransferResponse>('id')

interface OwnershipTransferTableProps {
  data: OwnershipTransferResponse[]
}

function OwnershipTransferTable({ data }: OwnershipTransferTableProps) {
  return (
    <EntityTable
      tableId="ownership-transfer"
      data={data}
      getColumns={getOwnershipTransferColumns}
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

const ownershipTransferFormSchema = z.object({
  date: z.string().min(1, 'Date is required'),
})

type OwnershipTransferFormValues = z.infer<typeof ownershipTransferFormSchema>

interface OwnershipTransferMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

function OwnershipTransferMutateDialog({
  open,
  onOpenChange,
}: OwnershipTransferMutateDialogProps) {
  const { t } = useTranslation(['documents', 'common'])

  const { form, handleSubmit, handleOpenChange } = useMutateDialog<
    OwnershipTransferFormValues,
    { id: string },
    OwnershipTransferFormValues & { items: never[] }
  >({
    open,
    onOpenChange,
    schema: ownershipTransferFormSchema,
    defaultValues: {
      date: '',
    },
    transformPayload: values => ({
      ...values,
      items: [],
    }),
    createFn: ownershipTransferCreate,
    queryKey: ownershipTransferListQueryKey(),
    entityLabel: t('documents:ownershipTransfer.singular'),
    formId: 'ownership-transfer-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={t('documents:ownershipTransfer.create')}
      description={t('documents:ownershipTransfer.create')}
      formId="ownership-transfer-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="ownership-transfer-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<OwnershipTransferFormValues> name="date" label={t('documents:acceptance.columns.date')} type="datetime-local" />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Lifecycle Dialog ---

interface OwnershipTransferLifecycleDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: OwnershipTransferResponse | null
  action: 'execute' | 'revert'
}

function OwnershipTransferLifecycleDialog({ open, onOpenChange, currentRow, action }: OwnershipTransferLifecycleDialogProps) {
  const { t } = useTranslation(['documents'])
  return (
    <LifecycleDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      action={action}
      executeFn={ownershipTransferExecute}
      revertFn={ownershipTransferRevert}
      queryKey={ownershipTransferListQueryKey()}
      entityLabel={t('documents:ownershipTransfer.singular')}
    />
  )
}

// --- Entity Dialogs ---

const OwnershipTransferDialogs = createEntityDialogs({
  useEntity: useOwnershipTransfer,
  MutateDialog: OwnershipTransferMutateDialog,
  supportsUpdate: false,
  LifecycleDialog: OwnershipTransferLifecycleDialog,
  lifecyclePropName: 'action',
})

// --- Primary Buttons ---

const OwnershipTransferPrimaryButtons = createPrimaryButtons({
  useEntity: useOwnershipTransfer,
  createLabel: 'documents:ownershipTransfer.create',
  i18nNamespaces: ['documents'],
})

// --- Page Component ---

export function OwnershipTransfers() {
  const { t } = useTranslation(['documents'])
  const queryResult = useOwnershipTransferList()

  return (
    <EntityPage
      provider={OwnershipTransferProvider}
      title={t('documents:ownershipTransfer.title')}
      queryResult={queryResult}
      primaryButtons={OwnershipTransferPrimaryButtons}
      table={OwnershipTransferTable}
      dialogs={OwnershipTransferDialogs}
    />
  )
}
