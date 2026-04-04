import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { PhysicalTransferResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { physicalDocumentCreate, physicalDocumentExecute, physicalDocumentHardDelete, physicalDocumentRevert, physicalDocumentSoftDelete, physicalDocumentUpdate } from '~/generated/client'
import { physicalTransferListQueryKey, usePhysicalTransferList } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<PhysicalTransferResponse, DialogType>('PhysicalTransfer')

const DataTableRowActions = createRowActions<PhysicalTransferResponse>({ useEntity, lifecycle: true })

function getColumns(t: TFunction): ColumnDef<PhysicalTransferResponse>[] {
  return [
    selectColumn<PhysicalTransferResponse>(),
    textColumn<PhysicalTransferResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<PhysicalTransferResponse>('date', t('common:table.date')),
    statusColumn<PhysicalTransferResponse>('status', t('common:table.status'), documentStatusColors),
    actionsColumn<PhysicalTransferResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/internal/physical-transfer/')
const globalFilterFn = createGlobalFilter<PhysicalTransferResponse>('documentNumber')

function PhysicalTransferTable({ data }: { data: PhysicalTransferResponse[] }) {
  return <EntityTable tableId="physical-transfer" data={data} getColumns={getColumns} routeApi={route} globalFilterFn={globalFilterFn} i18nNamespaces={['common']} />
}

const formSchema = z.object({
  documentNumber: z.string().min(1),
  date: z.string().min(1),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: PhysicalTransferResponse | null }) {
  const { t } = useTranslation(['common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open, onOpenChange, currentRow, schema: formSchema,
    defaultValues: { documentNumber: '', date: '' },
    mapRowToForm: row => ({ documentNumber: row.documentNumber, date: row.date?.split('T')[0] ?? '' }),
    createFn: physicalDocumentCreate, updateFn: physicalDocumentUpdate,
    queryKey: physicalTransferListQueryKey(), entityLabel: t('common:nav.physicalTransfer'), formId: 'physical-transfer-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.physicalTransfer')} formId="physical-transfer-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="physical-transfer-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="date" label={t('common:table.date')} type="datetime-local" />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({ useEntity, hardDeleteFn: physicalDocumentHardDelete, softDeleteFn: physicalDocumentSoftDelete, queryKey: physicalTransferListQueryKey, entityLabel: 'common:nav.physicalTransfer', i18nNamespaces: ['common'] })

function PhysicalLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: PhysicalTransferResponse | null, variant: 'execute' | 'revert' }) {
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={physicalDocumentExecute} revertFn={physicalDocumentRevert} queryKey={physicalTransferListQueryKey()} entityLabel="Physical Transfer" />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: PhysicalLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity, createLabel: 'common:actions.create', i18nNamespaces: ['common'] })

export function PhysicalTransferPage() {
  const { t } = useTranslation(['common'])
  const queryResult = usePhysicalTransferList()

  return <EntityPage provider={Provider} title={t('common:nav.physicalTransfer')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={PhysicalTransferTable} dialogs={Dialogs} />
}

export function PhysicalTransferDetail() {
  return <div className="p-4">Physical Transfer Detail — TODO</div>
}
