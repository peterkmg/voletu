import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchFlatRow, DispatchItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { DetailField, DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { Skeleton } from '~/components/ui/skeleton'
import { dispatchDocumentCreate, dispatchDocumentExecute, dispatchDocumentHardDelete, dispatchDocumentRevert, dispatchDocumentSoftDelete, dispatchDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useDispatchCompositeGet } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeGet'
import { flowDispatchFlatQueryQueryKey, useFlowDispatchFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowDispatchFlatQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'
import { formatDate, formatDateTime } from '~/lib/formatters'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<DispatchFlatRow, DialogType>('Bunkering')

const DataTableRowActions = createRowActions<DispatchFlatRow>({ useEntity, lifecycle: true, getDetailPath: row => `/outgoing/bunkering/${row.documentId}` })

function getColumns(t: TFunction): ColumnDef<DispatchFlatRow>[] {
  return [
    // Document-level columns (groupRole: 'doc' — shown only on first row of group)
    { ...textColumn<DispatchFlatRow>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<DispatchFlatRow>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<DispatchFlatRow>('contractorIdName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    // Item-level columns (groupRole: 'item' — shown on every row)
    { ...textColumn<DispatchFlatRow>('productIdName', t('common:table.product'), { primary: false }), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<DispatchFlatRow>('storageIdName', t('common:columns.storage'), { primary: false }), meta: { label: t('common:columns.storage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<DispatchFlatRow>('dispatchedAmount', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    { ...statusColumn<DispatchFlatRow>('status', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Actions (doc-level)
    { ...actionsColumn<DispatchFlatRow>(DataTableRowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/outgoing/bunkering/')
const detailRoute = getRouteApi('/_authenticated/outgoing/bunkering/$id')
const globalFilterFn = createGlobalFilter<DispatchFlatRow>('documentNumber', 'productIdName', 'storageIdName')

function BunkeringTable({ data, actions }: { data: DispatchFlatRow[], actions?: React.ReactNode }) {
  return (
    <EntityTable
      tableId="bunkering"
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
  contractorId: z.string().uuid(),
  bunkerType: z.enum(['DOMESTIC', 'EXPORT']),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: DispatchFlatRow | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: formSchema,
    defaultValues: { documentNumber: '', date: '', contractorId: '', bunkerType: 'DOMESTIC' },
    mapRowToForm: row => ({ documentNumber: row.documentNumber, date: row.date?.split('T')[0] ?? '', contractorId: '', bunkerType: 'DOMESTIC' }),
    transformPayload: v => ({ ...v, dispatchMethod: 'BUNKERING' as const, dispatchPurpose: 'EXTERNAL' as const, bunkerType: v.bunkerType as 'DOMESTIC' | 'EXPORT' }),
    createFn: dispatchDocumentCreate,
    updateFn: dispatchDocumentUpdate,
    queryKey: flowDispatchFlatQueryQueryKey({ dispatchMethod: 'BUNKERING' }),
    entityLabel: t('common:nav.bunkering'),
    formId: 'bunkering-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.bunkering')} formId="bunkering-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="bunkering-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
          <EntityPickerField<FormValues> name="contractorId" label={t('common:table.contractor')} queryResult={companiesQuery} />
          <TextField<FormValues> name="bunkerType" label={t('common:columns.bunkerType')} />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({ useEntity, hardDeleteFn: dispatchDocumentHardDelete, softDeleteFn: dispatchDocumentSoftDelete, queryKey: () => flowDispatchFlatQueryQueryKey({ dispatchMethod: 'BUNKERING' }), entityLabel: 'common:nav.bunkering', i18nNamespaces: ['common'] })

function BunkeringLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: DispatchFlatRow | null, variant: 'execute' | 'revert' }) {
  const { t } = useTranslation(['common'])
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={dispatchDocumentExecute} revertFn={dispatchDocumentRevert} queryKey={flowDispatchFlatQueryQueryKey({ dispatchMethod: 'BUNKERING' })} entityLabel={t('common:document.bunkering')} />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: BunkeringLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity })

export function BunkeringPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useFlowDispatchFlatQuery({ dispatchMethod: 'BUNKERING' })

  return <EntityPage provider={Provider} title={t('common:nav.bunkering')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={BunkeringTable} dialogs={Dialogs} />
}

export function BunkeringDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useDispatchCompositeGet(id, { embed: 'names' })

  if (isLoading || !data?.data)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.bunkering'), entityLabel: t('common:document.bunkering'), backTo: '/outgoing/bunkering', executeFn: dispatchDocumentExecute, revertFn: dispatchDocumentRevert, queryKey: flowDispatchFlatQueryQueryKey({ dispatchMethod: 'BUNKERING' }), statusColorMap: statusColors }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={(
        <div className="grid grid-cols-3 gap-4">
          <DetailField label={t('common:table.date')}>{formatDate(doc.date)}</DetailField>
          <DetailField label={t('common:table.contractor')}>{doc.contractorIdName ?? doc.contractorId}</DetailField>
          <DetailField label={t('common:columns.bunkerType')}>{doc.bunkerType ?? '—'}</DetailField>
        </div>
      )}
      itemsContent={(
        <ChildItemsTable
          items={doc.items}
          columns={[
            textColumn<DispatchItemResponse>('productIdName', t('common:table.product')),
            textColumn<DispatchItemResponse>('storageIdName', t('common:columns.storage')),
            numericColumn<DispatchItemResponse>('dispatchedAmount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'EXECUTED'}
          sectionTitle={t('common:sections.dispatchItems')}
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
