export {} // module boundary for top-level await

const fetchMock = vi.fn()

vi.stubGlobal('fetch', fetchMock)

vi.mock('~/platform/runtime/api-base-url', () => ({
  getApiBaseUrl: vi.fn(() => 'http://runtime-api:3000'),
}))

const { useNodeStore } = await import('~/stores/node-store')
const { applyHealthSnapshot, fetchHealth } = await import('../health')

beforeEach(() => {
  vi.clearAllMocks()
  useNodeStore.getState().reset()
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
})
