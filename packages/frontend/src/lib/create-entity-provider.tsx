import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

interface EntityContextType<
  TRow,
  TDialogType extends string,
> {
  open: TDialogType | null
  setOpen: (str: TDialogType | null) => void
  currentRow: TRow | null
  setCurrentRow: React.Dispatch<React.SetStateAction<TRow | null>>
}

export function createEntityProvider<
  TRow extends { id: string },
  TDialogType extends string = string,
>(displayName: string) {
  const Context = React.createContext<EntityContextType<TRow, TDialogType> | null>(null)

  function Provider({ children }: { children: React.ReactNode }) {
    const [open, setOpen] = useDialogState<TDialogType>(null)
    const [currentRow, setCurrentRow] = useState<TRow | null>(null)

    return (
      <Context value={{ open, setOpen, currentRow, setCurrentRow }}>
        {children}
      </Context>
    )
  }
  Provider.displayName = `${displayName}Provider`

  function useEntity() {
    const ctx = React.use(Context)
    if (!ctx) {
      throw new Error(`use${displayName} must be used within <${displayName}Provider>`)
    }
    return ctx
  }

  return { Provider, useEntity } as const
}
