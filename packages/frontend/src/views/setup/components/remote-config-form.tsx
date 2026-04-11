import { zodResolver } from '@hookform/resolvers/zod'
import { useNavigate } from '@tanstack/react-router'
import { CheckCircle, Loader2, XCircle } from 'lucide-react'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { Button } from '~/components/ui/button'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import { checkHealth } from '~/platform/runtime/health'
import { useSetupFlow } from '~/views/setup/hooks/use-setup-flow'

const remoteSchema = z.object({
  apiUrl: z.string().url().min(1),
})

type RemoteFormValues = z.infer<typeof remoteSchema>

type HealthStatus = 'idle' | 'testing' | 'success' | 'error'

interface RemoteConfigFormProps {
  onBack: () => void
}

export function RemoteConfigForm({ onBack }: RemoteConfigFormProps) {
  const { t } = useTranslation(['auth', 'common'])
  const navigate = useNavigate()
  const { isSubmitting, error, submitRemoteConfig } = useSetupFlow()
  const [healthStatus, setHealthStatus] = useState<HealthStatus>('idle')

  const form = useForm<RemoteFormValues>({
    resolver: zodResolver(remoteSchema),
    defaultValues: {
      apiUrl: 'http://127.0.0.1:3000',
    },
  })

  const testConnection = async () => {
    const apiUrl = form.getValues('apiUrl')
    const valid = await form.trigger('apiUrl')
    if (!valid)
      return

    setHealthStatus('testing')
    const healthy = await checkHealth(apiUrl)
    setHealthStatus(healthy ? 'success' : 'error')
  }

  const onSubmit = async (values: RemoteFormValues) => {
    try {
      await submitRemoteConfig(values.apiUrl)
      await navigate({ to: '/sign-in' })
    }
    catch {
      // error handled by hook
    }
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="grid gap-4">
        <FormField
          control={form.control}
          name="apiUrl"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t('auth:setup.apiUrl')}</FormLabel>
              <FormControl>
                <Input
                  placeholder="http://127.0.0.1:3000"
                  {...field}
                  onChange={(e) => {
                    field.onChange(e)
                    setHealthStatus('idle')
                  }}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <div className="flex items-center gap-2">
          <Button
            type="button"
            variant="outline"
            onClick={testConnection}
            disabled={healthStatus === 'testing'}
          >
            {healthStatus === 'testing' && (
              <Loader2 className="size-4 animate-spin" />
            )}
            {t('auth:setup.testConnection')}
          </Button>
          {healthStatus === 'success' && (
            <span className="flex items-center gap-1 text-sm text-green-600">
              <CheckCircle className="size-4" />
              {t('auth:setup.healthCheckSuccess')}
            </span>
          )}
          {healthStatus === 'error' && (
            <span className="flex items-center gap-1 text-sm text-destructive">
              <XCircle className="size-4" />
              {t('auth:setup.healthCheckError')}
            </span>
          )}
        </div>

        {error && (
          <p className="text-sm text-destructive">{error}</p>
        )}

        <div className="flex gap-2 justify-end">
          <Button type="button" variant="outline" onClick={onBack}>
            {t('common:actions.back')}
          </Button>
          <Button
            type="submit"
            disabled={healthStatus !== 'success' || isSubmitting}
          >
            {isSubmitting && <Loader2 className="size-4 animate-spin" />}
            {t('auth:setup.saveAndContinue')}
          </Button>
        </div>
      </form>
    </Form>
  )
}
