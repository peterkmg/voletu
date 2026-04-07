import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { AcceptanceFlatRow, AcceptanceItemResponse } from '~/generated/types'
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
import { acceptanceDocumentCreate, acceptanceDocumentExecute, acceptanceDocumentHardDelete, acceptanceDocumentRevert, acceptanceDocumentSoftDelete, acceptanceDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { flowAcceptanceFlatQueryQueryKey, useFlowAcceptanceFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowAcceptanceFlatQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'
import { formatDate, formatDateTime } from '~/lib/formatters'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<AcceptanceFlatRow, DialogType>('ExternalAcceptance')

const DataTableRowActions = createRowActions<AcceptanceFlatRow>({ useEntity, lifecycle: true, getDetailPath: row => `/incoming/external/${row.documentId}` })

function getColumns(t: TFunction): ColumnDef<AcceptanceFlatRow>[] {
  return [
    // Document-level columns (groupRole: 'doc' — shown only on first row of group)
    { ...textColumn<AcceptanceFlatRow>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<AcceptanceFlatRow>('dateAccepted', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<AcceptanceFlatRow>('contractorIdName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    { ...textColumn<AcceptanceFlatRow>('sourceEntity', t('common:table.source'), { primary: false, sizing: 'capped', maxSize: 180 }), meta: { label: t('common:table.source'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Item-level columns (groupRole: 'item' — shown on every row)
    { ...textColumn<AcceptanceFlatRow>('productIdName', t('common:table.product'), { primary: false }), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<AcceptanceFlatRow>('storageIdName', t('common:columns.storage'), { primary: false }), meta: { label: t('common:columns.storage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<AcceptanceFlatRow>('acceptedAmount', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    { ...statusColumn<AcceptanceFlatRow>('status', t('common:table.status'), statusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Actions (doc-level)
    { ...actionsColumn<AcceptanceFlatRow>(DataTableRowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/incoming/external/')
const detailRoute = getRouteApi('/_authenticated/incoming/external/$id')
const globalFilterFn = createGlobalFilter<AcceptanceFlatRow>('documentNumber', 'productIdName', 'storageIdName')

function ExternalAcceptanceTable({ data, actions }: { data: AcceptanceFlatRow[], actions?: React.ReactNode }) {
  return (
    <EntityTable
      tableId="external-acceptance"
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
  dateAccepted: z.string().min(1),
  contractorId: z.string().uuid('Contractor is required'),
  sourceEntity: z.string().nullable().optional(),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: AcceptanceFlatRow | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: formSchema,
    defaultValues: { documentNumber: '', dateAccepted: '', contractorId: '', sourceEntity: '' },
    mapRowToForm: row => ({
      documentNumber: row.documentNumber,
      dateAccepted: row.dateAccepted?.split('T')[0] ?? '',
      contractorId: row.documentId ?? '',
      sourceEntity: row.sourceEntity ?? '',
    }),
    transformPayload: v => ({ ...v, arrivalType: 'EXTERNAL' as const, sourceEntity: v.sourceEntity || null }),
    createFn: acceptanceDocumentCreate,
    updateFn: acceptanceDocumentUpdate,
    queryKey: flowAcceptanceFlatQueryQueryKey(),
    entityLabel: t('common:nav.externalAcceptance'),
    formId: 'external-acceptance-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.externalAcceptance')} formId="external-acceptance-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="external-acceptance-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="dateAccepted" label={t('common:table.date')} type="datetime-local" />
          <EntityPickerField<FormValues> name="contractorId" label={t('common:table.contractor')} queryResult={companiesQuery} />
          <TextField<FormValues> name="sourceEntity" label={t('common:table.source')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({
  useEntity,
  hardDeleteFn: acceptanceDocumentHardDelete,
  softDeleteFn: acceptanceDocumentSoftDelete,
  queryKey: flowAcceptanceFlatQueryQueryKey,
  entityLabel: 'common:nav.externalAcceptance',
  i18nNamespaces: ['common'],
})

function AcceptanceLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: AcceptanceFlatRow | null, variant: 'execute' | 'revert' }) {
  const { t } = useTranslation(['common'])
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={acceptanceDocumentExecute} revertFn={acceptanceDocumentRevert} queryKey={flowAcceptanceFlatQueryQueryKey()} entityLabel={t('common:document.acceptance')} />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: AcceptanceLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity })

export function ExternalAcceptancePage() {
  const { t } = useTranslation(['common'])
  const queryResult = useFlowAcceptanceFlatQuery()

  return <EntityPage provider={Provider} title={t('common:nav.externalAcceptance')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={ExternalAcceptanceTable} dialogs={Dialogs} />
}

export function ExternalAcceptanceDetail() {
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useAcceptanceCompositeGet(id, { embed: 'names' })

  if (isLoading || !data?.data)
    return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.externalAcceptance'), entityLabel: t('common:document.acceptance'), backTo: '/incoming/external', executeFn: acceptanceDocumentExecute, revertFn: acceptanceDocumentRevert, queryKey: flowAcceptanceFlatQueryQueryKey(), statusColorMap: statusColors }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={(
        <div className="grid grid-cols-3 gap-4">
          <DetailField label={t('common:table.date')}>{formatDate(doc.dateAccepted)}</DetailField>
          <DetailField label={t('common:table.contractor')}>{doc.contractorIdName ?? '—'}</DetailField>
          <DetailField label={t('common:table.source')}>{doc.sourceEntity ?? '—'}</DetailField>
        </div>
      )}
      itemsContent={(
        <ChildItemsTable
          items={doc.items}
          columns={[
            textColumn<AcceptanceItemResponse>('productIdName', t('common:table.product')),
            textColumn<AcceptanceItemResponse>('storageIdName', t('common:columns.storage')),
            numericColumn<AcceptanceItemResponse>('acceptedAmount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'EXECUTED'}
          sectionTitle={t('common:sections.acceptanceItems')}
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
