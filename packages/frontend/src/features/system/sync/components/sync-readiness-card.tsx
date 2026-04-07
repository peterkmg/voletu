import { CheckCircle2, Circle, ListChecks } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import { useNodeStore } from '~/stores/node-store'
import { BaseAssignmentDialog } from './base-assignment-dialog'

interface ChecklistItemProps {
  done: boolean
  label: string
  action?: {
    label: string
    onClick: () => void
  }
}

function ChecklistItem({ done, label, action }: ChecklistItemProps) {
  return (
    <li className="flex items-center justify-between gap-2">
      <span className="flex items-center gap-2">
        {done
          ? <CheckCircle2 className="size-4 text-green-500 shrink-0" />
          : <Circle className="size-4 text-muted-foreground/50 shrink-0" />}
        <span className={done ? 'text-muted-foreground line-through' : ''}>
          {label}
        </span>
      </span>
      {action && !done && (
        <Button variant="outline" size="sm" onClick={action.onClick}>
          {action.label}
        </Button>
      )}
    </li>
  )
}

export function SyncReadinessCard() {
  const { t } = useTranslation('system')
  const status = useNodeStore(s => s.status)
  const [baseDialogOpen, setBaseDialogOpen] = useState(false)

  // Only show for peripheral nodes that aren't fully configured
  if (status.nodeType === 'CENTRAL' || status.nodeType === null)
    return null

  const isInitialized = status.isInitialized
  const centralConnected = status.workerState === 'OnlineIdle'
    || status.workerState === 'Syncing'
  const basesAssigned = status.assignedBaseIds.length > 0
  const fullSync = isInitialized && centralConnected && basesAssigned

  // Hide when fully configured
  if (fullSync)
    return null

  return (
    <>
      <Card className="border-yellow-200 bg-yellow-50/50 dark:border-yellow-800/50 dark:bg-yellow-950/10">
        <CardHeader className="pb-3">
          <CardTitle className="flex items-center gap-2 text-base">
            <ListChecks className="size-5" />
            {t('sync.readiness.title')}
          </CardTitle>
          <CardDescription>
            {t('sync.readiness.description')}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <ol className="space-y-2.5">
            <ChecklistItem
              done={isInitialized}
              label={t('sync.readiness.initialized')}
            />
            <ChecklistItem
              done={centralConnected}
              label={t('sync.readiness.centralConnected')}
            />
            <ChecklistItem
              done={basesAssigned}
              label={t('sync.readiness.basesAssigned')}
              action={{
                label: t('sync.readiness.assignBases'),
                onClick: () => setBaseDialogOpen(true),
              }}
            />
            <ChecklistItem
              done={fullSync}
              label={t('sync.readiness.fullSyncActive')}
            />
          </ol>
        </CardContent>
      </Card>

      <BaseAssignmentDialog
        open={baseDialogOpen}
        onOpenChange={setBaseDialogOpen}
      />
    </>
  )
}
