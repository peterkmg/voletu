import type { Row } from '@tanstack/react-table'
import type { ReactNode } from 'react'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, expect, it, vi } from 'vitest'
import { TooltipProvider } from '~/components/ui/tooltip'
import { createRowActions } from '~/lib/create-row-actions'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}))

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}))

vi.mock('~/stores/auth-store', () => ({
  useAuthStore: (selector: (state: { user: { role: string } | null }) => unknown) =>
    selector({ user: { role: 'OPERATOR' } }),
}))

interface PipelineRow {
  id: string
  pipelineStatus: 'PENDING' | 'DRAFT' | 'EXECUTED'
}

function makeRow(row: PipelineRow): Row<PipelineRow> {
  return { original: row } as unknown as Row<PipelineRow>
}

function Wrapper({ children }: { children: ReactNode }) {
  return <TooltipProvider>{children}</TooltipProvider>
}

function setup(row: PipelineRow) {
  const openUpdate = vi.fn()
  const openDelete = vi.fn()
  const openLifecycle = vi.fn()
  const openIssueAcceptance = vi.fn()
  const Component = createRowActions<PipelineRow>({
    useEntity: () => ({ openUpdate, openDelete, openLifecycle, openIssueAcceptance }),
    getDetailPath: r => `/incoming/truck/${r.id}`,
    pipelineActions: {
      editVisible: r => r.pipelineStatus === 'PENDING',
      issueAcceptance: { visible: r => r.pipelineStatus === 'PENDING' },
    },
  })
  render(<Component row={makeRow(row)} />, { wrapper: Wrapper })
  return { openUpdate, openDelete, openLifecycle, openIssueAcceptance }
}

describe('createRowActions — pipelineActions config', () => {
  it('renders inline Edit and Issue acceptance buttons for a PENDING row', () => {
    setup({ id: 'wb-1', pipelineStatus: 'PENDING' })
    expect(screen.getByRole('button', { name: /common:actions\.edit/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /documents:actions\.issueAcceptance/i })).toBeInTheDocument()
  })

  it('hides Edit and Issue acceptance for a DRAFT row', () => {
    setup({ id: 'wb-2', pipelineStatus: 'DRAFT' })
    expect(screen.queryByRole('button', { name: /common:actions\.edit/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /documents:actions\.issueAcceptance/i })).not.toBeInTheDocument()
  })

  it('hides Edit and Issue acceptance for an EXECUTED row', () => {
    setup({ id: 'wb-3', pipelineStatus: 'EXECUTED' })
    expect(screen.queryByRole('button', { name: /common:actions\.edit/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: /documents:actions\.issueAcceptance/i })).not.toBeInTheDocument()
  })

  it('clicking Issue acceptance dispatches openIssueAcceptance with the row', async () => {
    const { openIssueAcceptance } = setup({ id: 'wb-4', pipelineStatus: 'PENDING' })
    await userEvent.click(screen.getByRole('button', { name: /documents:actions\.issueAcceptance/i }))
    expect(openIssueAcceptance).toHaveBeenCalledWith(
      { id: 'wb-4', pipelineStatus: 'PENDING' },
    )
  })

  it('clicking Edit dispatches openUpdate', async () => {
    const { openUpdate } = setup({ id: 'wb-5', pipelineStatus: 'PENDING' })
    await userEvent.click(screen.getByRole('button', { name: /common:actions\.edit/i }))
    expect(openUpdate).toHaveBeenCalledWith({ id: 'wb-5', pipelineStatus: 'PENDING' })
  })
})
