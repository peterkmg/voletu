import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { CargoFlowFlatRow } from '~/generated/types'
import { getRouteApi, useNavigate } from '@tanstack/react-router'
import { ArrowDownToLine, ArrowLeftRight, ArrowUpFromLine, ChevronDown, Plus, Search } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { actionsColumn, createGlobalFilter, DataTableColumnHeader, dateColumn, EntityTable, numericColumn, statusColumn, textColumn } from '~/components/data-table'
import { RowActions } from '~/components/data-table/row-actions'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { Badge } from '~/components/ui/badge'
import { Button } from '~/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '~/components/ui/dropdown-menu'
import { Skeleton } from '~/components/ui/skeleton'
import { useFlowCargoFlowFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowCargoFlowFlatQuery'
import { flowTypeColors, statusColors } from '~/lib/badge-colors'
import { cn } from '~/lib/utils'

function CargoFlowRowActions({ row }: { row: { original: CargoFlowFlatRow } }) {
  const navigate = useNavigate()
  const { t } = useTranslation('common')

  return (
    <RowActions
      actions={[
        {
          label: t('actions.viewDetails'),
          icon: Search,
          inline: true,
          onClick: () => navigate({ to: `${row.original.flowRoute}/${row.original.documentId}` }),
        },
      ]}
    />
  )
}

const cargoFlowStatusColors = statusColors

function getColumns(t: TFunction): ColumnDef<CargoFlowFlatRow>[] {
  return [
    // Document-level columns (groupRole: 'doc' — shown only on first row of group)
    {
      accessorKey: 'type',
      header: ({ column }) => <DataTableColumnHeader column={column} title="Type" />,
      minSize: 130,
      maxSize: 130,
      meta: { label: 'Type', sizingCategory: 'capped', groupRole: 'doc' as const },
      cell: ({ row }) => {
        const type = row.getValue('type') as string
        const Icon = type === 'Incoming' ? ArrowDownToLine : type === 'Outgoing' ? ArrowUpFromLine : ArrowLeftRight
        return (
          <Badge variant="outline" className={cn('shrink-0', flowTypeColors[type])}>
            <Icon className="size-3" />
            {type}
          </Badge>
        )
      },
    },
    { ...textColumn<CargoFlowFlatRow>('operation', 'Operation', { primary: false, sizing: 'capped', maxSize: 180 }), meta: { label: 'Operation', sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...textColumn<CargoFlowFlatRow>('documentNumber', t('common:table.documentNumber'), { sizing: 'capped', maxSize: 200 }), meta: { label: t('common:table.documentNumber'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    { ...dateColumn<CargoFlowFlatRow>('date', t('common:table.date')), meta: { label: t('common:table.date'), sizingCategory: 'capped', align: 'left' as const, groupRole: 'doc' as const } },
    { ...textColumn<CargoFlowFlatRow>('contractorName', t('common:table.contractor'), { primary: false }), meta: { label: t('common:table.contractor'), sizingCategory: 'flex', groupRole: 'doc' as const } },
    { ...statusColumn<CargoFlowFlatRow>('status', t('common:table.status'), cargoFlowStatusColors), meta: { label: t('common:table.status'), sizingCategory: 'capped', groupRole: 'doc' as const } },
    // Item-level columns (groupRole: 'item' — shown on every row)
    { ...textColumn<CargoFlowFlatRow>('productName', t('common:table.product')), meta: { label: t('common:table.product'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...textColumn<CargoFlowFlatRow>('storageName', t('common:columns.storage')), meta: { label: t('common:columns.storage'), sizingCategory: 'flex', groupRole: 'item' as const } },
    { ...numericColumn<CargoFlowFlatRow>('quantity', t('common:table.quantity')), meta: { label: t('common:table.quantity'), sizingCategory: 'capped', align: 'right' as const, groupRole: 'item' as const } },
    // Actions (doc-level)
    { ...actionsColumn<CargoFlowFlatRow>(CargoFlowRowActions, 1), meta: { sizingCategory: 'fixed', groupRole: 'doc' as const } },
  ]
}

const route = getRouteApi('/_authenticated/cargo-flow/')
const globalFilterFn = createGlobalFilter<CargoFlowFlatRow>('documentNumber', 'contractorName', 'operation', 'productName', 'storageName')

function CargoFlowTable({ data }: { data: CargoFlowFlatRow[] }) {
  return (
    <EntityTable
      tableId="cargo-flow"
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
      groupKey="documentId"
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
  const { data, isLoading } = useFlowCargoFlowFlatQuery()

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
                <CargoFlowTable data={data?.data ?? []} />
              </div>
            )}
      </Main>
    </>
  )
}
