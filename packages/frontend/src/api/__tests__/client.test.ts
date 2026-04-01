import { client, getBaseUrl, setApiBaseUrl } from '~/api/client'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'

// ---------------------------------------------------------------------------
// Mocks
// ---------------------------------------------------------------------------

vi.mock('~/auth/session', () => ({
  isTokenExpiringSoon: vi.fn(() => false),
}))

const { isTokenExpiringSoon } = await import('~/auth/session') as any

const fetchSpy = vi.spyOn(globalThis, 'fetch')

beforeEach(() => {
  vi.clearAllMocks()
  useAuthStore.setState({ status: 'valid', accessToken: 'test-token', refreshToken: 'rt', user: null })
  useNodeStore.getState().reset()
  setApiBaseUrl('http://localhost:3000')
})

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function jsonResponse(body: object, status = 200) {
  return new Response(JSON.stringify(body), {
    status,
    statusText: status === 200 ? 'OK' : 'Error',
    headers: { 'Content-Type': 'application/json' },
  })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe('client()', () => {
  it('attaches Authorization header when token exists', async () => {
    fetchSpy.mockResolvedValue(jsonResponse({ success: true, data: {} }))

    await client({ method: 'GET', url: '/test' })

    const headers = fetchSpy.mock.calls[0]![1]!.headers as Record<string, string>
    expect(headers.Authorization).toBe('Bearer test-token')
  })

  it('omits Authorization header when no token', async () => {
    useAuthStore.setState({ accessToken: null })
    fetchSpy.mockResolvedValue(jsonResponse({ success: true, data: {} }))

    await client({ method: 'GET', url: '/test' })

    const headers = fetchSpy.mock.calls[0]![1]!.headers as Record<string, string>
    expect(headers.Authorization).toBeUndefined()
  })

  it('adds Idempotency-Key header for mutating requests', async () => {
    fetchSpy.mockResolvedValue(jsonResponse({ success: true, data: {} }))

    await client({ method: 'POST', url: '/test', data: {} })

    const headers = fetchSpy.mock.calls[0]![1]!.headers as Record<string, string>
    expect(headers['Idempotency-Key']).toBeDefined()
    expect(headers['Idempotency-Key']).toMatch(
      /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/,
    )
  })

  it('does not add Idempotency-Key for GET requests', async () => {
    fetchSpy.mockResolvedValue(jsonResponse({ success: true, data: {} }))

    await client({ method: 'GET', url: '/test' })

    const headers = fetchSpy.mock.calls[0]![1]!.headers as Record<string, string>
    expect(headers['Idempotency-Key']).toBeUndefined()
  })

  it('parses success envelope correctly', async () => {
    const payload = { success: true, data: { id: '1', name: 'test' } }
    fetchSpy.mockResolvedValue(jsonResponse(payload))

    const result = await client({ method: 'GET', url: '/test' })

    expect(result.data).toEqual(payload)
    expect(result.status).toBe(200)
  })

  it('throws on error envelope', async () => {
    const payload = { success: false, error: { message: 'Bad input' } }
    fetchSpy.mockResolvedValue(jsonResponse(payload))

    await expect(client({ method: 'GET', url: '/test' }))
      .rejects.toThrow('Bad input')
  })

  it('handles 204 No Content', async () => {
    fetchSpy.mockResolvedValue(new Response(null, { status: 204, statusText: 'No Content' }))

    const result = await client({ method: 'DELETE', url: '/test' })

    expect(result.status).toBe(204)
    expect(result.data).toBeUndefined()
  })

  it('throws on non-ok response', async () => {
    fetchSpy.mockResolvedValue(
      new Response('Internal Server Error', { status: 500, statusText: 'ISE' }),
    )

    await expect(client({ method: 'GET', url: '/test' }))
      .rejects.toThrow('Internal Server Error')
  })

  // ---------------------------------------------------------------------------
  // 401 + refresh
  // ---------------------------------------------------------------------------

  it('on 401: calls onUnauthorized and replays if refresh succeeds', async () => {
    const onUnauthorized = vi.fn().mockResolvedValue(true)
    useAuthStore.setState({ onUnauthorized } as any)

    fetchSpy
      .mockResolvedValueOnce(new Response('Unauthorized', { status: 401 }))
      .mockResolvedValueOnce(jsonResponse({ success: true, data: {} }))

    const result = await client({ method: 'GET', url: '/test' })

    expect(onUnauthorized).toHaveBeenCalledTimes(1)
    expect(fetchSpy).toHaveBeenCalledTimes(2)
    expect(result.data).toEqual({ success: true, data: {} })
  })

  it('on 401: throws if onUnauthorized returns false', async () => {
    const onUnauthorized = vi.fn().mockResolvedValue(false)
    useAuthStore.setState({ onUnauthorized } as any)

    fetchSpy.mockResolvedValue(new Response('Unauthorized', { status: 401 }))

    await expect(client({ method: 'GET', url: '/test' }))
      .rejects.toThrow('Session expired')
  })

  // ---------------------------------------------------------------------------
  // Proactive refresh
  // ---------------------------------------------------------------------------

  it('proactively refreshes when token is near expiry', async () => {
    vi.mocked(isTokenExpiringSoon).mockReturnValue(true)
    const onUnauthorized = vi.fn().mockResolvedValue(true)
    useAuthStore.setState({ onUnauthorized } as any)
    fetchSpy.mockResolvedValue(jsonResponse({ success: true, data: {} }))

    await client({ method: 'GET', url: '/test' })

    expect(onUnauthorized).toHaveBeenCalledTimes(1)
  })

  // ---------------------------------------------------------------------------
  // 403 NODE_NOT_INITIALIZED
  // ---------------------------------------------------------------------------

  it('on 403 NODE_NOT_INITIALIZED: sets node store to uninitialized', async () => {
    useNodeStore.getState().setStatus({ isInitialized: true })
    fetchSpy.mockResolvedValue(
      new Response(JSON.stringify({ error: { code: 'NODE_NOT_INITIALIZED' } }), {
        status: 403,
        statusText: 'Forbidden',
      }),
    )

    await expect(client({ method: 'GET', url: '/test' })).rejects.toThrow()

    expect(useNodeStore.getState().status.isInitialized).toBe(false)
  })

  // ---------------------------------------------------------------------------
  // Base URL
  // ---------------------------------------------------------------------------

  it('uses configured base URL', async () => {
    setApiBaseUrl('http://custom:8080/')
    fetchSpy.mockResolvedValue(jsonResponse({ success: true, data: {} }))

    await client({ method: 'GET', url: '/api/test' })

    expect(fetchSpy.mock.calls[0]![0]).toBe('http://custom:8080/api/test')
  })

  it('getBaseUrl returns default when not configured', () => {
    delete (globalThis as any).__VOLETU_API_BASE_URL__
    expect(getBaseUrl()).toBe('http://127.0.0.1:3000')
  })
})
