import { render } from '@testing-library/react'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { AuthenticatedLayout } from '~/components/layout/authenticated-layout'

const useHealthCheckMock = vi.fn()
const useNodeStatusMock = vi.fn()
const navigateMock = vi.fn()

vi.mock('@tanstack/react-router', () => ({
  Outlet: () => <div data-testid="outlet" />,
  useNavigate: () => navigateMock,
}))

vi.mock('~/hooks/use-node-status', () => ({
  useHealthCheck: () => useHealthCheckMock(),
  useNodeStatus: () => useNodeStatusMock(),
}))

vi.mock('~/components/ui/sidebar', () => ({
  SidebarInset: ({ children }: { children: React.ReactNode }) => <main>{children}</main>,
  SidebarProvider: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
}))

vi.mock('~/components/layout/app-sidebar', () => ({
  AppSidebar: () => <aside data-testid="sidebar" />,
}))

describe('authenticatedLayout', () => {
  beforeEach(() => {
    localStorage.clear()
    vi.clearAllMocks()
  })

  it('starts both health and node-status polling for the authenticated shell', () => {
    render(<AuthenticatedLayout />)

    expect(useHealthCheckMock).toHaveBeenCalledTimes(1)
    expect(useNodeStatusMock).toHaveBeenCalledTimes(1)
  })
})
