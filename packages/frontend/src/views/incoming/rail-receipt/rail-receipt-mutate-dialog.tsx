import type { ArrayPath } from 'react-hook-form'
import type { RailReceiptForm, RailReceiptManifest, RailReceiptMeasurement, RailReceiptWeight } from './rail-receipt-form-config'
import type { RailReceiptPipelineResponse } from '~/generated/types'
import type { TransportRailWaybillCompositeCreateMutationResponse } from '~/generated/types/DocumentTransportTypes/TransportRailWaybillCompositeCreate'
import type { TransportRailWaybillCompositeUpdateMutationRequest } from '~/generated/types/DocumentTransportTypes/TransportRailWaybillCompositeUpdate'
import type { RailWagonManifestResponse } from '~/generated/types/RailWagonManifestResponse'
import type { RailWagonMeasurementResponse } from '~/generated/types/RailWagonMeasurementResponse'
import type { RailWagonWeightResponse } from '~/generated/types/RailWagonWeightResponse'
import { useQueryClient } from '@tanstack/react-query'
import { useCallback, useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import {
  CompositeFormDialog,
  DocHeaderSection,
  DocItemsTable,
} from '~/components/composite-form'
import { useTransportRailWaybillCompositeCreate } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeCreate'
import { useTransportRailWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet'
import { useTransportRailWaybillCompositeUpdate } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeUpdate'
import { railReceiptPipelineQueryQueryKey } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import {
  emptyRailReceiptForm,
  emptyRailReceiptManifest,
  emptyRailReceiptMeasurement,
  emptyRailReceiptWeight,

  railReceiptFormSchema,
  railReceiptHeaderSpec,

  railReceiptManifestColumns,
  railReceiptManifestFields,
  railReceiptManifestSchema,

  railReceiptMeasurementColumns,
  railReceiptMeasurementFields,
  railReceiptMeasurementSchema,

  railReceiptWeightColumns,
  railReceiptWeightFields,
  railReceiptWeightSchema,
} from './rail-receipt-form-config'

interface RailReceiptMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void

  currentRow?: RailReceiptPipelineResponse | null
}

function toMeasurementRow(measurement: RailWagonMeasurementResponse): RailReceiptMeasurement {
  return {
    id: measurement.id,
    measuredHeight: measurement.measuredHeight,
    labDensity: measurement.labDensity ?? null,
    calculatedMass: measurement.calculatedMass,
  }
}

function toWeightRow(weight: RailWagonWeightResponse): RailReceiptWeight {
  return {
    id: weight.id,
    grossWeight: weight.grossWeight,
    tareWeight: weight.tareWeight,
    netProductWeight: weight.netProductWeight,
  }
}

function toManifestRow(manifest: RailWagonManifestResponse): RailReceiptManifest {
  return {
    id: manifest.id,
    wagonNumber: manifest.wagonNumber,
    productId: manifest.productId,
    declaredVolume: manifest.declaredVolume,
    declaredDensity: manifest.declaredDensity,
    declaredMass: manifest.declaredMass,
    measurements: (manifest.measurements ?? []).map(toMeasurementRow),
    weights: (manifest.weights ?? []).map(toWeightRow),
  }
}

function toCreatePayload(data: RailReceiptForm) {
  return {
    documentNumber: data.documentNumber ?? '',
    date: data.date ?? '',
    senderId: data.senderId ?? '',
    baseId: data.baseId ?? '',
    manifests: data.manifests.map(manifest => ({
      wagonNumber: manifest.wagonNumber,
      productId: manifest.productId,
      declaredVolume: manifest.declaredVolume,
      declaredDensity: manifest.declaredDensity,
      declaredMass: manifest.declaredMass,
      measurements: manifest.measurements.map(measurement => ({
        wagonNumber: manifest.wagonNumber,
        measuredHeight: measurement.measuredHeight,
        labDensity: measurement.labDensity ?? null,
        calculatedMass: measurement.calculatedMass,
      })),
      weights: manifest.weights.map(weight => ({
        wagonNumber: manifest.wagonNumber,
        grossWeight: weight.grossWeight,
        tareWeight: weight.tareWeight,
        netProductWeight: weight.netProductWeight,
      })),
    })),
  }
}

export function RailReceiptMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: RailReceiptMutateDialogProps) {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const createMutation = useTransportRailWaybillCompositeCreate()
  const updateMutation = useTransportRailWaybillCompositeUpdate()

  const isUpdate = currentRow != null
  const waybillId = currentRow?.id ?? null

  const composite = useTransportRailWaybillCompositeGet(waybillId ?? '', undefined, {
    query: { enabled: Boolean(open && waybillId) },
  })
  const loaded = composite.data?.data

  const defaultValues = useMemo<RailReceiptForm>(() => {
    if (!isUpdate || !loaded)
      return emptyRailReceiptForm
    const wb = loaded.waybill
    return {
      documentNumber: wb.documentNumber,
      date: wb.date,
      senderId: wb.senderId,
      baseId: wb.baseId,
      manifests: (loaded.wagonManifests ?? []).map(toManifestRow),
    }
  }, [isUpdate, loaded])

  const mutationFn = useCallback(
    async (data: RailReceiptForm): Promise<TransportRailWaybillCompositeCreateMutationResponse> => {
      if (isUpdate && waybillId) {
        return updateMutation.mutateAsync({
          id: waybillId,
          data: data as unknown as TransportRailWaybillCompositeUpdateMutationRequest,
        })
      }
      return createMutation.mutateAsync({ data: toCreatePayload(data) })
    },
    [isUpdate, waybillId, createMutation, updateMutation],
  )

  const handleSuccess = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: railReceiptPipelineQueryQueryKey() })
    toast.success(
      t(isUpdate ? 'rail-receipt:toast.updated' : 'rail-receipt:toast.created'),
    )
  }, [isUpdate, queryClient, t])

  const dialogKey = isUpdate ? (loaded?.waybill.id ?? 'edit-loading') : 'create'

  return (
    <CompositeFormDialog<RailReceiptForm, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={railReceiptFormSchema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={isUpdate ? 'rail-receipt:dialog.title.edit' : 'rail-receipt:dialog.title.create'}
    >
      <DocHeaderSection fields={railReceiptHeaderSpec} />
      <DocItemsTable<RailReceiptForm, RailReceiptManifest>
        name={'manifests' as ArrayPath<RailReceiptForm>}
        columns={railReceiptManifestColumns}
        rowSchema={railReceiptManifestSchema}
        rowFields={railReceiptManifestFields}
        emptyRow={emptyRailReceiptManifest}
        sectionTitleKey="rail-receipt:section.manifests"
        rowDrawerExtra={rowIndex => (
          <>
            <DocItemsTable<RailReceiptForm, RailReceiptMeasurement>
              name={`manifests.${rowIndex}.measurements` as ArrayPath<RailReceiptForm>}
              columns={railReceiptMeasurementColumns}
              rowSchema={railReceiptMeasurementSchema}
              rowFields={railReceiptMeasurementFields}
              emptyRow={emptyRailReceiptMeasurement}
              sectionTitleKey="rail-receipt:section.measurements"
              density="compact"
            />
            <DocItemsTable<RailReceiptForm, RailReceiptWeight>
              name={`manifests.${rowIndex}.weights` as ArrayPath<RailReceiptForm>}
              columns={railReceiptWeightColumns}
              rowSchema={railReceiptWeightSchema}
              rowFields={railReceiptWeightFields}
              emptyRow={emptyRailReceiptWeight}
              sectionTitleKey="rail-receipt:section.weights"
              density="compact"
            />
          </>
        )}
      />
    </CompositeFormDialog>
  )
}
