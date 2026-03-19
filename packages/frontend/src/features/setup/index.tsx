import { useNavigate } from '@tanstack/react-router'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import { LocalConfigForm } from '~/features/setup/components/local-config-form'
import { ModeSelector } from '~/features/setup/components/mode-selector'
import { RemoteConfigForm } from '~/features/setup/components/remote-config-form'

type SetupStep = 'selecting' | 'remote' | 'local'

export function Setup() {
  const { t } = useTranslation(['auth'])
  const navigate = useNavigate()
  const [step, setStep] = useState<SetupStep>('selecting')

  const handleBack = () => setStep('selecting')

  const handleLocalComplete = () => {
    navigate({ to: '/sign-in' })
  }

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
          <LocalConfigForm onBack={handleBack} onComplete={handleLocalComplete} />
        )}
      </CardContent>
    </Card>
  )
}
