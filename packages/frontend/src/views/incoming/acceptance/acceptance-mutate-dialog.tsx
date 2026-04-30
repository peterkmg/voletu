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

  currentRow?: AcceptanceFlatRow | null

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

  const composite = useAcceptanceCompositeGet(documentId ?? '', undefined, {
    query: { enabled: Boolean(open && documentId) },
  })
  const loaded = composite.data?.data

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

    queryClient.invalidateQueries({ queryKey: flowTruckReceiptQueryQueryKey() })
    queryClient.invalidateQueries({ queryKey: railReceiptPipelineQueryQueryKey() })
    toast.success(
      t(isUpdate ? 'acceptance:toast.updated' : 'acceptance:toast.created'),
    )
  }, [isUpdate, queryClient, t])

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

  const basisLocked = isUpdate || hasPrefill

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
