export {}

const fetchMock = vi.fn()
const freshJwt = `x.${btoa(JSON.stringify({ exp: Math.floor(Date.now() / 1000) + 3600 }))}.y`
const refreshedJwt = `x.${btoa(JSON.stringify({ exp: Math.floor(Date.now() / 1000) + 7200 }))}.y`

vi.stubGlobal('fetch', fetchMock)

vi.mock('~/platform/runtime/api-base-url', () => ({
  getApiBaseUrl: vi.fn(() => 'http://runtime-api:3000'),
}))

const { useNodeStore } = await import('~/stores/node-store')
const { useAuthStore } = await import('~/stores/auth-store')
const { applyHealthSnapshot, fetchHealth, fetchNodeStatus } = await import('~/platform/runtime/health')

beforeEach(() => {
  vi.clearAllMocks()
  useNodeStore.getState().reset()
  useAuthStore.setState({
    status: 'valid',
    accessToken: freshJwt,
    refreshToken: 'refresh-token',
    user: null,
  })
})

afterAll(() => {
  vi.unstubAllGlobals()
})

describe('health runtime helpers', () => {
  it('fetches health and hydrates initialization fields', async () => {
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
        {
          status: 200,
          headers: { 'Content-Type': 'application/json' },
        },
      ),
    )

    const health = await fetchHealth()
    applyHealthSnapshot(health)

    expect(fetchMock).toHaveBeenCalledTimes(1)
    expect(fetchMock.mock.calls[0]?.[0]).toBe('http://runtime-api:3000/health')
    expect(health).toEqual({
      status: 'ok',
      isInitialized: true,
      nodeType: 'CENTRAL',
      nodeName: 'HQ',
    })
    expect(useNodeStore.getState().status).toMatchObject({
      isInitialized: true,
      nodeType: 'CENTRAL',
      nodeName: 'HQ',
    })
  })

  it('refreshes and replays node status requests after 401', async () => {
    const onUnauthorized = vi.fn(async () => {
      useAuthStore.setState({ accessToken: refreshedJwt })
      return true
    })
    useAuthStore.setState({ onUnauthorized } as any)
    fetchMock
      .mockResolvedValueOnce(new Response('Unauthorized', { status: 401 }))
      .mockResolvedValueOnce(
        new Response(
          JSON.stringify({
            success: true,
            data: {
              isInitialized: true,
              nodeType: 'PERIPHERAL',
              nodeName: 'Tank 1',
              workerState: 'OnlineIdle',
              lastSyncAt: null,
              centralApiUrl: null,
              assignedBaseIds: ['base-1'],
            },
          }),
          { status: 200, headers: { 'Content-Type': 'application/json' } },
        ),
      )

    const status = await fetchNodeStatus()

    expect(onUnauthorized).toHaveBeenCalledTimes(1)
    expect(fetchMock).toHaveBeenCalledTimes(2)
    expect((fetchMock.mock.calls[0]?.[1]?.headers as Headers).get('Authorization')).toBe(`Bearer ${freshJwt}`)
    expect((fetchMock.mock.calls[1]?.[1]?.headers as Headers).get('Authorization')).toBe(`Bearer ${refreshedJwt}`)
    expect(status.nodeName).toBe('Tank 1')
  })
})
