import type { RailWaybillResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type RailWaybillsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface RailWaybillsContextType {
  open: RailWaybillsDialogType | null
  setOpen: (str: RailWaybillsDialogType | null) => void
  currentRow: RailWaybillResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<RailWaybillResponse | null>>
}

const RailWaybillsContext = React.createContext<RailWaybillsContextType | null>(null)

export function RailWaybillsProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<RailWaybillsDialogType>(null)
  const [currentRow, setCurrentRow] = useState<RailWaybillResponse | null>(null)

  return (
    <RailWaybillsContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </RailWaybillsContext>
  )
}

export function useRailWaybills() {
  const ctx = React.use(RailWaybillsContext)
  if (!ctx) {
    throw new Error('useRailWaybills must be used within <RailWaybillsProvider>')
  }
  return ctx
}
