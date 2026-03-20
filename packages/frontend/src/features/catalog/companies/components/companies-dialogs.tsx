import { useCompanies } from './companies-provider'
import { CompanyDeleteDialog } from './company-delete-dialog'
import { CompanyMutateDialog } from './company-mutate-dialog'

export function CompaniesDialogs() {
  const { open, setOpen, currentRow } = useCompanies()

  return (
    <>
      <CompanyMutateDialog
        open={open === 'create' || open === 'update'}
        onOpenChange={() => setOpen(null)}
        currentRow={open === 'update' ? currentRow : null}
      />
      <CompanyDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="soft"
      />
      <CompanyDeleteDialog
        open={open === 'hard-delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        variant="hard"
      />
    </>
  )
}
