import { useEffect, useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { Alert, AlertDescription, AlertTitle } from '~/components/ui/alert'
import { Skeleton } from '~/components/ui/skeleton'
import { findLeafLabel, findParentGroupLabel } from './axis-utils'
import { EmptyState } from './components/empty-states'
import { InventoryMatrix } from './components/inventory-matrix'
import { MatrixToolbar } from './components/matrix-toolbar'
import { MovementsSheet } from './components/movements-sheet'
import { useInventoryMatrixData } from './hooks/use-inventory-matrix-data'
import { useDashboardStore } from './state/dashboard-store'

export function DashboardView() {
  const { t } = useTranslation('common')
  const [searchQuery, setSearchQuery] = useState('')
  const [sheetCell, setSheetCell] = useState<{ productId: string, storageId: string } | null>(null)

  const data = useInventoryMatrixData(searchQuery)
  const contractorId = useDashboardStore(s => s.contractorId)
  const setContractorId = useDashboardStore(s => s.setContractorId)
  const orientation = useDashboardStore(s => s.orientation)
  const productGroupTotals = useDashboardStore(s => s.productGroupTotals)
  const productTypeTotals = useDashboardStore(s => s.productTypeTotals)
  const warehouseTotals = useDashboardStore(s => s.warehouseTotals)
  const baseTotals = useDashboardStore(s => s.baseTotals)

  // Auto-select the first contractor if nothing is selected yet
  useEffect(() => {
    if (!contractorId && data.contractors.length > 0) {
      setContractorId(data.contractors[0]!.id)
    }
  }, [contractorId, data.contractors, setContractorId])

  const contractorLabel = useMemo(
    () => data.contractors.find(c => c.id === contractorId)?.label ?? '',
    [contractorId, data.contractors],
  )

  const sheetContext = useMemo(() => {
    if (!sheetCell || !data.vm)
      return null
    const productName = findLeafLabel(data.vm.productAxis.root, sheetCell.productId)
    const storageName = findLeafLabel(data.vm.storageAxis.root, sheetCell.storageId)
    const warehouseName = findParentGroupLabel(data.vm.storageAxis.root, sheetCell.storageId, 'warehouse')
    const balance = data.vm.cell(sheetCell.productId, sheetCell.storageId)
    return { productName, storageName, warehouseName, balance, contractorName: contractorLabel }
  }, [sheetCell, data.vm, contractorLabel])

  // --- Branching render ---
  if (data.isLoading && !data.hasAnyData) {
    return (
      <DashboardShell>
        <Skeleton className="h-96 w-full" />
      </DashboardShell>
    )
  }

  if (data.isError) {
    return (
      <DashboardShell>
        <Alert variant="destructive">
          <AlertTitle>{t('error.shortTitle')}</AlertTitle>
          <AlertDescription>
            {String((data.error as Error | null)?.message ?? t('error.unknown'))}
          </AlertDescription>
        </Alert>
      </DashboardShell>
    )
  }

  if (data.contractors.length === 0) {
    return (
      <DashboardShell>
        <EmptyState variant="no-contractors" />
      </DashboardShell>
    )
  }

  const toolbar = (
    <MatrixToolbar
      contractors={data.contractors}
      searchQuery={searchQuery}
      onSearchChange={setSearchQuery}
    />
  )

  // No stock at all for the selected contractor (and not a search situation).
  if ((data.vm?.stats.nonEmptyCellCount ?? 0) === 0 && !searchQuery) {
    return (
      <DashboardShell toolbar={toolbar}>
        <EmptyState variant="no-stock" />
      </DashboardShell>
    )
  }

  // Search narrowed everything out
  if (data.vm && (data.vm.stats.leafRowCount === 0 || data.vm.stats.leafColCount === 0) && searchQuery) {
    return (
      <DashboardShell toolbar={toolbar}>
        <EmptyState variant="no-search" onClearSearch={() => setSearchQuery('')} />
      </DashboardShell>
    )
  }

  return (
    <DashboardShell toolbar={toolbar}>
      {data.vm && (
        <InventoryMatrix
          vm={data.vm}
          orientation={orientation}
          subtotals={{
            productGroup: productGroupTotals,
            productType: productTypeTotals,
            warehouse: warehouseTotals,
            base: baseTotals,
          }}
          onCellClick={(productId, storageId) => setSheetCell({ productId, storageId })}
        />
      )}
      <MovementsSheet
        open={sheetCell != null}
        onOpenChange={(open) => {
          if (!open)
            setSheetCell(null)
        }}
        contractorName={sheetContext?.contractorName ?? null}
        productName={sheetContext?.productName ?? null}
        storageName={sheetContext?.storageName ?? null}
        warehouseName={sheetContext?.warehouseName ?? null}
        balance={sheetContext?.balance}
      />
    </DashboardShell>
  )
}

function DashboardShell({ children, toolbar }: { children: React.ReactNode, toolbar?: React.ReactNode }) {
  return (
    <>
      <Header />
      <Main fixed className="flex flex-col gap-4">
        {toolbar}
        {children}
      </Main>
    </>
  )
}
