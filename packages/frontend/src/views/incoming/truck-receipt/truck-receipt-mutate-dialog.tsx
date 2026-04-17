/**
 * Per-document mutate dialog for Truck Receipt (truck waybill) - create + edit.
 *
 * Composes the shared `<CompositeFormDialog>` with the truck-receipt-specific
 * header spec / items table coming from `truck-receipt-form-config.tsx`, and
 * wires the Kubb-generated composite create and update mutations.
 *
 * Edit-mode `defaultValues` are pre-fetched via `useTransportTruckWaybillCompositeGet`
 * (gated on `open && isUpdate`). While the fetch is in flight the form
 * renders with `emptyTruckReceiptCreate`; once data arrives, the dialog is
 * re-mounted with the real values by keying `<CompositeFormDialog>` on the
 * loaded waybill id. This keeps the component stateless w.r.t. the fetch
 * and avoids a stale-form flash on open.
 */

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
  /**
   * Row currently selected in the pipeline list. When the row's
   * `pipelineStatus === 'PENDING'`, `id` identifies the truck waybill
   * (the basis document) and the dialog opens in edit mode. Otherwise
   * the row maps to an acceptance, which this dialog does not handle,
   * and the parent should not pass the row through. When `null`, the
   * dialog opens in create mode.
   */
  currentRow?: TruckReceiptPipelineResponse | null
}

/** Drop server-only fields and keep only the shape the composite request expects. */
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

  // Pre-fetch the full composite only when editing.
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
        // The update wire type accepts optional per-item `id`, which we
        // don't yet round-trip in the dialog (every row is treated as an
        // insert on submit). A follow-up will preserve the loaded item ids
        // so edits become true updates rather than delete+insert pairs.
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

  // `key` forces a fresh mount once the edit-mode fetch resolves so that
  // defaultValues are applied via react-hook-form's initialization path.
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
