import type { PortResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type PortsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface PortsContextType {
  open: PortsDialogType | null
  setOpen: (str: PortsDialogType | null) => void
  currentRow: PortResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<PortResponse | null>>
}

const PortsContext = React.createContext<PortsContextType | null>(null)

export function PortsProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<PortsDialogType>(null)
  const [currentRow, setCurrentRow] = useState<PortResponse | null>(null)

  return (
    <PortsContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </PortsContext>
  )
}

export function usePorts() {
  const ctx = React.use(PortsContext)
  if (!ctx) {
    throw new Error('usePorts must be used within <PortsProvider>')
  }
  return ctx
}
