import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import { getRouteApi, useNavigate } from '@tanstack/react-router'
import { ChevronDown, Plus, Search } from 'lucide-react'
import { useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { createGlobalFilter, dateColumn, EntityTable, statusColumn, textColumn } from '~/components/data-table'
import { RowActions } from '~/components/data-table/row-actions'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { Button } from '~/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '~/components/ui/dropdown-menu'
import { Skeleton } from '~/components/ui/skeleton'
import { useAcceptanceDocumentQuery } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentQuery'
import { useDispatchDocumentQuery } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentQuery'
import { useBlendingDocumentList } from '~/generated/hooks/DocumentOperationsHooks/useBlendingDocumentList'
import { useOwnershipTransferList } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferList'
import { usePhysicalTransferList } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { useReconciliationList } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationList'
import { useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { useTruckDispatchPipelineQuery } from '~/generated/hooks/FlowsHooks/useTruckDispatchPipelineQuery'
import { documentStatusColors, pipelineStatusColors } from '~/lib/badge-colors'

interface CargoFlowRow {
  id: string
  documentNumber: string
  date: string
  type: string
  operation: string
  contractorName: string
  status: string
  flowRoute: string
}

function CargoFlowRowActions({ row }: { row: { original: CargoFlowRow } }) {
  const navigate = useNavigate()
  const { t } = useTranslation('common')

  return (
    <RowActions
      actions={[
        {
          label: t('actions.viewDetails'),
          icon: Search,
          inline: true,
          onClick: () => navigate({ to: `${row.original.flowRoute}/${row.original.id}` }),
        },
      ]}
    />
  )
}

const cargoFlowStatusColors = { ...documentStatusColors, ...pipelineStatusColors }

function getColumns(t: TFunction): ColumnDef<CargoFlowRow>[] {
  return [
    textColumn<CargoFlowRow>('type', 'Type', { primary: false }),
    textColumn<CargoFlowRow>('operation', 'Operation', { primary: false }),
    textColumn<CargoFlowRow>('documentNumber', t('common:table.documentNumber')),
    dateColumn<CargoFlowRow>('date', t('common:table.date')),
    textColumn<CargoFlowRow>('contractorName', t('common:table.contractor'), { primary: false }),
    statusColumn<CargoFlowRow>('status', t('common:table.status'), cargoFlowStatusColors),
    { id: 'actions', cell: ({ row }) => <CargoFlowRowActions row={row} />, size: 48, enableHiding: false },
  ]
}

const route = getRouteApi('/_authenticated/cargo-flow/')
const globalFilterFn = createGlobalFilter<CargoFlowRow>('documentNumber', 'contractorName', 'operation')

function useCargoFlowData(): { data: CargoFlowRow[], isLoading: boolean } {
  const truckReceipt = useFlowTruckReceiptQuery()
  const railReceipt = useRailReceiptPipelineQuery()
  const truckDispatch = useTruckDispatchPipelineQuery()
  const externalAcceptance = useAcceptanceDocumentQuery({ truckWaybillId: 'isNull' as any, railWaybillId: 'isNull' as any, transitDispatchId: 'isNull' as any })
  const directDispatch = useDispatchDocumentQuery({ dispatchMethod: 'VESSEL_TERMINAL' as any, dispatchPurpose: 'EXTERNAL' as any, embed: 'names' })
  const bunkering = useDispatchDocumentQuery({ dispatchMethod: 'BUNKERING' as any, embed: 'names' })
  const blending = useBlendingDocumentList({ embed: 'names' })
  const physical = usePhysicalTransferList()
  const ownership = useOwnershipTransferList()
  const reconciliation = useReconciliationList()

  const isLoading = truckReceipt.isLoading || railReceipt.isLoading || truckDispatch.isLoading

  const data = useMemo(() => {
    const rows: CargoFlowRow[] = []

    for (const r of truckReceipt.data?.data ?? []) {
      rows.push({ id: r.pipelineStatus === 'PENDING' ? r.id : (r.actionId ?? r.id), documentNumber: r.actionDocumentNumber ?? r.basisDocumentNumber, date: r.basisDate, type: 'Incoming', operation: 'Truck Receipt', contractorName: r.contractorName, status: r.pipelineStatus, flowRoute: '/incoming/truck' })
    }
    for (const r of railReceipt.data?.data ?? []) {
      rows.push({ id: r.pipelineStatus === 'PENDING' ? r.id : (r.actionId ?? r.id), documentNumber: r.actionDocumentNumber ?? r.basisDocumentNumber, date: r.basisDate, type: 'Incoming', operation: 'Rail Receipt', contractorName: r.contractorName, status: r.pipelineStatus, flowRoute: '/incoming/rail' })
    }
    for (const r of truckDispatch.data?.data ?? []) {
      rows.push({ id: r.id, documentNumber: r.documentNumber, date: r.date, type: 'Outgoing', operation: 'Truck Dispatch', contractorName: r.contractorName, status: r.pipelineStatus, flowRoute: '/outgoing/truck' })
    }
    for (const r of externalAcceptance.data?.data ?? []) {
      rows.push({ id: r.id, documentNumber: r.documentNumber, date: r.dateAccepted, type: 'Incoming', operation: 'External Acceptance', contractorName: '', status: r.status, flowRoute: '/incoming/external' })
    }
    for (const r of directDispatch.data?.data ?? []) {
      rows.push({ id: r.id, documentNumber: r.documentNumber, date: r.date, type: 'Outgoing', operation: 'Direct Dispatch', contractorName: r.contractorIdName ?? '', status: r.status, flowRoute: '/outgoing/direct' })
    }
    for (const r of bunkering.data?.data ?? []) {
      rows.push({ id: r.id, documentNumber: r.documentNumber, date: r.date, type: 'Outgoing', operation: 'Bunkering', contractorName: r.contractorIdName ?? '', status: r.status, flowRoute: '/outgoing/bunkering' })
    }
    for (const r of blending.data?.data ?? []) {
      rows.push({ id: r.id, documentNumber: r.documentNumber, date: r.date, type: 'Internal', operation: 'Blending', contractorName: r.contractorIdName ?? '', status: r.status, flowRoute: '/internal/blending' })
    }
    for (const r of physical.data?.data ?? []) {
      rows.push({ id: r.id, documentNumber: r.documentNumber, date: r.date, type: 'Internal', operation: 'Physical Transfer', contractorName: '', status: r.status, flowRoute: '/internal/physical-transfer' })
    }
    for (const r of ownership.data?.data ?? []) {
      rows.push({ id: r.id, documentNumber: r.id, date: r.date, type: 'Internal', operation: 'Ownership Transfer', contractorName: '', status: r.status, flowRoute: '/internal/ownership-transfer' })
    }
    for (const r of reconciliation.data?.data ?? []) {
      rows.push({ id: r.id, documentNumber: r.documentNumber, date: r.date, type: 'Internal', operation: 'Reconciliation', contractorName: '', status: r.status, flowRoute: '/internal/reconciliation' })
    }

    rows.sort((a, b) => b.date.localeCompare(a.date))
    return rows
  }, [truckReceipt.data, railReceipt.data, truckDispatch.data, externalAcceptance.data, directDispatch.data, bunkering.data, blending.data, physical.data, ownership.data, reconciliation.data])

  return { data, isLoading }
}

function CargoFlowTable({ data }: { data: CargoFlowRow[] }) {
  return (
    <EntityTable
      tableId="cargo-flow"
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
    />
  )
}

function CreateDropdown() {
  const navigate = useNavigate()
  const { t } = useTranslation('common')

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button size="sm">
          <Plus className="mr-1 size-4" />
          {t('actions.create')}
          <ChevronDown className="ml-1 size-3" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-56">
        <DropdownMenuLabel>{t('nav.incoming')}</DropdownMenuLabel>
        <DropdownMenuGroup>
          <DropdownMenuItem onClick={() => navigate({ to: '/incoming/truck', search: { create: true } })}>{t('nav.truckReceipt')}</DropdownMenuItem>
          <DropdownMenuItem onClick={() => navigate({ to: '/incoming/rail', search: { create: true } })}>{t('nav.railReceipt')}</DropdownMenuItem>
          <DropdownMenuItem onClick={() => navigate({ to: '/incoming/external', search: { create: true } })}>{t('nav.externalAcceptance')}</DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuLabel>{t('nav.outgoing')}</DropdownMenuLabel>
        <DropdownMenuGroup>
          <DropdownMenuItem onClick={() => navigate({ to: '/outgoing/truck', search: { create: true } })}>{t('nav.truckDispatch')}</DropdownMenuItem>
          <DropdownMenuItem onClick={() => navigate({ to: '/outgoing/direct', search: { create: true } })}>{t('nav.directDispatch')}</DropdownMenuItem>
          <DropdownMenuItem onClick={() => navigate({ to: '/outgoing/bunkering', search: { create: true } })}>{t('nav.bunkering')}</DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuLabel>{t('nav.internal')}</DropdownMenuLabel>
        <DropdownMenuGroup>
          <DropdownMenuItem onClick={() => navigate({ to: '/internal/physical-transfer', search: { create: true } })}>{t('nav.physicalTransfer')}</DropdownMenuItem>
          <DropdownMenuItem onClick={() => navigate({ to: '/internal/ownership-transfer', search: { create: true } })}>{t('nav.ownershipTransfer')}</DropdownMenuItem>
          <DropdownMenuItem onClick={() => navigate({ to: '/internal/blending', search: { create: true } })}>{t('nav.blending')}</DropdownMenuItem>
          <DropdownMenuItem onClick={() => navigate({ to: '/internal/reconciliation', search: { create: true } })}>{t('nav.reconciliation')}</DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

export function CargoFlowPage() {
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useCargoFlowData()

  return (
    <>
      <Header fixed />
      <Main fixed className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <h2 className="text-2xl font-bold tracking-tight">{t('common:nav.cargoFlow')}</h2>
          <CreateDropdown />
        </div>
        {isLoading
          ? (
              <div className="flex flex-1 flex-col gap-4">
                <Skeleton className="h-9 w-64" />
                <div className="flex-1 rounded-md border">
                  <div className="space-y-3 p-4">
                    {Array.from({ length: 8 }, (_, i) => <Skeleton key={i} className="h-8 w-full" />)}
                  </div>
                </div>
              </div>
            )
          : (
              <div className="flex flex-1 flex-col min-h-0">
                <CargoFlowTable data={data} />
              </div>
            )}
      </Main>
    </>
  )
}
