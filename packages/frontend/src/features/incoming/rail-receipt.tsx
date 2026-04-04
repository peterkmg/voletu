import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { RailReceiptPipelineResponse } from '~/generated/types'
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
import { transportRailWaybillCreate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { railReceiptPipelineQueryQueryKey, useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { pipelineStatusColors } from '~/lib/badge-colors'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'

type DialogType = 'create'

const { Provider, useEntity } = createEntityProvider<RailReceiptPipelineResponse, DialogType>('RailReceipt')

function DataTableRowActions({ row }: { row: { original: RailReceiptPipelineResponse } }) {
  const navigate = useNavigate()
  const { t } = useTranslation('common')
  const r = row.original
  const targetId = r.pipelineStatus === 'PENDING' ? r.id : (r.actionId ?? r.id)

  return (
    <DropdownMenuItem onClick={() => navigate({ to: `/incoming/rail/${targetId}` })}>
      <Eye className="mr-2 size-4" />
      {t('actions.viewDetails')}
    </DropdownMenuItem>
  )
}

function getColumns(t: TFunction): ColumnDef<RailReceiptPipelineResponse>[] {
  return [
    selectColumn<RailReceiptPipelineResponse>(),
    textColumn<RailReceiptPipelineResponse>('basisDocumentNumber', t('common:table.waybillNumber')),
    textColumn<RailReceiptPipelineResponse>('basisDate', t('common:table.date')),
    textColumn<RailReceiptPipelineResponse>('contractorName', t('common:table.contractor')),
    textColumn<RailReceiptPipelineResponse>('productName', t('common:table.product')),
    textColumn<RailReceiptPipelineResponse>('expectedQuantity', t('common:table.expectedQty')),
    statusColumn<RailReceiptPipelineResponse>('pipelineStatus', t('common:table.status'), pipelineStatusColors),
    textColumn<RailReceiptPipelineResponse>('actionDocumentNumber', t('common:table.acceptanceNumber')),
    textColumn<RailReceiptPipelineResponse>('actualQuantity', t('common:table.actualQty')),
    actionsColumn<RailReceiptPipelineResponse>(DataTableRowActions),
  ]
}

const routeApi = getRouteApi('/_authenticated/incoming/rail/')
const globalFilterFn = createGlobalFilter<RailReceiptPipelineResponse>('basisDocumentNumber', 'contractorName')

function RailReceiptTable({ data }: { data: RailReceiptPipelineResponse[] }) {
  return <EntityTable tableId="rail-receipt" data={data} getColumns={getColumns} routeApi={routeApi} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
}

const waybillSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  senderId: z.string().uuid(),
})

type WaybillFormValues = z.infer<typeof waybillSchema>

function WaybillMutateDialog({ open, onOpenChange }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: RailReceiptPipelineResponse | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, handleSubmit, handleOpenChange } = useMutateDialog({
    open, onOpenChange, schema: waybillSchema,
    defaultValues: { documentNumber: '', date: '', senderId: '' },
    createFn: transportRailWaybillCreate,
    queryKey: railReceiptPipelineQueryQueryKey(),
    entityLabel: t('common:nav.railReceipt'),
    formId: 'rail-waybill-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={t('common:actions.create')} description="Rail Waybill" formId="rail-waybill-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="rail-waybill-form" onSubmit={handleSubmit} className="space-y-5">
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

export function RailReceiptPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useRailReceiptPipelineQuery()

  return <EntityPage provider={Provider} title={t('common:nav.railReceipt')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={RailReceiptTable} dialogs={Dialogs} />
}

export function RailReceiptDetail() {
  return <div className="p-4">Rail Receipt Detail — TODO</div>
}
