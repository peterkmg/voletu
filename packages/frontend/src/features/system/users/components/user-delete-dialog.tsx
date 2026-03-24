import type { UserResponse } from '~/generated/types'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/entity-delete-dialog'
import { systemUserDelete } from '~/generated/client'
import { systemUserListQueryKey } from '~/generated/hooks/SystemUserHooks/useSystemUserList'

interface UserDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: UserResponse | null
}

export function UserDeleteDialog({ open, onOpenChange, currentRow }: UserDeleteDialogProps) {
  const { t } = useTranslation(['common', 'system'])
  return (
    <EntityDeleteDialog
      open={open}
      onOpenChange={onOpenChange}
      currentRow={currentRow}
      hardDeleteFn={systemUserDelete}
      queryKey={systemUserListQueryKey()}
      entityLabel={t('system:users.singular')}
    />
  )
}
