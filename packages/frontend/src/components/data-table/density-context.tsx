/* eslint-disable react-refresh/only-export-components -- context files export both Provider and hook */
import { createContext, use, useCallback, useState } from 'react'

export type TableDensity = 'compact' | 'normal' | 'comfortable'

interface DensityContextValue {
  density: TableDensity
  setDensity: (d: TableDensity) => void
}

const DensityContext = createContext<DensityContextValue>({
  density: 'normal',
  setDensity: () => {},
})

const STORAGE_KEY = 'voletu.table-density'

export function DensityProvider({ children }: { children: React.ReactNode }) {
  const [density, setDensityState] = useState<TableDensity>(() => {
    const stored = localStorage.getItem(STORAGE_KEY)
    return (stored as TableDensity) || 'normal'
  })

  const setDensity = useCallback((d: TableDensity) => {
    setDensityState(d)
    localStorage.setItem(STORAGE_KEY, d)
  }, [])

  return (
    <DensityContext value={{ density, setDensity }}>
      {children}
    </DensityContext>
  )
}

export function useDensity() {
  return use(DensityContext)
}

export const densityClasses: Record<TableDensity, string> = {
  compact: 'py-1',
  normal: 'py-2',
  comfortable: 'py-3',
}
