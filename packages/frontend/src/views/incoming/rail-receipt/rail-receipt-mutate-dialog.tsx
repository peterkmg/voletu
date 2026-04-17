/**
 * Per-document mutate dialog for Rail Receipt (rail waybill) - create + edit.
 *
 * The rail composite is the deepest aggregate in the system: a waybill owns
 * many wagon manifests, each of which owns its measurements and weights. The
 * dialog handles all three levels in one screen by rendering an outer
 * `<DocItemsTable>` for manifests and using its `rowDrawerExtra` slot to mount
 * two compact inner `<DocItemsTable>` instances for the currently-edited
 * manifest's measurements and weights. This validates that the shared
 * composite-form abstraction supports recursive nesting end-to-end.
 *
 * The form-state is the *update* composite shape (`UpdateRailWaybillCompositeRequest`).
 * On create we strip every `id: null` (the wire create shape forbids `id`) and
 * synthesise the denormalised `wagonNumber` field that lives on each child in
 * the create-side schema. On update the payload round-trips ids unchanged so
 * the backend's recursive diff treats existing rows as updates.
 */

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
  /**
   * Row currently selected in the pipeline list. When the row's
   * `pipelineStatus === 'PENDING'`, `id` identifies the rail waybill
   * (the basis document) and the dialog opens in edit mode. Otherwise the
   * row maps to an acceptance, which this dialog does not handle. When
   * `null`, the dialog opens in create mode.
   */
  currentRow?: RailReceiptPipelineResponse | null
}

/**
 * Convert a measurement returned by the API into the form-state row shape.
 * The wire response carries server-only fields (timestamps, audit ids) we
 * drop here so RHF default values stay clean.
 */
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

/**
 * Reshape the form-state into the create-side wire payload.
 *
 * The create wire shape:
 *   - has no `id` field on any row (ids are server-assigned);
 *   - carries a denormalised `wagonNumber` on every child (measurement/weight)
 *     even though the same value lives on the parent manifest.
 *
 * We strip ids and synthesise the wagonNumber here so the form state can stay
 * a single uniform shape (the update shape).
 */
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

  // Pre-fetch the full composite only when editing. Same approach as the
  // truck-receipt and physical-transfer dialogs: gate on `open && isUpdate`
  // and rely on `key` to remount once the data arrives.
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

  // `key` forces a fresh mount once the edit-mode fetch resolves so RHF
  // initialises with the loaded values.
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
