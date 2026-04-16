import type { StartupState } from '~/tauri/commands'

export {} // module boundary for top-level await

vi.mock('~/platform/runtime/api-base-url', () => ({
  setApiBaseUrl: vi.fn(),
}))

vi.mock('../health', () => ({
  fetchHealth: vi.fn(),
  applyHealthSnapshot: vi.fn(),
}))

const runtimeApi = await import('~/platform/runtime/api-base-url') as unknown as {
  setApiBaseUrl: ReturnType<typeof vi.fn>
}
const health = await import('../health') as unknown as {
  fetchHealth: ReturnType<typeof vi.fn>
  applyHealthSnapshot: ReturnType<typeof vi.fn>
}
const { useAuthStore } = await import('~/stores/auth-store')
const { useStartupStore } = await import('~/stores/startup-store')
const { ensureBootstrapped, resetRuntimeBootstrap } = await import('../bootstrap')
const { useRuntimeStore } = await import('../runtime-store')

function deferred<T>() {
  let resolve!: (value: T | PromiseLike<T>) => void
  let reject!: (reason?: unknown) => void
  const promise = new Promise<T>((res, rej) => {
    resolve = res
    reject = rej
  })
  return { promise, resolve, reject }
}

const startupState: StartupState = {
  needsSetup: false,
  mode: 'remote',
  apiBaseUrl: 'http://configured-api:3000',
  isDebugBuild: false,
}

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
  health.fetchHealth.mockResolvedValue({
    status: 'ok',
    isInitialized: true,
    nodeType: 'CENTRAL',
    nodeName: 'HQ',
  })
})

describe('ensureBootstrapped()', () => {
  it('transitions idle -> booting -> ready', async () => {
    const gate = deferred<void>()
    const refresh = vi.fn(async () => {
      await gate.promise
      useStartupStore.setState({ startupState })
    })
    const boot = vi.fn(async () => {
      useAuthStore.setState({ status: 'valid' })
    })

    useStartupStore.setState({ refresh })
    useAuthStore.setState({ status: 'unknown', boot })

    const promise = ensureBootstrapped()

    expect(useRuntimeStore.getState().bootstrapStatus).toBe('booting')

    gate.resolve()
    await promise

    expect(refresh).toHaveBeenCalledTimes(1)
    expect(runtimeApi.setApiBaseUrl).toHaveBeenCalledWith('http://configured-api:3000')
    expect(boot).toHaveBeenCalledTimes(1)
    expect(health.fetchHealth).toHaveBeenCalledTimes(1)
    expect(health.applyHealthSnapshot).toHaveBeenCalledWith({
      status: 'ok',
      isInitialized: true,
      nodeType: 'CENTRAL',
      nodeName: 'HQ',
    })
    expect(useRuntimeStore.getState()).toMatchObject({
      bootstrapStatus: 'ready',
      bootstrapError: null,
      healthStatus: 'hydrated',
    })
  })

  it('surfaces bootstrap failure as error state', async () => {
    const boot = vi.fn(async () => {
      throw new Error('boot failed')
    })

    useStartupStore.setState({
      startupState,
      refresh: vi.fn(async () => {}),
    })
    useAuthStore.setState({ status: 'unknown', boot })

    await expect(ensureBootstrapped()).rejects.toThrow('boot failed')

    expect(useRuntimeStore.getState()).toMatchObject({
      bootstrapStatus: 'error',
      bootstrapError: 'boot failed',
    })
  })

  it('does not run twice concurrently', async () => {
    const gate = deferred<void>()
    const refresh = vi.fn(async () => {
      await gate.promise
      useStartupStore.setState({ startupState })
    })
    const boot = vi.fn(async () => {
      useAuthStore.setState({ status: 'valid' })
    })

    useStartupStore.setState({ refresh })
    useAuthStore.setState({ status: 'unknown', boot })

    const first = ensureBootstrapped()
    const second = ensureBootstrapped()

    expect(refresh).toHaveBeenCalledTimes(1)

    gate.resolve()
    await Promise.all([first, second])

    expect(boot).toHaveBeenCalledTimes(1)
    expect(health.fetchHealth).toHaveBeenCalledTimes(1)
  })

  it('fetches health when startupState is null (browser / Docker mode)', async () => {
    const boot = vi.fn(async () => {
      useAuthStore.setState({ status: 'valid' })
    })

    // startupState stays null — simulates Tauri IPC unavailable
    useStartupStore.setState({ refresh: vi.fn(async () => {}) })
    useAuthStore.setState({ status: 'unknown', boot })

    await ensureBootstrapped()

    expect(health.fetchHealth).toHaveBeenCalledTimes(1)
    expect(health.applyHealthSnapshot).toHaveBeenCalled()
    expect(useRuntimeStore.getState()).toMatchObject({
      bootstrapStatus: 'ready',
      healthStatus: 'hydrated',
    })
  })

  it('skips health fetch when needsSetup is true', async () => {
    const setupState: StartupState = { ...startupState, needsSetup: true }
    const boot = vi.fn(async () => {
      useAuthStore.setState({ status: 'valid' })
    })

    useStartupStore.setState({
      startupState: setupState,
      refresh: vi.fn(async () => {}),
    })
    useAuthStore.setState({ status: 'unknown', boot })

    await ensureBootstrapped()

    expect(health.fetchHealth).not.toHaveBeenCalled()
    expect(useRuntimeStore.getState()).toMatchObject({
      bootstrapStatus: 'ready',
      healthStatus: 'idle',
    })
  })

  it('treats health probe failures as unavailable runtime state instead of fatal bootstrap errors', async () => {
    const boot = vi.fn(async () => {
      useAuthStore.setState({ status: 'valid' })
    })

    useStartupStore.setState({
      startupState,
      refresh: vi.fn(async () => {}),
    })
    useAuthStore.setState({ status: 'unknown', boot })
    health.fetchHealth.mockRejectedValue(new Error('api restarting'))

    await expect(ensureBootstrapped()).resolves.toBeUndefined()

    expect(useRuntimeStore.getState()).toMatchObject({
      bootstrapStatus: 'ready',
      bootstrapError: null,
      healthStatus: 'unavailable',
    })
    expect(health.applyHealthSnapshot).not.toHaveBeenCalled()
  })
})
