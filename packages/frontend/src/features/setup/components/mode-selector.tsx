import { Globe, HardDrive } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Card, CardDescription, CardHeader, CardTitle } from '~/components/ui/card'

type SetupMode = 'remote' | 'local'

interface ModeSelectorProps {
  onSelect: (mode: SetupMode) => void
}

export function ModeSelector({ onSelect }: ModeSelectorProps) {
  const { t } = useTranslation(['auth'])

  return (
    <div className="grid gap-4">
      <p className="text-sm text-muted-foreground text-center">
        {t('auth:setup.modeSelection')}
      </p>
      <div className="grid gap-3 sm:grid-cols-2">
        <Card
          className="cursor-pointer transition-colors hover:border-primary"
          onClick={() => onSelect('remote')}
        >
          <CardHeader className="items-center text-center">
            <Globe className="size-8 text-muted-foreground" />
            <CardTitle className="text-base">{t('auth:setup.remote')}</CardTitle>
            <CardDescription>{t('auth:setup.remoteDescription')}</CardDescription>
          </CardHeader>
        </Card>
        <Card
          className="cursor-pointer transition-colors hover:border-primary"
          onClick={() => onSelect('local')}
        >
          <CardHeader className="items-center text-center">
            <HardDrive className="size-8 text-muted-foreground" />
            <CardTitle className="text-base">{t('auth:setup.local')}</CardTitle>
            <CardDescription>{t('auth:setup.localDescription')}</CardDescription>
          </CardHeader>
        </Card>
      </div>
    </div>
  )
}
