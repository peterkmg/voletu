import { useNavigate, useRouter } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'

interface GeneralErrorProps {
  statusCode?: number
  title?: string
  message?: string
}

export function GeneralError({
  statusCode = 500,
  title,
  message,
}: GeneralErrorProps) {
  const { t } = useTranslation('common')
  const navigate = useNavigate()
  const router = useRouter()

  return (
    <div className="flex min-h-svh flex-col items-center justify-center gap-4">
      <h1 className="text-7xl font-bold text-muted-foreground">
        {statusCode}
      </h1>
      <p className="text-lg font-semibold">{title ?? t('error.title')}</p>
      <p className="text-muted-foreground">{message ?? t('error.message')}</p>
      <div className="flex gap-2">
        <Button variant="outline" onClick={() => router.history.back()}>
          {t('error.goBack')}
        </Button>
        <Button onClick={() => navigate({ to: '/' })}>{t('error.dashboard')}</Button>
      </div>
    </div>
  )
}

export function NotFound() {
  const { t } = useTranslation('common')
  const navigate = useNavigate()
  const router = useRouter()

  return (
    <div className="flex min-h-svh flex-col items-center justify-center gap-4">
      <h1 className="text-7xl font-bold text-muted-foreground">404</h1>
      <p className="text-lg text-muted-foreground">{t('error.notFound')}</p>
      <div className="flex gap-2">
        <Button variant="outline" onClick={() => router.history.back()}>
          {t('actions.back')}
        </Button>
        <Button onClick={() => navigate({ to: '/' })}>
          {t('nav.dashboard')}
        </Button>
      </div>
    </div>
  )
}
