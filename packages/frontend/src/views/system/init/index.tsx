import { zodResolver } from '@hookform/resolvers/zod'
import { useQueryClient } from '@tanstack/react-query'
import { useNavigate } from '@tanstack/react-router'
import { Loader2 } from 'lucide-react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { PasswordInput } from '~/components/forms/password-input'
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
import { Separator } from '~/components/ui/separator'
import { useNodeCompleteInitialization } from '~/generated/hooks/SystemAuthHooks/useNodeCompleteInitialization'
import { extractErrorMessage } from '~/lib/error'
import { useNodeStore } from '~/stores/node-store'

const initFormSchema = z
  .object({
    name: z.string().min(2).max(100),
    nodeType: z.enum(['CENTRAL', 'PERIPHERAL']),
    centralApiUrl: z.string().url().optional().or(z.literal('')),
    newUsername: z.string().min(3).max(50),
    newPassword: z.string().min(5),
    fullname: z.string().min(2).max(100).optional().or(z.literal('')),
  })
  .refine(
    data =>
      data.nodeType !== 'PERIPHERAL' || (data.centralApiUrl && data.centralApiUrl.length > 0),
    { message: 'forms.validation.centralUrlRequired', path: ['centralApiUrl'] },
  )

type InitFormValues = z.infer<typeof initFormSchema>

export function InitializePage() {
  const { t } = useTranslation(['system', 'common'])
  const navigate = useNavigate()
  const queryClient = useQueryClient()
  const mutation = useNodeCompleteInitialization()

  const form = useForm<InitFormValues>({
    resolver: zodResolver(initFormSchema),
    defaultValues: {
      name: '',
      nodeType: 'PERIPHERAL',
      centralApiUrl: '',
      newUsername: '',
      newPassword: '',
      fullname: '',
    },
    mode: 'onTouched',
  })

  const nodeType = form.watch('nodeType')

  async function onSubmit(values: InitFormValues) {
    try {
      await mutation.mutateAsync({
        data: {
          nodeType: values.nodeType,
          nodeName: values.name || undefined,
          centralApiUrl: (values.nodeType === 'PERIPHERAL' && values.centralApiUrl) ? values.centralApiUrl : undefined,
          newUsername: values.newUsername,
          newPassword: values.newPassword,
          fullname: values.fullname || undefined,
        },
      })

      useNodeStore.getState().setStatus({ isInitialized: true })
      queryClient.invalidateQueries({ queryKey: ['health'] })
      await navigate({ to: '/' })
    }
    catch {

    }
  }

  const canSubmit = form.formState.isValid && !mutation.isPending

  return (
    <Card className="w-full max-w-md">
      <CardHeader>
        <CardTitle>{t('system:node.init.pageTitle')}</CardTitle>
        <CardDescription>{t('system:node.init.description')}</CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="grid gap-4">
            <p className="text-sm font-medium">{t('system:node.init.sectionNode')}</p>

            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('system:node.init.nodeName')}</FormLabel>
                  <FormControl>
                    <Input placeholder={t('system:node.init.nodeNamePlaceholder')} {...field} />
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

            <Separator />

            <p className="text-sm font-medium">{t('system:node.init.sectionAdmin')}</p>

            <FormField
              control={form.control}
              name="newUsername"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('system:node.init.username')}</FormLabel>
                  <FormControl>
                    <Input autoComplete="username" {...field} />
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
                  <FormLabel>{t('system:node.init.password')}</FormLabel>
                  <FormControl>
                    <PasswordInput autoComplete="new-password" {...field} />
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

            {mutation.error && (
              <p className="text-sm text-destructive">
                {extractErrorMessage(mutation.error, t('common:toast.error'))}
              </p>
            )}

            <Button type="submit" className="w-full" disabled={!canSubmit}>
              {mutation.isPending && <Loader2 className="size-4 animate-spin" />}
              {t('system:node.init.submit')}
            </Button>
          </form>
        </Form>
      </CardContent>
    </Card>
  )
}
