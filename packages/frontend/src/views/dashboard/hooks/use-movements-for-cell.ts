import type { CargoFlowFlatRow } from '~/generated/types/CargoFlowFlatRow'
import { useMemo } from 'react'
import { useFlowCargoFlowFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowCargoFlowFlatQuery'

export interface CellMovementsResult {
  movements: CargoFlowFlatRow[]
  isLoading: boolean
  isError: boolean
  error: unknown
}

export function useMovementsForCell(params: {
  contractorName: string | null
  productName: string | null
  storageName: string | null
  enabled: boolean
  limit?: number
}): CellMovementsResult {
  const q = useFlowCargoFlowFlatQuery(undefined, { query: { enabled: params.enabled } })

  const movements = useMemo(() => {
    const rows = ((q.data as any)?.data ?? []) as CargoFlowFlatRow[]

    if (!params.contractorName || !params.productName || !params.storageName)
      return []

    return rows
      .filter(r =>
        r.contractorName === params.contractorName
        && r.productName === params.productName
        && r.storageName === params.storageName,
      )
      .sort((a, b) => (b.date ?? '').localeCompare(a.date ?? ''))
      .slice(0, params.limit ?? 50)
  }, [q.data, params.contractorName, params.productName, params.storageName, params.limit])

  return {
    movements,
    isLoading: q.isLoading,
    isError: q.isError,
    error: q.error,
  }
}
