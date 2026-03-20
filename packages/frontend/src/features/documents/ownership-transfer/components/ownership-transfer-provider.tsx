import type { OwnershipTransferResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type OwnershipTransferDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

interface OwnershipTransferContextType {
  open: OwnershipTransferDialogType | null
  setOpen: (str: OwnershipTransferDialogType | null) => void
  currentRow: OwnershipTransferResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<OwnershipTransferResponse | null>>
}

const OwnershipTransferContext = React.createContext<OwnershipTransferContextType | null>(null)

export function OwnershipTransferProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<OwnershipTransferDialogType>(null)
  const [currentRow, setCurrentRow] = useState<OwnershipTransferResponse | null>(null)

  return (
    <OwnershipTransferContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </OwnershipTransferContext>
  )
}

export function useOwnershipTransfer() {
  const ctx = React.use(OwnershipTransferContext)
  if (!ctx) {
    throw new Error('useOwnershipTransfer must be used within <OwnershipTransferProvider>')
  }
  return ctx
}
