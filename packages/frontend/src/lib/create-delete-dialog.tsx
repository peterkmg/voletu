import { useTranslation } from 'react-i18next'
import { EntityDeleteDialog } from '~/components/dialogs/entity-delete-dialog'

interface EntityHook<TRow> {
  open: string | null
  setOpen: (v: null) => void
  currentRow: TRow | null
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

type DeleteDialogConfig<TRow extends { id: string }> =
  | SoftDeleteConfig<TRow>
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
    const { open, setOpen, currentRow } = useEntity()
    const { t } = useTranslation(i18nNamespaces as unknown as string[])

    if (softDeleteFn) {
      return (
        <>
          <EntityDeleteDialog
            open={open === 'delete'}
            onOpenChange={() => setOpen(null)}
            currentRow={currentRow}
            variant="soft"
            hardDeleteFn={hardDeleteFn}
            softDeleteFn={softDeleteFn}
            queryKey={queryKey()}
            entityLabel={t(entityLabel)}
          />
          <EntityDeleteDialog
            open={open === 'hard-delete'}
            onOpenChange={() => setOpen(null)}
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
        open={open === 'delete'}
        onOpenChange={() => setOpen(null)}
        currentRow={currentRow}
        hardDeleteFn={hardDeleteFn}
        queryKey={queryKey()}
        entityLabel={t(entityLabel)}
      />
    )
  }

  return DeleteDialog
}
