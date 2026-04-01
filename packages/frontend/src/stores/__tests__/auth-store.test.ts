import type { AuthSession } from '~/shared/auth/session'

vi.mock('~/shared/auth/session', () => ({
  loadStoredSession: vi.fn(() => null),
  persistSession: vi.fn(),
  clearStoredSession: vi.fn(),
}))

// Import store AFTER the mock so the module-level `loadStoredSession()` call
// inside auth-store.ts sees the mocked (null-returning) version.
const { useAuthStore } = await import('~/stores/auth-store')
const { persistSession, clearStoredSession } = await import(
  '~/shared/auth/session'
)

const fakeUser = {
  id: 'u1',
  email: 'a@b.com',
  name: 'Alice',
} as AuthSession['user']

const fakeSession: AuthSession = {
  accessToken: 'at-123',
  refreshToken: 'rt-456',
  user: fakeUser,
}

beforeEach(() => {
  vi.clearAllMocks()
  useAuthStore.getState().auth.reset()
})

describe('auth-store', () => {
  describe('initial state', () => {
    it('starts with null user and empty tokens', () => {
      const { auth } = useAuthStore.getState()
      expect(auth.user).toBeNull()
      expect(auth.accessToken).toBe('')
      expect(auth.refreshToken).toBe('')
    })

    it('starts with isInitializing = true', () => {
      const { auth } = useAuthStore.getState()
      expect(auth.isInitializing).toBe(true)
    })
  })

  describe('setSession()', () => {
    it('sets user, accessToken, and refreshToken', () => {
      useAuthStore.getState().auth.setSession(fakeSession)

      const { auth } = useAuthStore.getState()
      expect(auth.user).toEqual(fakeUser)
      expect(auth.accessToken).toBe('at-123')
      expect(auth.refreshToken).toBe('rt-456')
    })

    it('calls persistSession with the session', () => {
      useAuthStore.getState().auth.setSession(fakeSession)
      expect(persistSession).toHaveBeenCalledWith(fakeSession)
    })

    it('preserves isInitializing when setting session', () => {
      useAuthStore.getState().auth.setInitialized()
      useAuthStore.getState().auth.setSession(fakeSession)

      expect(useAuthStore.getState().auth.isInitializing).toBe(false)
    })
  })

  describe('clearSession()', () => {
    it('resets user and tokens to defaults', () => {
      useAuthStore.getState().auth.setSession(fakeSession)
      useAuthStore.getState().auth.clearSession()

      const { auth } = useAuthStore.getState()
      expect(auth.user).toBeNull()
      expect(auth.accessToken).toBe('')
      expect(auth.refreshToken).toBe('')
    })

    it('calls clearStoredSession', () => {
      useAuthStore.getState().auth.clearSession()
      expect(clearStoredSession).toHaveBeenCalled()
    })
  })

  describe('reset()', () => {
    it('clears user and tokens', () => {
      useAuthStore.getState().auth.setSession(fakeSession)
      useAuthStore.getState().auth.reset()

      const { auth } = useAuthStore.getState()
      expect(auth.user).toBeNull()
      expect(auth.accessToken).toBe('')
      expect(auth.refreshToken).toBe('')
    })

    it('calls clearStoredSession', () => {
      useAuthStore.getState().auth.reset()
      expect(clearStoredSession).toHaveBeenCalled()
    })
  })

  describe('setInitialized()', () => {
    it('sets isInitializing to false', () => {
      // Manually restore isInitializing because reset() does not touch it
      useAuthStore.setState({
        auth: { ...useAuthStore.getState().auth, isInitializing: true },
      })
      expect(useAuthStore.getState().auth.isInitializing).toBe(true)
      useAuthStore.getState().auth.setInitialized()
      expect(useAuthStore.getState().auth.isInitializing).toBe(false)
    })

    it('preserves existing user/token state', () => {
      useAuthStore.getState().auth.setSession(fakeSession)
      useAuthStore.getState().auth.setInitialized()

      const { auth } = useAuthStore.getState()
      expect(auth.user).toEqual(fakeUser)
      expect(auth.accessToken).toBe('at-123')
    })
  })
})
