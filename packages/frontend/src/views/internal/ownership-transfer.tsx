import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { OwnershipTransferFlatRow, OwnershipTransferItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { ownershipDocumentCreate, ownershipDocumentExecute, ownershipDocumentHardDelete, ownershipDocumentRevert, ownershipDocumentSoftDelete, ownershipDocumentUpdate } from '~/generated/client'
import { useOwnershipTransferCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferCompositeGet'
import { flowOwnershipTransferFlatQueryQueryKey, useFlowOwnershipTransferFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowOwnershipTransferFlatQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { statusColors } from '~/lib/badge-colors'
import { defineDocumentViews } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'

function getColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<OwnershipTransferFlatRow> }>,
): ColumnDef<OwnershipTransferFlatRow>[] {
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
    { ...actionsColumn<OwnershipTransferFlatRow>(RowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/internal/ownership-transfer/')
const detailRoute = getRouteApi('/_authenticated/internal/ownership-transfer/$id')
const globalFilterFn = createGlobalFilter<OwnershipTransferFlatRow>('productIdName', 'fromContractorIdName', 'toContractorIdName')

function OwnershipTransferTable({
  data,
  actions,
  RowActions,
}: {
  data: OwnershipTransferFlatRow[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<OwnershipTransferFlatRow> }>
}) {
  return (
    <EntityTable
      tableId="ownership-transfer"
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

function useOwnershipTransferTitle() {
  return useTranslation(['common']).t('common:nav.ownershipTransfer')
}

function useOwnershipTransferText() {
  const title = useOwnershipTransferTitle()
  const { t } = useTranslation(['common'])

  return {
    title,
    entityLabel: t('common:document.ownershipTransfer'),
  }
}

const formSchema = z.object({
  date: z.string().min(1),
})

type FormValues = z.infer<typeof formSchema>

interface OwnershipTransferDetailData {
  id: string
  status: string
  date: string
  items?: OwnershipTransferItemResponse[]
  executedAt?: string | null
}

function MutateDialog({ open, onOpenChange, currentRow, onCreated }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: OwnershipTransferFlatRow | null, onCreated?: (id: string) => void }) {
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
    onCreated,
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

const ownershipTransferViewDefinition = defineDocumentViews<OwnershipTransferFlatRow, OwnershipTransferDetailData>({
  displayName: 'OwnershipTransfer',
  useText: useOwnershipTransferText,
  useQuery: useFlowOwnershipTransferFlatQuery,
  Table: OwnershipTransferTable,
  MutateDialog,
  deleteDialog: {
    hardDeleteFn: ownershipDocumentHardDelete,
    softDeleteFn: ownershipDocumentSoftDelete,
    queryKey: flowOwnershipTransferFlatQueryQueryKey,
    entityLabel: 'common:nav.ownershipTransfer',
    i18nNamespaces: ['common'],
  },
  documentActions: {
    enableRowLifecycle: true,
    executeFn: ownershipDocumentExecute,
    revertFn: ownershipDocumentRevert,
    queryKey: flowOwnershipTransferFlatQueryQueryKey,
  },
  rowActions: {
    getDetailPath: row => `/internal/ownership-transfer/${row.documentId}`,
  },
  detail: {
    useDetailId: () => detailRoute.useParams().id,
    useDetailQuery: id => useOwnershipTransferCompositeGet(id, { embed: 'names' }),
    backTo: '/internal/ownership-transfer',
    statusColorMap: statusColors,
    getDocument: data => ({
      id: data.id,
      documentNumber: data.id,
      status: data.status,
    }),
    renderFormContent: ({ data, t }) => {
      return (
        <div className="grid grid-cols-3 gap-4">
          <DetailField label={t('common:table.date')}>{formatDate(data.date)}</DetailField>
        </div>
      )
    },
    renderItemsContent: ({ data, isLocked, t }) => {
      return (
        <ChildItemsTable
          items={data.items ?? []}
          columns={[
            textColumn<OwnershipTransferItemResponse>('fromContractorIdName', t('common:columns.fromContractor')),
            textColumn<OwnershipTransferItemResponse>('toContractorIdName', t('common:columns.toContractor')),
            textColumn<OwnershipTransferItemResponse>('productIdName', t('common:table.product')),
            textColumn<OwnershipTransferItemResponse>('storageIdName', t('common:columns.storage')),
            numericColumn<OwnershipTransferItemResponse>('amount', t('common:table.quantity')),
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

export function OwnershipTransferPage() {
  return <ownershipTransferViewDefinition.View />
}

export function OwnershipTransferDetail() {
  return <ownershipTransferViewDefinition.DetailView />
}
