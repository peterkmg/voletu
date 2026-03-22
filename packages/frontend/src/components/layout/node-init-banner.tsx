import { AlertTriangle } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'

export function NodeInitBanner() {
  const { t } = useTranslation('system')
  const status = useNodeStore(s => s.status)
  const user = useAuthStore(s => s.auth.user)

  if (status.isInitialized) return null

  const isAdmin = user?.role === 'ADMIN'

  return (
    <div className="border-b border-amber-200 bg-amber-50 px-4 py-3 dark:border-amber-900/50 dark:bg-amber-950/50">
      <div className="flex items-center gap-3">
        <AlertTriangle className="size-5 shrink-0 text-amber-600 dark:text-amber-400" />
        <div className="flex-1">
          <p className="text-sm font-medium text-amber-800 dark:text-amber-200">
            {t('node.notInitialized.title')}
          </p>
          <p className="text-sm text-amber-700 dark:text-amber-300">
            {isAdmin
              ? t('node.notInitialized.adminMessage')
              : t('node.notInitialized.userMessage')}
          </p>
        </div>
      </div>
    </div>
  )
}
