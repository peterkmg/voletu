import type { DashboardData } from '../types'
import { useCallback, useMemo } from 'react'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductGroupList } from '~/generated/hooks/CatalogHooks/useCatalogProductGroupList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogProductTypeList } from '~/generated/hooks/CatalogHooks/useCatalogProductTypeList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { useLedgerBalanceList } from '~/generated/hooks/LedgerHooks/useLedgerBalanceList'
import { buildMatrixVM } from '../build-matrix-vm'
import { useDashboardStore } from '../state/dashboard-store'

export function useInventoryMatrixData(searchQuery: string): DashboardData {
  const companies = useCatalogCompanyList()
  const products = useCatalogProductList()
  const productGroups = useCatalogProductGroupList()
  const productTypes = useCatalogProductTypeList()
  const storages = useCatalogStorageList()
  const warehouses = useCatalogWarehouseList()
  const bases = useCatalogBaseList()
  const ledger = useLedgerBalanceList()

  const all = useMemo(
    () => [companies, products, productGroups, productTypes, storages, warehouses, bases, ledger],
    [companies, products, productGroups, productTypes, storages, warehouses, bases, ledger],
  )
  const isLoading = all.some(q => q.isLoading)
  const isError = all.some(q => q.isError)
  const hasAnyData = all.every(q => q.data != null)
  const error = all.find(q => q.isError)?.error

  const { contractorId, showType, showBase } = useDashboardStore()

  const contractors = useMemo(() => {
    const list = (companies.data as any)?.data ?? []

    return list
      .filter((c: any) => c.isContractor === true)
      .map((c: any) => ({ id: c.id, label: c.commonName ?? c.legalName ?? c.id }))
      .sort((a: { label: string }, b: { label: string }) => a.label.localeCompare(b.label))
  }, [companies.data])

  const vm = useMemo(() => {
    if (!hasAnyData || !contractorId)
      return null

    return buildMatrixVM({
      contractorId,
      ledgerBalances: (ledger.data as any)?.data ?? [],
      products: (products.data as any)?.data ?? [],
      productGroups: (productGroups.data as any)?.data ?? [],
      productTypes: (productTypes.data as any)?.data ?? [],
      storages: (storages.data as any)?.data ?? [],
      warehouses: (warehouses.data as any)?.data ?? [],
      bases: (bases.data as any)?.data ?? [],
      showType,
      showBase,
      searchQuery,
    })
  }, [
    hasAnyData,
    contractorId,
    showType,
    showBase,
    searchQuery,
    ledger.data,
    products.data,
    productGroups.data,
    productTypes.data,
    storages.data,
    warehouses.data,
    bases.data,
  ])

  const refetchAll = useCallback(() => {
    all.forEach(q => q.refetch?.())
  }, [all])

  return {
    vm,
    contractors,
    isLoading,
    isError,
    error,
    hasAnyData,
    refetchAll,
  }
}
