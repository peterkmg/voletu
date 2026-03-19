import { UserCreateDrawer } from './user-create-drawer'
import { UserDeleteDialog } from './user-delete-dialog'
import { useUsers } from './users-provider'

export function UsersDialogs() {
  const { open, setOpen, currentRow } = useUsers()

  return (
    <>
      <UserCreateDrawer
        open={open === 'create'}
        onOpenChange={() => setOpen(null)}
      />
      <UserDeleteDialog
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
      />
    </>
  )
}
