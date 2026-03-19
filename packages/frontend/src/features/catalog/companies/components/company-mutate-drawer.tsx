import type { CompanyResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { Button } from '~/components/ui/button'
import { Checkbox } from '~/components/ui/checkbox'
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
import { catalogCompanyCreate, catalogCompanyUpdate } from '~/generated/client'
import { catalogCompanyListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { queryClient } from '~/shared/api/query-client'

const companyFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  legalName: z.string().nullable().optional(),
  isContractor: z.boolean(),
  isExporter: z.boolean(),
  isManufacturer: z.boolean(),
  isSender: z.boolean(),
})

type CompanyFormValues = z.infer<typeof companyFormSchema>

interface CompanyMutateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: CompanyResponse | null
}

export function CompanyMutateDrawer({
  open,
  onOpenChange,
  currentRow,
}: CompanyMutateDrawerProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const form = useForm<CompanyFormValues>({
    resolver: zodResolver(companyFormSchema),
    defaultValues: {
      commonName: '',
      legalName: '',
      isContractor: false,
      isExporter: false,
      isManufacturer: false,
      isSender: false,
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        legalName: currentRow.legalName ?? '',
        isContractor: currentRow.isContractor,
        isExporter: currentRow.isExporter,
        isManufacturer: currentRow.isManufacturer,
        isSender: currentRow.isSender,
      })
    }
    else {
      form.reset({
        commonName: '',
        legalName: '',
        isContractor: false,
        isExporter: false,
        isManufacturer: false,
        isSender: false,
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: CompanyFormValues) => {
    try {
      const payload = {
        ...values,
        legalName: values.legalName || null,
      }

      if (isUpdate && currentRow) {
        await catalogCompanyUpdate(currentRow.id, payload)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:company.singular'),
          }),
        )
      }
      else {
        await catalogCompanyCreate(payload)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:company.singular'),
          }),
        )
      }

      await queryClient.invalidateQueries({ queryKey: catalogCompanyListQueryKey() })
      onOpenChange(false)
      form.reset()
    }
    catch (err) {
      toast.error(
        err instanceof Error ? err.message : t('common:toast.error'),
      )
    }
  }

  const booleanFields = [
    { name: 'isContractor' as const, label: t('catalog:company.form.isContractor') },
    { name: 'isExporter' as const, label: t('catalog:company.form.isExporter') },
    { name: 'isManufacturer' as const, label: t('catalog:company.form.isManufacturer') },
    { name: 'isSender' as const, label: t('catalog:company.form.isSender') },
  ]

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
              ? t('catalog:company.edit')
              : t('catalog:company.create')}
          </SheetTitle>
          <SheetDescription>
            {isUpdate
              ? t('catalog:company.edit')
              : t('catalog:company.create')}
          </SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="company-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="commonName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:company.form.commonName')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="legalName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('catalog:company.form.legalName')}</FormLabel>
                  <FormControl>
                    <Input {...field} value={field.value ?? ''} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className="space-y-3">
              {booleanFields.map(({ name, label }) => (
                <FormField
                  key={name}
                  control={form.control}
                  name={name}
                  render={({ field }) => (
                    <FormItem className="flex items-center gap-2">
                      <FormControl>
                        <Checkbox
                          checked={field.value}
                          onCheckedChange={field.onChange}
                        />
                      </FormControl>
                      <FormLabel className="!mt-0">{label}</FormLabel>
                    </FormItem>
                  )}
                />
              ))}
            </div>
          </form>
        </Form>
        <SheetFooter className="gap-2">
          <SheetClose asChild>
            <Button variant="outline">{t('common:actions.close')}</Button>
          </SheetClose>
          <Button form="company-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}
