import type { EntityDialogState } from './entity-dialog-state'
import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/dialogs/entity-delete-dialog'

interface EntityHook<TRow> {
  dialog: EntityDialogState<TRow> | null
  closeDialog: () => void
}

interface SoftDeleteConfig<TRow extends { id: string }> {
  hardDeleteFn: (id: string) => Promise<unknown>
  softDeleteFn: (id: string) => Promise<unknown>
  queryKey: () => readonly unknown[]
  entityLabel: string
  i18nNamespaces: readonly [string, ...string[]]
  useEntity: () => EntityHook<TRow>
}

interface HardDeleteConfig<TRow extends { id: string }> {
  hardDeleteFn: (id: string) => Promise<unknown>
  softDeleteFn?: undefined
  queryKey: () => readonly unknown[]
  entityLabel: string
  i18nNamespaces: readonly [string, ...string[]]
  useEntity: () => EntityHook<TRow>
}

type DeleteDialogConfig<TRow extends { id: string }>
  = | SoftDeleteConfig<TRow>
    | HardDeleteConfig<TRow>

export function createDeleteDialog<TRow extends { id: string }>(
  config: DeleteDialogConfig<TRow>,
) {
  const {
    hardDeleteFn,
    softDeleteFn,
    queryKey,
    entityLabel,
    i18nNamespaces,
    useEntity,
  } = config

  function DeleteDialog() {
    const { dialog, closeDialog } = useEntity()
    const { t } = useTranslation(i18nNamespaces as unknown as string[])
    const currentRow = dialog?.kind === 'delete' ? dialog.row : null

    if (softDeleteFn) {
      return (
        <>
          <EntityDeleteDialog
            open={dialog?.kind === 'delete' && dialog.mode !== 'hard'}
            onOpenChange={closeDialog}
            currentRow={currentRow}
            variant="soft"
            hardDeleteFn={hardDeleteFn}
            softDeleteFn={softDeleteFn}
            queryKey={queryKey()}
            entityLabel={t(entityLabel)}
          />
          <EntityDeleteDialog
            open={dialog?.kind === 'delete' && dialog.mode === 'hard'}
            onOpenChange={closeDialog}
            currentRow={currentRow}
            variant="hard"
            hardDeleteFn={hardDeleteFn}
            softDeleteFn={softDeleteFn}
            queryKey={queryKey()}
            entityLabel={t(entityLabel)}
          />
        </>
      )
    }

    return (
      <EntityDeleteDialog
        open={dialog?.kind === 'delete'}
        onOpenChange={closeDialog}
        currentRow={currentRow}
        hardDeleteFn={hardDeleteFn}
        queryKey={queryKey()}
        entityLabel={t(entityLabel)}
      />
    )
  }

  return DeleteDialog
}
