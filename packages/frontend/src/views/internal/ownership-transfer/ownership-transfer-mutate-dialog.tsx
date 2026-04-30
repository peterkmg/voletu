import type { OwnershipTransferCreate, OwnershipTransferItem } from './ownership-transfer-form-config'
import type { OwnershipTransferFlatRow, OwnershipTransferItemResponse } from '~/generated/types'
import type { OwnershipTransferCompositeUpdateMutationRequest } from '~/generated/types/DocumentOperationsTypes/OwnershipTransferCompositeUpdate'
import { useQueryClient } from '@tanstack/react-query'
import { useCallback, useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import {
  CompositeFormDialog,
  DocHeaderSection,
  DocItemsTable,
} from '~/components/composite-form'
import { useOwnershipTransferCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferCompositeGet'
import { useOwnershipTransferCompositeUpdate } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferCompositeUpdate'
import { useOwnershipTransferCreate } from '~/generated/hooks/DocumentOperationsHooks/useOwnershipTransferCreate'
import { flowOwnershipTransferFlatQueryQueryKey } from '~/generated/hooks/FlowsHooks/useFlowOwnershipTransferFlatQuery'
import {
  emptyOwnershipTransferCreate,
  emptyOwnershipTransferItem,

  ownershipTransferCreateSchema,
  ownershipTransferHeaderSpec,

  ownershipTransferItemColumns,
  ownershipTransferItemFields,
  ownershipTransferItemSchema,
  ownershipTransferUpdateSchema,
} from './ownership-transfer-form-config'

interface OwnershipTransferMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void

  currentRow?: OwnershipTransferFlatRow | null
}

function toItemRequest(item: OwnershipTransferItemResponse): OwnershipTransferItem {
  return {
    storageId: item.storageId,
    productId: item.productId,
    fromContractorId: item.fromContractorId,
    toContractorId: item.toContractorId,
    amount: item.amount,
  }
}

export function OwnershipTransferMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: OwnershipTransferMutateDialogProps) {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const createMutation = useOwnershipTransferCreate()
  const updateMutation = useOwnershipTransferCompositeUpdate()

  const isUpdate = currentRow != null
  const documentId = currentRow?.documentId ?? null

  const composite = useOwnershipTransferCompositeGet(documentId ?? '', undefined, {
    query: { enabled: Boolean(open && documentId) },
  })
  const loaded = composite.data?.data

  const defaultValues = useMemo<OwnershipTransferCreate>(() => {
    if (!isUpdate || !loaded)
      return emptyOwnershipTransferCreate
    return {
      date: loaded.date,
      items: (loaded.items ?? []).map(toItemRequest),
    }
  }, [isUpdate, loaded])

  const mutationFn = useCallback(
    async (
      data: OwnershipTransferCreate,
    ): Promise<unknown> => {
      if (isUpdate && documentId) {
        return updateMutation.mutateAsync({
          id: documentId,
          data: data as unknown as OwnershipTransferCompositeUpdateMutationRequest,
        })
      }
      return createMutation.mutateAsync({ data })
    },
    [isUpdate, documentId, createMutation, updateMutation],
  )

  const handleSuccess = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: flowOwnershipTransferFlatQueryQueryKey() })
    toast.success(
      t(isUpdate ? 'ownership-transfer:toast.updated' : 'ownership-transfer:toast.created'),
    )
  }, [isUpdate, queryClient, t])

  const dialogKey = isUpdate ? (loaded?.id ?? 'edit-loading') : 'create'

  return (
    <CompositeFormDialog<OwnershipTransferCreate, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={isUpdate ? ownershipTransferUpdateSchema : ownershipTransferCreateSchema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={
        isUpdate
          ? 'ownership-transfer:dialog.title.edit'
          : 'ownership-transfer:dialog.title.create'
      }
    >
      <DocHeaderSection fields={ownershipTransferHeaderSpec} />
      <DocItemsTable<OwnershipTransferCreate, OwnershipTransferItem>
        name="items"
        columns={ownershipTransferItemColumns}
        rowSchema={ownershipTransferItemSchema}
        rowFields={ownershipTransferItemFields}
        emptyRow={emptyOwnershipTransferItem}
        sectionTitleKey="ownership-transfer:section.items"
      />
    </CompositeFormDialog>
  )
}
