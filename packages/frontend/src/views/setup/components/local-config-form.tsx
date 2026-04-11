import type { Resolver } from 'react-hook-form'
import type { DbType, SaveLocalConfigPayload } from '~/tauri/commands'
import { zodResolver } from '@hookform/resolvers/zod'
import { Loader2 } from 'lucide-react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { PasswordInput } from '~/components/forms/password-input'
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { Separator } from '~/components/ui/separator'
import { useSetupFlow } from '~/views/setup/hooks/use-setup-flow'

const localSchema = z.object({
  dbType: z.enum(['sqlite', 'postgres', 'mysql']),
  sqliteFile: z.string().optional(),
  host: z.string().optional(),
  port: z.coerce.number().int().positive().optional(),
  database: z.string().optional(),
  username: z.string().optional(),
  dbPassword: z.string().min(1, 'Password is required'),
  jwtExpirationSeconds: z.coerce.number().int().positive(),
  jwtRefreshExpirationSeconds: z.coerce.number().int().positive(),
})

type LocalFormValues = z.output<typeof localSchema>

interface LocalConfigFormProps {
  onBack: () => void
}

export function LocalConfigForm({ onBack }: LocalConfigFormProps) {
  const { t } = useTranslation(['auth', 'common'])
  const { isSubmitting, error, submitLocalConfig } = useSetupFlow()

  // SAFETY: zodResolver handles z.coerce input/output type mismatch at runtime correctly;
  // cast needed because Zod v4 coerce types expose `unknown` as input which RHF can't spread.
  const form = useForm<LocalFormValues>({
    resolver: zodResolver(localSchema) as unknown as Resolver<LocalFormValues>,
    defaultValues: {
      dbType: 'sqlite',
      sqliteFile: 'voletu.db',
      host: '127.0.0.1',
      port: 5432,
      database: '',
      username: '',
      dbPassword: '',
      jwtExpirationSeconds: 3600,
      jwtRefreshExpirationSeconds: 86400,
    },
  })

  const dbType = form.watch('dbType') as DbType
  const isSqlite = dbType === 'sqlite'

  const onSubmit = async (values: LocalFormValues) => {
    const payload: SaveLocalConfigPayload = {
      dbType: values.dbType as DbType,
      dbPassword: values.dbPassword,
      jwtExpirationSeconds: values.jwtExpirationSeconds,
      jwtRefreshExpirationSeconds: values.jwtRefreshExpirationSeconds,
      ...(isSqlite
        ? { sqliteFile: values.sqliteFile }
        : {
            host: values.host,
            port: values.port,
            database: values.database,
            username: values.username,
          }),
    }

    try {
      await submitLocalConfig(payload)
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
          name="dbType"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t('auth:setup.dbType')}</FormLabel>
              <Select
                onValueChange={field.onChange}
                defaultValue={field.value}
              >
                <FormControl>
                  <SelectTrigger className="w-full">
                    <SelectValue />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectItem value="sqlite">SQLite</SelectItem>
                  <SelectItem value="postgres">PostgreSQL</SelectItem>
                  <SelectItem value="mysql">MySQL</SelectItem>
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />

        {isSqlite
          ? (
              <FormField
                control={form.control}
                name="sqliteFile"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>{t('auth:setup.sqliteFile')}</FormLabel>
                    <FormControl>
                      <Input placeholder="voletu.db" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            )
          : (
              <>
                <div className="grid grid-cols-2 gap-3">
                  <FormField
                    control={form.control}
                    name="host"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>{t('auth:setup.host')}</FormLabel>
                        <FormControl>
                          <Input placeholder="127.0.0.1" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="port"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>{t('auth:setup.port')}</FormLabel>
                        <FormControl>
                          <Input type="number" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </div>
                <FormField
                  control={form.control}
                  name="database"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t('auth:setup.database')}</FormLabel>
                      <FormControl>
                        <Input placeholder="voletu" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="username"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t('auth:setup.dbUsername')}</FormLabel>
                      <FormControl>
                        <Input {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </>
            )}

        <FormField
          control={form.control}
          name="dbPassword"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t('auth:setup.dbPassword')}</FormLabel>
              <FormControl>
                <PasswordInput {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <Separator />

        <div className="grid grid-cols-2 gap-3">
          <FormField
            control={form.control}
            name="jwtExpirationSeconds"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('auth:setup.jwtExpiration')}</FormLabel>
                <FormControl>
                  <Input type="number" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="jwtRefreshExpirationSeconds"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('auth:setup.jwtRefreshExpiration')}</FormLabel>
                <FormControl>
                  <Input type="number" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>

        {error && (
          <p className="text-sm text-destructive">{error}</p>
        )}

        <div className="flex gap-2 justify-end">
          <Button type="button" variant="outline" onClick={onBack}>
            {t('common:actions.back')}
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting && <Loader2 className="size-4 animate-spin" />}
            {t('auth:setup.saveAndStart')}
          </Button>
        </div>
      </form>
    </Form>
  )
}
