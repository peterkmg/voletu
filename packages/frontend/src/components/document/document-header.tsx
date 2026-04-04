import { useNavigate } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { ArrowLeft, MoreVertical } from 'lucide-react'
import { Button } from '~/components/ui/button'
import { StatusBadge } from '~/components/ui/status-badge'
import { LifecycleActions } from './lifecycle-actions'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'

interface DocumentHeaderProps {
  title: string
  documentNumber: string
  subtitle?: string
  status: string
  statusColorMap?: Record<string, string>
  backTo: string
  executeFn: (id: string) => Promise<unknown>
  revertFn: (id: string) => Promise<unknown>
  queryKey: readonly unknown[]
  entityLabel: string
  documentId: string
  onDelete?: () => void
}

export function DocumentHeader({
  title,
  documentNumber,
  subtitle,
  status,
  statusColorMap,
  backTo,
  executeFn,
  revertFn,
  queryKey,
  entityLabel,
  documentId,
  onDelete,
}: DocumentHeaderProps) {
  const navigate = useNavigate()
  const { t } = useTranslation('common')

  return (
    <div className="space-y-3">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" onClick={() => navigate({ to: backTo })}>
          <ArrowLeft className="h-4 w-4" />
        </Button>
        <div className="flex-1 min-w-0">
          <h1 className="text-lg font-semibold truncate">
            {title}{' '}
            <span className="text-muted-foreground">{documentNumber}</span>
          </h1>
          {subtitle && (
            <p className="text-sm text-muted-foreground">{subtitle}</p>
          )}
        </div>
        <StatusBadge value={status} colorMap={statusColorMap} />
        <LifecycleActions
          documentId={documentId}
          status={status}
          executeFn={executeFn}
          revertFn={revertFn}
          queryKey={queryKey}
          entityLabel={entityLabel}
        />
        {onDelete && (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon">
                <MoreVertical className="h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem
                onClick={onDelete}
                className="text-destructive"
              >
                {t('actions.delete')}
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )}
      </div>
      {status === 'POSTED' && (
        <div className="rounded-md border border-yellow-600/50 bg-yellow-950/20 px-4 py-2 text-sm text-yellow-500">
          {t('document.executedWarning', 'This document is executed and locked. Only Senior Supervisors can revert it to draft.')}
        </div>
      )}
    </div>
  )
}
