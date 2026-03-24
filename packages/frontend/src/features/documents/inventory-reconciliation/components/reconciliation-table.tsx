import type { InventoryReconciliationResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive, Play } from 'lucide-react'
import { useCallback, useMemo } from 'react'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { getReconciliationColumns } from './reconciliation-columns'

const route = getRouteApi('/_authenticated/documents/inventory-reconciliation/')
const globalFilterFn = createGlobalFilter<InventoryReconciliationResponse>('documentNumber')

interface ReconciliationTableProps {
  data: InventoryReconciliationResponse[]
}

export function ReconciliationTable({ data }: ReconciliationTableProps) {
  const { data: warehousesData } = useCatalogWarehouseList()
  const warehouseMap = useMemo(() => {
    const map = new Map<string, string>()
    for (const w of warehousesData?.data ?? []) map.set(w.id, w.commonName)
    return map
  }, [warehousesData])

  const getColumns = useCallback(
    (t: Parameters<typeof getReconciliationColumns>[0]) => getReconciliationColumns(t, { warehouseMap }),
    [warehouseMap],
  )

  return (
    <EntityTable
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['documents', 'common']}
      bulkActions={t => [
        {
          label: t('common:actions.execute'),
          icon: Play,
          onClick: (rows) => {
            const draftRows = rows.filter(r => r.status === 'DRAFT')
            void draftRows // TODO: wire bulk execute API
          },
        },
        {
          label: t('common:actions.softDelete'),
          icon: Archive,
          variant: 'destructive',
          onClick: (rows) => {
            void rows // TODO: wire bulk soft-delete API
          },
        },
      ]}
    />
  )
}
