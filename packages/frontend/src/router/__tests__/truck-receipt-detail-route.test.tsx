import { screen, waitFor } from '@testing-library/react'

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

vi.mock('~/components/layout/authenticated-layout', async () => {
  const { Outlet } = await import('@tanstack/react-router')
  return {
    AuthenticatedLayout: () => <Outlet />,
  }
})

vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet', () => ({
  useTransportTruckWaybillCompositeGet: vi.fn(() => ({
    isLoading: false,
    data: {
      data: {
        waybill: {
          id: 'wb-1',
          documentNumber: 'WB-001',
          date: '2026-04-01',
          senderId: 'sender-1',
          senderIdName: 'Sender Co',
        },
        items: [
          {
            id: 'item-1',
            productIdName: 'Diesel',
            declaredAmount: '125.5',
          },
        ],
      },
    },
  })),
}))

vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet', () => ({
  useAcceptanceCompositeGet: vi.fn(() => ({
    isLoading: false,
    data: undefined,
  })),
}))

const { renderFileRoute } = await import('~/router/testing/file-route-test-utils')
const { useAuthStore } = await import('~/stores/auth-store')
const { useNodeStore } = await import('~/stores/node-store')
const { useRuntimeStore } = await import('~/platform/runtime/runtime-store')
const { useStartupStore } = await import('~/stores/startup-store')

beforeEach(() => {
  vi.clearAllMocks()
  useNodeStore.getState().reset()
  useRuntimeStore.setState({
    bootstrapStatus: 'idle',
    bootstrapError: null,
    healthStatus: 'idle',
  })
  useStartupStore.setState({
    startupState: null,
    isLoading: false,
    error: null,
    refresh: async () => {},
  })
  useAuthStore.setState({
    status: 'valid',
    accessToken: 'access-token',
    refreshToken: 'refresh-token',
    user: {
      id: 'u1',
      username: 'operator',
      displayName: 'Operator',
      role: 'OPERATOR',
    } as never,
    boot: async () => {},
    onUnauthorized: async () => false,
    login: () => {},
    logout: () => {},
  })
})

describe('truck receipt detail route flow', () => {
  it('renders the pending waybill detail through the generated route tree', async () => {
    const { router } = renderFileRoute('/incoming/truck/wb-1')

    await waitFor(() => {
      expect(router.state.location.pathname).toBe('/incoming/truck/wb-1')
    })
    expect(await screen.findByText('WB-001')).toBeInTheDocument()
    expect(screen.getByText('Sender Co')).toBeInTheDocument()
    expect(screen.getByText('Diesel')).toBeInTheDocument()
  })
})
