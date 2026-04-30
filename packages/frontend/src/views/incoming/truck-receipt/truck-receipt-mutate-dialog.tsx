import type { TruckReceiptCreate, TruckReceiptItem } from './truck-receipt-form-config'
import type { TruckReceiptPipelineResponse, TruckWaybillItemResponse } from '~/generated/types'
import type { TransportTruckWaybillCompositeCreateMutationResponse } from '~/generated/types/DocumentTransportTypes/TransportTruckWaybillCompositeCreate'
import type { TransportTruckWaybillCompositeUpdateMutationRequest } from '~/generated/types/DocumentTransportTypes/TransportTruckWaybillCompositeUpdate'
import { useQueryClient } from '@tanstack/react-query'
import { useCallback, useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import {
  CompositeFormDialog,
  DocHeaderSection,
  DocItemsTable,
} from '~/components/composite-form'
import { useTransportTruckWaybillCompositeCreate } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeCreate'
import { useTransportTruckWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet'
import { useTransportTruckWaybillCompositeUpdate } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeUpdate'
import { flowTruckReceiptQueryQueryKey } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import {
  emptyTruckReceiptCreate,
  emptyTruckReceiptItem,

  truckReceiptCreateSchema,
  truckReceiptHeaderSpec,

  truckReceiptItemColumns,
  truckReceiptItemFields,
  truckReceiptItemSchema,
  truckReceiptUpdateSchema,
} from './truck-receipt-form-config'

interface TruckReceiptMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void

  currentRow?: TruckReceiptPipelineResponse | null
}

function toItemRequest(item: TruckWaybillItemResponse): TruckReceiptItem {
  return {
    productId: item.productId,
    declaredAmount: item.declaredAmount,
  }
}

export function TruckReceiptMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: TruckReceiptMutateDialogProps) {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const createMutation = useTransportTruckWaybillCompositeCreate()
  const updateMutation = useTransportTruckWaybillCompositeUpdate()

  const isUpdate = currentRow != null
  const waybillId = currentRow?.id ?? null

  const composite = useTransportTruckWaybillCompositeGet(waybillId ?? '', undefined, {
    query: { enabled: Boolean(open && waybillId) },
  })
  const loaded = composite.data?.data

  const defaultValues = useMemo<TruckReceiptCreate>(() => {
    if (!isUpdate || !loaded)
      return emptyTruckReceiptCreate
    const wb = loaded.waybill
    return {
      documentNumber: wb.documentNumber,
      date: wb.date,
      senderId: wb.senderId,
      baseId: wb.baseId,
      items: (loaded.items ?? []).map(toItemRequest),
    }
  }, [isUpdate, loaded])

  const mutationFn = useCallback(
    async (
      data: TruckReceiptCreate,
    ): Promise<TransportTruckWaybillCompositeCreateMutationResponse> => {
      if (isUpdate && waybillId) {
        return updateMutation.mutateAsync({
          id: waybillId,
          data: data as unknown as TransportTruckWaybillCompositeUpdateMutationRequest,
        })
      }
      return createMutation.mutateAsync({ data })
    },
    [isUpdate, waybillId, createMutation, updateMutation],
  )

  const handleSuccess = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: flowTruckReceiptQueryQueryKey() })
    toast.success(
      t(isUpdate ? 'truck-receipt:toast.updated' : 'truck-receipt:toast.created'),
    )
  }, [isUpdate, queryClient, t])

  const dialogKey = isUpdate ? (loaded?.waybill.id ?? 'edit-loading') : 'create'

  return (
    <CompositeFormDialog<TruckReceiptCreate, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={isUpdate ? truckReceiptUpdateSchema : truckReceiptCreateSchema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={
        isUpdate ? 'truck-receipt:dialog.title.edit' : 'truck-receipt:dialog.title.create'
      }
    >
      <DocHeaderSection fields={truckReceiptHeaderSpec} />
      <DocItemsTable<TruckReceiptCreate, TruckReceiptItem>
        name="items"
        columns={truckReceiptItemColumns}
        rowSchema={truckReceiptItemSchema}
        rowFields={truckReceiptItemFields}
        emptyRow={emptyTruckReceiptItem}
        sectionTitleKey="truck-receipt:section.items"
      />
    </CompositeFormDialog>
  )
}
