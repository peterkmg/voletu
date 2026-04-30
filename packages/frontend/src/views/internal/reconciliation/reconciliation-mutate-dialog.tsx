import type { ReconciliationAdjustment, ReconciliationCreate } from './reconciliation-form-config'
import type { InventoryAdjustmentResponse, ReconciliationFlatRow } from '~/generated/types'
import type { InventoryReconciliationCompositeUpdateMutationRequest } from '~/generated/types/DocumentOperationsTypes/InventoryReconciliationCompositeUpdate'
import { useQueryClient } from '@tanstack/react-query'
import { useCallback, useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import {
  CompositeFormDialog,
  DocHeaderSection,
  DocItemsTable,
} from '~/components/composite-form'
import { useAdjustmentList } from '~/generated/hooks/DocumentOperationsHooks/useAdjustmentList'
import { useInventoryReconciliationCompositeCreate } from '~/generated/hooks/DocumentOperationsHooks/useInventoryReconciliationCompositeCreate'
import { useInventoryReconciliationCompositeUpdate } from '~/generated/hooks/DocumentOperationsHooks/useInventoryReconciliationCompositeUpdate'
import { useReconciliationGet } from '~/generated/hooks/DocumentOperationsHooks/useReconciliationGet'
import { flowReconciliationFlatQueryQueryKey } from '~/generated/hooks/FlowsHooks/useFlowReconciliationFlatQuery'
import {
  emptyReconciliationAdjustment,
  emptyReconciliationCreate,

  reconciliationAdjustmentColumns,
  reconciliationAdjustmentFields,
  reconciliationAdjustmentSchema,

  reconciliationCreateSchema,
  reconciliationHeaderSpec,
  reconciliationUpdateSchema,
} from './reconciliation-form-config'

interface ReconciliationMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void

  currentRow?: ReconciliationFlatRow | null
}

function toAdjustmentRequest(
  adjustment: InventoryAdjustmentResponse,
): ReconciliationAdjustment {
  return {
    storageId: adjustment.storageId,
    productId: adjustment.productId,
    adjustmentType: adjustment.adjustmentType,
    amount: adjustment.amount,
    reason: adjustment.reason ?? '',
  }
}

export function ReconciliationMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: ReconciliationMutateDialogProps) {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const createMutation = useInventoryReconciliationCompositeCreate()
  const updateMutation = useInventoryReconciliationCompositeUpdate()

  const isUpdate = currentRow != null
  const documentId = currentRow?.documentId ?? null

  const headerQuery = useReconciliationGet(documentId ?? '', undefined, {
    query: { enabled: Boolean(open && documentId) },
  })
  const loadedHeader = headerQuery.data?.data

  const adjustmentsQuery = useAdjustmentList({
    query: { enabled: Boolean(open && documentId) },
  })
  const loadedAdjustments = useMemo(() => {
    if (!documentId)
      return [] as InventoryAdjustmentResponse[]
    return (adjustmentsQuery.data?.data ?? []).filter(
      (row: InventoryAdjustmentResponse) => row.reconciliationId === documentId,
    )
  }, [adjustmentsQuery.data, documentId])

  const adjustmentsLoaded = !documentId || adjustmentsQuery.isSuccess

  const defaultValues = useMemo<ReconciliationCreate>(() => {
    if (!isUpdate || !loadedHeader || !adjustmentsLoaded)
      return emptyReconciliationCreate
    return {
      documentNumber: loadedHeader.documentNumber,
      date: loadedHeader.date,
      contractorId: loadedHeader.contractorId,
      warehouseId: loadedHeader.warehouseId,
      adjustments: loadedAdjustments.map(toAdjustmentRequest),
    }
  }, [isUpdate, loadedHeader, loadedAdjustments, adjustmentsLoaded])

  const mutationFn = useCallback(
    async (
      data: ReconciliationCreate,
    ): Promise<unknown> => {
      if (isUpdate && documentId) {
        return updateMutation.mutateAsync({
          id: documentId,
          data: data as unknown as InventoryReconciliationCompositeUpdateMutationRequest,
        })
      }
      return createMutation.mutateAsync({ data })
    },
    [isUpdate, documentId, createMutation, updateMutation],
  )

  const handleSuccess = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: flowReconciliationFlatQueryQueryKey() })
    toast.success(
      t(isUpdate ? 'reconciliation:toast.updated' : 'reconciliation:toast.created'),
    )
  }, [isUpdate, queryClient, t])

  const dialogKey = isUpdate
    ? loadedHeader && adjustmentsLoaded
      ? loadedHeader.id
      : 'edit-loading'
    : 'create'

  return (
    <CompositeFormDialog<ReconciliationCreate, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={isUpdate ? reconciliationUpdateSchema : reconciliationCreateSchema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={
        isUpdate
          ? 'reconciliation:dialog.title.edit'
          : 'reconciliation:dialog.title.create'
      }
    >
      <DocHeaderSection fields={reconciliationHeaderSpec} />
      <DocItemsTable<ReconciliationCreate, ReconciliationAdjustment>
        name="adjustments"
        columns={reconciliationAdjustmentColumns}
        rowSchema={reconciliationAdjustmentSchema}
        rowFields={reconciliationAdjustmentFields}
        emptyRow={emptyReconciliationAdjustment}
        sectionTitleKey="reconciliation:section.adjustments"
      />
    </CompositeFormDialog>
  )
}
