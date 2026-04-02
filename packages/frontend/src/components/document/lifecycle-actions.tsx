import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { useAuthStore } from '~/stores/auth-store'

// Role UUIDs matching backend enums::RoleType
const ROLE_ADMIN = '019c8cc2-8913-774a-a432-4dee8eb3f194'
const ROLE_SENIOR_SUPERVISOR = '019c8cc4-3538-7b66-8ce5-6faad856b217'
const ROLE_SUPERVISOR = '019c8cc4-9048-7b61-9443-52858a953a17'

const SUPERVISOR_OR_HIGHER = new Set([ROLE_SUPERVISOR, ROLE_SENIOR_SUPERVISOR, ROLE_ADMIN])
const SENIOR_OR_HIGHER = new Set([ROLE_SENIOR_SUPERVISOR, ROLE_ADMIN])

export function isSupervisorOrHigher(roleId: string | undefined): boolean {
  return !!roleId && SUPERVISOR_OR_HIGHER.has(roleId)
}

export function isSeniorOrHigher(roleId: string | undefined): boolean {
  return !!roleId && SENIOR_OR_HIGHER.has(roleId)
}

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
  const { t } = useTranslation('common')
  const user = useAuthStore((s) => s.user)
  const [dialogOpen, setDialogOpen] = useState(false)
  const [dialogAction, setDialogAction] = useState<'execute' | 'revert'>('execute')

  const isDraft = status === 'draft' || status === 'Draft'
  const isPosted = status === 'posted' || status === 'Posted' || status === 'executed'

  const canExecute = isDraft && isSupervisorOrHigher(user?.role)
  const canRevert = isPosted && isSeniorOrHigher(user?.role)

  return (
    <>
      {canExecute && (
        <Button
          size="sm"
          onClick={() => { setDialogAction('execute'); setDialogOpen(true) }}
        >
          {t('actions.execute')}
        </Button>
      )}
      {canRevert && (
        <Button
          variant="destructive"
          size="sm"
          onClick={() => { setDialogAction('revert'); setDialogOpen(true) }}
        >
          {t('actions.revertToDraft')}
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
