import type { ProductGroupResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type ProductGroupsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface ProductGroupsContextType {
  open: ProductGroupsDialogType | null
  setOpen: (str: ProductGroupsDialogType | null) => void
  currentRow: ProductGroupResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<ProductGroupResponse | null>>
}

const ProductGroupsContext = React.createContext<ProductGroupsContextType | null>(null)

export function ProductGroupsProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<ProductGroupsDialogType>(null)
  const [currentRow, setCurrentRow] = useState<ProductGroupResponse | null>(null)

  return (
    <ProductGroupsContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </ProductGroupsContext>
  )
}

export function useProductGroups() {
  const ctx = React.use(ProductGroupsContext)
  if (!ctx) {
    throw new Error('useProductGroups must be used within <ProductGroupsProvider>')
  }
  return ctx
}
