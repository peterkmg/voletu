import type { UserResponse } from '~/generated/types'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { systemUserDelete } from '~/generated/client'
import { systemUserListQueryKey } from '~/generated/hooks/SystemUserHooks/useSystemUserList'
import { queryClient } from '~/shared/api/query-client'

interface UserDeleteDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: UserResponse | null
}

export function UserDeleteDialog({
  open,
  onOpenChange,
  currentRow,
}: UserDeleteDialogProps) {
  const { t } = useTranslation(['common', 'system'])
  const [loading, setLoading] = useState(false)

  const handleConfirm = async () => {
    if (!currentRow)
      return
    setLoading(true)

    try {
      await systemUserDelete(currentRow.id)
      toast.success(
        t('common:toast.deleteSuccess', {
          entity: t('system:users.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: systemUserListQueryKey() })
      onOpenChange(false)
    }
    catch (err) {
      toast.error(err instanceof Error ? err.message : t('common:toast.error'))
    }
    finally {
      setLoading(false)
    }
  }

  return (
    <ConfirmDialog
      open={open}
      onOpenChange={onOpenChange}
      title={t('common:confirm.deleteTitle')}
      description={t('common:confirm.deleteDescription')}
      confirmLabel={t('common:actions.delete')}
      cancelLabel={t('common:actions.cancel')}
      variant="destructive"
      onConfirm={handleConfirm}
      loading={loading}
    />
  )
}
