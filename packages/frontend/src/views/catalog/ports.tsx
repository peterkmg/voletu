import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PortResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, textColumn } from '~/components/data-table'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogPortCreate, catalogPortHardDelete, catalogPortSoftDelete, catalogPortUpdate } from '~/generated/client'
import { catalogPortListQueryKey, useCatalogPortList } from '~/generated/hooks/CatalogHooks/useCatalogPortList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { defineCrudViews } from '~/lib/define-crud-views'

// --- Columns ---

function getPortColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<PortResponse> }>,
): ColumnDef<PortResponse>[] {
  return [
    textColumn<PortResponse>('commonName', t('catalog:port.columns.commonName')),
    textColumn<PortResponse>('country', t('catalog:port.columns.longName'), { primary: false, sizing: 'capped', maxSize: 200 }),
    { ...dateColumn<PortResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), sizingCategory: 'capped', requiresRole: 'senior_supervisor' } },
    actionsColumn<PortResponse>(RowActions, 2),
  ]
}

// --- Table ---

const portsRoute = getRouteApi('/_authenticated/catalog/ports/')
const portsGlobalFilterFn = createGlobalFilter<PortResponse>('commonName', 'country')

interface PortsTableProps {
  data: PortResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<PortResponse> }>
}

function PortsTable({ data, actions, RowActions }: PortsTableProps) {
  return (
    <EntityTable
      tableId="ports"
      data={data}
      getColumns={t => getPortColumns(t, RowActions)}
      routeApi={portsRoute}
      globalFilterFn={portsGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
      actions={actions}
    />
  )
}

function usePortsTitle() {
  return useTranslation(['catalog']).t('catalog:port.title')
}

// --- Mutate Dialog ---

const portFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  country: z.string().nullable().optional(),
})

type PortFormValues = z.infer<typeof portFormSchema>

interface PortMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: PortResponse | null
  onCreated?: (id: string) => void
}

function PortMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: PortMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: portFormSchema,
    defaultValues: {
      commonName: '',
      country: '',
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      country: row.country ?? '',
    }),
    transformPayload: values => ({
      ...values,
      country: values.country || null,
    }),
    createFn: catalogPortCreate,
    updateFn: catalogPortUpdate,
    queryKey: catalogPortListQueryKey(),
    entityLabel: t('catalog:port.singular'),
    formId: 'port-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:port.edit') : t('catalog:port.create')}
      description={isUpdate ? t('catalog:port.edit') : t('catalog:port.create')}
      formId="port-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="port-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<PortFormValues> name="commonName" label={t('catalog:port.form.commonName')} />
          <TextField<PortFormValues> name="country" label={t('catalog:port.form.country')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}

const portsViewDefinition = defineCrudViews<PortResponse>({
  displayName: 'Ports',
  useTitle: usePortsTitle,
  useQuery: useCatalogPortList,
  Table: PortsTable,
  MutateDialog: PortMutateDialog,
  deleteDialog: {
    hardDeleteFn: catalogPortHardDelete,
    softDeleteFn: catalogPortSoftDelete,
    queryKey: catalogPortListQueryKey,
    entityLabel: 'catalog:port.singular',
    i18nNamespaces: ['common', 'catalog'],
  },
})

export function Ports() {
  return <portsViewDefinition.View />
}
