import type { CargoFlowFlatRow } from '~/generated/types/CargoFlowFlatRow'
import { useMemo } from 'react'
import { useFlowCargoFlowFlatQuery } from '~/generated/hooks/FlowsHooks/useFlowCargoFlowFlatQuery'

export interface CellMovementsResult {
  movements: CargoFlowFlatRow[]
  isLoading: boolean
  isError: boolean
  error: unknown
}

/**
 * Client-side filter by name triple.
 *
 * Known limitation: filters by (contractorName, productName, storageName) because
 * the generated CargoFlowFlatRow does not carry ids. Two catalog rows sharing the
 * same common_name for product or storage would cross-leak between cells. Mitigation
 * in the UI: document the limitation near the sheet. Long-term fix: add ids to the
 * cargo-flow flat payload or replace with an id-filtered endpoint.
 */
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
