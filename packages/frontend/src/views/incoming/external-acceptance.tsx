import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { AcceptanceFlatRow, AcceptanceItemResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { DetailField } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { acceptanceDocumentExecute, acceptanceDocumentHardDelete, acceptanceDocumentRevert, acceptanceDocumentSoftDelete } from '~/generated/client'
import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { flowAcceptanceFlatQueryQueryKey, useFlowAcceptanceFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowAcceptanceFlatQuery'
import { statusColors } from '~/lib/badge-colors'
import { defineDocumentViews } from '~/lib/define-document-views'
import { formatDate, formatDateTime } from '~/lib/formatters'
import { AcceptanceMutateDialog } from './acceptance/acceptance-mutate-dialog'

function getColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<AcceptanceFlatRow> }>,
): ColumnDef<AcceptanceFlatRow>[] {
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
    { ...actionsColumn<AcceptanceFlatRow>(RowActions), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/incoming/external/')
const detailRoute = getRouteApi('/_authenticated/incoming/external/$id')
const globalFilterFn = createGlobalFilter<AcceptanceFlatRow>('documentNumber', 'productIdName', 'storageIdName')

function ExternalAcceptanceTable({
  data,
  actions,
  RowActions,
}: {
  data: AcceptanceFlatRow[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<AcceptanceFlatRow> }>
}) {
  return (
    <EntityTable
      tableId="external-acceptance"
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

function useExternalAcceptanceTitle() {
  return useTranslation(['common']).t('common:nav.externalAcceptance')
}

function useExternalAcceptanceText() {
  const title = useExternalAcceptanceTitle()
  const { t } = useTranslation(['common'])

  return {
    title,
    entityLabel: t('common:document.acceptance'),
  }
}

interface ExternalAcceptanceDetailData {
  id: string
  documentNumber: string
  status: string
  dateAccepted: string
  contractorIdName?: string | null
  sourceEntity?: string | null
  items: AcceptanceItemResponse[]
  executedAt?: string | null
}

const externalAcceptanceViewDefinition = defineDocumentViews<AcceptanceFlatRow, ExternalAcceptanceDetailData>({
  displayName: 'ExternalAcceptance',
  useText: useExternalAcceptanceText,
  useQuery: useFlowAcceptanceFlatQuery,
  Table: ExternalAcceptanceTable,
  MutateDialog: AcceptanceMutateDialog,
  deleteDialog: {
    hardDeleteFn: acceptanceDocumentHardDelete,
    softDeleteFn: acceptanceDocumentSoftDelete,
    queryKey: flowAcceptanceFlatQueryQueryKey,
    entityLabel: 'common:nav.externalAcceptance',
    i18nNamespaces: ['common'],
  },
  documentActions: {
    enableRowLifecycle: true,
    executeFn: acceptanceDocumentExecute,
    revertFn: acceptanceDocumentRevert,
    queryKey: flowAcceptanceFlatQueryQueryKey,
  },
  rowActions: {
    getDetailPath: row => `/incoming/external/${row.documentId}`,
  },
  detail: {
    useDetailId: () => detailRoute.useParams().id,
    useDetailQuery: id => useAcceptanceCompositeGet(id, { embed: 'names' }),
    backTo: '/incoming/external',
    statusColorMap: statusColors,
    getDocument: data => ({
      id: data.id,
      documentNumber: data.documentNumber,
      status: data.status,
    }),
    renderFormContent: ({ data, t }) => {
      return (
        <div className="grid grid-cols-3 gap-4">
          <DetailField label={t('common:table.date')}>{formatDate(data.dateAccepted)}</DetailField>
          <DetailField label={t('common:table.contractor')}>{data.contractorIdName ?? '—'}</DetailField>
          <DetailField label={t('common:table.source')}>{data.sourceEntity ?? '—'}</DetailField>
        </div>
      )
    },
    renderItemsContent: ({ data, isLocked, t }) => {
      return (
        <ChildItemsTable
          items={data.items}
          columns={[
            textColumn<AcceptanceItemResponse>('productIdName', t('common:table.product')),
            textColumn<AcceptanceItemResponse>('storageIdName', t('common:columns.storage')),
            numericColumn<AcceptanceItemResponse>('acceptedAmount', t('common:table.quantity')),
          ]}
          isLocked={isLocked}
          sectionTitle={t('common:sections.acceptanceItems')}
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

export function ExternalAcceptancePage() {
  return <externalAcceptanceViewDefinition.View />
}

export function ExternalAcceptanceDetail() {
  return <externalAcceptanceViewDefinition.DetailView />
}
