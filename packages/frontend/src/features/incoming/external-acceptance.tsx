import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { AcceptanceItemResponse, AcceptanceResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, selectColumn, statusColumn, textColumn } from '~/components/data-table'
import { DocumentDetailPage } from '~/components/document'
import { ChildItemsTable } from '~/components/document/child-items-table'
import { Skeleton } from '~/components/ui/skeleton'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { acceptanceDocumentCreate, acceptanceDocumentExecute, acceptanceDocumentHardDelete, acceptanceDocumentRevert, acceptanceDocumentSoftDelete, acceptanceDocumentUpdate } from '~/generated/client'
import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
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
    { ...dateColumn<AcceptanceResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), requiresRole: 'senior_supervisor' } },
    actionsColumn<AcceptanceResponse>(DataTableRowActions),
  ]
}

const route = getRouteApi('/_authenticated/incoming/external/')
const detailRoute = getRouteApi('/_authenticated/incoming/external/$id')
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
  const { id } = detailRoute.useParams()
  const { t } = useTranslation(['common'])
  const { data, isLoading } = useAcceptanceCompositeGet(id)

  if (isLoading || !data?.data) return <div className="p-4"><Skeleton className="h-64 w-full" /></div>

  const doc = data.data

  return (
    <DocumentDetailPage
      config={{ title: t('common:nav.externalAcceptance'), entityLabel: 'Acceptance', backTo: '/incoming/external', executeFn: acceptanceDocumentExecute, revertFn: acceptanceDocumentRevert, queryKey: acceptanceDocumentQueryQueryKey(), statusColorMap: documentStatusColors }}
      document={{ id: doc.id, documentNumber: doc.documentNumber, status: doc.status }}
      formContent={
        <div className="grid grid-cols-3 gap-4">
          <div><span className="text-sm text-muted-foreground">{t('common:table.date')}</span><p>{doc.dateAccepted}</p></div>
          <div><span className="text-sm text-muted-foreground">{t('common:table.source')}</span><p>{doc.sourceEntity ?? '—'}</p></div>
        </div>
      }
      itemsContent={
        <ChildItemsTable
          items={doc.items}
          columns={[
            textColumn<AcceptanceItemResponse>('productIdName', t('common:table.product')),
            textColumn<AcceptanceItemResponse>('storageIdName', 'Storage'),
            textColumn<AcceptanceItemResponse>('contractorIdName', t('common:table.contractor')),
            textColumn<AcceptanceItemResponse>('acceptedAmount', t('common:table.quantity')),
          ]}
          isLocked={doc.status === 'POSTED'}
          sectionTitle="Acceptance Items"
        />
      }
      metadataContent={doc.executedAt ? <div className="text-sm"><span className="text-muted-foreground">Executed at:</span> {doc.executedAt}</div> : null}
    />
  )
}
