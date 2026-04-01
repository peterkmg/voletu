import { zodResolver } from '@hookform/resolvers/zod'
import { useNavigate } from '@tanstack/react-router'
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
import { useLogin } from '~/features/auth/hooks/use-login'

const loginSchema = z.object({
  username: z.string().min(1),
  password: z.string().min(1),
})

type LoginFormValues = z.infer<typeof loginSchema>

interface LoginFormProps {
  redirect?: string
}

export function LoginForm({ redirect }: LoginFormProps) {
  const { t } = useTranslation(['auth', 'common'])
  const navigate = useNavigate()
  const { login, isLoading, error } = useLogin()

  const form = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      username: '',
      password: '',
    },
  })

  const onSubmit = async (values: LoginFormValues) => {
    try {
      await login(values.username, values.password)
      await navigate({ to: redirect ?? '/' })
    }
    catch {
      // error is handled by useLogin
    }
  }

  return (
    <Card className="w-full max-w-sm">
      <CardHeader>
        <CardTitle>{t('auth:login.title')}</CardTitle>
        <CardDescription>{t('auth:login.subtitle')}</CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="grid gap-4">
            <FormField
              control={form.control}
              name="username"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('auth:login.username')}</FormLabel>
                  <FormControl>
                    <Input
                      autoComplete="username"
                      autoFocus
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('auth:login.password')}</FormLabel>
                  <FormControl>
                    <PasswordInput
                      autoComplete="current-password"
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            {error && (
              <p className="text-sm text-destructive">{error}</p>
            )}
            <Button type="submit" className="w-full" disabled={isLoading}>
              {t('auth:login.submit')}
            </Button>
          </form>
        </Form>
      </CardContent>
    </Card>
  )
}
