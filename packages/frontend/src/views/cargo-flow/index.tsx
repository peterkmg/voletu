import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { CargoFlowFlatRow } from '~/generated/types'
import { getRouteApi, useNavigate } from '@tanstack/react-router'
import {
  ArrowDownToLine,
  ArrowLeftRight,
  ArrowUpFromLine,
  ArrowUpRight,
  ChevronDown,
  Plus,
} from 'lucide-react'
import { useTranslation } from 'react-i18next'
import {
  actionsColumn,
  createGlobalFilter,
  DataTableColumnHeader,
  dateColumn,
  EntityTable,
  getStoredTableMode,
  numericColumn,
  statusColumn,
  textColumn,
} from '~/components/data-table'
import { RowActions } from '~/components/data-table/row-actions'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { Badge } from '~/components/ui/badge'
import { Button } from '~/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { Skeleton } from '~/components/ui/skeleton'
import { useFlowCargoFlowFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowCargoFlowFlatQuery'
import { usePageTitle } from '~/hooks/use-page-title'
import { flowTypeColors, statusColors } from '~/lib/badge-colors'
import { cn } from '~/lib/utils'
import { createDocumentViewTargets } from '~/router/view-targets'

function CargoFlowRowActions({ row }: { row: { original: CargoFlowFlatRow } }) {
  const navigate = useNavigate()
  const { t } = useTranslation('common')

  return (
    <RowActions
      actions={[
        {
          label: t('actions.viewDetails'),
          icon: ArrowUpRight,
          inline: true,
          onClick: () =>
            navigate({
              to: `${row.original.flowRoute}/${row.original.documentId}`,
            }),
        },
      ]}
    />
  )
}

function getColumns(t: TFunction): ColumnDef<CargoFlowFlatRow>[] {
  return [
    {
      accessorKey: 'type',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('common:columns.type')}
        />
      ),
      minSize: 130,
      maxSize: 130,
      meta: {
        label: t('common:columns.type'),
        sizingCategory: 'capped',
        groupRole: 'doc' as const,
      },
      cell: ({ row }) => {
        const type = row.getValue('type') as string
        const Icon
          = type === 'Incoming'
            ? ArrowDownToLine
            : type === 'Outgoing'
              ? ArrowUpFromLine
              : ArrowLeftRight
        return (
          <Badge
            variant="outline"
            className={cn('shrink-0', flowTypeColors[type])}
          >
            <Icon className="size-3" />
            {type}
          </Badge>
        )
      },
    },
    {
      ...textColumn<CargoFlowFlatRow>(
        'operation',
        t('common:columns.operation'),
        { primary: false, sizing: 'capped', maxSize: 180 },
      ),
      meta: {
        label: t('common:columns.operation'),
        sizingCategory: 'capped',
        groupRole: 'doc' as const,
      },
    },
    {
      ...textColumn<CargoFlowFlatRow>(
        'documentNumber',
        t('common:table.documentNumber'),
        { sizing: 'capped', maxSize: 200 },
      ),
      meta: {
        label: t('common:table.documentNumber'),
        sizingCategory: 'capped',
        groupRole: 'doc' as const,
      },
    },
    {
      ...dateColumn<CargoFlowFlatRow>('date', t('common:table.date')),
      meta: {
        label: t('common:table.date'),
        sizingCategory: 'capped',
        align: 'left' as const,
        groupRole: 'doc' as const,
      },
    },
    {
      ...textColumn<CargoFlowFlatRow>(
        'contractorName',
        t('common:table.contractor'),
        { primary: false },
      ),
      meta: {
        label: t('common:table.contractor'),
        sizingCategory: 'flex',
        groupRole: 'doc' as const,
      },
    },
    {
      ...textColumn<CargoFlowFlatRow>(
        'productName',
        t('common:table.product'),
        { primary: false },
      ),
      meta: {
        label: t('common:table.product'),
        sizingCategory: 'flex',
        groupRole: 'item' as const,
      },
    },
    {
      ...textColumn<CargoFlowFlatRow>(
        'storageName',
        t('common:columns.storage'),
        { primary: false },
      ),
      meta: {
        label: t('common:columns.storage'),
        sizingCategory: 'flex',
        groupRole: 'item' as const,
      },
    },
    {
      ...numericColumn<CargoFlowFlatRow>(
        'quantity',
        t('common:table.quantity'),
      ),
      meta: {
        label: t('common:table.quantity'),
        sizingCategory: 'capped',
        align: 'right' as const,
        groupRole: 'item' as const,
      },
    },
    {
      ...statusColumn<CargoFlowFlatRow>(
        'status',
        t('common:table.status'),
        statusColors,
      ),
      meta: {
        label: t('common:table.status'),
        sizingCategory: 'capped',
        groupRole: 'doc' as const,
      },
    },
    {
      ...actionsColumn<CargoFlowFlatRow>(CargoFlowRowActions, 1),
      meta: { sizingCategory: 'fixed', groupRole: 'doc' as const },
    },
  ]
}

const route = getRouteApi('/_authenticated/cargo-flow/')
const globalFilterFn = createGlobalFilter<CargoFlowFlatRow>(
  'documentNumber',
  'contractorName',
  'operation',
  'productName',
  'storageName',
)

function CargoFlowTable({
  data,
  pageCount,
  isPaginated,
  actions,
}: {
  data: CargoFlowFlatRow[]
  pageCount: number
  isPaginated: boolean
  actions?: React.ReactNode
}) {
  return (
    <EntityTable
      tableId="cargo-flow"
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
      groupKey="documentId"
      actions={actions}
      serverPagination={isPaginated ? { pageCount } : undefined}
      manualFiltering={isPaginated}
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
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.truckReceipt)}
          >
            {t('nav.truckReceipt')}
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.railReceipt)}
          >
            {t('nav.railReceipt')}
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.externalAcceptance)}
          >
            {t('nav.externalAcceptance')}
          </DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuLabel>{t('nav.outgoing')}</DropdownMenuLabel>
        <DropdownMenuGroup>
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.truckDispatch)}
          >
            {t('nav.truckDispatch')}
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.directDispatch)}
          >
            {t('nav.directDispatch')}
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.bunkering)}
          >
            {t('nav.bunkering')}
          </DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuLabel>{t('nav.internal')}</DropdownMenuLabel>
        <DropdownMenuGroup>
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.physicalTransfer)}
          >
            {t('nav.physicalTransfer')}
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.ownershipTransfer)}
          >
            {t('nav.ownershipTransfer')}
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.blending)}
          >
            {t('nav.blending')}
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => navigate(createDocumentViewTargets.reconciliation)}
          >
            {t('nav.reconciliation')}
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

export function CargoFlowPage() {
  const { t } = useTranslation('common')
  const search = route.useSearch()
  const isPaginated = getStoredTableMode('cargo-flow') === 'paginated'
  const page = isPaginated ? (search.page ?? 1) : 1
  const pageSize = isPaginated ? (search.pageSize ?? 10) : 9999
  const filter = search.filter?.trim() || undefined
  const { data, isLoading } = useFlowCargoFlowFlatQuery({
    page,
    perPage: pageSize,
    filter,
  })

  usePageTitle(t('nav.cargoFlow'))

  const payload = data?.data
  const pageCount = payload ? Math.ceil(payload.total / pageSize) : 0

  return (
    <>
      <Header fixed />
      <Main fixed className="flex flex-1 flex-col gap-4">
        {isLoading
          ? (
              <div className="flex flex-1 flex-col gap-4">
                <Skeleton className="h-9 w-64" />
                <div className="flex-1 rounded-md border">
                  <div className="space-y-3 p-4">
                    {Array.from({ length: 8 }, (_, i) => (
                      <Skeleton key={i} className="h-8 w-full" />
                    ))}
                  </div>
                </div>
              </div>
            )
          : (
              <div className="flex flex-1 flex-col min-h-0">
                <CargoFlowTable
                  data={payload?.items ?? []}
                  pageCount={pageCount}
                  isPaginated={isPaginated}
                  actions={<CreateDropdown />}
                />
              </div>
            )}
      </Main>
    </>
  )
}
