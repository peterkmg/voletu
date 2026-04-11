import { useNavigate } from '@tanstack/react-router'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import { useStartupStore } from '~/stores/startup-store'
import { LocalConfigForm } from '~/views/setup/components/local-config-form'
import { ModeSelector } from '~/views/setup/components/mode-selector'
import { RemoteConfigForm } from '~/views/setup/components/remote-config-form'

type SetupStep = 'selecting' | 'remote' | 'local'

export function Setup() {
  const { t } = useTranslation(['auth'])
  const navigate = useNavigate()
  const [step, setStep] = useState<SetupStep>('selecting')
  const needsSetup = useStartupStore(s => s.startupState?.needsSetup)

  // Navigate to sign-in reactively when setup completes.
  // This fires after applyStartupState sets needsSetup=false
  // from either local or remote config flows.
  useEffect(() => {
    if (needsSetup === false) {
      void navigate({ to: '/sign-in' })
    }
  }, [needsSetup, navigate])

  const handleBack = () => setStep('selecting')

  return (
    <Card className="w-full max-w-lg">
      <CardHeader>
        <CardTitle>{t('auth:setup.title')}</CardTitle>
        <CardDescription>{t('auth:setup.subtitle')}</CardDescription>
      </CardHeader>
      <CardContent>
        {step === 'selecting' && (
          <ModeSelector onSelect={mode => setStep(mode)} />
        )}
        {step === 'remote' && (
          <RemoteConfigForm onBack={handleBack} />
        )}
        {step === 'local' && (
          <LocalConfigForm onBack={handleBack} />
        )}
      </CardContent>
    </Card>
  )
}
