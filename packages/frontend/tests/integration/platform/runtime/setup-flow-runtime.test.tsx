import type { ReactNode } from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { act, renderHook, waitFor } from '@testing-library/react'

export {}

const fetchMock = vi.fn()

vi.mock('~/tauri/commands', () => ({
  saveRemoteConfig: vi.fn(),
  saveLocalConfig: vi.fn(),
  startLocalApi: vi.fn(),
}))

vi.mock('~/platform/runtime/health', () => ({
  fetchHealth: vi.fn(),
  applyHealthSnapshot: vi.fn(),
  fetchNodeStatus: vi.fn(),
  applyNodeStatusSnapshot: vi.fn(),
  waitForApiHealthy: vi.fn(),
}))

vi.stubGlobal('fetch', fetchMock)

const runtimeHealth = await import('~/platform/runtime/health') as unknown as {
  fetchHealth: ReturnType<typeof vi.fn>
  applyHealthSnapshot: ReturnType<typeof vi.fn>
  fetchNodeStatus: ReturnType<typeof vi.fn>
  applyNodeStatusSnapshot: ReturnType<typeof vi.fn>
  waitForApiHealthy: ReturnType<typeof vi.fn>
}
const tauriCommands = await import('~/tauri/commands') as unknown as {
  saveRemoteConfig: ReturnType<typeof vi.fn>
  saveLocalConfig: ReturnType<typeof vi.fn>
  startLocalApi: ReturnType<typeof vi.fn>
}
const { useSetupFlow } = await import('~/views/setup/hooks/use-setup-flow')
const { useHealthCheck, useNodeStatus } = await import('~/hooks/use-node-status')
const { getApiBaseUrl, setApiBaseUrl } = await import('~/platform/runtime/api-base-url')
const { useAuthStore } = await import('~/stores/auth-store')
const { useNodeStore } = await import('~/stores/node-store')
const { useStartupStore } = await import('~/stores/startup-store')

function deferred<T>() {
  let resolve!: (value: T | PromiseLike<T>) => void
  let reject!: (reason?: unknown) => void
  const promise = new Promise<T>((res, rej) => {
    resolve = res
    reject = rej
  })
  return { promise, resolve, reject }
}

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        gcTime: 0,
        retry: false,
      },
    },
  })

  return function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    )
  }
}

beforeEach(() => {
  vi.clearAllMocks()
  setApiBaseUrl('http://existing-api:3000')
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
  useNodeStore.getState().reset()
})

afterAll(() => {
  vi.unstubAllGlobals()
})

describe('setup flow runtime integration', () => {
  it('invalid remote URL does not become the active runtime base URL', async () => {
    fetchMock.mockRejectedValue(new Error('offline'))
    runtimeHealth.fetchHealth.mockRejectedValue(new Error('offline'))
    const { result } = renderHook(() => useSetupFlow())

    await act(async () => {
      await expect(
        result.current.submitRemoteConfig('http://broken-api:4000'),
      ).rejects.toThrow()
    })

    expect(getApiBaseUrl()).toBe('http://existing-api:3000')
    expect(tauriCommands.saveRemoteConfig).not.toHaveBeenCalled()
  })

  it('valid remote setup activates the runtime base URL only after persistence succeeds', async () => {
    fetchMock.mockResolvedValue(
      new Response(JSON.stringify({ success: true }), { status: 200 }),
    )
    runtimeHealth.fetchHealth.mockResolvedValue({
      status: 'ok',
      isInitialized: true,
      nodeType: 'CENTRAL',
      nodeName: 'HQ',
    })
    const gate = deferred<{
      needsSetup: boolean
      mode: 'remote'
      apiBaseUrl: string
      isDebugBuild: boolean
    }>()
    tauriCommands.saveRemoteConfig.mockReturnValue(gate.promise)

    const { result } = renderHook(() => useSetupFlow())

    let submitPromise!: Promise<void>
    await act(async () => {
      submitPromise = result.current.submitRemoteConfig('http://new-api:4000')
    })

    await waitFor(() => {
      expect(tauriCommands.saveRemoteConfig).toHaveBeenCalledWith({
        remoteApiUrl: 'http://new-api:4000',
      })
    })
    expect(getApiBaseUrl()).toBe('http://existing-api:3000')

    await act(async () => {
      gate.resolve({
        needsSetup: false,
        mode: 'remote',
        apiBaseUrl: 'http://new-api:4000',
        isDebugBuild: false,
      })
      await submitPromise
    })

    expect(getApiBaseUrl()).toBe('http://new-api:4000')
  })
})

describe('node runtime hooks', () => {
  it('useHealthCheck delegates health polling to the shared runtime helper', async () => {
    fetchMock.mockResolvedValue(
      new Response(
        JSON.stringify({
          success: true,
          data: {
            status: 'ok',
            isInitialized: true,
            nodeType: 'CENTRAL',
            nodeName: 'HQ',
          },
        }),
        { status: 200, headers: { 'Content-Type': 'application/json' } },
      ),
    )
    runtimeHealth.fetchHealth.mockResolvedValue({
      status: 'ok',
      isInitialized: true,
      nodeType: 'CENTRAL',
      nodeName: 'HQ',
    })
    useAuthStore.setState({ status: 'valid', accessToken: 'access-token' })

    renderHook(() => useHealthCheck(), { wrapper: createWrapper() })

    await waitFor(() => {
      expect(runtimeHealth.fetchHealth).toHaveBeenCalledTimes(1)
    })
    await waitFor(() => {
      expect(runtimeHealth.applyHealthSnapshot).toHaveBeenCalledWith({
        status: 'ok',
        isInitialized: true,
        nodeType: 'CENTRAL',
        nodeName: 'HQ',
      })
    })
  })

  it('useNodeStatus delegates status polling to the shared runtime helper', async () => {
    fetchMock.mockResolvedValue(
      new Response(
        JSON.stringify({
          success: true,
          data: {
            isInitialized: true,
            nodeType: 'CENTRAL',
            nodeName: 'HQ',
            workerState: 'OnlineIdle',
            lastSyncAt: null,
            assignedBaseIds: [],
          },
        }),
        { status: 200, headers: { 'Content-Type': 'application/json' } },
      ),
    )
    runtimeHealth.fetchNodeStatus.mockResolvedValue({
      isInitialized: true,
      nodeType: 'CENTRAL',
      nodeName: 'HQ',
      workerState: 'OnlineIdle',
      lastSyncAt: null,
      assignedBaseIds: [],
    })
    useAuthStore.setState({ status: 'valid', accessToken: 'access-token' })
    useNodeStore.getState().setStatus({ isInitialized: true, nodeType: 'CENTRAL' })

    renderHook(() => useNodeStatus(), { wrapper: createWrapper() })

    await waitFor(() => {
      expect(runtimeHealth.fetchNodeStatus).toHaveBeenCalledTimes(1)
    })
    await waitFor(() => {
      expect(runtimeHealth.applyNodeStatusSnapshot).toHaveBeenCalledWith({
        isInitialized: true,
        nodeType: 'CENTRAL',
        nodeName: 'HQ',
        workerState: 'OnlineIdle',
        lastSyncAt: null,
        assignedBaseIds: [],
      })
    })
  })
})
