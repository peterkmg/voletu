import { screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { renderWithProviders } from '@tests/common'
import { LanguageSwitch } from '~/components/language-switch'
import { NavUser } from '~/components/layout/nav-user'
import { TitlebarMenu } from '~/components/layout/titlebar-menu'

const { signOutAction, changeLanguagePreference } = vi.hoisted(() => ({
  signOutAction: vi.fn(),
  changeLanguagePreference: vi.fn(),
}))

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key === 'language.switch' ? 'Switch language' : key,
    i18n: { language: 'en', changeLanguage: vi.fn() },
  }),
}))

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}))

vi.mock('~/context/theme-provider', () => ({
  useTheme: () => ({ theme: 'system', setTheme: vi.fn() }),
}))

vi.mock('~/stores/startup-store', () => ({
  useStartupStore: (selector: (state: { startupState: null }) => unknown) =>
    selector({ startupState: null }),
}))

vi.mock('~/stores/auth-store', () => ({
  useAuthStore: Object.assign(
    (selector: (state: { user: { fullname: string, username: string } | null }) => unknown) =>
      selector({ user: { fullname: 'Test User', username: 'test' } }),
    { getState: () => ({ logout: vi.fn() }) },
  ),
}))

vi.mock('~/components/ui/sidebar', async () => {
  return {
    SidebarMenu: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
    SidebarMenuItem: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
    SidebarMenuButton: ({ children, ...props }: React.ComponentProps<'button'>) => <button type="button" {...props}>{children}</button>,
    useSidebar: () => ({ isMobile: false }),
  }
})

vi.mock('~/components/layout/data/sidebar-data', () => ({
  getSidebarData: () => ({ navGroups: [] }),
}))

vi.mock('~/lib/devtools', () => ({
  toggleDevTools: vi.fn(() => false),
}))

vi.mock('~/components/layout/actions/session-actions', () => ({
  signOutAction,
}))

vi.mock('~/components/layout/actions/language-actions', () => ({
  languageOptions: [
    { code: 'en', label: 'English' },
    { code: 'ru', label: 'Русский' },
  ],
  changeLanguagePreference,
}))

describe('shared layout actions', () => {
  beforeEach(() => {
    signOutAction.mockClear()
    changeLanguagePreference.mockClear()
  })

  it('routes titlebar sign-out through the shared session action', async () => {
    const user = userEvent.setup()

    renderWithProviders(<TitlebarMenu />)

    await user.click(screen.getByRole('button', { name: 'titlebar.file' }))
    await user.click(screen.getByRole('menuitem', { name: 'titlebar.signOut' }))
    await user.click(screen.getByRole('button', { name: 'titlebar.signOut' }))

    await waitFor(() => {
      expect(signOutAction).toHaveBeenCalledTimes(1)
    })
  })

  it('routes nav-user sign-out through the shared session action', async () => {
    const user = userEvent.setup()

    renderWithProviders(<NavUser />)

    await user.click(screen.getByRole('button'))
    await user.click(screen.getByRole('menuitem', { name: /auth:session.logout/i }))
    await user.click(screen.getByRole('button', { name: 'common:actions.confirm' }))

    await waitFor(() => {
      expect(signOutAction).toHaveBeenCalledTimes(1)
    })
  })

  it('routes language changes through the shared language action', async () => {
    const user = userEvent.setup()

    renderWithProviders(<LanguageSwitch />)

    await user.click(screen.getByRole('button', { name: 'Switch language' }))
    await user.click(screen.getByRole('menuitem', { name: 'Русский' }))

    await waitFor(() => {
      expect(changeLanguagePreference).toHaveBeenCalledWith(expect.anything(), 'ru')
    })
  })
})
