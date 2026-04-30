import { useNavigate } from '@tanstack/react-router'
import { ArrowLeft, MoreVertical } from 'lucide-react'
import { useEffect } from 'react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { StatusBadge } from '~/components/ui/status-badge'
import { LifecycleActions } from './lifecycle-actions'

export interface ActionDescriptor {
  /** Stable identifier used as React key. Recommended for i18n contexts where label may collide across locales. Falls back to label when omitted. */
  id?: string
  label: string
  onClick: () => void
  disabled?: boolean
  disabledReason?: string
  variant?: 'default' | 'primary' | 'destructive'
}

interface DocumentHeaderProps {
  title: string
  documentNumber: string
  subtitle?: string
  status: string
  statusColorMap?: Record<string, string>
  backTo: string
  /**
   * Legacy lifecycle slot — renders Execute / Revert buttons through
   * `LifecycleActions`, which carries its own confirm-dialog + mutation
   * wiring. Used by both pipeline (acceptance) and non-pipeline (blending,
   * ownership-transfer, etc.) detail pages.
   *
   * Complementary to `actions`: callers MUST NOT include Execute / Revert
   * descriptors (`id: 'execute' | 'revert'`) in `actions` while also
   * passing this slot, or the same buttons will render twice.
   */
  executeFn?: (id: string) => Promise<unknown>
  revertFn?: (id: string) => Promise<unknown>
  queryKey?: readonly unknown[]
  entityLabel: string
  documentId: string
  onDelete?: () => void
  /**
   * Non-lifecycle action buttons (Edit, Issue acceptance, ...). Renders
   * before the legacy lifecycle slot. See `executeFn` / `revertFn` for the
   * complementary-slot contract.
   */
  actions?: ActionDescriptor[]
}

function mapActionVariant(
  variant: ActionDescriptor['variant'],
): 'default' | 'destructive' {
  if (variant === 'destructive')
    return 'destructive'
  // 'primary' maps to 'default' since the Button component has no separate
  // 'primary' variant; the existing Execute button also uses the default variant.
  return 'default'
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
  actions,
}: DocumentHeaderProps) {
  const navigate = useNavigate()
  const { t } = useTranslation(['common', 'documents'])

  useEffect(() => {
    if (!import.meta.env.DEV)
      return
    if (!(actions && executeFn && revertFn && queryKey))
      return
    const conflictingId = actions.find(a => a.id === 'execute' || a.id === 'revert')
    if (conflictingId) {
      console.warn(
        `[DocumentHeader] action id "${conflictingId.id}" duplicates the lifecycle slot. `
        + 'Choose either the descriptor or the executeFn/revertFn slot, not both.',
      )
    }
  }, [actions, executeFn, revertFn, queryKey])

  return (
    <div className="space-y-3">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" onClick={() => navigate({ to: backTo })}>
          <ArrowLeft className="h-4 w-4" />
        </Button>
        <div className="flex-1 min-w-0">
          <h1 className="text-lg font-semibold truncate">
            {title}
            {' '}
            <span className="text-muted-foreground">{documentNumber}</span>
          </h1>
          {subtitle && (
            <p className="text-sm text-muted-foreground">{subtitle}</p>
          )}
        </div>
        <StatusBadge value={status} colorMap={statusColorMap} />
        {actions?.map(action => (
          <Button
            key={action.id ?? action.label}
            size="sm"
            variant={mapActionVariant(action.variant)}
            disabled={action.disabled}
            title={action.disabledReason}
            onClick={action.onClick}
          >
            {action.label}
          </Button>
        ))}
        {executeFn && revertFn && queryKey && (
          <LifecycleActions
            documentId={documentId}
            status={status}
            executeFn={executeFn}
            revertFn={revertFn}
            queryKey={queryKey}
            entityLabel={entityLabel}
          />
        )}
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
      {status === 'EXECUTED' && (
        <div className="rounded-md border border-amber-200 bg-amber-50 px-4 py-2 text-sm text-amber-800 dark:border-amber-800 dark:bg-amber-950/30 dark:text-amber-200">
          {t('documents:document.executedWarning')}
        </div>
      )}
    </div>
  )
}
