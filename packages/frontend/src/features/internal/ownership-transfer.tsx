import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { OwnershipTransferFlatRow, OwnershipTransferItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { DetailField, DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { Skeleton } from '~/components/ui/skeleton'
import { ownershipDocumentCreate, ownershipDocumentExecute, ownershipDocumentHardDelete, ownershipDocumentRevert, ownershipDocumentSoftDelete, ownershipDocumentUpdate } from '~/generated/client'
import { useOwnershipTransferCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferCompositeGet'
import { flowOwnershipTransferFlatQueryQueryKey, useFlowOwnershipTransferFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowOwnershipTransferFlatQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'
import { formatDate, formatDateTime } from '~/lib/formatters'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<OwnershipTransferFlatRow, DialogType>('OwnershipTransfer')

const DataTableRowActions = createRowActions<OwnershipTransferFlatRow>({ useEntity, lifecycle: true, getDetailPath: row => `/internal/ownership-transfer/${row.documentId}` })

function getColumns(t: TFunction): ColumnDef<OwnershipTransferFlatRow>[] {
  return [
    // Document-level columns (groupRole: 'doc' — shown only on first row of group)
    { ...dateColumn<OwnershipTransferFlatRow>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    // Item-level columns (groupRole: 'item' — shown on every row)
    { ...textColumn<OwnershipTransferFlatRow>('productIdName', t('common:table.product'), { primary: false }), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<OwnershipTransferFlatRow>('storageIdName', t('common:columns.storage'), { primary: false }), meta: { label: t('common:columns.storage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<OwnershipTransferFlatRow>('fromContractorIdName', t('common:columns.fromContractor'), { primary: false }), meta: { label: t('common:columns.fromContractor'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<OwnershipTransferFlatRow>('toContractorIdName', t('common:columns.toContractor'), { primary: false }), meta: { label: t('common:columns.toContractor'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<OwnershipTransferFlatRow>('amount', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    { ...statusColumn<OwnershipTransferFlatRow>('status', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Actions (doc-level)
    { ...actionsColumn<OwnershipTransferFlatRow>(DataTableRowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/internal/ownership-transfer/')
const detailRoute = getRouteApi('/_authenticated/internal/ownership-transfer/$id')
const globalFilterFn = createGlobalFilter<OwnershipTransferFlatRow>('productIdName', 'fromContractorIdName', 'toContractorIdName')

function OwnershipTransferTable({ data, actions }: { data: OwnershipTransferFlatRow[], actions?: React.ReactNode }) {
  return (
    <EntityTable
      tableId="ownership-transfer"
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
      groupKey="documentId"
      actions={actions}
    />
  )
}

const formSchema = z.object({
  date: z.string().min(1),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: OwnershipTransferFlatRow | null }) {
  const { t } = useTranslation(['common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: formSchema,
    defaultValues: { date: '' },
    mapRowToForm: row => ({ date: row.date?.split('T')[0] ?? '' }),
    createFn: ownershipDocumentCreate,
    updateFn: ownershipDocumentUpdate,
    queryKey: flowOwnershipTransferFlatQueryQueryKey(),
    entityLabel: t('common:nav.ownershipTransfer'),
    formId: 'ownership-transfer-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.ownershipTransfer')} formId="ownership-transfer-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="ownership-transfer-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({ useEntity, hardDeleteFn: ownershipDocumentHardDelete, softDeleteFn: ownershipDocumentSoftDelete, queryKey: flowOwnershipTransferFlatQueryQueryKey, entityLabel: 'common:nav.ownershipTransfer', i18nNamespaces: ['common'] })

function OwnershipLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: OwnershipTransferFlatRow | null, variant: 'execute' | 'revert' }) {
  const { t } = useTranslation(['common'])
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={ownershipDocumentExecute} revertFn={ownershipDocumentRevert} queryKey={flowOwnershipTransferFlatQueryQueryKey()} entityLabel={t('common:document.ownershipTransfer')} />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: OwnershipLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity })

export function OwnershipTransferPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useFlowOwnershipTransferFlatQuery()

  return <EntityPage provider={Provider} title={t('common:nav.ownershipTransfer')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={OwnershipTransferTable} dialogs={Dialogs} />
}

export function OwnershipTransferDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useOwnershipTransferCompositeGet(id, { embed: 'names' })

  if (isLoading || !data?.data)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data
  const items = doc.items ?? []

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.ownershipTransfer'), entityLabel: t('common:document.ownershipTransfer'), backTo: '/internal/ownership-transfer', executeFn: ownershipDocumentExecute, revertFn: ownershipDocumentRevert, queryKey: flowOwnershipTransferFlatQueryQueryKey(), statusColorMap: statusColors }}
      document={{ id: doc.id, documentNumber: doc.id, status: doc.status }}
      formContent={(
        <div className="grid grid-cols-3 gap-4">
          <DetailField label={t('common:table.date')}>{formatDate(doc.date)}</DetailField>
        </div>
      )}
      itemsContent={(
        <ChildItemsTable
          items={items}
          columns={[
            textColumn<OwnershipTransferItemResponse>('fromContractorIdName', t('common:columns.fromContractor')),
            textColumn<OwnershipTransferItemResponse>('toContractorIdName', t('common:columns.toContractor')),
            textColumn<OwnershipTransferItemResponse>('productIdName', t('common:table.product')),
            textColumn<OwnershipTransferItemResponse>('storageIdName', t('common:columns.storage')),
            numericColumn<OwnershipTransferItemResponse>('amount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'EXECUTED'}
          sectionTitle={t('common:sections.transferItems')}
        />
      )}
      metadataContent={doc.executedAt
        ? (
            <div className="text-sm">
              <span className="text-muted-foreground">
                {t('common:metadata.executedAt')}
                :
              </span>
              {' '}
              {formatDateTime(doc.executedAt)}
            </div>
          )
        : null}
    />
  )
}
