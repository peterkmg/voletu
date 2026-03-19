import type { PhysicalTransferResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type PhysicalTransferDialogType = 'create' | 'execute' | 'revert'

interface PhysicalTransferContextType {
  open: PhysicalTransferDialogType | null
  setOpen: (str: PhysicalTransferDialogType | null) => void
  currentRow: PhysicalTransferResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<PhysicalTransferResponse | null>>
}

const PhysicalTransferContext = React.createContext<PhysicalTransferContextType | null>(null)

export function PhysicalTransferProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<PhysicalTransferDialogType>(null)
  const [currentRow, setCurrentRow] = useState<PhysicalTransferResponse | null>(null)

  return (
    <PhysicalTransferContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </PhysicalTransferContext>
  )
}

export function usePhysicalTransfer() {
  const ctx = React.use(PhysicalTransferContext)
  if (!ctx) {
    throw new Error('usePhysicalTransfer must be used within <PhysicalTransferProvider>')
  }
  return ctx
}
