import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { describe, expect, it, vi } from 'vitest'
import { DocumentHeader } from '~/components/document/document-header'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}))

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}))

// Supervisor role id taken from src/lib/rbac.ts so isSupervisorOrHigher() returns true.
const SUPERVISOR_ROLE_ID = '019c8cc4-9048-7b61-9443-52858a953a17'

vi.mock('~/stores/auth-store', () => ({
  useAuthStore: Object.assign(
    (selector: (state: { user: { role: string } | null }) => unknown) =>
      selector({ user: { role: SUPERVISOR_ROLE_ID } }),
    { getState: () => ({ user: { role: SUPERVISOR_ROLE_ID } }) },
  ),
}))

describe('documentHeader actions prop', () => {
  const baseProps = {
    title: 'Truck waybill',
    documentNumber: 'WB-2412-09',
    status: 'PENDING',
    backTo: '/incoming/truck',
    entityLabel: 'truck waybill',
    documentId: 'doc-123',
  }

  it('renders each action as a button with its label', () => {
    const onEdit = vi.fn()
    const onIssue = vi.fn()
    render(
      <DocumentHeader
        {...baseProps}
        actions={[
          { label: 'Edit', onClick: onEdit },
          { label: 'Issue acceptance', onClick: onIssue, variant: 'primary' },
        ]}
      />,
    )
    expect(screen.getByRole('button', { name: 'Edit' })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: 'Issue acceptance' })).toBeInTheDocument()
  })

  it('disables an action when disabled is true and surfaces tooltip via title', async () => {
    const onClick = vi.fn()
    render(
      <DocumentHeader
        {...baseProps}
        actions={[
          { label: 'Edit', onClick, disabled: true, disabledReason: 'Locked: acceptance executed' },
        ]}
      />,
    )
    const btn = screen.getByRole('button', { name: 'Edit' })
    expect(btn).toBeDisabled()
    expect(btn).toHaveAttribute('title', 'Locked: acceptance executed')
    await userEvent.click(btn)
    expect(onClick).not.toHaveBeenCalled()
  })

  it('still supports legacy executeFn / revertFn props (existing behavior preserved)', () => {
    const executeFn = vi.fn()
    const revertFn = vi.fn()
    render(
      <DocumentHeader
        {...baseProps}
        status="DRAFT"
        executeFn={executeFn}
        revertFn={revertFn}
        queryKey={[{ url: '/x' }]}
      />,
    )
    expect(screen.getByRole('button', { name: /lifecycle\.execute/i })).toBeInTheDocument()
  })
})
