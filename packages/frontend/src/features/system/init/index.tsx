import { zodResolver } from '@hookform/resolvers/zod'
import { useQueryClient } from '@tanstack/react-query'
import { useNavigate } from '@tanstack/react-router'
import { useForm, useWatch } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { PasswordInput } from '~/components/password-input'
import { Button } from '~/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { useIdempotencyKey } from '~/hooks/use-idempotency-key'
import { useAuthStore } from '~/stores/auth-store'

const initFormSchema = z
  .object({
    nodeName: z.string().min(2).max(100),
    nodeType: z.enum(['CENTRAL', 'PERIPHERAL']),
    centralApiUrl: z.string().url().optional().or(z.literal('')),
    action: z.enum(['REPLACE', 'DELETE']),
    newUsername: z.string().min(3).max(50).optional().or(z.literal('')),
    newPassword: z.string().min(8).optional().or(z.literal('')),
    fullname: z.string().min(2).max(100).optional().or(z.literal('')),
  })
  .refine(
    data =>
      data.nodeType !== 'PERIPHERAL' || (data.centralApiUrl && data.centralApiUrl.length > 0),
    { message: 'Central Server URL is required for Peripheral nodes', path: ['centralApiUrl'] },
  )
  .refine(
    data =>
      data.action !== 'REPLACE'
      || (data.newUsername && data.newUsername.length >= 3
        && data.newPassword && data.newPassword.length >= 8),
    { message: 'Username and password are required when replacing default admin', path: ['newUsername'] },
  )

type InitFormValues = z.infer<typeof initFormSchema>

function getBaseUrl(): string {
  return (
    (globalThis as { __VOLETU_API_BASE_URL__?: string }).__VOLETU_API_BASE_URL__
    ?? 'http://127.0.0.1:3000'
  ).replace(/\/+$/, '')
}

export function InitializePage() {
  const { t } = useTranslation(['system', 'common'])
  const navigate = useNavigate()
  const queryClient = useQueryClient()
  const idempotencyKey = useIdempotencyKey()

  const form = useForm<InitFormValues>({
    resolver: zodResolver(initFormSchema),
    defaultValues: {
      nodeName: '',
      nodeType: 'PERIPHERAL',
      centralApiUrl: '',
      action: 'REPLACE',
      newUsername: '',
      newPassword: '',
      fullname: '',
    },
  })

  const nodeType = useWatch({ control: form.control, name: 'nodeType' })
  const action = useWatch({ control: form.control, name: 'action' })

  async function onSubmit(values: InitFormValues) {
    const token = useAuthStore.getState().auth.accessToken
    try {
      const res = await fetch(`${getBaseUrl()}/node/initialize`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
          'Idempotency-Key': idempotencyKey,
        },
        body: JSON.stringify({
          action: values.action,
          nodeType: values.nodeType,
          nodeName: values.nodeName || undefined,
          centralApiUrl: (values.nodeType === 'PERIPHERAL' && values.centralApiUrl) ? values.centralApiUrl : undefined,
          newUsername: values.action === 'REPLACE' ? values.newUsername : undefined,
          newPassword: values.action === 'REPLACE' ? values.newPassword : undefined,
          fullname: values.action === 'REPLACE' ? (values.fullname || undefined) : undefined,
        }),
      })

      if (!res.ok) {
        const body = await res.json().catch(() => null)
        throw new Error(body?.error?.message ?? `Request failed with status ${res.status}`)
      }

      toast.success(t('system:node.init.success'))
      queryClient.invalidateQueries({ queryKey: ['health'] })
      navigate({ to: '/' })
    }
    catch (err) {
      toast.error(err instanceof Error ? err.message : t('common:toast.error'))
    }
  }

  return (
    <>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">
            {t('system:node.init.pageTitle')}
          </h2>
        </div>

        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
            <Card>
              <CardHeader>
                <CardTitle>{t('system:node.init.nodeType')}</CardTitle>
                <CardDescription>
                  Configure the node identity and type
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <FormField
                  control={form.control}
                  name="nodeName"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t('system:node.init.nodeName')}</FormLabel>
                      <FormControl>
                        <Input placeholder="e.g. Warehouse Alpha" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={form.control}
                  name="nodeType"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t('system:node.init.nodeType')}</FormLabel>
                      <Select
                        onValueChange={field.onChange}
                        defaultValue={field.value}
                      >
                        <FormControl>
                          <SelectTrigger>
                            <SelectValue />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          <SelectItem value="CENTRAL">
                            {t('system:node.init.central')}
                          </SelectItem>
                          <SelectItem value="PERIPHERAL">
                            {t('system:node.init.peripheral')}
                          </SelectItem>
                        </SelectContent>
                      </Select>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {nodeType === 'PERIPHERAL' && (
                  <FormField
                    control={form.control}
                    name="centralApiUrl"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>{t('system:node.init.centralUrl')}</FormLabel>
                        <FormControl>
                          <Input
                            placeholder="https://central.example.com"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                )}
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>{t('system:node.init.adminAction')}</CardTitle>
                <CardDescription>
                  Choose what to do with the default admin account
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <FormField
                  control={form.control}
                  name="action"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t('system:node.init.adminAction')}</FormLabel>
                      <Select
                        onValueChange={field.onChange}
                        defaultValue={field.value}
                      >
                        <FormControl>
                          <SelectTrigger>
                            <SelectValue />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          <SelectItem value="REPLACE">
                            {t('system:node.init.actionReplace')}
                          </SelectItem>
                          <SelectItem value="DELETE">
                            {t('system:node.init.actionDelete')}
                          </SelectItem>
                        </SelectContent>
                      </Select>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {action === 'REPLACE' && (
                  <>
                    <FormField
                      control={form.control}
                      name="newUsername"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>{t('system:node.init.newUsername')}</FormLabel>
                          <FormControl>
                            <Input {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="newPassword"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>{t('system:node.init.newPassword')}</FormLabel>
                          <FormControl>
                            <PasswordInput {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    <FormField
                      control={form.control}
                      name="fullname"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>{t('system:node.init.fullname')}</FormLabel>
                          <FormControl>
                            <Input {...field} />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  </>
                )}
              </CardContent>
            </Card>

            <Button
              type="submit"
              size="lg"
              disabled={form.formState.isSubmitting}
            >
              {t('system:node.init.submit')}
            </Button>
          </form>
        </Form>
      </Main>
    </>
  )
}
