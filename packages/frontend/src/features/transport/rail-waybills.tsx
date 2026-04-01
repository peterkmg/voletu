import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { RailWaybillResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { Archive } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, selectColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { CompanyMutateDialog } from '~/features/catalog/companies'
import { transportRailWaybillCreate, transportRailWaybillHardDelete, transportRailWaybillSoftDelete, transportRailWaybillUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { transportRailWaybillListQueryKey, useTransportRailWaybillList } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type RailWaybillsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: RailWaybillsProvider, useEntity: useRailWaybills }
  = createEntityProvider<RailWaybillResponse, RailWaybillsDialogType>('RailWaybills')

// --- Row actions ---

const DataTableRowActions = createRowActions<RailWaybillResponse>({ useEntity: useRailWaybills })

// --- Columns ---

function getRailWaybillColumns(t: TFunction): ColumnDef<RailWaybillResponse>[] {
  return [
    selectColumn<RailWaybillResponse>(),
    textColumn<RailWaybillResponse>('documentNumber', t('transport:rail.columns.waybillNumber')),
    dateColumn<RailWaybillResponse>('date', t('transport:rail.columns.date')),
    resolvedColumn<RailWaybillResponse>('senderId', t('transport:rail.columns.sender'), 'senderIdName'),
    dateColumn<RailWaybillResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<RailWaybillResponse>(DataTableRowActions),
  ]
}

// --- Table ---

const route = getRouteApi('/_authenticated/transport/rail-waybills/')
const globalFilterFn = createGlobalFilter<RailWaybillResponse>('documentNumber')

function RailWaybillsTable({ data }: { data: RailWaybillResponse[] }) {
  return (
    <EntityTable
      tableId="rail-waybills"
      data={data}
      getColumns={getRailWaybillColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['transport', 'common']}
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

// --- Mutate dialog ---

const railWaybillFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  senderId: z.string().min(1, 'Sender ID is required'),
})

type RailWaybillFormValues = z.infer<typeof railWaybillFormSchema>

interface RailWaybillMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: RailWaybillResponse | null
}

function RailWaybillMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: RailWaybillMutateDialogProps) {
  const { t } = useTranslation(['transport', 'common'])

  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: railWaybillFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      senderId: '',
    },
    mapRowToForm: (row: RailWaybillResponse) => ({
      documentNumber: row.documentNumber,
      date: row.date,
      senderId: row.senderId,
    }),
    createFn: transportRailWaybillCreate,
    updateFn: transportRailWaybillUpdate,
    queryKey: transportRailWaybillListQueryKey(),
    entityLabel: t('transport:rail.singular'),
    formId: 'rail-waybill-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('transport:rail.edit') : t('transport:rail.create')}
      description={isUpdate ? t('transport:rail.edit') : t('transport:rail.create')}
      formId="rail-waybill-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="rail-waybill-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<RailWaybillFormValues> name="documentNumber" label={t('transport:rail.form.documentNumber')} />
          <TextField<RailWaybillFormValues> name="date" label={t('transport:rail.form.date')} type="date" />
          <EntityPickerField<RailWaybillFormValues>
            name="senderId"
            label={t('transport:rail.form.senderId')}
            queryResult={companiesQuery}
            displayField="commonName"
            allowCreate
            createDialog={CompanyMutateDialog}
          />
        </form>
      </Form>
    </FormDialog>
  )
}

// --- Delete dialog ---

const RailWaybillDeleteDialog = createDeleteDialog({
  useEntity: useRailWaybills,
  hardDeleteFn: transportRailWaybillHardDelete,
  softDeleteFn: transportRailWaybillSoftDelete,
  queryKey: transportRailWaybillListQueryKey,
  entityLabel: 'transport:rail.singular',
  i18nNamespaces: ['common', 'transport'],
})

// --- Entity dialogs ---

const RailWaybillsDialogs = createEntityDialogs({
  useEntity: useRailWaybills,
  MutateDialog: RailWaybillMutateDialog,
  DeleteDialog: RailWaybillDeleteDialog,
})

// --- Primary buttons ---

const RailWaybillsPrimaryButtons = createPrimaryButtons({
  useEntity: useRailWaybills,
  createLabel: 'transport:rail.create',
  i18nNamespaces: ['transport'],
})

// --- Page component ---

export function RailWaybills() {
  const { t } = useTranslation(['transport'])
  const queryResult = useTransportRailWaybillList()

  return (
    <EntityPage
      provider={RailWaybillsProvider}
      title={t('transport:rail.title')}
      queryResult={queryResult}
      primaryButtons={RailWaybillsPrimaryButtons}
      table={RailWaybillsTable}
      dialogs={RailWaybillsDialogs}
    />
  )
}
