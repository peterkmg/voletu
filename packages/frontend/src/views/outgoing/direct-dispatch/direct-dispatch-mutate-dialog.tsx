import type { ArrayPath } from 'react-hook-form'
import type { DirectDispatchCreate, DirectDispatchItem, DirectDispatchMeasurement } from './direct-dispatch-form-config'
import type { DispatchFlatRow } from '~/generated/types'
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
import {

  directDispatchCreateSchema,
  directDispatchHeaderSpec,

  directDispatchItemColumns,
  directDispatchItemFields,
  directDispatchItemSchema,

  directDispatchMeasurementColumns,
  directDispatchMeasurementFields,
  directDispatchMeasurementSchema,
  directDispatchUpdateSchema,
  emptyDirectDispatchCreate,
  emptyDirectDispatchItem,
  emptyDirectDispatchMeasurement,
} from './direct-dispatch-form-config'

interface DirectDispatchMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void

  currentRow?: DispatchFlatRow | null
  onCreated?: (id: string) => void
}

function toItemRequest(item: DispatchItemResponse): DirectDispatchItem {
  return {
    productId: item.productId,
    storageId: item.storageId,
    dispatchedAmount: item.dispatchedAmount,
  }
}

function toMeasurementRequest(m: DispatchMeasurementResponse): DirectDispatchMeasurement {
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

export function DirectDispatchMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: DirectDispatchMutateDialogProps) {
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

  const defaultValues = useMemo<DirectDispatchCreate>(() => {
    if (!isUpdate || !loaded)
      return emptyDirectDispatchCreate
    return {
      documentNumber: loaded.documentNumber,
      date: loaded.date,
      dispatchPurpose: 'EXTERNAL',
      dispatchMethod: 'VESSEL_TERMINAL',
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
    async (data: DirectDispatchCreate): Promise<unknown> => {
      const payload: DirectDispatchCreate = {
        ...data,
        dispatchMethod: 'VESSEL_TERMINAL',
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

  const handleSuccess = useCallback(
    (saved: unknown) => {
      queryClient.invalidateQueries({
        queryKey: flowDispatchFlatQueryQueryKey({
          dispatchMethod: 'VESSEL_TERMINAL',
          dispatchPurpose: 'EXTERNAL',
        }),
      })
      toast.success(
        t(isUpdate ? 'direct-dispatch:toast.updated' : 'direct-dispatch:toast.created'),
      )

      if (!isUpdate && onCreated) {
        const data = (saved as { data?: { document?: { id?: string } } } | null)?.data
        const newId = data?.document?.id
        if (newId)
          onCreated(newId)
      }
    },
    [isUpdate, queryClient, t, onCreated],
  )

  const dialogKey = isUpdate ? (loaded?.id ?? 'edit-loading') : 'create'

  return (
    <CompositeFormDialog<DirectDispatchCreate, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={isUpdate ? directDispatchUpdateSchema : directDispatchCreateSchema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={
        isUpdate
          ? 'direct-dispatch:dialog.title.edit'
          : 'direct-dispatch:dialog.title.create'
      }
    >
      <DocHeaderSection fields={directDispatchHeaderSpec} />
      <DocItemsTable<DirectDispatchCreate, DirectDispatchItem>
        name={'items' as ArrayPath<DirectDispatchCreate>}
        columns={directDispatchItemColumns}
        rowSchema={directDispatchItemSchema}
        rowFields={directDispatchItemFields}
        emptyRow={emptyDirectDispatchItem}
        sectionTitleKey="direct-dispatch:section.items"
      />
      <DocItemsTable<DirectDispatchCreate, DirectDispatchMeasurement>
        name={'storageMeasurements' as ArrayPath<DirectDispatchCreate>}
        columns={directDispatchMeasurementColumns}
        rowSchema={directDispatchMeasurementSchema}
        rowFields={directDispatchMeasurementFields}
        emptyRow={emptyDirectDispatchMeasurement}
        sectionTitleKey="direct-dispatch:section.storageMeasurements"
      />
    </CompositeFormDialog>
  )
}
