import type { PortResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
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
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from '~/components/ui/sheet'
import {
  createPort,
  invalidatePorts,
  updatePort,
} from '../data/port-api'

const portFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  country: z.string().nullable().optional(),
})

type PortFormValues = z.infer<typeof portFormSchema>

interface PortMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: PortResponse | null
}

export function PortMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: PortMutateDrawerProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<PortFormValues>({
    resolver: zodResolver(portFormSchema),
    defaultValues: {
      commonName: '',
      country: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        country: currentRow.country ?? '',
      })
    }
    else {
      form.reset({
        commonName: '',
        country: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: PortFormValues) => {
    try {
      const payload = {
        ...values,
        country: values.country || null,
      }

      if (isUpdate && currentRow) {
        await updatePort(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:port.singular'),
          }),
        )
      }
      else {
        await createPort(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:port.singular'),
          }),
        )
      }

      await invalidatePorts()
      onOpenChange(false)
      form.reset()
    }
    catch (err) {
      toast.error(
        err instanceof Error ? err.message : t('common:toast.error'),
      )
    }
  }

  return (
    <Sheet
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
    >
      <SheetContent className="flex flex-col">
        <SheetHeader className="text-start">
          <SheetTitle>
            {isUpdate
              ? t('catalog:port.edit')
              : t('catalog:port.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('catalog:port.edit')
              : t('catalog:port.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="port-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="commonName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:port.form.commonName')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="country"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:port.form.country')}</FormLabel>
                  <FormControl>
                    <Input {...field} value={field.value ?? ''} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </form>
        </Form>
        <SheetFooter className="gap-2">
          <SheetClose asChild>
            <Button variant="outline">{t('common:actions.close')}</Button>
          </SheetClose>
          <Button form="port-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
