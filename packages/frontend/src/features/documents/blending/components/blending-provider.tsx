import type { BlendingResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type BlendingDialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

interface BlendingContextType {
  open: BlendingDialogType | null
  setOpen: (str: BlendingDialogType | null) => void
  currentRow: BlendingResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<BlendingResponse | null>>
}

const BlendingContext = React.createContext<BlendingContextType | null>(null)

export function BlendingProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<BlendingDialogType>(null)
  const [currentRow, setCurrentRow] = useState<BlendingResponse | null>(null)

  return (
    <BlendingContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </BlendingContext>
  )
}

export function useBlending() {
  const ctx = React.use(BlendingContext)
  if (!ctx) {
    throw new Error('useBlending must be used within <BlendingProvider>')
  }
  return ctx
}
