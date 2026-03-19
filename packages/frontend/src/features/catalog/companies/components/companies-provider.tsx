import type { CompanyResponse } from '~/generated/types'
import * as React from 'react'
import { useState } from 'react'
import useDialogState from '~/hooks/use-dialog-state'

type CompaniesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

interface CompaniesContextType {
  open: CompaniesDialogType | null
  setOpen: (str: CompaniesDialogType | null) => void
  currentRow: CompanyResponse | null
  setCurrentRow: React.Dispatch<React.SetStateAction<CompanyResponse | null>>
}

const CompaniesContext = React.createContext<CompaniesContextType | null>(null)

export function CompaniesProvider({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useDialogState<CompaniesDialogType>(null)
  const [currentRow, setCurrentRow] = useState<CompanyResponse | null>(null)

  return (
    <CompaniesContext value={{ open, setOpen, currentRow, setCurrentRow }}>
      {children}
    </CompaniesContext>
  )
}

export function useCompanies() {
  const ctx = React.use(CompaniesContext)
  if (!ctx) {
    throw new Error('useCompanies must be used within <CompaniesProvider>')
  }
  return ctx
}
