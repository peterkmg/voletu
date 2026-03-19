import type { DispatchResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type DispatchDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

interface DispatchContextType {
  open: DispatchDialogType | null
  setOpen: (str: DispatchDialogType | null) => void
  currentRow: DispatchResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<DispatchResponse | null>>
}

const DispatchContext = React.createContext<DispatchContextType | null>(null)

export function DispatchProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<DispatchDialogType>(null)
  const [currentRow, setCurrentRow] = useState<DispatchResponse | null>(null)

  return (
    <DispatchContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </DispatchContext>
  )
}

export function useDispatch() {
  const ctx = React.use(DispatchContext)
  if (!ctx) {
    throw new Error('useDispatch must be used within <DispatchProvider>')
  }
  return ctx
}
