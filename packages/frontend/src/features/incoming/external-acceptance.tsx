import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { AcceptanceResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { acceptanceDocumentCreate, acceptanceDocumentExecute, acceptanceDocumentHardDelete, acceptanceDocumentRevert, acceptanceDocumentSoftDelete, acceptanceDocumentUpdate } from '~/generated/client'
import { acceptanceDocumentQueryQueryKey, useAcceptanceDocumentQuery } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceDocumentQuery'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { documentStatusColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

type DialogType = 'create' | 'update' | 'delete' | 'hard-delete' | 'execute' | 'revert'

const { Provider, useEntity } = createEntityProvider<AcceptanceResponse, DialogType>('ExternalAcceptance')

const DataTableRowActions = createRowActions<AcceptanceResponse>({ useEntity, lifecycle: true })

function getColumns(t: TFunction): ColumnDef<AcceptanceResponse>[] {
  return [
    selectColumn<AcceptanceResponse>(),
    textColumn<AcceptanceResponse>('documentNumber', t('common:table.documentNumber')),
    dateColumn<AcceptanceResponse>('dateAccepted', t('common:table.date')),
    textColumn<AcceptanceResponse>('sourceEntity', t('common:table.source')),
    statusColumn<AcceptanceResponse>('status', t('common:table.status'), documentStatusColors),
    actionsColumn<AcceptanceResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/incoming/external/')
const globalFilterFn = createGlobalFilter<AcceptanceResponse>('documentNumber')

function ExternalAcceptanceTable({ data }: { data: AcceptanceResponse[] }) {
  return (
    <EntityTable
      tableId="external-acceptance"
      data={data}
      getColumns={getColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['common']}
    />
  )
}

const formSchema = z.object({
  documentNumber: z.string().min(1),
  dateAccepted: z.string().min(1),
  sourceEntity: z.string().nullable().optional(),
})

type FormValues = z.infer<typeof formSchema>

function MutateDialog({ open, onOpenChange, currentRow }: { open: boolean, onOpenChange: (o: boolean) => void, currentRow?: AcceptanceResponse | null }) {
  const { t } = useTranslation(['common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: formSchema,
    defaultValues: { documentNumber: '', dateAccepted: '', sourceEntity: '' },
    mapRowToForm: row => ({
      documentNumber: row.documentNumber,
      dateAccepted: row.dateAccepted?.split('T')[0] ?? '',
      sourceEntity: row.sourceEntity ?? '',
    }),
    transformPayload: v => ({ ...v, arrivalType: 'EXTERNAL' as const, sourceEntity: v.sourceEntity || null }),
    createFn: acceptanceDocumentCreate,
    updateFn: acceptanceDocumentUpdate,
    queryKey: acceptanceDocumentQueryQueryKey(),
    entityLabel: t('common:nav.externalAcceptance'),
    formId: 'external-acceptance-form',
  })

  return (
    <FormDialog open={open} onOpenChange={handleOpenChange} title={isUpdate ? t('common:actions.edit') : t('common:actions.create')} description={t('common:nav.externalAcceptance')} formId="external-acceptance-form" isSubmitting={form.formState.isSubmitting}>
      <Form {...form}>
        <form id="external-acceptance-form" onSubmit={handleSubmit} className="space-y-5">
          <TextField<FormValues> name="documentNumber" label={t('common:table.documentNumber')} />
          <TextField<FormValues> name="dateAccepted" label={t('common:table.date')} type="datetime-local" />
          <TextField<FormValues> name="sourceEntity" label={t('common:table.source')} nullable />
        </form>
      </Form>
    </FormDialog>
  )
}

const DeleteDialog = createDeleteDialog({
  useEntity,
  hardDeleteFn: acceptanceDocumentHardDelete,
  softDeleteFn: acceptanceDocumentSoftDelete,
  queryKey: acceptanceDocumentQueryQueryKey,
  entityLabel: 'common:nav.externalAcceptance',
  i18nNamespaces: ['common'],
})

function AcceptanceLifecycleDialog({ open, onOpenChange, currentRow, variant }: { open: boolean, onOpenChange: () => void, currentRow: AcceptanceResponse | null, variant: 'execute' | 'revert' }) {
  return <LifecycleDialog open={open} onOpenChange={onOpenChange} currentRow={currentRow} action={variant} executeFn={acceptanceDocumentExecute} revertFn={acceptanceDocumentRevert} queryKey={acceptanceDocumentQueryQueryKey()} entityLabel="Acceptance Document" />
}

const Dialogs = createEntityDialogs({ useEntity, MutateDialog, DeleteDialog, LifecycleDialog: AcceptanceLifecycleDialog, lifecyclePropName: 'variant' })
const PrimaryButtons = createPrimaryButtons({ useEntity, createLabel: 'common:actions.create', i18nNamespaces: ['common'] })

export function ExternalAcceptancePage() {
  const { t } = useTranslation(['common'])
  const queryResult = useAcceptanceDocumentQuery({ truckWaybillId: 'isNull' as any, railWaybillId: 'isNull' as any, transitDispatchId: 'isNull' as any })

  return <EntityPage provider={Provider} title={t('common:nav.externalAcceptance')} queryResult={queryResult} primaryButtons={PrimaryButtons} table={ExternalAcceptanceTable} dialogs={Dialogs} />
}

export function ExternalAcceptanceDetail() {
  return <div className="p-4">External Acceptance Detail — TODO</div>
}
