import type { BaseResponse } from '~/generated/types/BaseResponse'
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
  createBase,
  invalidateBases,
  updateBase,
} from '../data/base-api'

const baseFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  longName: z.string().nullable().optional(),
})

type BaseFormValues = z.infer<typeof baseFormSchema>

interface BaseMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: BaseResponse | null
}

export function BaseMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: BaseMutateDrawerProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<BaseFormValues>({
    resolver: zodResolver(baseFormSchema),
    defaultValues: {
      commonName: '',
      longName: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        longName: currentRow.longName ?? '',
      })
    }
    else {
      form.reset({
        commonName: '',
        longName: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: BaseFormValues) => {
    try {
      const payload = {
        ...values,
        longName: values.longName || null,
      }

      if (isUpdate && currentRow) {
        await updateBase(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:base.singular'),
          }),
        )
      }
      else {
        await createBase(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:base.singular'),
          }),
        )
      }

      await invalidateBases()
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
              ? t('catalog:base.edit')
              : t('catalog:base.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('catalog:base.edit')
              : t('catalog:base.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="base-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="commonName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:base.form.commonName')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="longName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:base.form.longName')}</FormLabel>
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
          <Button form="base-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
