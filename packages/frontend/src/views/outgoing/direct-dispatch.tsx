import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchFlatRow, DispatchItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { dispatchDocumentCreate, dispatchDocumentExecute, dispatchDocumentHardDelete, dispatchDocumentRevert, dispatchDocumentSoftDelete, dispatchDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useDispatchCompositeGet } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeGet'
import { flowDispatchFlatQueryQueryKey, useFlowDispatchFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowDispatchFlatQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { defineDocumentViews } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'

function getColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<DispatchFlatRow> }>,
): ColumnDef<DispatchFlatRow>[] {
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
    { ...actionsColumn<DispatchFlatRow>(RowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/outgoing/direct/')
const detailRoute = getRouteApi('/_authenticated/outgoing/direct/$id')
const globalFilterFn = createGlobalFilter<DispatchFlatRow>('documentNumber', 'productIdName', 'storageIdName')

function DirectDispatchTable({
  data,
  actions,
  RowActions,
}: {
  data: DispatchFlatRow[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<DispatchFlatRow> }>
}) {
  return (
    <EntityTable
      tableId="direct-dispatch"
      data={data}
      getColumns={t => getColumns(t, RowActions)}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
      groupKey="documentId"
      actions={actions}
    />
  )
}

function useDirectDispatchTitle() {
  return useTranslation(['common']).t('common:nav.directDispatch')
}

function useDirectDispatchText() {
  const title = useDirectDispatchTitle()
  const { t } = useTranslation(['common'])

  return {
    title,
    entityLabel: t('common:document.directDispatch'),
  }
}

const formSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  contractorId: z.string().uuid(),
})

type FormValues = z.infer<typeof formSchema>

interface DirectDispatchDetailData {
  id: string
  documentNumber: string
  status: string
  date: string
  contractorId?: string | null
  contractorIdName?: string | null
  items: DispatchItemResponse[]
  executedAt?: string | null
}

function MutateDialog({ open, onOpenChange, currentRow, onCreated }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: DispatchFlatRow | null, onCreated?: (id: string) => void }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: formSchema,
    defaultValues: { documentNumber: '', date: '', contractorId: '' },
    mapRowToForm: row => ({ documentNumber: row.documentNumber, date: row.date?.split('T')[0] ?? '', contractorId: '' }),
    transformPayload: v => ({ ...v, dispatchMethod: 'VESSEL_TERMINAL' as const, dispatchPurpose: 'EXTERNAL' as const }),
    createFn: dispatchDocumentCreate,
    updateFn: dispatchDocumentUpdate,
    queryKey: flowDispatchFlatQueryQueryKey({ dispatchMethod: 'VESSEL_TERMINAL', dispatchPurpose: 'EXTERNAL' }),
    entityLabel: t('common:nav.directDispatch'),
    formId: 'direct-dispatch-form',
    onCreated,
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.directDispatch')} formId="direct-dispatch-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="direct-dispatch-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
          <EntityPickerField<FormValues> name="contractorId" label={t('common:table.contractor')} queryResult={companiesQuery} />
        </form>
      </Form>
    </FormDialog>
  )
}

const directDispatchViewDefinition = defineDocumentViews<DispatchFlatRow, DirectDispatchDetailData>({
  displayName: 'DirectDispatch',
  useText: useDirectDispatchText,
  useQuery: () => useFlowDispatchFlatQuery({ dispatchMethod: 'VESSEL_TERMINAL', dispatchPurpose: 'EXTERNAL' }),
  Table: DirectDispatchTable,
  MutateDialog,
  deleteDialog: {
    hardDeleteFn: dispatchDocumentHardDelete,
    softDeleteFn: dispatchDocumentSoftDelete,
    queryKey: () => flowDispatchFlatQueryQueryKey({ dispatchMethod: 'VESSEL_TERMINAL', dispatchPurpose: 'EXTERNAL' }),
    entityLabel: 'common:nav.directDispatch',
    i18nNamespaces: ['common'],
  },
  documentActions: {
    enableRowLifecycle: true,
    executeFn: dispatchDocumentExecute,
    revertFn: dispatchDocumentRevert,
    queryKey: () => flowDispatchFlatQueryQueryKey({ dispatchMethod: 'VESSEL_TERMINAL', dispatchPurpose: 'EXTERNAL' }),
  },
  rowActions: {
    getDetailPath: row => `/outgoing/direct/${row.documentId}`,
  },
  detail: {
    useDetailId: () => detailRoute.useParams().id,
    useDetailQuery: id => useDispatchCompositeGet(id, { embed: 'names' }),
    backTo: '/outgoing/direct',
    statusColorMap: statusColors,
    getDocument: data => ({
      id: data.id,
      documentNumber: data.documentNumber,
      status: data.status,
    }),
    renderFormContent: ({ data, t }) => {
      return (
        <div className="grid grid-cols-3 gap-4">
          <DetailField label={t('common:table.date')}>{formatDate(data.date)}</DetailField>
          <DetailField label={t('common:table.contractor')}>{data.contractorIdName ?? data.contractorId}</DetailField>
        </div>
      )
    },
    renderItemsContent: ({ data, isLocked, t }) => {
      return (
        <ChildItemsTable
          items={data.items}
          columns={[
            textColumn<DispatchItemResponse>('productIdName', t('common:table.product')),
            textColumn<DispatchItemResponse>('storageIdName', t('common:columns.storage')),
            numericColumn<DispatchItemResponse>('dispatchedAmount', t('common:table.quantity')),
          ]}
          isLocked={isLocked}
          sectionTitle={t('common:sections.dispatchItems')}
        />
      )
    },
    renderMetadataContent: ({ data, t }) => {
      if (!data.executedAt) {
        return null
      }
      return (
        <div className="text-sm">
          <span className="text-muted-foreground">
            {t('common:metadata.executedAt')}
            :
          </span>
          {' '}
          {formatDateTime(data.executedAt)}
        </div>
      )
    },
  },
})

export function DirectDispatchPage() {
  return <directDispatchViewDefinition.View />
}

export function DirectDispatchDetail() {
  return <directDispatchViewDefinition.DetailView />
}
