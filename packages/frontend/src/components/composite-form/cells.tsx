/**
 * Reusable cell renderers + display formatters for `<DocItemsTable>` columns.
 *
 * Background: each `ColumnSpec` may opt into a `render(value, row)` for
 * custom display. Many doc tables hold foreign-key UUIDs (productId,
 * storageId, contractorId, baseId) that should resolve to human-readable
 * `commonName`s, and Decimal-as-string amounts that should render with
 * French-style grouping (space thousands separator, comma decimal).
 *
 * Each `*Cell` runs its own list query — React Query dedupes identical
 * keys, so multiple cells in one table share a single network request.
 * `useMemo` keeps the per-cell lookup cheap on re-render. Falls back to
 * the raw id if the catalog list hasn't loaded yet (or the id is stale),
 * so tables remain readable during loading rather than blanking out.
 */

import { useMemo } from 'react'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'

/**
 * French-style decimal formatter: NBSP thousand separator, comma decimal.
 *
 *   `2314`     -> `'2 314'`
 *   `12345.67` -> `'12 345,67'`
 *   `0.001`    -> `'0,001'`
 *
 * Three-fraction-digit cap mirrors the backend Decimal precision used
 * across receipt/dispatch/measurement amounts. `null`/`undefined`/`''`
 * round-trip to an empty string so empty cells stay blank rather than
 * displaying `'NaN'`.
 */
const amountFormatter = new Intl.NumberFormat('fr-FR', {
  minimumFractionDigits: 0,
  maximumFractionDigits: 3,
})

export function formatAmount(value: unknown): string {
  if (value === null || value === undefined || value === '')
    return ''
  const n = typeof value === 'number' ? value : Number(value)
  if (Number.isNaN(n))
    return String(value)
  return amountFormatter.format(n)
}

/**
 * Generic catalog-list lookup. Each Catalog*List response shape is
 * `{ data: Array<{ id: string, commonName: string, ... }> }` (plus the
 * outer envelope wrapping it under `.data` again). Mirrors the picker's
 * default `displayField = 'commonName'`.
 */
function useNameLookup(
  list: ReadonlyArray<Record<string, unknown>> | undefined,
  id: string | null | undefined,
): string {
  return useMemo(() => {
    if (!id)
      return ''
    if (!list)
      return id
    const match = list.find(item => item.id === id)
    if (!match)
      return id
    return (match.commonName as string | undefined) ?? id
  }, [id, list])
}

interface CellProps {
  id: string | null | undefined
}

export function ProductCell({ id }: CellProps) {
  const query = useCatalogProductList()
  const list = query.data?.data as ReadonlyArray<Record<string, unknown>> | undefined
  const name = useNameLookup(list, id)
  return <span>{name}</span>
}

export function ContractorCell({ id }: CellProps) {
  const query = useCatalogCompanyList()
  const list = query.data?.data as ReadonlyArray<Record<string, unknown>> | undefined
  const name = useNameLookup(list, id)
  return <span>{name}</span>
}

export function BaseCell({ id }: CellProps) {
  const query = useCatalogBaseList()
  const list = query.data?.data as ReadonlyArray<Record<string, unknown>> | undefined
  const name = useNameLookup(list, id)
  return <span>{name}</span>
}

/**
 * Storage cell. Storage entries carry a `commonName` (per StorageResponse);
 * the picker also uses `commonName` as its default display field, so the
 * cell mirrors the picker label exactly. Future: if `StorageBasePicker`
 * grows a richer label (e.g. "Base / Storage #N"), update both sides
 * together.
 */
export function StorageCell({ id }: CellProps) {
  const query = useCatalogStorageList()
  const list = query.data?.data as ReadonlyArray<Record<string, unknown>> | undefined
  const name = useNameLookup(list, id)
  return <span>{name}</span>
}
