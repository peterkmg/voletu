import type { Row } from '@tanstack/react-table'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { defineCrudViews } from '~/lib/define-crud-views'

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

vi.mock('~/components/layout/header', () => ({
  Header: () => <div data-testid="header" />,
}))

vi.mock('~/components/layout/main', () => ({
  Main: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="main">{children}</div>
  ),
}))

vi.mock('~/hooks/use-page-title', () => ({
  usePageTitle: vi.fn(),
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
}

const firstRow: TestRow = { id: 'row-1', name: 'Alpha' }

describe('defineCrudViews', () => {
  it('assembles provider, row actions, primary buttons, dialogs, and page wiring from one config', async () => {
    const user = userEvent.setup()

    const crudViewDefinition = defineCrudViews<TestRow>({
      displayName: 'TestRows',
      useTitle: () => 'test.rows',
      useQuery: () => ({
        data: { data: [firstRow] },
        isLoading: false,
      }),
      Table: ({
        data,
        actions,
        RowActions,
      }: {
        data: TestRow[]
        actions?: React.ReactNode
        RowActions: React.ComponentType<{ row: Row<TestRow> }>
      }) => (
        <div>
          <div data-testid="row-name">{data[0]?.name}</div>
          {actions}
          <RowActions row={{ original: data[0] } as Row<TestRow>} />
        </div>
      ),
      MutateDialog: ({
        open,
        currentRow,
      }: {
        open: boolean
        onOpenChange: () => void
        currentRow: TestRow | null
      }) => (
        open ? <div data-testid="mutate-dialog">{currentRow?.name ?? 'create'}</div> : null
      ),
      deleteDialog: {
        hardDeleteFn: vi.fn(async () => undefined),
        softDeleteFn: vi.fn(async () => undefined),
        queryKey: () => ['test-rows'],
        entityLabel: 'test.entity',
        i18nNamespaces: ['common'],
      },
    })

    render(<crudViewDefinition.View />)

    expect(screen.getByTestId('row-name')).toHaveTextContent('Alpha')
    expect(screen.getByTestId('header')).toBeInTheDocument()
    expect(screen.getByTestId('main')).toBeInTheDocument()

    await user.click(screen.getByRole('button', { name: 'common:actions.create' }))
    expect(screen.getByTestId('mutate-dialog')).toHaveTextContent('create')

    await user.click(screen.getByRole('button', { name: 'common:actions.edit' }))
    expect(screen.getByTestId('mutate-dialog')).toHaveTextContent('Alpha')

    await user.click(screen.getByRole('button', { name: 'common:actions.softDelete' }))
    expect(screen.getByTestId('delete-dialog')).toHaveTextContent('soft:Alpha')
  })
})
