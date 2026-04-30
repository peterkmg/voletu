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

  currentRow?: BlendingFlatRow | null
}

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
