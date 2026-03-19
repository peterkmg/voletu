import type { StorageResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type StoragesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface StoragesContextType {
  open: StoragesDialogType | null
  setOpen: (str: StoragesDialogType | null) => void
  currentRow: StorageResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<StorageResponse | null>>
}

const StoragesContext = React.createContext<StoragesContextType | null>(null)

export function StoragesProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<StoragesDialogType>(null)
  const [currentRow, setCurrentRow] = useState<StorageResponse | null>(null)

  return (
    <StoragesContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </StoragesContext>
  )
}

export function useStorages() {
  const ctx = React.use(StoragesContext)
  if (!ctx) {
    throw new Error('useStorages must be used within <StoragesProvider>')
  }
  return ctx
}
