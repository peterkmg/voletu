import type { Orientation, Uuid } from '../types'
// packages/frontend/src/views/dashboard/state/dashboard-store.ts
import { create } from 'zustand'
import { createJSONStorage, persist } from 'zustand/middleware'

interface DashboardState {
  // Selection
  contractorId: Uuid | null

  // Layout
  orientation: Orientation

  // Structure toggles
  showType: boolean
  showBase: boolean

  // Subtotal toggles — replaces the single showSubtotals with four named ones.
  // Gated in the UI: productTypeTotals is disabled when showType is off;
  // baseTotals is disabled when showBase is off. Store keeps the value so
  // re-enabling the structure toggle restores the prior choice.
  productGroupTotals: boolean
  productTypeTotals: boolean
  warehouseTotals: boolean
  baseTotals: boolean

  // Setters
  setContractorId: (id: Uuid | null) => void
  setOrientation: (o: Orientation) => void
  setShowType: (v: boolean) => void
  setShowBase: (v: boolean) => void
  setProductGroupTotals: (v: boolean) => void
  setProductTypeTotals: (v: boolean) => void
  setWarehouseTotals: (v: boolean) => void
  setBaseTotals: (v: boolean) => void
}

export const useDashboardStore = create<DashboardState>()(
  persist(
    set => ({
      contractorId: null,
      orientation: 'products-as-rows',
      showType: false,
      showBase: false,
      productGroupTotals: false,
      productTypeTotals: false,
      warehouseTotals: false,
      baseTotals: false,

      setContractorId: id => set({ contractorId: id }),
      setOrientation: o => set({ orientation: o }),
      setShowType: v => set({ showType: v }),
      setShowBase: v => set({ showBase: v }),
      setProductGroupTotals: v => set({ productGroupTotals: v }),
      setProductTypeTotals: v => set({ productTypeTotals: v }),
      setWarehouseTotals: v => set({ warehouseTotals: v }),
      setBaseTotals: v => set({ baseTotals: v }),
    }),
    {
      name: 'voletu:dashboard:v2',
      storage: createJSONStorage(() => localStorage),
      partialize: state => ({
        contractorId: state.contractorId,
        orientation: state.orientation,
        showType: state.showType,
        showBase: state.showBase,
        productGroupTotals: state.productGroupTotals,
        productTypeTotals: state.productTypeTotals,
        warehouseTotals: state.warehouseTotals,
        baseTotals: state.baseTotals,
      }),
    },
  ),
)
