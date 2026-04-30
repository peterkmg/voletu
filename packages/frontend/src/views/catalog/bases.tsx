import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BaseResponse } from '~/generated/types/BaseResponse'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, textColumn } from '~/components/data-table'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogBaseCreate, catalogBaseHardDelete, catalogBaseSoftDelete, catalogBaseUpdate } from '~/generated/client'
import { catalogBaseListQueryKey, useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { defineCrudViews } from '~/lib/define-crud-views'

function getBaseColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<BaseResponse> }>,
): ColumnDef<BaseResponse>[] {
  return [
    textColumn<BaseResponse>('commonName', t('entities:commonName'), { sizing: 'capped', maxSize: 250 }),
    textColumn<BaseResponse>('longName', t('entities:longName'), { primary: false }),
    { ...dateColumn<BaseResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), sizingCategory: 'capped', requiresRole: 'senior_supervisor' } },
    actionsColumn<BaseResponse>(RowActions, 2),
  ]
}

const basesRoute = getRouteApi('/_authenticated/catalog/bases/')
const basesGlobalFilterFn = createGlobalFilter<BaseResponse>('commonName', 'longName')

interface BasesTableProps {
  data: BaseResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<BaseResponse> }>
}

function BasesTable({ data, actions, RowActions }: BasesTableProps) {
  return (
    <EntityTable
      tableId="bases"
      data={data}
      getColumns={t => getBaseColumns(t, RowActions)}
      routeApi={basesRoute}
      globalFilterFn={basesGlobalFilterFn}
      i18nNamespaces={['catalog', 'entities', 'common']}
      actions={actions}
    />
  )
}

function useBasesTitle() {
  return useTranslation(['catalog']).t('catalog:base.title')
}

const baseFormSchema = z.object({
  commonName: z.string().min(1),
  longName: z.string().nullable().optional(),
})

type BaseFormValues = z.infer<typeof baseFormSchema>

interface BaseMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: BaseResponse | null
  onCreated?: (id: string) => void
}

export function BaseMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: BaseMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'entities', 'common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: baseFormSchema,
    defaultValues: {
      commonName: '',
      longName: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      longName: row.longName ?? '',
    }),
    transformPayload: values => ({
      ...values,
      longName: values.longName || null,
    }),
    createFn: catalogBaseCreate,
    updateFn: catalogBaseUpdate,
    queryKey: catalogBaseListQueryKey(),
    entityLabel: t('catalog:base.singular'),
    formId: 'base-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:base.edit') : t('catalog:base.create')}
      description={isUpdate ? t('catalog:base.edit') : t('catalog:base.create')}
      formId="base-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="base-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<BaseFormValues> name="commonName" label={t('entities:commonName')} />
          <TextField<BaseFormValues> name="longName" label={t('entities:longName')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}

const basesViewDefinition = defineCrudViews<BaseResponse>({
  displayName: 'Bases',
  useTitle: useBasesTitle,
  useQuery: useCatalogBaseList,
  Table: BasesTable,
  MutateDialog: BaseMutateDialog,
  deleteDialog: {
    hardDeleteFn: catalogBaseHardDelete,
    softDeleteFn: catalogBaseSoftDelete,
    queryKey: catalogBaseListQueryKey,
    entityLabel: 'catalog:base.singular',
    i18nNamespaces: ['common', 'catalog'],
  },
})

export function Bases() {
  return <basesViewDefinition.View />
}
