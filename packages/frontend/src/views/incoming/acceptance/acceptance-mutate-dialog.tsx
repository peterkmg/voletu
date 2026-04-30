/**
 * Per-document mutate dialog for Acceptance (create + edit).
 *
 * Composes the shared `<CompositeFormDialog>` with the Acceptance-specific
 * header spec / items table coming from `acceptance-form-config.tsx`, and
 * wires the Kubb-generated composite create and update mutations.
 *
 * Edit-mode `defaultValues` are pre-fetched via `useAcceptanceCompositeGet`
 * (gated on `open && isUpdate`). While the fetch is in flight the form
 * renders with `emptyAcceptanceCreate`; once data arrives, the dialog is
 * re-mounted with the real values by keying `<CompositeFormDialog>` on the
 * loaded document id. This keeps the component stateless w.r.t. the fetch
 * and avoids a stale-form flash on open.
 */

import type { AcceptanceCreate, AcceptanceItem } from './acceptance-form-config'
import type { BasisPrefillRef } from './use-basis-prefill'
import type { AcceptanceFlatRow } from '~/generated/types'
import type { AcceptanceCompositeCreateMutationResponse } from '~/generated/types/DocumentAcceptanceTypes/AcceptanceCompositeCreate'
import type { AcceptanceCompositeUpdateMutationRequest } from '~/generated/types/DocumentAcceptanceTypes/AcceptanceCompositeUpdate'
import { useQueryClient } from '@tanstack/react-query'
import { useCallback, useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import {
  CompositeFormDialog,
  DocHeaderSection,
  DocItemsTable,
} from '~/components/composite-form'
import { useAcceptanceCompositeCreate } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeCreate'
import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { useAcceptanceCompositeUpdate } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeUpdate'
import { flowAcceptanceFlatQueryQueryKey } from '~/generated/hooks/FlowsHooks/useFlowAcceptanceFlatQuery'
import { flowTruckReceiptQueryQueryKey } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { railReceiptPipelineQueryQueryKey } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { AcceptanceBasisSection } from './acceptance-basis-section'
import {
  acceptanceCreateSchema,
  acceptanceHeaderSpec,
  acceptanceItemColumns,
  acceptanceItemFields,
  acceptanceItemSchema,
  acceptanceUpdateSchema,
  emptyAcceptanceCreate,
  emptyAcceptanceItem,
  makeAcceptanceUpdateSchema,
} from './acceptance-form-config'
import { toAcceptanceFormValue } from './acceptance-item-mapping'
import { useBasisPrefill } from './use-basis-prefill'

interface AcceptanceMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  /**
   * Row currently selected in the flat list. `documentId` identifies the
   * acceptance; `id` is item-scoped and must not be used for the update.
   * When `null`, the dialog opens in create mode.
   */
  currentRow?: AcceptanceFlatRow | null
  /**
   * Optional basis pre-fill source. When set, the dialog opens in create
   * mode with the matching basis tab pre-selected and locked, contractor
   * defaulted from the waybill, and one item row per waybill item.
   * Mutually exclusive with `currentRow` (edit mode).
   */
  prefillBasis?: BasisPrefillRef
}

export function AcceptanceMutateDialog({
  open,
  onOpenChange,
  currentRow,
  prefillBasis,
}: AcceptanceMutateDialogProps) {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const createMutation = useAcceptanceCompositeCreate()
  const updateMutation = useAcceptanceCompositeUpdate()

  const isUpdate = currentRow != null
  const documentId = currentRow?.documentId ?? null
  const hasPrefill = !isUpdate && prefillBasis != null

  // Pre-fetch the full composite only when editing. While the query is
  // resolving, `data` is undefined and `emptyAcceptanceCreate` is used as a
  // placeholder; the dialog is keyed on the loaded doc id below so that the
  // form re-initializes exactly once when the real data lands.
  const composite = useAcceptanceCompositeGet(documentId ?? '', undefined, {
    query: { enabled: Boolean(open && documentId) },
  })
  const loaded = composite.data?.data

  // Pre-fetch the basis composite when opening from a row trigger / detail CTA.
  // The hook is gated on `open && hasPrefill`, so it is a no-op for the
  // standalone-create and edit paths.
  const prefill = useBasisPrefill(prefillBasis, Boolean(open && hasPrefill))

  const defaultValues = useMemo<AcceptanceCreate>(() => {
    if (isUpdate)
      return loaded ? toAcceptanceFormValue(loaded) : emptyAcceptanceCreate
    if (hasPrefill)
      return prefill.prefilled ?? emptyAcceptanceCreate
    return emptyAcceptanceCreate
  }, [isUpdate, loaded, hasPrefill, prefill.prefilled])

  const mutationFn = useCallback(
    async (data: AcceptanceCreate): Promise<AcceptanceCompositeCreateMutationResponse> => {
      if (isUpdate && documentId) {
        // Update mode validates through acceptanceUpdateSchema, which wraps
        // the generated update composite schema. Existing item ids are kept
        // in defaultValues so the backend can diff rows in place.
        return updateMutation.mutateAsync({
          id: documentId,
          data: data as unknown as AcceptanceCompositeUpdateMutationRequest,
        })
      }
      return createMutation.mutateAsync({ data })
    },
    [isUpdate, documentId, createMutation, updateMutation],
  )

  const handleSuccess = useCallback(() => {
    // Always invalidate the acceptance flat query so the action list refreshes.
    queryClient.invalidateQueries({ queryKey: flowAcceptanceFlatQueryQueryKey() })
    // Risk §6.2: when a new acceptance is created (with or without basis),
    // invalidate the truck/rail pipeline queries so the PENDING-waybill
    // picker (and the row-action gate on the basis lists) reflects the
    // post-create pipeline status without a manual refresh. Also runs in
    // edit mode, where the pipeline-status pill on the basis detail can
    // change as a side-effect of the edit.
    queryClient.invalidateQueries({ queryKey: flowTruckReceiptQueryQueryKey() })
    queryClient.invalidateQueries({ queryKey: railReceiptPipelineQueryQueryKey() })
    toast.success(
      t(isUpdate ? 'acceptance:toast.updated' : 'acceptance:toast.created'),
    )
  }, [isUpdate, queryClient, t])

  // `key` forces a fresh mount once the edit-mode fetch (or prefill fetch)
  // resolves so that defaultValues are applied via react-hook-form's
  // initialization path. The loading-phase key encodes kind + basisId so
  // that closing and reopening the dialog with a different basis remounts
  // cleanly instead of briefly reusing the previous instance's form state.
  let dialogKey: string
  if (isUpdate) {
    dialogKey = loaded?.id ?? `edit-loading-${documentId ?? ''}`
  }
  else if (hasPrefill) {
    dialogKey = prefill.prefilled
      ? `prefill-${prefillBasis!.kind}-${prefillBasis!.basisId}`
      : `prefill-loading-${prefillBasis!.kind}-${prefillBasis!.basisId}`
  }
  else {
    dialogKey = 'create'
  }

  // The basis tab is locked in edit mode (rule 2.6) and in row-trigger /
  // detail-CTA create mode (rule 2.4); only the standalone external create
  // path leaves the tab unlocked.
  const basisLocked = isUpdate || hasPrefill

  // Edit-mode schema is the lifecycle-aware factory variant when the
  // original composite is loaded — it asserts arrivalType and basis FKs
  // match the loaded values (rule 2.6, defense-in-depth on top of the UI
  // tab-lock). The stateless `acceptanceUpdateSchema` is used while the
  // fetch is in flight; submission is gated on `loaded` anyway because
  // `mutationFn` requires `documentId`, so the stateless variant is only
  // exercised by tests / future callers without a loaded composite.
  const schema = isUpdate
    ? loaded
      ? makeAcceptanceUpdateSchema(loaded as unknown as Parameters<typeof makeAcceptanceUpdateSchema>[0])
      : acceptanceUpdateSchema
    : acceptanceCreateSchema

  return (
    <CompositeFormDialog<AcceptanceCreate, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={schema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={isUpdate ? 'acceptance:dialog.title.edit' : 'acceptance:dialog.title.create'}
    >
      <AcceptanceBasisSection
        mode={isUpdate ? 'edit' : 'create'}
        locked={basisLocked}
        lockedHintNumber={hasPrefill ? prefill.lockedHintNumber ?? undefined : undefined}
      />
      <DocHeaderSection fields={acceptanceHeaderSpec} />
      <DocItemsTable<AcceptanceCreate, AcceptanceItem>
        name="items"
        columns={acceptanceItemColumns}
        rowSchema={acceptanceItemSchema}
        rowFields={acceptanceItemFields}
        emptyRow={emptyAcceptanceItem}
        sectionTitleKey="acceptance:section.items"
      />
    </CompositeFormDialog>
  )
}
