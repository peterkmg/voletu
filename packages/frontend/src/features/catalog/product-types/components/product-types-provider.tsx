import type { ProductTypeResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type ProductTypesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface ProductTypesContextType {
  open: ProductTypesDialogType | null
  setOpen: (str: ProductTypesDialogType | null) => void
  currentRow: ProductTypeResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<ProductTypeResponse | null>>
}

const ProductTypesContext = React.createContext<ProductTypesContextType | null>(null)

export function ProductTypesProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<ProductTypesDialogType>(null)
  const [currentRow, setCurrentRow] = useState<ProductTypeResponse | null>(null)

  return (
    <ProductTypesContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </ProductTypesContext>
  )
}

export function useProductTypes() {
  const ctx = React.use(ProductTypesContext)
  if (!ctx) {
    throw new Error('useProductTypes must be used within <ProductTypesProvider>')
  }
  return ctx
}
