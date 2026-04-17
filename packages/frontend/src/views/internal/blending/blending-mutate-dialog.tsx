/**
 * Per-document mutate dialog for Blending (create + edit).
 *
 * Composes the shared `<CompositeFormDialog>` with the blending header spec
 * plus two sibling `<DocItemsTable>` instances — one for the input
 * `components` collection and one for the output `results` collection — coming
 * from `blending-form-config.tsx`, and wires the Kubb-generated composite
 * create and update mutations.
 *
 * Edit-mode `defaultValues` are pre-fetched via `useBlendingCompositeGet`
 * (gated on `open && isUpdate`). While the fetch is in flight the form
 * renders with `emptyBlendingCreate`; once data arrives, the dialog is
 * re-mounted with the real values by keying `<CompositeFormDialog>` on the
 * loaded document id.
 *
 * Composite response shape: `BlendingCompositeResponse = { document, components, results }`,
 * so header fields are read from `loaded.document.X` while the two child
 * collections live at `loaded.components` / `loaded.results`.
 */

import type { BlendingComponent, BlendingCreate, BlendingResult } from './blending-form-config'
import type { BlendingComponentResponse, BlendingFlatRow, BlendingResultResponse } from '~/generated/types'
import type { BlendingCompositeUpdateMutationRequest } from '~/generated/types/DocumentOperationsTypes/BlendingCompositeUpdate'
import { useQueryClient } from '@tanstack/react-query'
import { useCallback, useMemo } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import {
  CompositeFormDialog,
  DocHeaderSection,
  DocItemsTable,
} from '~/components/composite-form'
import { useBlendingCompositeCreate } from '~/generated/hooks/DocumentOperationsHooks/useBlendingCompositeCreate'
import { useBlendingCompositeGet } from '~/generated/hooks/DocumentOperationsHooks/useBlendingCompositeGet'
import { useBlendingCompositeUpdate } from '~/generated/hooks/DocumentOperationsHooks/useBlendingCompositeUpdate'
import { flowBlendingFlatQueryQueryKey } from '~/generated/hooks/FlowsHooks/useFlowBlendingFlatQuery'
import {

  blendingComponentColumns,
  blendingComponentFields,
  blendingComponentSchema,

  blendingCreateSchema,
  blendingHeaderSpec,

  blendingResultColumns,
  blendingResultFields,
  blendingResultSchema,
  blendingUpdateSchema,
  emptyBlendingComponent,
  emptyBlendingCreate,
  emptyBlendingResult,
} from './blending-form-config'

interface BlendingMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  /**
   * Row currently selected in the flat list. `documentId` identifies the
   * blending document; `id` is row-scoped and must not be used for the update.
   * When `null`, the dialog opens in create mode.
   */
  currentRow?: BlendingFlatRow | null
}

/** Drop server-only fields and keep only the shape the composite request expects. */
function toComponentRequest(row: BlendingComponentResponse): BlendingComponent {
  return {
    sourceProductId: row.sourceProductId,
    storageId: row.storageId,
    amountUsed: row.amountUsed,
  }
}

function toResultRequest(row: BlendingResultResponse): BlendingResult {
  return {
    storageId: row.storageId,
    producedAmount: row.producedAmount,
  }
}

export function BlendingMutateDialog({
  open,
  onOpenChange,
  currentRow,
}: BlendingMutateDialogProps) {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const createMutation = useBlendingCompositeCreate()
  const updateMutation = useBlendingCompositeUpdate()

  const isUpdate = currentRow != null
  const documentId = currentRow?.documentId ?? null

  // Pre-fetch the full composite only when editing.
  const composite = useBlendingCompositeGet(documentId ?? '', undefined, {
    query: { enabled: Boolean(open && documentId) },
  })
  const loaded = composite.data?.data

  const defaultValues = useMemo<BlendingCreate>(() => {
    if (!isUpdate || !loaded)
      return emptyBlendingCreate
    return {
      documentNumber: loaded.document.documentNumber,
      date: loaded.document.date,
      contractorId: loaded.document.contractorId,
      targetProductId: loaded.document.targetProductId,
      components: (loaded.components ?? []).map(toComponentRequest),
      results: (loaded.results ?? []).map(toResultRequest),
    }
  }, [isUpdate, loaded])

  const mutationFn = useCallback(
    async (
      data: BlendingCreate,
    ): Promise<unknown> => {
      if (isUpdate && documentId) {
        return updateMutation.mutateAsync({
          id: documentId,
          data: data as unknown as BlendingCompositeUpdateMutationRequest,
        })
      }
      return createMutation.mutateAsync({ data })
    },
    [isUpdate, documentId, createMutation, updateMutation],
  )

  const handleSuccess = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: flowBlendingFlatQueryQueryKey() })
    toast.success(
      t(isUpdate ? 'blending:toast.updated' : 'blending:toast.created'),
    )
  }, [isUpdate, queryClient, t])

  // `key` forces a fresh mount once the edit-mode fetch resolves so that
  // defaultValues are applied via react-hook-form's initialization path.
  const dialogKey = isUpdate ? (loaded?.document.id ?? 'edit-loading') : 'create'

  return (
    <CompositeFormDialog<BlendingCreate, unknown>
      key={dialogKey}
      open={open}
      onOpenChange={onOpenChange}
      mode={isUpdate ? 'edit' : 'create'}
      schema={isUpdate ? blendingUpdateSchema : blendingCreateSchema}
      defaultValues={defaultValues}
      mutationFn={mutationFn}
      onSuccess={handleSuccess}
      titleKey={
        isUpdate
          ? 'blending:dialog.title.edit'
          : 'blending:dialog.title.create'
      }
    >
      <DocHeaderSection fields={blendingHeaderSpec} />
      <DocItemsTable<BlendingCreate, BlendingComponent>
        name="components"
        columns={blendingComponentColumns}
        rowSchema={blendingComponentSchema}
        rowFields={blendingComponentFields}
        emptyRow={emptyBlendingComponent}
        sectionTitleKey="blending:section.components"
      />
      <DocItemsTable<BlendingCreate, BlendingResult>
        name="results"
        columns={blendingResultColumns}
        rowSchema={blendingResultSchema}
        rowFields={blendingResultFields}
        emptyRow={emptyBlendingResult}
        sectionTitleKey="blending:section.results"
      />
    </CompositeFormDialog>
  )
}
