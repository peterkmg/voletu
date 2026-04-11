import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PhysicalTransferFlatRow, PhysicalTransferItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { physicalDocumentCreate, physicalDocumentExecute, physicalDocumentHardDelete, physicalDocumentRevert, physicalDocumentSoftDelete, physicalDocumentUpdate } from '~/generated/client'
import { usePhysicalTransferCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferCompositeGet'
import { flowPhysicalTransferFlatQueryQueryKey, useFlowPhysicalTransferFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowPhysicalTransferFlatQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { defineDocumentViews } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'

function getColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<PhysicalTransferFlatRow> }>,
): ColumnDef<PhysicalTransferFlatRow>[] {
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
    { ...actionsColumn<PhysicalTransferFlatRow>(RowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/internal/physical-transfer/')
const detailRoute = getRouteApi('/_authenticated/internal/physical-transfer/$id')
const globalFilterFn = createGlobalFilter<PhysicalTransferFlatRow>('documentNumber', 'productIdName')

function PhysicalTransferTable({
  data,
  actions,
  RowActions,
}: {
  data: PhysicalTransferFlatRow[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<PhysicalTransferFlatRow> }>
}) {
  return (
    <EntityTable
      tableId="physical-transfer"
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

function usePhysicalTransferTitle() {
  return useTranslation(['common']).t('common:nav.physicalTransfer')
}

function usePhysicalTransferText() {
  const title = usePhysicalTransferTitle()
  const { t } = useTranslation(['common'])

  return {
    title,
    entityLabel: t('common:document.physicalTransfer'),
  }
}

const formSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
})

type FormValues = z.infer<typeof formSchema>

interface PhysicalTransferDetailData {
  id: string
  documentNumber: string
  status: string
  date: string
  contractorIdName?: string | null
  items?: PhysicalTransferItemResponse[]
  executedAt?: string | null
}

function MutateDialog({ open, onOpenChange, currentRow, onCreated }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: PhysicalTransferFlatRow | null, onCreated?: (id: string) => void }) {
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
    onCreated,
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

const physicalTransferViewDefinition = defineDocumentViews<PhysicalTransferFlatRow, PhysicalTransferDetailData>({
  displayName: 'PhysicalTransfer',
  useText: usePhysicalTransferText,
  useQuery: useFlowPhysicalTransferFlatQuery,
  Table: PhysicalTransferTable,
  MutateDialog,
  deleteDialog: {
    hardDeleteFn: physicalDocumentHardDelete,
    softDeleteFn: physicalDocumentSoftDelete,
    queryKey: flowPhysicalTransferFlatQueryQueryKey,
    entityLabel: 'common:nav.physicalTransfer',
    i18nNamespaces: ['common'],
  },
  documentActions: {
    enableRowLifecycle: true,
    executeFn: physicalDocumentExecute,
    revertFn: physicalDocumentRevert,
    queryKey: flowPhysicalTransferFlatQueryQueryKey,
  },
  rowActions: {
    getDetailPath: row => `/internal/physical-transfer/${row.documentId}`,
  },
  detail: {
    useDetailId: () => detailRoute.useParams().id,
    useDetailQuery: id => usePhysicalTransferCompositeGet(id, { embed: 'names' }),
    backTo: '/internal/physical-transfer',
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
          <DetailField label={t('common:table.contractor')}>{data.contractorIdName ?? '—'}</DetailField>
        </div>
      )
    },
    renderItemsContent: ({ data, isLocked, t }) => {
      return (
        <ChildItemsTable
          items={data.items ?? []}
          columns={[
            textColumn<PhysicalTransferItemResponse>('productIdName', t('common:table.product')),
            textColumn<PhysicalTransferItemResponse>('fromStorageIdName', t('common:columns.fromStorage')),
            textColumn<PhysicalTransferItemResponse>('toStorageIdName', t('common:columns.toStorage')),
            numericColumn<PhysicalTransferItemResponse>('amount', t('common:table.quantity')),
          ]}
          isLocked={isLocked}
          sectionTitle={t('common:sections.transferItems')}
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

export function PhysicalTransferPage() {
  return <physicalTransferViewDefinition.View />
}

export function PhysicalTransferDetail() {
  return <physicalTransferViewDefinition.DetailView />
}
