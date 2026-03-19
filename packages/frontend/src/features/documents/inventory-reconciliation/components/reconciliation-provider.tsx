import type { InventoryReconciliationResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type ReconciliationDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

interface ReconciliationContextType {
  open: ReconciliationDialogType | null
  setOpen: (str: ReconciliationDialogType | null) => void
  currentRow: InventoryReconciliationResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<InventoryReconciliationResponse | null>>
}

const ReconciliationContext = React.createContext<ReconciliationContextType | null>(null)

export function ReconciliationProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<ReconciliationDialogType>(null)
  const [currentRow, setCurrentRow] = useState<InventoryReconciliationResponse | null>(null)

  return (
    <ReconciliationContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </ReconciliationContext>
  )
}

export function useReconciliation() {
  const ctx = React.use(ReconciliationContext)
  if (!ctx) {
    throw new Error('useReconciliation must be used within <ReconciliationProvider>')
  }
  return ctx
}
