import type { WarehouseResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/form-dialog'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import { BaseMutateDialog } from '~/features/catalog/bases/components/base-mutate-dialog'
import { catalogWarehouseCreate, catalogWarehouseUpdate } from '~/generated/client'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { catalogWarehouseListQueryKey } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { queryClient } from '~/shared/api/query-client'

const warehouseFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  baseId: z.string().min(1, 'Base is required'),
})

type WarehouseFormValues = z.infer<typeof warehouseFormSchema>

interface WarehouseMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: WarehouseResponse | null
  onCreated?: (id: string) => void
}

export function WarehouseMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: WarehouseMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])
  const isUpdate = !!currentRow

  const basesQuery = useCatalogBaseList()

  const form = useForm<WarehouseFormValues>({
    resolver: zodResolver(warehouseFormSchema),
    defaultValues: {
      commonName: '',
      baseId: '',
    },
  })

  useEffect(() => {
    if (currentRow) {
      form.reset({
        commonName: currentRow.commonName,
        baseId: currentRow.baseId,
      })
    }
    else {
      form.reset({
        commonName: '',
        baseId: '',
      })
    }
  }, [currentRow, form])

  const onSubmit = async (values: WarehouseFormValues) => {
    try {
      if (isUpdate && currentRow) {
        await catalogWarehouseUpdate(currentRow.id, values)
        toast.success(
          t('common:toast.updateSuccess', {
            entity: t('catalog:warehouse.singular'),
          }),
        )
      }
      else {
        const result = await catalogWarehouseCreate(values)
        toast.success(
          t('common:toast.createSuccess', {
            entity: t('catalog:warehouse.singular'),
          }),
        )
        if (onCreated && result?.data?.id) {
          onCreated(result.data.id)
        }
      }

      await queryClient.invalidateQueries({ queryKey: catalogWarehouseListQueryKey() })
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
    <FormDialog
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
      title={isUpdate ? t('catalog:warehouse.edit') : t('catalog:warehouse.create')}
      description={isUpdate ? t('catalog:warehouse.edit') : t('catalog:warehouse.create')}
      formId="warehouse-form"
    >
      <Form {...form}>
        <form
          id="warehouse-form"
          onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-5"
        >
          <FormField
            control={form.control}
            name="commonName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('catalog:warehouse.form.commonName')}</FormLabel>
                <FormControl>
                  <Input {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <EntityPickerField<WarehouseFormValues>
            name="baseId"
            label={t('catalog:warehouse.form.baseId')}
            placeholder={t('catalog:warehouse.form.selectBase')}
            queryResult={basesQuery}
            displayField="commonName"
            allowCreate
            createDialog={BaseMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}
