import type { WarehouseResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type WarehousesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface WarehousesContextType {
  open: WarehousesDialogType | null
  setOpen: (str: WarehousesDialogType | null) => void
  currentRow: WarehouseResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<WarehouseResponse | null>>
}

const WarehousesContext = React.createContext<WarehousesContextType | null>(null)

export function WarehousesProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<WarehousesDialogType>(null)
  const [currentRow, setCurrentRow] = useState<WarehouseResponse | null>(null)

  return (
    <WarehousesContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </WarehousesContext>
  )
}

export function useWarehouses() {
  const ctx = React.use(WarehousesContext)
  if (!ctx) {
    throw new Error('useWarehouses must be used within <WarehousesProvider>')
  }
  return ctx
}
