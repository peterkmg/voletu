export {}

vi.mock('~/auth/session', () => ({
  loadSession: vi.fn(() => null),
  saveSession: vi.fn(),
  clearSession: vi.fn(),
  verifyToken: vi.fn(),
  refreshTokens: vi.fn(),
  isTokenExpiringSoon: vi.fn(() => false),
  toSession: vi.fn(),
}))

const sessionMocks = await import('~/auth/session') as any
const { useAuthStore } = await import('~/stores/auth-store')

const fakeUser = { id: 'u1', username: 'admin', role: 'ADMIN', displayName: 'Admin' } as any

function storedSession(overrides: Record<string, unknown> = {}) {
  return { accessToken: 'old-access', refreshToken: 'old-refresh', user: { id: 'u1' }, ...overrides }
}

function refreshResult(overrides: Record<string, unknown> = {}) {
  return { accessToken: 'new-access', refreshToken: 'new-refresh', user: fakeUser, ...overrides }
}

beforeEach(() => {
  vi.clearAllMocks()
  useAuthStore.setState({ status: 'unknown', accessToken: null, refreshToken: null, user: null })
})

describe('boot()', () => {
  it('no stored tokens -> status becomes unauthenticated', async () => {
    vi.mocked(sessionMocks.loadSession).mockReturnValue(null)

    await useAuthStore.getState().boot()

    expect(useAuthStore.getState().status).toBe('unauthenticated')
  })

  it('stored tokens + verifyToken succeeds -> status becomes valid, user updated', async () => {
    vi.mocked(sessionMocks.loadSession).mockReturnValue(storedSession())
    vi.mocked(sessionMocks.verifyToken).mockResolvedValue(fakeUser)

    await useAuthStore.getState().boot()

    const state = useAuthStore.getState()
    expect(state.status).toBe('valid')
    expect(state.user).toEqual(fakeUser)
  })

  it('stored tokens + verifyToken fails + refreshTokens succeeds -> status valid with new tokens', async () => {
    vi.mocked(sessionMocks.loadSession).mockReturnValue(storedSession())
    vi.mocked(sessionMocks.verifyToken).mockRejectedValue(new Error('expired'))
    vi.mocked(sessionMocks.refreshTokens).mockResolvedValue(refreshResult())

    await useAuthStore.getState().boot()

    const state = useAuthStore.getState()
    expect(state.status).toBe('valid')
    expect(state.accessToken).toBe('new-access')
    expect(state.refreshToken).toBe('new-refresh')
    expect(state.user).toEqual(fakeUser)
  })

  it('stored tokens + verifyToken fails + refreshTokens fails -> status unauthenticated', async () => {
    vi.mocked(sessionMocks.loadSession).mockReturnValue(storedSession())
    vi.mocked(sessionMocks.verifyToken).mockRejectedValue(new Error('expired'))
    vi.mocked(sessionMocks.refreshTokens).mockRejectedValue(new Error('revoked'))

    await useAuthStore.getState().boot()

    const state = useAuthStore.getState()
    expect(state.status).toBe('unauthenticated')
    expect(state.accessToken).toBeNull()
    expect(state.refreshToken).toBeNull()
    expect(state.user).toBeNull()
  })

  it('stored tokens + verifyToken fails + no refresh token -> status unauthenticated', async () => {
    vi.mocked(sessionMocks.loadSession).mockReturnValue(storedSession({ refreshToken: null }))
    vi.mocked(sessionMocks.verifyToken).mockRejectedValue(new Error('expired'))

    await useAuthStore.getState().boot()

    expect(useAuthStore.getState().status).toBe('unauthenticated')
    expect(sessionMocks.refreshTokens).not.toHaveBeenCalled()
  })

  it('calls saveSession after successful refresh', async () => {
    vi.mocked(sessionMocks.loadSession).mockReturnValue(storedSession())
    vi.mocked(sessionMocks.verifyToken).mockRejectedValue(new Error('expired'))
    const session = refreshResult()
    vi.mocked(sessionMocks.refreshTokens).mockResolvedValue(session)

    await useAuthStore.getState().boot()

    expect(sessionMocks.saveSession).toHaveBeenCalledWith(session)
  })

  it('calls clearSession when ending in unauthenticated', async () => {
    vi.mocked(sessionMocks.loadSession).mockReturnValue(null)

    await useAuthStore.getState().boot()

    expect(sessionMocks.clearSession).toHaveBeenCalled()
  })

  it('transitions through validating state during verification', async () => {
    vi.mocked(sessionMocks.loadSession).mockReturnValue(storedSession())

    const statusLog: string[] = []
    const unsub = useAuthStore.subscribe(s => statusLog.push(s.status))

    vi.mocked(sessionMocks.verifyToken).mockResolvedValue(fakeUser)

    await useAuthStore.getState().boot()
    unsub()

    expect(statusLog).toContain('validating')
    expect(statusLog).toContain('valid')
  })
})

describe('onUnauthorized()', () => {
  it('refreshTokens succeeds -> status valid, returns true', async () => {
    useAuthStore.setState({ status: 'valid', refreshToken: 'rt' })
    vi.mocked(sessionMocks.refreshTokens).mockResolvedValue(refreshResult())

    const result = await useAuthStore.getState().onUnauthorized()

    expect(result).toBe(true)
    expect(useAuthStore.getState().status).toBe('valid')
  })

  it('refreshTokens fails -> status unauthenticated, returns false', async () => {
    useAuthStore.setState({ status: 'valid', refreshToken: 'rt' })
    vi.mocked(sessionMocks.refreshTokens).mockRejectedValue(new Error('revoked'))

    const result = await useAuthStore.getState().onUnauthorized()

    expect(result).toBe(false)
    expect(useAuthStore.getState().status).toBe('unauthenticated')
  })

  it('no refresh token -> status unauthenticated, returns false', async () => {
    useAuthStore.setState({ status: 'valid', refreshToken: null })

    const result = await useAuthStore.getState().onUnauthorized()

    expect(result).toBe(false)
    expect(useAuthStore.getState().status).toBe('unauthenticated')
    expect(sessionMocks.refreshTokens).not.toHaveBeenCalled()
  })

  it('concurrent calls share one inflight promise (dedup)', async () => {
    useAuthStore.setState({ status: 'valid', refreshToken: 'rt' })
    let resolveRefresh!: (value: unknown) => void
    vi.mocked(sessionMocks.refreshTokens).mockImplementation(
      () => new Promise((r) => { resolveRefresh = r }),
    )

    const p1 = useAuthStore.getState().onUnauthorized()
    const p2 = useAuthStore.getState().onUnauthorized()

    resolveRefresh!(refreshResult())

    const [r1, r2] = await Promise.all([p1, p2])
    expect(r1).toBe(true)
    expect(r2).toBe(true)
    expect(sessionMocks.refreshTokens).toHaveBeenCalledTimes(1)
  })
})

describe('login()', () => {
  it('sets status=valid with tokens and user', () => {
    const session = { accessToken: 'at', refreshToken: 'rt', user: fakeUser }

    useAuthStore.getState().login(session)

    const state = useAuthStore.getState()
    expect(state.status).toBe('valid')
    expect(state.accessToken).toBe('at')
    expect(state.refreshToken).toBe('rt')
    expect(state.user).toEqual(fakeUser)
  })

  it('calls saveSession', () => {
    const session = { accessToken: 'at', refreshToken: 'rt', user: fakeUser }

    useAuthStore.getState().login(session)

    expect(sessionMocks.saveSession).toHaveBeenCalledWith(session)
  })
})

describe('logout()', () => {
  it('sets status=unauthenticated, clears tokens and user', () => {
    useAuthStore.setState({ status: 'valid', accessToken: 'at', refreshToken: 'rt', user: fakeUser })

    useAuthStore.getState().logout()

    const state = useAuthStore.getState()
    expect(state.status).toBe('unauthenticated')
    expect(state.accessToken).toBeNull()
    expect(state.refreshToken).toBeNull()
    expect(state.user).toBeNull()
  })

  it('calls clearSession', () => {
    useAuthStore.getState().logout()

    expect(sessionMocks.clearSession).toHaveBeenCalled()
  })
})
