import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PortResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogPortCreate, catalogPortHardDelete, catalogPortSoftDelete, catalogPortUpdate } from '~/generated/client'
import { catalogPortListQueryKey, useCatalogPortList } from '~/generated/hooks/CatalogHooks/useCatalogPortList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type PortsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: PortsProvider, useEntity: usePorts }
  = createEntityProvider<PortResponse, PortsDialogType>('Ports')

// --- Row Actions ---

const DataTableRowActions = createRowActions<PortResponse>({ useEntity: usePorts })

// --- Columns ---

function getPortColumns(t: TFunction): ColumnDef<PortResponse>[] {
  return [
    textColumn<PortResponse>('commonName', t('catalog:port.columns.commonName'), { sizing: 'capped', maxSize: 250 }),
    textColumn<PortResponse>('country', t('catalog:port.columns.longName'), { primary: false, sizing: 'capped', maxSize: 180 }),
    dateColumn<PortResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<PortResponse>(DataTableRowActions, 2),
  ]
}

// --- Table ---

const portsRoute = getRouteApi('/_authenticated/catalog/ports/')
const portsGlobalFilterFn = createGlobalFilter<PortResponse>('commonName', 'country')

interface PortsTableProps {
  data: PortResponse[]
}

function PortsTable({ data }: PortsTableProps) {
  return (
    <EntityTable
      tableId="ports"
      data={data}
      getColumns={getPortColumns}
      routeApi={portsRoute}
      globalFilterFn={portsGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
    />
  )
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
}

function PortMutateDialog({
  open,
  onOpenChange,
  currentRow,
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

// --- Delete Dialog ---

const PortDeleteDialog = createDeleteDialog({
  useEntity: usePorts,
  hardDeleteFn: catalogPortHardDelete,
  softDeleteFn: catalogPortSoftDelete,
  queryKey: catalogPortListQueryKey,
  entityLabel: 'catalog:port.singular',
  i18nNamespaces: ['common', 'catalog'],
})

// --- Entity Dialogs ---

const PortsDialogs = createEntityDialogs({
  useEntity: usePorts,
  MutateDialog: PortMutateDialog,
  DeleteDialog: PortDeleteDialog,
})

// --- Primary Buttons ---

const PortsPrimaryButtons = createPrimaryButtons({ useEntity: usePorts })

// --- Page ---

export function Ports() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogPortList()

  return (
    <EntityPage
      provider={PortsProvider}
      title={t('catalog:port.title')}
      queryResult={queryResult}
      primaryButtons={PortsPrimaryButtons}
      table={PortsTable}
      dialogs={PortsDialogs}
    />
  )
}
