import type { PhysicalTransferCreate, PhysicalTransferItem } from './physical-transfer-form-config'
import type { PhysicalTransferFlatRow, PhysicalTransferItemResponse } from '~/generated/types'
import type { PhysicalTransferCompositeUpdateMutationRequest } from '~/generated/types/DocumentOperationsTypes/PhysicalTransferCompositeUpdate'
import { useQueryClient } from '@tanstack/react-query'
import { useCallback, useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import {
  CompositeFormDialog,
  DocHeaderSection,
  DocItemsTable,
} from '~/components/composite-form'
import { usePhysicalTransferCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferCompositeGet'
import { usePhysicalTransferCompositeUpdate } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferCompositeUpdate'
import { usePhysicalTransferCreate } from '~/generated/hooks/DocumentOperationsHooks/usePhysicalTransferCreate'
import { flowPhysicalTransferFlatQueryQueryKey } from '~/generated/hooks/FlowsHooks/useFlowPhysicalTransferFlatQuery'
import {
  emptyPhysicalTransferCreate,
  emptyPhysicalTransferItem,

  physicalTransferCreateSchema,
  physicalTransferHeaderSpec,

  physicalTransferItemColumns,
  physicalTransferItemFields,
  physicalTransferItemSchema,
  physicalTransferUpdateSchema,
} from './physical-transfer-form-config'

interface PhysicalTransferMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void

  currentRow?: PhysicalTransferFlatRow | null
}

function toItemRequest(item: PhysicalTransferItemResponse): PhysicalTransferItem {
  return {
    productId: item.productId,
    fromStorageId: item.fromStorageId,
    toStorageId: item.toStorageId,
    amount: item.amount,
  }
}

export function PhysicalTransferMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: PhysicalTransferMutateDialogProps) {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const createMutation = usePhysicalTransferCreate()
  const updateMutation = usePhysicalTransferCompositeUpdate()

  const isUpdate = currentRow != null
  const documentId = currentRow?.documentId ?? null

  const composite = usePhysicalTransferCompositeGet(documentId ?? '', undefined, {
    query: { enabled: Boolean(open && documentId) },
  })
  const loaded = composite.data?.data

  const defaultValues = useMemo<PhysicalTransferCreate>(() => {
    if (!isUpdate || !loaded)
      return emptyPhysicalTransferCreate
    return {
      documentNumber: loaded.documentNumber,
      date: loaded.date,
      contractorId: loaded.contractorId,
      startCargoOps: loaded.startCargoOps,
      endCargoOps: loaded.endCargoOps,
      items: (loaded.items ?? []).map(toItemRequest),
    }
  }, [isUpdate, loaded])

  const mutationFn = useCallback(
    async (
      data: PhysicalTransferCreate,
    ): Promise<unknown> => {
      if (isUpdate && documentId) {
        return updateMutation.mutateAsync({
          id: documentId,
          data: data as unknown as PhysicalTransferCompositeUpdateMutationRequest,
        })
      }
      return createMutation.mutateAsync({ data })
    },
    [isUpdate, documentId, createMutation, updateMutation],
  )

  const handleSuccess = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: flowPhysicalTransferFlatQueryQueryKey() })
    toast.success(
      t(isUpdate ? 'physical-transfer:toast.updated' : 'physical-transfer:toast.created'),
    )
  }, [isUpdate, queryClient, t])

  const dialogKey = isUpdate ? (loaded?.id ?? 'edit-loading') : 'create'

  return (
    <CompositeFormDialog<PhysicalTransferCreate, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={isUpdate ? physicalTransferUpdateSchema : physicalTransferCreateSchema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={
        isUpdate
          ? 'physical-transfer:dialog.title.edit'
          : 'physical-transfer:dialog.title.create'
      }
    >
      <DocHeaderSection fields={physicalTransferHeaderSpec} />
      <DocItemsTable<PhysicalTransferCreate, PhysicalTransferItem>
        name="items"
        columns={physicalTransferItemColumns}
        rowSchema={physicalTransferItemSchema}
        rowFields={physicalTransferItemFields}
        emptyRow={emptyPhysicalTransferItem}
        sectionTitleKey="physical-transfer:section.items"
      />
    </CompositeFormDialog>
  )
}
