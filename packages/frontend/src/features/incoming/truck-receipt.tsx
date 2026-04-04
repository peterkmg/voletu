import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { TruckReceiptPipelineResponse } from '~/generated/types'
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
import { transportTruckWaybillCreate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { flowTruckReceiptQueryQueryKey, useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { pipelineStatusColors } from '~/lib/badge-colors'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity } = createEntityProvider<TruckReceiptPipelineResponse, DialogType>('TruckReceipt')

// Row action: View Details only (navigate to detail page)
function DataTableRowActions({ row }: { row: { original: TruckReceiptPipelineResponse } }) {
  const navigate = useNavigate()
  const { t } = useTranslation('common')
  const r = row.original

  const targetId = r.pipelineStatus === 'PENDING' ? r.id : (r.actionId ?? r.id)

  return (
    <DropdownMenuItem onClick={() => navigate({ to: `/incoming/truck/${targetId}` })}>
      <Eye className="mr-2 size-4" />
      {t('actions.viewDetails')}
    </DropdownMenuItem>
  )
}

function getColumns(t: TFunction): ColumnDef<TruckReceiptPipelineResponse>[] {
  return [
    selectColumn<TruckReceiptPipelineResponse>(),
    textColumn<TruckReceiptPipelineResponse>('basisDocumentNumber', t('common:table.waybillNumber')),
    textColumn<TruckReceiptPipelineResponse>('basisDate', t('common:table.date')),
    textColumn<TruckReceiptPipelineResponse>('contractorName', t('common:table.contractor')),
    textColumn<TruckReceiptPipelineResponse>('productName', t('common:table.product')),
    textColumn<TruckReceiptPipelineResponse>('expectedQuantity', t('common:table.expectedQty')),
    statusColumn<TruckReceiptPipelineResponse>('pipelineStatus', t('common:table.status'), pipelineStatusColors),
    textColumn<TruckReceiptPipelineResponse>('actionDocumentNumber', t('common:table.acceptanceNumber')),
    textColumn<TruckReceiptPipelineResponse>('actualQuantity', t('common:table.actualQty')),
    actionsColumn<TruckReceiptPipelineResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/incoming/truck/')
const globalFilterFn = createGlobalFilter<TruckReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

function TruckReceiptTable({ data }: { data: TruckReceiptPipelineResponse[] }) {
  return (
    <EntityTable
      tableId="truck-receipt"
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
    />
  )
}

// Waybill creation dialog
const waybillSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  senderId: z.string().uuid(),
})

type WaybillFormValues = z.infer<typeof waybillSchema>

function WaybillMutateDialog({ open, onOpenChange }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: TruckReceiptPipelineResponse | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, handleSubmit, handleOpenChange } = useMutateDialog({
    open, onOpenChange, schema: waybillSchema,
    defaultValues: { documentNumber: '', date: '', senderId: '' },
    createFn: transportTruckWaybillCreate,
    queryKey: flowTruckReceiptQueryQueryKey(),
    entityLabel: t('common:nav.truckReceipt'),
    formId: 'truck-waybill-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={t('common:actions.create')} description="Truck Waybill" formId="truck-waybill-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="truck-waybill-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<WaybillFormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<WaybillFormValues> name="date" label={t('common:table.date')} type="date" />
          <EntityPickerField<WaybillFormValues> name="senderId" label={t('common:table.contractor')} queryResult={companiesQuery} />
        </form>
      </Form>
    </FormDialog>
  )
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog: WaybillMutateDialog })

function PrimaryButtons() {
  const { t } = useTranslation('common')
  const { setOpen, setCurrentRow } = useEntity()

  return (
    <Button size="sm" onClick={() => { setCurrentRow(null); setOpen('create') }}>
      {t('actions.create')} Waybill
    </Button>
  )
}

export function TruckReceiptPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useFlowTruckReceiptQuery()

  return (
    <EntityPage
      provider={Provider}
      title={t('common:nav.truckReceipt')}
      queryResult={queryResult}
      primaryButtons={PrimaryButtons}
      table={TruckReceiptTable}
      dialogs={Dialogs}
    />
  )
}

export function TruckReceiptDetail() {
  return <div className="p-4">Truck Receipt Detail — TODO</div>
}
