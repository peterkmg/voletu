import type { SyncUiState } from '../sync-ui-state'
import { CheckCircle2, ChevronDown, Circle, ListChecks, ServerCog } from 'lucide-react'
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
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '~/components/ui/collapsible'
import { cn } from '~/lib/utils'
import { useNodeStore } from '~/stores/node-store'
import { deriveSyncUiState } from '../sync-ui-state'
import { BaseAssignmentDialog } from './base-assignment-dialog'
import { ChangeCentralUrlDialog } from './change-central-url-dialog'

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

// Maps the fine-grained UI state to the runtime-status label shown next to the
// "Node configured" title. Only states reachable while setupComplete are
// considered here.
const RUNTIME_LABEL_KEY: Record<'online' | 'syncing' | 'offline', string> = {
  online: 'node.syncState.online',
  syncing: 'node.syncState.syncing',
  offline: 'node.syncState.offline',
}
const RUNTIME_DOT: Record<'online' | 'syncing' | 'offline', string> = {
  online: 'bg-green-500',
  syncing: 'bg-green-500',
  offline: 'bg-red-500',
}

export function SyncReadinessCard() {
  const { t } = useTranslation('system')
  const status = useNodeStore(s => s.status)
  const basesLoaded = useNodeStore(s => s.basesLoaded)
  const centralVerifiedOnce = useNodeStore(s => s.centralVerifiedOnce)
  const [baseDialogOpen, setBaseDialogOpen] = useState(false)
  const [urlDialogOpen, setUrlDialogOpen] = useState(false)

  const uiState: SyncUiState = deriveSyncUiState(status, basesLoaded)

  if (uiState === 'central')
    return null

  if (uiState === 'setupIncomplete') {
    const isInitialized = status.isInitialized
    const basesAssigned = status.assignedBaseIds.length > 0
    // Sticky: once the central link has been verified during this session, keep
    // the step checked even if the worker later goes Offline/Backoff.
    const centralConnected = centralVerifiedOnce
    const fullSync = isInitialized && centralConnected && basesAssigned

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
              <ChecklistItem done={isInitialized} label={t('sync.readiness.initialized')} />
              <ChecklistItem done={centralConnected} label={t('sync.readiness.centralConnected')} />
              <ChecklistItem
                done={basesAssigned}
                label={t('sync.readiness.basesAssigned')}
                action={{
                  label: t('sync.readiness.assignBases'),
                  onClick: () => setBaseDialogOpen(true),
                }}
              />
              <ChecklistItem done={fullSync} label={t('sync.readiness.fullSyncActive')} />
            </ol>
          </CardContent>
        </Card>

        <BaseAssignmentDialog open={baseDialogOpen} onOpenChange={setBaseDialogOpen} />
      </>
    )
  }

  // Remaining cases: 'online' | 'syncing' | 'offline' — setup is complete.
  // Render a collapsible summary so the user can still reach reconfiguration
  // actions.
  const runtime = uiState
  const baseCount = status.assignedBaseIds.length

  return (
    <>
      <Card>
        <Collapsible>
          <CardHeader className="pb-3">
            <CollapsibleTrigger
              className="group flex w-full items-center justify-between gap-2 text-left"
              data-testid="sync-readiness-trigger"
            >
              <div className="flex flex-col gap-1">
                <CardTitle className="flex items-center gap-2 text-base">
                  <ServerCog className="size-5" />
                  {t('sync.readiness.configured.title')}
                </CardTitle>
                <span className="flex items-center gap-1.5 text-xs text-muted-foreground">
                  <span className={cn('size-2 rounded-full', RUNTIME_DOT[runtime])} />
                  {t(RUNTIME_LABEL_KEY[runtime])}
                </span>
              </div>
              <ChevronDown
                className="size-5 text-muted-foreground transition-transform group-data-[state=open]:rotate-180"
              />
            </CollapsibleTrigger>
          </CardHeader>
          <CollapsibleContent>
            <CardContent className="space-y-3 pt-0">
              <ConfiguredRow
                label={t('sync.readiness.configured.centralApiUrl')}
                value={status.centralApiUrl ?? t('sync.readiness.configured.centralApiUrlUnknown')}
                valueClassName="font-mono text-xs break-all"
                action={(
                  <Button variant="outline" size="sm" onClick={() => setUrlDialogOpen(true)}>
                    {t('sync.readiness.configured.change')}
                  </Button>
                )}
              />
              <ConfiguredRow
                label={t('sync.readiness.configured.assignedBases', { count: baseCount })}
                action={(
                  <Button variant="outline" size="sm" onClick={() => setBaseDialogOpen(true)}>
                    {t('sync.readiness.configured.manage')}
                  </Button>
                )}
              />
            </CardContent>
          </CollapsibleContent>
        </Collapsible>
      </Card>

      <BaseAssignmentDialog open={baseDialogOpen} onOpenChange={setBaseDialogOpen} />
      <ChangeCentralUrlDialog open={urlDialogOpen} onOpenChange={setUrlDialogOpen} />
    </>
  )
}

function ConfiguredRow({
  label,
  value,
  valueClassName,
  action,
}: {
  label: string
  value?: string
  valueClassName?: string
  action: React.ReactNode
}) {
  return (
    <div className="flex items-start justify-between gap-3">
      <div className="min-w-0 flex-1 space-y-0.5">
        <div className="text-xs uppercase tracking-wide text-muted-foreground">{label}</div>
        {value !== undefined && (
          <div className={cn('text-sm', valueClassName)}>{value}</div>
        )}
      </div>
      <div className="shrink-0">{action}</div>
    </div>
  )
}
