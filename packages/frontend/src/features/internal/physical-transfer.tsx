import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PhysicalTransferFlatRow, PhysicalTransferItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { Skeleton } from '~/components/ui/skeleton'
import { physicalDocumentCreate, physicalDocumentExecute, physicalDocumentHardDelete, physicalDocumentRevert, physicalDocumentSoftDelete, physicalDocumentUpdate } from '~/generated/client'
import { usePhysicalTransferCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferCompositeGet'
import { flowPhysicalTransferFlatQueryQueryKey, useFlowPhysicalTransferFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowPhysicalTransferFlatQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<PhysicalTransferFlatRow, DialogType>('PhysicalTransfer')

const DataTableRowActions = createRowActions<PhysicalTransferFlatRow>({ useEntity, lifecycle: true, getDetailPath: row => `/internal/physical-transfer/${row.documentId}` })

function getColumns(t: TFunction): ColumnDef<PhysicalTransferFlatRow>[] {
  return [
    // Document-level columns (groupRole: 'doc' — shown only on first row of group)
    { ...textColumn<PhysicalTransferFlatRow>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<PhysicalTransferFlatRow>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<PhysicalTransferFlatRow>('contractorIdName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    // Item-level columns (groupRole: 'item' — shown on every row)
    { ...textColumn<PhysicalTransferFlatRow>('productIdName', t('common:table.product'), { primary: false }), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<PhysicalTransferFlatRow>('fromStorageIdName', t('common:columns.fromStorage'), { primary: false }), meta: { label: t('common:columns.fromStorage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<PhysicalTransferFlatRow>('toStorageIdName', t('common:columns.toStorage'), { primary: false }), meta: { label: t('common:columns.toStorage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<PhysicalTransferFlatRow>('amount', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    { ...statusColumn<PhysicalTransferFlatRow>('status', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Actions (doc-level)
    { ...actionsColumn<PhysicalTransferFlatRow>(DataTableRowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/internal/physical-transfer/')
const detailRoute = getRouteApi('/_authenticated/internal/physical-transfer/$id')
const globalFilterFn = createGlobalFilter<PhysicalTransferFlatRow>('documentNumber', 'productIdName')

function PhysicalTransferTable({ data, actions }: { data: PhysicalTransferFlatRow[], actions?: React.ReactNode }) {
  return (
    <EntityTable
      tableId="physical-transfer"
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
  documentNumber: z.string().min(1),
  date: z.string().min(1),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: PhysicalTransferFlatRow | null }) {
  const { t } = useTranslation(['common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: formSchema,
    defaultValues: { documentNumber: '', date: '' },
    mapRowToForm: row => ({ documentNumber: row.documentNumber, date: row.date?.split('T')[0] ?? '' }),
    createFn: physicalDocumentCreate,
    updateFn: physicalDocumentUpdate,
    queryKey: flowPhysicalTransferFlatQueryQueryKey(),
    entityLabel: t('common:nav.physicalTransfer'),
    formId: 'physical-transfer-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.physicalTransfer')} formId="physical-transfer-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="physical-transfer-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({ useEntity, hardDeleteFn: physicalDocumentHardDelete, softDeleteFn: physicalDocumentSoftDelete, queryKey: flowPhysicalTransferFlatQueryQueryKey, entityLabel: 'common:nav.physicalTransfer', i18nNamespaces: ['common'] })

function PhysicalLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: PhysicalTransferFlatRow | null, variant: 'execute' | 'revert' }) {
  const { t } = useTranslation(['common'])
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={physicalDocumentExecute} revertFn={physicalDocumentRevert} queryKey={flowPhysicalTransferFlatQueryQueryKey()} entityLabel={t('common:document.physicalTransfer')} />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: PhysicalLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity })

export function PhysicalTransferPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useFlowPhysicalTransferFlatQuery()

  return <EntityPage provider={Provider} title={t('common:nav.physicalTransfer')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={PhysicalTransferTable} dialogs={Dialogs} />
}

export function PhysicalTransferDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = usePhysicalTransferCompositeGet(id, { embed: 'names' })

  if (isLoading || !data?.data)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data
  const items = doc.items ?? []

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.physicalTransfer'), entityLabel: t('common:document.physicalTransfer'), backTo: '/internal/physical-transfer', executeFn: physicalDocumentExecute, revertFn: physicalDocumentRevert, queryKey: flowPhysicalTransferFlatQueryQueryKey(), statusColorMap: statusColors }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={(
        <div className="grid grid-cols-3 gap-4">
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.date')}</span>
            <p>{formatDate(doc.date)}</p>
          </div>
          <div>
            <span className="text-sm text-muted-foreground">{t('common:table.contractor')}</span>
            <p>{doc.contractorIdName ?? '—'}</p>
          </div>
        </div>
      )}
      itemsContent={(
        <ChildItemsTable
          items={items}
          columns={[
            textColumn<PhysicalTransferItemResponse>('productIdName', t('common:table.product')),
            textColumn<PhysicalTransferItemResponse>('fromStorageIdName', t('common:columns.fromStorage')),
            textColumn<PhysicalTransferItemResponse>('toStorageIdName', t('common:columns.toStorage')),
            numericColumn<PhysicalTransferItemResponse>('amount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'EXECUTED'}
          sectionTitle={t('common:sections.transferItems')}
        />
      )}
      metadataContent={doc.executedAt
        ? (
            <div className="text-sm">
              <span className="text-muted-foreground">{t('common:metadata.executedAt')}:</span>
              {' '}
              {formatDateTime(doc.executedAt)}
            </div>
          )
        : null}
    />
  )
}
