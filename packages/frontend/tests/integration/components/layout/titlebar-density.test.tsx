import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { renderWithProviders } from '@tests/common'
import { useDensity } from '~/components/data-table/density'
import { TitlebarMenu } from '~/components/layout/titlebar-menu'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key,
    i18n: { language: 'en', changeLanguage: vi.fn() },
  }),
}))

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}))

vi.mock('~/context/theme-provider', () => ({
  useTheme: () => ({ theme: 'system', setTheme: vi.fn() }),
}))

vi.mock('~/stores/auth-store', () => ({
  useAuthStore: Object.assign(
    (selector: (state: { logout: () => void }) => unknown) =>
      selector({ logout: vi.fn() }),
    { getState: () => ({ logout: vi.fn() }) },
  ),
}))

vi.mock('~/stores/startup-store', () => ({
  useStartupStore: (selector: (state: { startupState: null }) => unknown) =>
    selector({ startupState: null }),
}))

vi.mock('~/components/layout/data/sidebar-data', () => ({
  getSidebarData: () => ({ navGroups: [] }),
}))

vi.mock('~/lib/devtools', () => ({
  toggleDevTools: vi.fn(() => false),
}))

function DensityValue() {
  const { density } = useDensity()
  return <div data-testid="density-value">{density}</div>
}

describe('titlebar density integration', () => {
  beforeEach(() => {
    localStorage.removeItem('voletu.table-density')
  })

  it('updates the shared density state from the titlebar menu', async () => {
    const user = userEvent.setup()

    renderWithProviders(
      <>
        <DensityValue />
        <TitlebarMenu />
      </>,
    )

    expect(screen.getByTestId('density-value')).toHaveTextContent('normal')

    await user.click(screen.getByRole('button', { name: 'titlebar.view' }))
    await user.click(screen.getByRole('menuitemradio', { name: 'titlebar.densityComfortable' }))

    await waitFor(() => {
      expect(screen.getByTestId('density-value')).toHaveTextContent('comfortable')
    })
    expect(localStorage.getItem('voletu.table-density')).toBe('comfortable')
  })
})
