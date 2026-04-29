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
import {

  acceptanceCreateSchema,
  acceptanceHeaderSpec,

  acceptanceItemColumns,
  acceptanceItemFields,
  acceptanceItemSchema,
  acceptanceUpdateSchema,
  emptyAcceptanceCreate,
  emptyAcceptanceItem,
} from './acceptance-form-config'
import { toAcceptanceItemFormValue } from './acceptance-item-mapping'

interface AcceptanceMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  /**
   * Row currently selected in the flat list. `documentId` identifies the
   * acceptance; `id` is item-scoped and must not be used for the update.
   * When `null`, the dialog opens in create mode.
   */
  currentRow?: AcceptanceFlatRow | null
}

export function AcceptanceMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: AcceptanceMutateDialogProps) {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const createMutation = useAcceptanceCompositeCreate()
  const updateMutation = useAcceptanceCompositeUpdate()

  const isUpdate = currentRow != null
  const documentId = currentRow?.documentId ?? null

  // Pre-fetch the full composite only when editing. While the query is
  // resolving, `data` is undefined and `emptyAcceptanceCreate` is used as a
  // placeholder; the dialog is keyed on the loaded doc id below so that the
  // form re-initializes exactly once when the real data lands.
  const composite = useAcceptanceCompositeGet(documentId ?? '', undefined, {
    query: { enabled: Boolean(open && documentId) },
  })
  const loaded = composite.data?.data

  const defaultValues = useMemo<AcceptanceCreate>(() => {
    if (!isUpdate || !loaded)
      return emptyAcceptanceCreate
    return {
      documentNumber: loaded.documentNumber,
      dateAccepted: loaded.dateAccepted,
      arrivalType: loaded.arrivalType,
      contractorId: loaded.contractorId,
      sourceEntity: loaded.sourceEntity ?? null,
      items: loaded.items.map(toAcceptanceItemFormValue),
    }
  }, [isUpdate, loaded])

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
    queryClient.invalidateQueries({ queryKey: flowAcceptanceFlatQueryQueryKey() })
    toast.success(
      t(isUpdate ? 'acceptance:toast.updated' : 'acceptance:toast.created'),
    )
  }, [isUpdate, queryClient, t])

  // `key` forces a fresh mount once the edit-mode fetch resolves so that
  // defaultValues are applied via react-hook-form's initialization path.
  const dialogKey = isUpdate ? (loaded?.id ?? 'edit-loading') : 'create'

  return (
    <CompositeFormDialog<AcceptanceCreate, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={isUpdate ? acceptanceUpdateSchema : acceptanceCreateSchema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={isUpdate ? 'acceptance:dialog.title.edit' : 'acceptance:dialog.title.create'}
    >
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
