/* eslint-disable react/component-hook-factories */

import type { Row } from '@tanstack/react-table'
import type {
  EntityDialogsLifecycleConfig,
  MutateDialogProps,
} from './create-entity-dialogs'
import { EntityPage } from '~/components/entity-page'
import { createDeleteDialog } from './create-delete-dialog'
import { createEntityDialogs } from './create-entity-dialogs'
import { createEntityProvider } from './create-entity-provider'
import { createPrimaryButtons } from './create-primary-buttons'
import { createRowActions } from './create-row-actions'

export interface CrudViewQueryResult<TRow> {
  data?: { data?: TRow[] }
  isLoading: boolean
}

export interface CrudViewTableProps<TRow extends { id: string }> {
  data: TRow[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<TRow> }>
}

export interface CrudViewDeleteDialogConfig {
  hardDeleteFn: (id: string) => Promise<unknown>
  softDeleteFn?: (id: string) => Promise<unknown>
  queryKey: () => readonly unknown[]
  entityLabel: string
  i18nNamespaces: readonly [string, ...string[]]
}

export interface CrudViewConfig<TRow extends { id: string }> {
  displayName: string
  useTitle: () => string
  useQuery: () => CrudViewQueryResult<TRow>
  Table: React.ComponentType<CrudViewTableProps<TRow>>
  MutateDialog: React.ComponentType<MutateDialogProps<TRow>>
  deleteDialog?: CrudViewDeleteDialogConfig
  supportsUpdate?: boolean
  rowActions?: {
    lifecycle?: boolean
    deleteOnly?: boolean
    disableEdit?: boolean
    getDetailPath?: (row: TRow) => string
  }
}

export type CrudViewDefinitionConfig<TRow extends { id: string }>
  = CrudViewConfig<TRow>
    & EntityDialogsLifecycleConfig<TRow>

export function defineCrudViews<TRow extends { id: string }>(
  config: CrudViewDefinitionConfig<TRow>,
) {
  const { Provider, useEntity } = createEntityProvider<TRow>(config.displayName)
  const RowActions = createRowActions<TRow>({
    useEntity,
    ...config.rowActions,
  })
  const DeleteDialog = config.deleteDialog
    ? createDeleteDialog({
        useEntity,
        ...config.deleteDialog,
      })
    : undefined
  const dialogsConfigBase = {
    useEntity,
    MutateDialog: config.MutateDialog,
    DeleteDialog,
    supportsUpdate: config.supportsUpdate,
  } satisfies Omit<
    Parameters<typeof createEntityDialogs<TRow>>[0],
    'LifecycleDialog' | 'lifecyclePropName'
  >
  const Dialogs = ('LifecycleDialog' in config && config.LifecycleDialog)
    ? (config.lifecyclePropName === 'action'
        ? createEntityDialogs({
            ...dialogsConfigBase,
            LifecycleDialog: config.LifecycleDialog,
            lifecyclePropName: 'action',
          })
        : createEntityDialogs({
            ...dialogsConfigBase,
            LifecycleDialog: config.LifecycleDialog,
            lifecyclePropName: 'variant',
          }))
    : createEntityDialogs(dialogsConfigBase)
  const PrimaryButtons = createPrimaryButtons({ useEntity })

  function TableWithActions({
    data,
    actions,
  }: {
    data: TRow[]
    actions?: React.ReactNode
  }) {
    return (
      <config.Table
        data={data}
        actions={actions}
        RowActions={RowActions}
      />
    )
  }

  function View() {
    const title = config.useTitle()
    const queryResult = config.useQuery()

    return (
      <EntityPage
        provider={Provider}
        title={title}
        queryResult={queryResult}
        primaryButtons={PrimaryButtons}
        table={TableWithActions}
        dialogs={Dialogs}
      />
    )
  }

  return {
    Dialogs,
    View,
    PrimaryButtons,
    Provider,
    RowActions,
    useEntity,
  } as const
}
