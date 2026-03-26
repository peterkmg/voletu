import { Wrench } from 'lucide-react'
import { useCallback, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuShortcut,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { toggleDevTools } from '~/lib/devtools'
import { extractErrorMessage } from '~/lib/error'
import { cn } from '~/lib/utils'
import { useAuthStore } from '~/stores/auth-store'
import { useStartupStore } from '~/stores/startup-store'

export function DebugTrigger() {
  const { t } = useTranslation()
  const [seeding, setSeeding] = useState(false)

  const handleSeed = useCallback(async () => {
    const apiBaseUrl = useStartupStore.getState().startupState?.apiBaseUrl
    if (!apiBaseUrl) {
      toast.error('API not configured — complete setup first')
      return
    }

    setSeeding(true)
    try {
      const token = useAuthStore.getState().auth.accessToken
      const response = await fetch(`${apiBaseUrl}/dev/seed`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Idempotency-Key': crypto.randomUUID(),
          ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
      })

      const body = await response.json() as {
        success: boolean
        data?: Record<string, number>
        error?: { message?: string }
      }

      if (!body.success) {
        throw new Error(body.error?.message ?? 'Seed failed')
      }

      const counts = body.data ?? {}
      const summary = Object.entries(counts)
        .map(([k, v]) => `${k}: ${v}`)
        .join(', ')
      toast.success(`Seeded — ${summary}`)
    }
    catch (err) {
      toast.error(extractErrorMessage(err, 'Seed failed'))
    }
    finally {
      setSeeding(false)
    }
  }, [])

  const handleToggleDevTools = useCallback(() => {
    const next = toggleDevTools()
    toast.info(next ? t('debug.devToolsEnabled') : t('debug.devToolsDisabled'))
  }, [t])

  return (
    <DropdownMenu modal={false}>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className={cn(
            'fixed z-50 flex size-7 items-center justify-center rounded-md',
            'bg-muted/60 text-muted-foreground opacity-40',
            'transition-opacity hover:opacity-100',
            'focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring',
          )}
          style={{ top: 'calc(var(--titlebar-h) + 0.5rem)', right: '0.75rem' }}
          aria-label="Debug tools"
        >
          <Wrench size={14} />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="min-w-44">
        <DropdownMenuItem
          onSelect={handleSeed}
          disabled={seeding}
        >
          {seeding ? t('debug.seeding') : t('debug.seedDatabase')}
        </DropdownMenuItem>
        <DropdownMenuItem onSelect={handleToggleDevTools}>
          {t('debug.toggleDevTools')}
          <DropdownMenuShortcut>Ctrl+Shift+D</DropdownMenuShortcut>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
