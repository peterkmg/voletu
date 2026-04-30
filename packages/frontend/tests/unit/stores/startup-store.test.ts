export {}

vi.mock('~/tauri/commands', () => ({
  getStartupState: vi.fn(),
}))

const { getStartupState } = await import('~/tauri/commands') as any
const { useStartupStore } = await import('~/stores/startup-store')

beforeEach(() => {
  vi.clearAllMocks()
  useStartupStore.setState({ startupState: null, isLoading: false, error: null })
})

describe('startup-store', () => {
  describe('initial state', () => {
    it('starts with null state, not loading, no error', () => {
      const state = useStartupStore.getState()
      expect(state.startupState).toBeNull()
      expect(state.isLoading).toBe(false)
      expect(state.error).toBeNull()
    })
  })

  describe('refresh()', () => {
    it('loads startup state from Tauri command', async () => {
      const mockState = { apiBaseUrl: 'http://localhost:3000', isDebugBuild: true }
      vi.mocked(getStartupState).mockResolvedValue(mockState)

      await useStartupStore.getState().refresh()

      const state = useStartupStore.getState()
      expect(state.startupState).toEqual(mockState)
      expect(state.isLoading).toBe(false)
      expect(state.error).toBeNull()
    })

    it('sets loading state during fetch', async () => {
      let resolve!: (v: unknown) => void
      vi.mocked(getStartupState).mockImplementation(
        () => new Promise((r) => { resolve = r }),
      )

      const promise = useStartupStore.getState().refresh()
      expect(useStartupStore.getState().isLoading).toBe(true)

      resolve({ apiBaseUrl: 'http://localhost:3000', isDebugBuild: false })
      await promise

      expect(useStartupStore.getState().isLoading).toBe(false)
    })

    it('sets error on failure', async () => {
      vi.mocked(getStartupState).mockRejectedValue(new Error('Tauri not available'))

      await useStartupStore.getState().refresh()

      const state = useStartupStore.getState()
      expect(state.error).toBe('Tauri not available')
      expect(state.isLoading).toBe(false)
      expect(state.startupState).toBeNull()
    })
  })

  describe('applyStartupState()', () => {
    it('directly sets startup state', () => {
      const mockState = { apiBaseUrl: 'http://remote:3000', isDebugBuild: false }

      useStartupStore.getState().applyStartupState(mockState as any)

      expect(useStartupStore.getState().startupState).toEqual(mockState)
    })
  })
})
