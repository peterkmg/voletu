import { screen, waitFor } from '@testing-library/react'

export {}

const ensureBootstrapped = vi.fn(async () => {})

vi.mock('~/platform/runtime/bootstrap', () => ({
  ensureBootstrapped,
}))

vi.mock('~/components/layout/authenticated-layout', async () => {
  const { Outlet } = await import('@tanstack/react-router')
  return {
    AuthenticatedLayout: () => <Outlet />,
  }
})

vi.mock('~/views/auth/forms/sign-in-form', () => ({
  SignInForm: () => <div>sign-in-view</div>,
}))

vi.mock('~/views/system/init', () => ({
  InitializePage: () => <div>init-view</div>,
}))

const { renderFileRoute } = await import('~/router/testing/file-route-test-utils')
const { useAuthStore } = await import('~/stores/auth-store')
const { useNodeStore } = await import('~/stores/node-store')
const { useRuntimeStore } = await import('~/platform/runtime/runtime-store')
const { useStartupStore } = await import('~/stores/startup-store')

beforeEach(() => {
  vi.clearAllMocks()
  useNodeStore.getState().reset()
  useRuntimeStore.setState({
    bootstrapStatus: 'idle',
    bootstrapError: null,
    healthStatus: 'idle',
  })
  useStartupStore.setState({
    startupState: null,
    isLoading: false,
    error: null,
    refresh: async () => {},
  })
  useAuthStore.setState({
    status: 'unknown',
    accessToken: null,
    refreshToken: null,
    user: null,
    boot: async () => {},
    onUnauthorized: async () => false,
    login: () => {},
    logout: () => {},
  })
})

describe('authenticated route flows', () => {
  it('redirects unauthenticated users to sign-in through the real route tree', async () => {
    useAuthStore.setState({
      status: 'unauthenticated',
      accessToken: null,
      refreshToken: null,
      user: null,
    })

    const { router } = renderFileRoute('/system/users')

    await waitFor(() => {
      expect(router.state.location.pathname).toBe('/sign-in')
    })
    expect(router.state.location.search.redirect).toContain('/system/users')
    expect(screen.getByText('sign-in-view')).toBeInTheDocument()
    expect(ensureBootstrapped).toHaveBeenCalled()
  })

  it('redirects to setup when startup state requires configuration', async () => {
    useStartupStore.setState({
      startupState: {
        needsSetup: true,
        mode: 'remote',
        apiBaseUrl: 'http://configured-api:3000',
        isDebugBuild: false,
      },
    })

    const { router } = renderFileRoute('/system/users')

    await waitFor(() => {
      expect(router.state.location.pathname).toBe('/setup')
    })
  })

  it('redirects admins to init when the node is not initialized but runtime health is hydrated', async () => {
    useAuthStore.setState({
      status: 'valid',
      accessToken: 'access-token',
      refreshToken: 'refresh-token',
      user: {
        id: 'u1',
        username: 'admin',
        displayName: 'Admin',
        role: 'ADMIN',
      } as never,
    })
    useRuntimeStore.setState({
      bootstrapStatus: 'ready',
      bootstrapError: null,
      healthStatus: 'hydrated',
    })
    useNodeStore.getState().setStatus({ isInitialized: false })

    const { router } = renderFileRoute('/system/users')

    await waitFor(() => {
      expect(router.state.location.pathname).toBe('/init')
    })
    expect(screen.getByText('init-view')).toBeInTheDocument()
  })
})
