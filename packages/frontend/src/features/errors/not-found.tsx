import { useNavigate, useRouter } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'

export function NotFound() {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const router = useRouter()

  return (
    <div className="flex min-h-svh flex-col items-center justify-center gap-4">
      <h1 className="text-7xl font-bold text-muted-foreground">404</h1>
      <p className="text-lg text-muted-foreground">Page not found</p>
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
