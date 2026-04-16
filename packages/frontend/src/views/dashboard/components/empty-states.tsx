import { Link } from '@tanstack/react-router'
import { Inbox, Package, Search, Users } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '~/components/ui/card'

export type EmptyVariant = 'no-contractors' | 'no-ledger' | 'no-stock' | 'no-search'

export interface EmptyStateProps {
  variant: EmptyVariant
  onClearSearch?: () => void
}

export function EmptyState({ variant, onClearSearch }: EmptyStateProps) {
  const { t } = useTranslation('dashboard')

  if (variant === 'no-contractors') {
    return (
      <Card className="p-8">
        <CardHeader className="flex flex-col items-center gap-2">
          <Users className="size-8 text-muted-foreground" />
          <CardTitle>{t('empty.noContractors.title')}</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col items-center gap-4">
          <p className="text-sm text-muted-foreground">{t('empty.noContractors.body')}</p>
          <Button asChild>
            <Link to="/catalog/companies">{t('empty.noContractors.cta')}</Link>
          </Button>
        </CardContent>
      </Card>
    )
  }

  if (variant === 'no-ledger') {
    return (
      <Card className="p-8">
        <CardHeader className="flex flex-col items-center gap-2">
          <Inbox className="size-8 text-muted-foreground" />
          <CardTitle>{t('empty.noLedger.title')}</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col items-center gap-4">
          <p className="text-sm text-muted-foreground">{t('empty.noLedger.body')}</p>
          <Button asChild variant="secondary">
            <Link to="/cargo-flow">{t('empty.noLedger.cta')}</Link>
          </Button>
        </CardContent>
      </Card>
    )
  }

  if (variant === 'no-stock') {
    return (
      <Card className="p-8">
        <CardHeader className="flex flex-col items-center gap-2">
          <Package className="size-8 text-muted-foreground" />
          <CardTitle>{t('empty.noStock.title')}</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col items-center gap-2">
          <p className="text-sm text-muted-foreground">{t('empty.noStock.body')}</p>
        </CardContent>
      </Card>
    )
  }

  // no-search
  return (
    <Card className="p-8">
      <CardHeader className="flex flex-col items-center gap-2">
        <Search className="size-8 text-muted-foreground" />
        <CardTitle>{t('empty.noSearch.title')}</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col items-center gap-4">
        <p className="text-sm text-muted-foreground">{t('empty.noSearch.body')}</p>
        {onClearSearch && (
          <Button variant="outline" onClick={onClearSearch}>
            {t('empty.noSearch.cta')}
          </Button>
        )}
      </CardContent>
    </Card>
  )
}
