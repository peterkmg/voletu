import type { ArrayPath } from 'react-hook-form'
import type { TruckDispatchCreate, TruckDispatchItem, TruckDispatchMeasurement } from './truck-dispatch-form-config'
import type { DispatchItemResponse } from '~/generated/types/DispatchItemResponse'
import type { DispatchMeasurementResponse } from '~/generated/types/DispatchMeasurementResponse'
import type { DispatchCompositeUpdateMutationRequest } from '~/generated/types/DocumentDispatchTypes/DispatchCompositeUpdate'
import { useQueryClient } from '@tanstack/react-query'
import { useCallback, useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import {
  CompositeFormDialog,
  DocHeaderSection,
  DocItemsTable,
} from '~/components/composite-form'
import { useDispatchCompositeCreate } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeCreate'
import { useDispatchCompositeGet } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeGet'
import { useDispatchCompositeUpdate } from '~/generated/hooks/DocumentDispatchHooks/useDispatchCompositeUpdate'
import { flowDispatchFlatQueryQueryKey } from '~/generated/hooks/FlowsHooks/useFlowDispatchFlatQuery'
import { truckDispatchPipelineQueryQueryKey } from '~/generated/hooks/FlowsHooks/useTruckDispatchPipelineQuery'
import {
  emptyTruckDispatchCreate,
  emptyTruckDispatchItem,
  emptyTruckDispatchMeasurement,

  truckDispatchCreateSchema,
  truckDispatchHeaderSpec,

  truckDispatchItemColumns,
  truckDispatchItemFields,
  truckDispatchItemSchema,

  truckDispatchMeasurementColumns,
  truckDispatchMeasurementFields,
  truckDispatchMeasurementSchema,
  truckDispatchUpdateSchema,
} from './truck-dispatch-form-config'

export interface TruckDispatchMutationTarget {
  documentId: string
}

interface TruckDispatchMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void

  currentRow?: TruckDispatchMutationTarget | null
}

function toItemRequest(item: DispatchItemResponse): TruckDispatchItem {
  return {
    productId: item.productId,
    storageId: item.storageId,
    dispatchedAmount: item.dispatchedAmount,
  }
}

function toMeasurementRequest(m: DispatchMeasurementResponse): TruckDispatchMeasurement {
  return {
    storageId: m.storageId,
    beforeHeight: m.beforeHeight ?? null,
    beforeVolume: m.beforeVolume ?? null,
    beforeDensity: m.beforeDensity ?? null,
    beforeMass: m.beforeMass,
    afterHeight: m.afterHeight ?? null,
    afterVolume: m.afterVolume ?? null,
    afterDensity: m.afterDensity ?? null,
    afterMass: m.afterMass,
  }
}

export function TruckDispatchMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: TruckDispatchMutateDialogProps) {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const createMutation = useDispatchCompositeCreate()
  const updateMutation = useDispatchCompositeUpdate()

  const isUpdate = currentRow != null
  const documentId = currentRow?.documentId ?? null

  const composite = useDispatchCompositeGet(documentId ?? '', undefined, {
    query: { enabled: Boolean(open && documentId) },
  })
  const loaded = composite.data?.data

  const defaultValues = useMemo<TruckDispatchCreate>(() => {
    if (!isUpdate || !loaded)
      return emptyTruckDispatchCreate
    return {
      documentNumber: loaded.documentNumber,
      date: loaded.date,
      dispatchPurpose: 'EXTERNAL',
      dispatchMethod: 'TRUCK',
      contractorId: loaded.contractorId,
      destinationBaseId: loaded.destinationBaseId ?? null,
      receiverEntity: loaded.receiverEntity ?? null,
      startCargoOps: loaded.startCargoOps ?? null,
      endCargoOps: loaded.endCargoOps ?? null,
      bunkerType: loaded.bunkerType ?? null,
      exporterId: loaded.exporterId ?? null,
      portId: loaded.portId ?? null,
      items: (loaded.items ?? []).map(toItemRequest),
      storageMeasurements: (loaded.storageMeasurements ?? []).map(toMeasurementRequest),
    }
  }, [isUpdate, loaded])

  const mutationFn = useCallback(
    async (data: TruckDispatchCreate): Promise<unknown> => {
      const payload: TruckDispatchCreate = {
        ...data,
        dispatchMethod: 'TRUCK',
        dispatchPurpose: 'EXTERNAL',
      }
      if (isUpdate && documentId) {
        return updateMutation.mutateAsync({
          id: documentId,
          data: payload as unknown as DispatchCompositeUpdateMutationRequest,
        })
      }
      return createMutation.mutateAsync({ data: payload })
    },
    [isUpdate, documentId, createMutation, updateMutation],
  )

  const handleSuccess = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: truckDispatchPipelineQueryQueryKey() })
    queryClient.invalidateQueries({
      queryKey: flowDispatchFlatQueryQueryKey({
        dispatchMethod: 'TRUCK',
        dispatchPurpose: 'EXTERNAL',
      }),
    })
    toast.success(
      t(isUpdate ? 'truck-dispatch:toast.updated' : 'truck-dispatch:toast.created'),
    )
  }, [isUpdate, queryClient, t])

  const dialogKey = isUpdate ? (loaded?.id ?? 'edit-loading') : 'create'

  return (
    <CompositeFormDialog<TruckDispatchCreate, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={isUpdate ? truckDispatchUpdateSchema : truckDispatchCreateSchema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={
        isUpdate
          ? 'truck-dispatch:dialog.title.edit'
          : 'truck-dispatch:dialog.title.create'
      }
    >
      <DocHeaderSection fields={truckDispatchHeaderSpec} />
      <DocItemsTable<TruckDispatchCreate, TruckDispatchItem>
        name={'items' as ArrayPath<TruckDispatchCreate>}
        columns={truckDispatchItemColumns}
        rowSchema={truckDispatchItemSchema}
        rowFields={truckDispatchItemFields}
        emptyRow={emptyTruckDispatchItem}
        sectionTitleKey="truck-dispatch:section.items"
      />
      <DocItemsTable<TruckDispatchCreate, TruckDispatchMeasurement>
        name={'storageMeasurements' as ArrayPath<TruckDispatchCreate>}
        columns={truckDispatchMeasurementColumns}
        rowSchema={truckDispatchMeasurementSchema}
        rowFields={truckDispatchMeasurementFields}
        emptyRow={emptyTruckDispatchMeasurement}
        sectionTitleKey="truck-dispatch:section.storageMeasurements"
      />
    </CompositeFormDialog>
  )
}
