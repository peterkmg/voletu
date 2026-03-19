import type { UserResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type UsersDialogType = 'create' | 'delete'

interface UsersContextType {
  open: UsersDialogType | null
  setOpen: (str: UsersDialogType | null) => void
  currentRow: UserResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<UserResponse | null>>
}

const UsersContext = React.createContext<UsersContextType | null>(null)

export function UsersProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<UsersDialogType>(null)
  const [currentRow, setCurrentRow] = useState<UserResponse | null>(null)

  return (
    <UsersContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </UsersContext>
  )
}

export function useUsers() {
  const ctx = React.use(UsersContext)
  if (!ctx) {
    throw new Error('useUsers must be used within <UsersProvider>')
  }
  return ctx
}
