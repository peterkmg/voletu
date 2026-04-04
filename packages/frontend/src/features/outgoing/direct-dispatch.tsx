import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { DispatchResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, resolvedColumn, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { EntityPickerField } from '~/components/entity-picker'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { dispatchDocumentCreate, dispatchDocumentExecute, dispatchDocumentHardDelete, dispatchDocumentRevert, dispatchDocumentSoftDelete, dispatchDocumentUpdate } from '~/generated/client'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { dispatchDocumentQueryQueryKey, useDispatchDocumentQuery } from '~/generated/hooks/DocumentDispatchHooks/useDispatchDocumentQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<DispatchResponse, DialogType>('DirectDispatch')

const DataTableRowActions = createRowActions<DispatchResponse>({ useEntity, lifecycle: true })

function getColumns(t: TFunction): ColumnDef<DispatchResponse>[] {
  return [
    selectColumn<DispatchResponse>(),
    textColumn<DispatchResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<DispatchResponse>('date', t('common:table.date')),
    resolvedColumn<DispatchResponse>('contractorId', t('common:table.contractor'), 'contractorIdName'),
    statusColumn<DispatchResponse>('status', t('common:table.status'), documentStatusColors),
    actionsColumn<DispatchResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/outgoing/direct/')
const globalFilterFn = createGlobalFilter<DispatchResponse>('documentNumber')

function DirectDispatchTable({ data }: { data: DispatchResponse[] }) {
  return <EntityTable tableId="direct-dispatch" data={data} getColumns={getColumns} routeApi={route} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
}

const formSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
  contractorId: z.string().uuid(),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: DispatchResponse | null }) {
  const { t } = useTranslation(['common'])
  const companiesQuery = useCatalogCompanyList()

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open, onOpenChange, currentRow, schema: formSchema,
    defaultValues: { documentNumber: '', date: '', contractorId: '' },
    mapRowToForm: row => ({ documentNumber: row.documentNumber, date: row.date?.split('T')[0] ?? '', contractorId: row.contractorId }),
    transformPayload: v => ({ ...v, dispatchMethod: 'VESSEL_TERMINAL' as const, dispatchPurpose: 'EXTERNAL' as const }),
    createFn: dispatchDocumentCreate, updateFn: dispatchDocumentUpdate,
    queryKey: dispatchDocumentQueryQueryKey(), entityLabel: t('common:nav.directDispatch'), formId: 'direct-dispatch-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.directDispatch')} formId="direct-dispatch-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="direct-dispatch-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
          <EntityPickerField<FormValues> name="contractorId" label={t('common:table.contractor')} queryResult={companiesQuery} />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({ useEntity, hardDeleteFn: dispatchDocumentHardDelete, softDeleteFn: dispatchDocumentSoftDelete, queryKey: dispatchDocumentQueryQueryKey, entityLabel: 'common:nav.directDispatch', i18nNamespaces: ['common'] })

function DispatchLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: DispatchResponse | null, variant: 'execute' | 'revert' }) {
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={dispatchDocumentExecute} revertFn={dispatchDocumentRevert} queryKey={dispatchDocumentQueryQueryKey()} entityLabel="Dispatch Document" />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: DispatchLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity, createLabel: 'common:actions.create', i18nNamespaces: ['common'] })

export function DirectDispatchPage() {
  const { t } = useTranslation(['common'])
  const queryResult = useDispatchDocumentQuery({ dispatchMethod: 'VESSEL_TERMINAL' as any, dispatchPurpose: 'EXTERNAL' as any })

  return <EntityPage provider={Provider} title={t('common:nav.directDispatch')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={DirectDispatchTable} dialogs={Dialogs} />
}

export function DirectDispatchDetail() {
  return <div className="p-4">Direct Dispatch Detail — TODO</div>
}
