import type { BaseResponse } from '~/generated/types/BaseResponse'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type BasesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface BasesContextType {
  open: BasesDialogType | null
  setOpen: (str: BasesDialogType | null) => void
  currentRow: BaseResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<BaseResponse | null>>
}

const BasesContext = React.createContext<BasesContextType | null>(null)

export function BasesProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<BasesDialogType>(null)
  const [currentRow, setCurrentRow] = useState<BaseResponse | null>(null)

  return (
    <BasesContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </BasesContext>
  )
}

export function useBases() {
  const ctx = React.use(BasesContext)
  if (!ctx) {
    throw new Error('useBases must be used within <BasesProvider>')
  }
  return ctx
}
