import type { AcceptanceResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type AcceptanceDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

interface AcceptanceContextType {
  open: AcceptanceDialogType | null
  setOpen: (str: AcceptanceDialogType | null) => void
  currentRow: AcceptanceResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<AcceptanceResponse | null>>
}

const AcceptanceContext = React.createContext<AcceptanceContextType | null>(null)

export function AcceptanceProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<AcceptanceDialogType>(null)
  const [currentRow, setCurrentRow] = useState<AcceptanceResponse | null>(null)

  return (
    <AcceptanceContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </AcceptanceContext>
  )
}

export function useAcceptance() {
  const ctx = React.use(AcceptanceContext)
  if (!ctx) {
    throw new Error('useAcceptance must be used within <AcceptanceProvider>')
  }
  return ctx
}
