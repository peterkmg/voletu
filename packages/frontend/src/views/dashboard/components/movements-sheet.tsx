import { useTranslation } from 'react-i18next'
import { Alert, AlertDescription } from '~/components/ui/alert'
import { Badge } from '~/components/ui/badge'
import { Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle } from '~/components/ui/sheet'
import { Skeleton } from '~/components/ui/skeleton'
import { formatAmount, formatDate } from '~/lib/formatters'
import { useMovementsForCell } from '../hooks/use-movements-for-cell'

export interface MovementsSheetProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  contractorName: string | null
  productName: string | null
  storageName: string | null
  warehouseName: string | null
  balance: number | undefined
}

export function MovementsSheet({
  open,
  onOpenChange,
  contractorName,
  productName,
  storageName,
  warehouseName,
  balance,
}: MovementsSheetProps) {
  const { t } = useTranslation('dashboard')
  const { movements, isLoading, isError, error } = useMovementsForCell({
    contractorName,
    productName,
    storageName,
    enabled: open,
    limit: 50,
  })

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side="right" className="w-[480px] sm:max-w-none flex flex-col">
        <SheetHeader>
          <SheetTitle>{t('sheet.title')}</SheetTitle>
          <SheetDescription>
            {t('sheet.context', {
              product: productName ?? '',
              warehouse: warehouseName ?? '',
              storage: storageName ?? '',
            })}
          </SheetDescription>
          <div className="flex items-center gap-2 pt-2">
            <span className="text-sm text-muted-foreground">{contractorName}</span>
            <Badge variant="secondary">
              {t('sheet.balance')}
              :
              {formatAmount(balance)}
            </Badge>
          </div>
        </SheetHeader>

        <div className="mt-4 flex-1 overflow-y-auto">
          {isLoading && <Skeleton className="h-40 w-full" />}

          {isError && (
            <Alert variant="destructive">
              <AlertDescription>
                {String((error as Error | null)?.message ?? t('sheet.errorFallback'))}
              </AlertDescription>
            </Alert>
          )}

          {!isLoading && !isError && movements.length === 0 && (
            <p className="text-sm text-muted-foreground">{t('sheet.empty')}</p>
          )}

          {!isLoading && !isError && movements.length > 0 && (
            <ul className="space-y-2 text-sm">
              {movements.map(m => (
                <li key={m.id} className="flex items-center justify-between border-b pb-2">
                  <div className="flex flex-col">
                    <span className="font-mono text-xs text-muted-foreground">
                      {m.date ? formatDate(m.date) : '—'}
                    </span>
                    <span>{m.operation}</span>
                    <span className="text-xs text-muted-foreground">{m.documentNumber}</span>
                  </div>
                  <span className="tabular-nums">{formatAmount(m.quantity)}</span>
                </li>
              ))}
            </ul>
          )}
        </div>
      </SheetContent>
    </Sheet>
  )
}
