import { useMemo } from 'react'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'

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

export function StorageCell({ id }: CellProps) {
  const query = useCatalogStorageList()
  const list = query.data?.data as ReadonlyArray<Record<string, unknown>> | undefined
  const name = useNameLookup(list, id)

  return <span>{name}</span>
}
