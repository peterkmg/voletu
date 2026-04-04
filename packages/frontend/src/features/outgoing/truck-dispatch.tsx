import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { TruckDispatchPipelineResponse } from '~/generated/types'
import { getRouteApi, useNavigate } from '@tanstack/react-router'
import { Eye } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { Button } from '~/components/ui/button'
import { DropdownMenuItem } from '~/components/ui/dropdown-menu'
import { dispatchDocumentCreate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { truckDispatchPipelineQueryQueryKey, useTruckDispatchPipelineQuery } from '~/generated/hooks/FlowsHooks/useTruckDispatchPipelineQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { pipelineStatusColors } from '~/lib/badge-colors'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity } = createEntityProvider<TruckDispatchPipelineResponse, DialogType>('TruckDispatch')

function DataTableRowActions({ row }: { row: { original: TruckDispatchPipelineResponse } }) {
  const navigate = useNavigate()
  const { t } = useTranslation('common')

  return (
    <DropdownMenuItem onClick={() => navigate({ to: `/outgoing/truck/${row.original.id}` })}>
      <Eye className="mr-2 size-4" />
      {t('actions.viewDetails')}
    </DropdownMenuItem>
  )
}

function getColumns(t: TFunction): ColumnDef<TruckDispatchPipelineResponse>[] {
  return [
    selectColumn<TruckDispatchPipelineResponse>(),
    textColumn<TruckDispatchPipelineResponse>('documentNumber', t('common:table.documentNumber')),
    textColumn<TruckDispatchPipelineResponse>('date', t('common:table.date')),
    textColumn<TruckDispatchPipelineResponse>('contractorName', t('common:table.contractor')),
    textColumn<TruckDispatchPipelineResponse>('productName', t('common:table.product')),
    textColumn<TruckDispatchPipelineResponse>('dispatchedQuantity', t('common:table.quantity')),
    statusColumn<TruckDispatchPipelineResponse>('pipelineStatus', t('common:table.status'), pipelineStatusColors),
    actionsColumn<TruckDispatchPipelineResponse>(DataTableRowActions),
  ]
}

const routeApi = getRouteApi('/_authenticated/outgoing/truck/')
const globalFilterFn = createGlobalFilter<TruckDispatchPipelineResponse>('documentNumber', 'contractorName')

function TruckDispatchTable({ data }: { data: TruckDispatchPipelineResponse[] }) {
  return <EntityTable tableId="truck-dispatch" data={data} getColumns={getColumns} routeApi={routeApi} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
}

const dispatchSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  contractorId: z.string().uuid(),
})

type DispatchFormValues = z.infer<typeof dispatchSchema>

function DispatchMutateDialog({ open, onOpenChange }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: TruckDispatchPipelineResponse | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, handleSubmit, handleOpenChange } = useMutateDialog({
    open, onOpenChange, schema: dispatchSchema,
    defaultValues: { documentNumber: '', date: '', contractorId: '' },
    transformPayload: v => ({ ...v, dispatchMethod: 'TRUCK' as const, dispatchPurpose: 'EXTERNAL' as const }),
    createFn: dispatchDocumentCreate,
    queryKey: truckDispatchPipelineQueryQueryKey(),
    entityLabel: t('common:nav.truckDispatch'),
    formId: 'truck-dispatch-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={t('common:actions.create')} description="Truck Dispatch" formId="truck-dispatch-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="truck-dispatch-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<DispatchFormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<DispatchFormValues> name="date" label={t('common:table.date')} type="datetime-local" />
          <EntityPickerField<DispatchFormValues> name="contractorId" label={t('common:table.contractor')} queryResult={companiesQuery} />
        </form>
      </Form>
    </FormDialog>
  )
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog: DispatchMutateDialog })

function PrimaryButtons() {
  const { t } = useTranslation('common')
  const { setOpen, setCurrentRow } = useEntity()

  return (
    <Button size="sm" onClick={() => { setCurrentRow(null); setOpen('create') }}>
      {t('actions.create')} Dispatch
    </Button>
  )
}

export function TruckDispatchPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useTruckDispatchPipelineQuery()

  return <EntityPage provider={Provider} title={t('common:nav.truckDispatch')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={TruckDispatchTable} dialogs={Dialogs} />
}

export function TruckDispatchDetail() {
  return <div className="p-4">Truck Dispatch Detail — TODO</div>
}
