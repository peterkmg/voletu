import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { Button } from '~/components/ui/button'
import { isSeniorOrHigher, isSupervisorOrHigher } from '~/lib/rbac'

import { useAuthStore } from '~/stores/auth-store'

interface LifecycleActionsProps {
  documentId: string
  status: string
  executeFn: (id: string) => Promise<unknown>
  revertFn: (id: string) => Promise<unknown>
  queryKey: readonly unknown[]
  entityLabel: string
}

export function LifecycleActions({
  documentId,
  status,
  executeFn,
  revertFn,
  queryKey,
  entityLabel,
}: LifecycleActionsProps) {
  const { t } = useTranslation('documents')
  const user = useAuthStore(s => s.user)
  const [dialogOpen, setDialogOpen] = useState(false)
  const [dialogAction, setDialogAction] = useState<'execute' | 'revert'>('execute')

  const isDraft = status === 'DRAFT'
  const isPosted = status === 'EXECUTED'

  const canExecute = isDraft && isSupervisorOrHigher(user?.role)
  const canRevert = isPosted && isSeniorOrHigher(user?.role)

  return (
    <>
      {canExecute && (
        <Button
          size="sm"
          onClick={() => {
            setDialogAction('execute')
            setDialogOpen(true)
          }}
        >
          {t('lifecycle.execute')}
        </Button>
      )}
      {canRevert && (
        <Button
          variant="destructive"
          size="sm"
          onClick={() => {
            setDialogAction('revert')
            setDialogOpen(true)
          }}
        >
          {t('lifecycle.revert')}
        </Button>
      )}
      <LifecycleDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        currentRow={{ id: documentId }}
        action={dialogAction}
        executeFn={executeFn}
        revertFn={revertFn}
        queryKey={queryKey}
        entityLabel={entityLabel}
      />
    </>
  )
}
