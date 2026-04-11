import { act, waitFor } from '@testing-library/react'

export {}

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key,
    i18n: { language: 'en', changeLanguage: vi.fn() },
  }),
}))

vi.mock('~/platform/runtime/bootstrap', () => ({
  ensureBootstrapped: vi.fn(async () => {}),
}))

vi.mock('~/views/auth/forms/sign-in-form', () => ({
  SignInForm: () => <div>sign-in-view</div>,
}))

const { renderFileRoute } = await import('~/router/testing/file-route-test-utils')
const { useStartupStore } = await import('~/stores/startup-store')

beforeEach(() => {
  vi.clearAllMocks()
  useStartupStore.setState({
    startupState: {
      needsSetup: true,
      mode: 'remote',
      apiBaseUrl: 'http://configured-api:3000',
      isDebugBuild: false,
    },
    isLoading: false,
    error: null,
    refresh: async () => {},
  })
})

describe('setup route flow', () => {
  it('navigates to sign-in after setup state flips to complete', async () => {
    const { router } = renderFileRoute('/setup')

    await waitFor(() => {
      expect(router.state.location.pathname).toBe('/setup')
    })

    act(() => {
      useStartupStore.setState({
        startupState: {
          needsSetup: false,
          mode: 'remote',
          apiBaseUrl: 'http://configured-api:3000',
          isDebugBuild: false,
        },
      })
    })

    await waitFor(() => {
      expect(router.state.location.pathname).toBe('/sign-in')
    })
  })
})
