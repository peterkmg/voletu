import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { BaseResponse } from '~/generated/types/BaseResponse'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, selectColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogBaseCreate, catalogBaseHardDelete, catalogBaseSoftDelete, catalogBaseUpdate } from '~/generated/client'
import { catalogBaseListQueryKey, useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type BasesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: BasesProvider, useEntity: useBases }
  = createEntityProvider<BaseResponse, BasesDialogType>('Bases')

// --- Row Actions ---

const DataTableRowActions = createRowActions<BaseResponse>({ useEntity: useBases })

// --- Columns ---

function getBaseColumns(t: TFunction): ColumnDef<BaseResponse>[] {
  return [
    selectColumn<BaseResponse>(),
    textColumn<BaseResponse>('commonName', t('catalog:base.columns.commonName')),
    textColumn<BaseResponse>('longName', t('catalog:base.columns.longName'), { primary: false }),
    dateColumn<BaseResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<BaseResponse>(DataTableRowActions),
  ]
}

// --- Table ---

const basesRoute = getRouteApi('/_authenticated/catalog/bases/')
const basesGlobalFilterFn = createGlobalFilter<BaseResponse>('commonName', 'longName')

interface BasesTableProps {
  data: BaseResponse[]
}

function BasesTable({ data }: BasesTableProps) {
  return (
    <EntityTable
      tableId="bases"
      data={data}
      getColumns={getBaseColumns}
      routeApi={basesRoute}
      globalFilterFn={basesGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
      bulkActions={t => [
        {
          label: t('common:actions.softDelete'),
          icon: Archive,
          variant: 'destructive',
          onClick: (rows) => {
            void rows // TODO: wire bulk soft-delete API
          },
        },
      ]}
    />
  )
}

// --- Mutate Dialog ---

const baseFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
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
  const { t } = useTranslation(['catalog', 'common'])

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
          <TextField<BaseFormValues> name="commonName" label={t('catalog:base.form.commonName')} />
          <TextField<BaseFormValues> name="longName" label={t('catalog:base.form.longName')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete Dialog ---

const BaseDeleteDialog = createDeleteDialog({
  useEntity: useBases,
  hardDeleteFn: catalogBaseHardDelete,
  softDeleteFn: catalogBaseSoftDelete,
  queryKey: catalogBaseListQueryKey,
  entityLabel: 'catalog:base.singular',
  i18nNamespaces: ['common', 'catalog'],
})

// --- Entity Dialogs ---

const BasesDialogs = createEntityDialogs({
  useEntity: useBases,
  MutateDialog: BaseMutateDialog,
  DeleteDialog: BaseDeleteDialog,
})

// --- Primary Buttons ---

const BasesPrimaryButtons = createPrimaryButtons({
  useEntity: useBases,
  createLabel: 'catalog:base.create',
  i18nNamespaces: ['catalog'],
})

// --- Page ---

export function Bases() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogBaseList()

  return (
    <EntityPage
      provider={BasesProvider}
      title={t('catalog:base.title')}
      queryResult={queryResult}
      primaryButtons={BasesPrimaryButtons}
      table={BasesTable}
      dialogs={BasesDialogs}
    />
  )
}
