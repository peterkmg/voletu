import type { ProductResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type ProductsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface ProductsContextType {
  open: ProductsDialogType | null
  setOpen: (str: ProductsDialogType | null) => void
  currentRow: ProductResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<ProductResponse | null>>
}

const ProductsContext = React.createContext<ProductsContextType | null>(null)

export function ProductsProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<ProductsDialogType>(null)
  const [currentRow, setCurrentRow] = useState<ProductResponse | null>(null)

  return (
    <ProductsContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </ProductsContext>
  )
}

export function useProducts() {
  const ctx = React.use(ProductsContext)
  if (!ctx) {
    throw new Error('useProducts must be used within <ProductsProvider>')
  }
  return ctx
}
