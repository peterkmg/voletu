import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { TruckWaybillResponse } from '~/generated/types'
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
import { transportTruckWaybillCreate, transportTruckWaybillHardDelete, transportTruckWaybillSoftDelete, transportTruckWaybillUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { transportTruckWaybillListQueryKey, useTransportTruckWaybillList } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type TruckWaybillsDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: TruckWaybillsProvider, useEntity: useTruckWaybills }
  = createEntityProvider<TruckWaybillResponse, TruckWaybillsDialogType>('TruckWaybills')

// --- Row actions ---

const DataTableRowActions = createRowActions<TruckWaybillResponse>({ useEntity: useTruckWaybills })

// --- Columns ---

function getTruckWaybillColumns(t: TFunction): ColumnDef<TruckWaybillResponse>[] {
  return [
    selectColumn<TruckWaybillResponse>(),
    textColumn<TruckWaybillResponse>('documentNumber', t('transport:truck.columns.waybillNumber')),
    dateColumn<TruckWaybillResponse>('date', t('transport:truck.columns.date')),
    resolvedColumn<TruckWaybillResponse>('senderId', t('transport:truck.columns.sender'), 'senderIdName'),
    dateColumn<TruckWaybillResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<TruckWaybillResponse>(DataTableRowActions),
  ]
}

// --- Table ---

const route = getRouteApi('/_authenticated/transport/truck-waybills/')
const globalFilterFn = createGlobalFilter<TruckWaybillResponse>('documentNumber')

function TruckWaybillsTable({ data }: { data: TruckWaybillResponse[] }) {
  return (
    <EntityTable
      tableId="truck-waybills"
      data={data}
      getColumns={getTruckWaybillColumns}
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

const truckWaybillFormSchema = z.object({
  documentNumber: z.string().min(1, 'Document number is required'),
  date: z.string().min(1, 'Date is required'),
  senderId: z.string().min(1, 'Sender ID is required'),
})

type TruckWaybillFormValues = z.infer<typeof truckWaybillFormSchema>

interface TruckWaybillMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: TruckWaybillResponse | null
}

function TruckWaybillMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: TruckWaybillMutateDialogProps) {
  const { t } = useTranslation(['transport', 'common'])

  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: truckWaybillFormSchema,
    defaultValues: {
      documentNumber: '',
      date: '',
      senderId: '',
    },
    mapRowToForm: (row: TruckWaybillResponse) => ({
      documentNumber: row.documentNumber,
      date: row.date,
      senderId: row.senderId,
    }),
    createFn: transportTruckWaybillCreate,
    updateFn: transportTruckWaybillUpdate,
    queryKey: transportTruckWaybillListQueryKey(),
    entityLabel: t('transport:truck.singular'),
    formId: 'truck-waybill-form',
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('transport:truck.edit') : t('transport:truck.create')}
      description={isUpdate ? t('transport:truck.edit') : t('transport:truck.create')}
      formId="truck-waybill-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="truck-waybill-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<TruckWaybillFormValues> name="documentNumber" label={t('transport:truck.form.documentNumber')} />
          <TextField<TruckWaybillFormValues> name="date" label={t('transport:truck.form.date')} type="date" />
          <EntityPickerField<TruckWaybillFormValues>
            name="senderId"
            label={t('transport:truck.form.senderId')}
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

const TruckWaybillDeleteDialog = createDeleteDialog({
  useEntity: useTruckWaybills,
  hardDeleteFn: transportTruckWaybillHardDelete,
  softDeleteFn: transportTruckWaybillSoftDelete,
  queryKey: transportTruckWaybillListQueryKey,
  entityLabel: 'transport:truck.singular',
  i18nNamespaces: ['common', 'transport'],
})

// --- Entity dialogs ---

const TruckWaybillsDialogs = createEntityDialogs({
  useEntity: useTruckWaybills,
  MutateDialog: TruckWaybillMutateDialog,
  DeleteDialog: TruckWaybillDeleteDialog,
})

// --- Primary buttons ---

const TruckWaybillsPrimaryButtons = createPrimaryButtons({
  useEntity: useTruckWaybills,
  createLabel: 'transport:truck.create',
  i18nNamespaces: ['transport'],
})

// --- Page component ---

export function TruckWaybills() {
  const { t } = useTranslation(['transport'])
  const queryResult = useTransportTruckWaybillList()

  return (
    <EntityPage
      provider={TruckWaybillsProvider}
      title={t('transport:truck.title')}
      queryResult={queryResult}
      primaryButtons={TruckWaybillsPrimaryButtons}
      table={TruckWaybillsTable}
      dialogs={TruckWaybillsDialogs}
    />
  )
}
