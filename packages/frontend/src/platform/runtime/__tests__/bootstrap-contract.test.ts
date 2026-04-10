export {} // module boundary for top-level await

vi.mock('~/platform/runtime/api-base-url', () => ({
  setApiBaseUrl: vi.fn(),
}))

vi.mock('../health', () => ({
  fetchHealth: vi.fn(),
  applyHealthSnapshot: vi.fn(),
}))

const { useAuthStore } = await import('~/stores/auth-store')
const { useStartupStore } = await import('~/stores/startup-store')
const { ensureBootstrapped, resetRuntimeBootstrap } = await import('../bootstrap')
const { useRuntimeStore } = await import('../runtime-store')

beforeEach(() => {
  vi.clearAllMocks()
  resetRuntimeBootstrap()
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

describe('bootstrap contract', () => {
  it('is idempotent once the runtime is already ready', async () => {
    const refresh = vi.fn(async () => {})
    const boot = vi.fn(async () => {})

    useRuntimeStore.setState({
      bootstrapStatus: 'ready',
      bootstrapError: null,
      healthStatus: 'hydrated',
    })
    useStartupStore.setState({ refresh })
    useAuthStore.setState({ boot })

    await ensureBootstrapped()

    expect(refresh).not.toHaveBeenCalled()
    expect(boot).not.toHaveBeenCalled()
  })
})
