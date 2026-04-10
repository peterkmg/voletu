import type { StartupState } from '~/tauri/commands'

export {} // module boundary for top-level await

const ensureBootstrapped = vi.fn(async () => {})
const fetchMock = vi.fn()

vi.mock('~/platform/runtime/bootstrap', () => ({
  ensureBootstrapped,
}))

vi.stubGlobal('fetch', fetchMock)

const { Route: RootRoute } = await import('~/routes/__root')
const { Route: AuthenticatedRoute } = await import('~/routes/_authenticated/route')
const { useAuthStore } = await import('~/stores/auth-store')
const { useNodeStore } = await import('~/stores/node-store')
const { useRuntimeStore } = await import('~/platform/runtime/runtime-store')
const { useStartupStore } = await import('~/stores/startup-store')

const configuredStartupState: StartupState = {
  needsSetup: false,
  mode: 'remote',
  apiBaseUrl: 'http://configured-api:3000',
  isDebugBuild: false,
}

function beforeLoad(route: any, args: Record<string, unknown> = {}) {
  const handler = route.options.beforeLoad
  if (!handler) {
    throw new Error('Route is missing beforeLoad')
  }

  return Promise.resolve(
    handler({
      abortController: new AbortController(),
      buildLocation: vi.fn(),
      cause: 'enter',
      context: {},
      location: { href: '/dashboard' },
      navigate: vi.fn(),
      params: {},
      preload: false,
      search: {},
      ...args,
    }),
  )
}

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

afterAll(() => {
  vi.unstubAllGlobals()
})

describe('route bootstrap integration', () => {
  it('root route boot runs through the bootstrap orchestrator', async () => {
    await beforeLoad(RootRoute)

    expect(ensureBootstrapped).toHaveBeenCalledTimes(1)
  })

  it('authenticated route redirects from prepared node state without raw health fetches', async () => {
    useStartupStore.setState({
      startupState: configuredStartupState,
    })
    useAuthStore.setState({
      status: 'valid',
      accessToken: 'access-token',
      refreshToken: 'refresh-token',
      user: {
        id: 'u1',
        username: 'admin',
        displayName: 'Admin',
        role: 'ADMIN',
      } as any,
    })
    useRuntimeStore.setState({
      bootstrapStatus: 'ready',
      bootstrapError: null,
      healthStatus: 'hydrated',
    })
    useNodeStore.getState().setStatus({ isInitialized: false })

    await expect(beforeLoad(AuthenticatedRoute)).rejects.toMatchObject({
      options: { to: '/init' },
    })
    expect(fetchMock).not.toHaveBeenCalled()
  })

  it('authenticated route does not redirect when bootstrap could not confirm health', async () => {
    useStartupStore.setState({
      startupState: configuredStartupState,
    })
    useAuthStore.setState({
      status: 'valid',
      accessToken: 'access-token',
      refreshToken: 'refresh-token',
      user: {
        id: 'u1',
        username: 'admin',
        displayName: 'Admin',
        role: 'ADMIN',
      } as any,
    })
    useRuntimeStore.setState({
      bootstrapStatus: 'ready',
      bootstrapError: null,
      healthStatus: 'unavailable',
    })
    useNodeStore.getState().setStatus({ isInitialized: false })

    await expect(beforeLoad(AuthenticatedRoute)).resolves.toBeUndefined()
    expect(fetchMock).not.toHaveBeenCalled()
  })
})
