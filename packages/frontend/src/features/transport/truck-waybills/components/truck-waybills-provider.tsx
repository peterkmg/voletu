import type { TruckWaybillResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type TruckWaybillsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface TruckWaybillsContextType {
  open: TruckWaybillsDialogType | null
  setOpen: (str: TruckWaybillsDialogType | null) => void
  currentRow: TruckWaybillResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<TruckWaybillResponse | null>>
}

const TruckWaybillsContext = React.createContext<TruckWaybillsContextType | null>(null)

export function TruckWaybillsProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<TruckWaybillsDialogType>(null)
  const [currentRow, setCurrentRow] = useState<TruckWaybillResponse | null>(null)

  return (
    <TruckWaybillsContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </TruckWaybillsContext>
  )
}

export function useTruckWaybills() {
  const ctx = React.use(TruckWaybillsContext)
  if (!ctx) {
    throw new Error('useTruckWaybills must be used within <TruckWaybillsProvider>')
  }
  return ctx
}
