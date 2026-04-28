import type { Row } from '@tanstack/react-table'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}))

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}))

vi.mock('~/stores/auth-store', () => ({
  useAuthStore: (selector: (state: { user: { role: string } }) => unknown) =>
    selector({ user: { role: 'ADMIN' } }),
}))

vi.mock('~/components/data-table', () => ({
  RowActions: ({ actions }: { actions: Array<{ label: string, onClick: () => void }> }) => (
    <div>
      {actions.map(action => (
        <button key={action.label} type="button" onClick={action.onClick}>
          {action.label}
        </button>
      ))}
    </div>
  ),
}))

vi.mock('~/components/dialogs/entity-delete-dialog', () => ({
  EntityDeleteDialog: ({
    open,
    currentRow,
    variant,
  }: {
    open: boolean
    currentRow: { id: string, name: string } | null
    variant?: 'soft' | 'hard'
  }) => (
    open
      ? (
          <div data-testid="delete-dialog">
            {variant ?? 'delete'}
            :
            {currentRow?.name ?? 'none'}
          </div>
        )
      : null
  ),
}))

interface TestRow {
  id: string
  name: string
  status?: 'DRAFT' | 'EXECUTED'
}

const firstRow: TestRow = { id: 'row-1', name: 'Alpha', status: 'DRAFT' }
const secondRow: TestRow = { id: 'row-2', name: 'Beta', status: 'EXECUTED' }

describe('typed entity dialog state', () => {
  function expectDialogState(expected: unknown) {
    const state = screen.getByTestId('dialog-state').textContent
    expect(state ? JSON.parse(state) : null).toEqual(expected)
  }

  it('stores typed dialog actions instead of raw string state', async () => {
    const { Provider, useEntity } = createEntityProvider<TestRow>('TestEntity')
    const PrimaryButtons = createPrimaryButtons({ useEntity })

    function Harness() {
      const {
        dialog,
        openUpdate,
        openDelete,
        openLifecycle,
        closeDialog,
      } = useEntity()

      return (
        <>
          <PrimaryButtons />
          <button type="button" onClick={() => openUpdate(firstRow)}>update</button>
          <button type="button" onClick={() => openDelete(firstRow, 'soft')}>soft-delete</button>
          <button type="button" onClick={() => openDelete(firstRow, 'hard')}>hard-delete</button>
          <button type="button" onClick={() => openLifecycle(secondRow, 'revert')}>revert</button>
          <button type="button" onClick={closeDialog}>close</button>
          <pre data-testid="dialog-state">{JSON.stringify(dialog)}</pre>
        </>
      )
    }

    const user = userEvent.setup()

    render(
      <Provider>
        <Harness />
      </Provider>,
    )

    expectDialogState(null)

    await user.click(screen.getByRole('button', { name: 'common:actions.create' }))
    expectDialogState({ kind: 'create' })

    await user.click(screen.getByRole('button', { name: 'update' }))
    expectDialogState({
      kind: 'update',
      row: firstRow,
    })

    await user.click(screen.getByRole('button', { name: 'soft-delete' }))
    expectDialogState({
      kind: 'delete',
      mode: 'soft',
      row: firstRow,
    })

    await user.click(screen.getByRole('button', { name: 'hard-delete' }))
    expectDialogState({
      kind: 'delete',
      mode: 'hard',
      row: firstRow,
    })

    await user.click(screen.getByRole('button', { name: 'revert' }))
    expectDialogState({
      kind: 'lifecycle',
      action: 'revert',
      row: secondRow,
    })

    await user.click(screen.getByRole('button', { name: 'close' }))
    expectDialogState(null)
  })

  it('routes row actions and dialog helpers through the typed dialog state', async () => {
    const { Provider, useEntity } = createEntityProvider<TestRow>('TestEntity')
    const RowActions = createRowActions<TestRow>({ useEntity, lifecycle: true })
    const DeleteDialog = createDeleteDialog({
      useEntity,
      hardDeleteFn: vi.fn(async () => undefined),
      softDeleteFn: vi.fn(async () => undefined),
      queryKey: () => ['test-rows'],
      entityLabel: 'test.entity',
      i18nNamespaces: ['common'],
    })
    const Dialogs = createEntityDialogs({
      useEntity,
      MutateDialog: ({
        open,
        currentRow,
      }: {
        open: boolean
        onOpenChange: () => void
        currentRow: TestRow | null
      }) => (open ? <div data-testid="mutate-dialog">{currentRow?.name ?? 'create'}</div> : null),
      DeleteDialog,
      LifecycleDialog: ({
        open,
        currentRow,
        variant,
      }: {
        open: boolean
        onOpenChange: () => void
        currentRow: TestRow | null
        variant: 'execute' | 'revert'
      }) => (open
        ? (
            <div data-testid="lifecycle-dialog">
              {variant}
              :
              {currentRow?.name ?? 'none'}
            </div>
          )
        : null),
      lifecyclePropName: 'variant',
    })

    const user = userEvent.setup()

    render(
      <Provider>
        <RowActions row={{ original: firstRow } as Row<TestRow>} />
        <Dialogs />
      </Provider>,
    )

    await user.click(screen.getByRole('button', { name: 'common:actions.edit' }))
    expect(screen.getByTestId('mutate-dialog')).toHaveTextContent('Alpha')

    await user.click(screen.getByRole('button', { name: 'common:actions.softDelete' }))
    expect(screen.getByTestId('delete-dialog')).toHaveTextContent('soft:Alpha')

    await user.click(screen.getByRole('button', { name: 'documents:lifecycle.execute' }))
    expect(screen.getByTestId('lifecycle-dialog')).toHaveTextContent('execute:Alpha')
  })

  it('supports lifecycle dialogs that receive an action prop', async () => {
    const { Provider, useEntity } = createEntityProvider<TestRow>('TestEntity')
    const RowActions = createRowActions<TestRow>({ useEntity, lifecycle: true })
    const Dialogs = createEntityDialogs({
      useEntity,
      MutateDialog: () => null,
      LifecycleDialog: ({
        open,
        currentRow,
        action,
      }: {
        open: boolean
        onOpenChange: () => void
        currentRow: TestRow | null
        action: 'execute' | 'revert'
      }) => (open
        ? (
            <div data-testid="action-lifecycle-dialog">
              {action}
              :
              {currentRow?.name ?? 'none'}
            </div>
          )
        : null),
      lifecyclePropName: 'action',
    })

    const user = userEvent.setup()

    render(
      <Provider>
        <RowActions row={{ original: firstRow } as Row<TestRow>} />
        <Dialogs />
      </Provider>,
    )

    await user.click(screen.getByRole('button', { name: 'documents:lifecycle.execute' }))
    expect(screen.getByTestId('action-lifecycle-dialog')).toHaveTextContent('execute:Alpha')
  })
})
