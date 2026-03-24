import { Inbox } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { TableCell, TableRow } from '~/components/ui/table'

interface EmptyStateProps {
  colSpan: number
  message?: string
  icon?: React.ReactNode
  action?: React.ReactNode
}

export function EmptyState({ colSpan, message, icon, action }: EmptyStateProps) {
  const { t } = useTranslation('common')
  return (
    <TableRow>
      <TableCell colSpan={colSpan} className="h-32">
        <div className="flex flex-col items-center justify-center gap-2 text-muted-foreground">
          {icon ?? <Inbox className="h-8 w-8" />}
          <span className="text-sm">{message ?? t('table.noResults')}</span>
          {action}
        </div>
      </TableCell>
    </TableRow>
  )
}
