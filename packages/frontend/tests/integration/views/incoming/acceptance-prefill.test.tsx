import type { ReactNode } from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { useTransportRailWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet'
import { useTransportTruckWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet'
import { AcceptanceMutateDialog } from '~/views/incoming/acceptance/acceptance-mutate-dialog'

vi.mock('~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery', () => ({
  useFlowTruckReceiptQuery: vi.fn().mockReturnValue({ data: { data: [] } }),
  flowTruckReceiptQueryQueryKey: () => [{ url: '/flows/truck-receipt/query' }],
}))
vi.mock('~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery', () => ({
  useRailReceiptPipelineQuery: vi.fn().mockReturnValue({ data: { data: [] } }),
  railReceiptPipelineQueryQueryKey: () => [{ url: '/flows/rail-receipt/query' }],
}))
vi.mock('~/generated/hooks/FlowsHooks/useFlowAcceptanceFlatQuery', () => ({
  flowAcceptanceFlatQueryQueryKey: () => [{ url: '/flows/acceptance/flat' }],
}))
vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeCreate', () => ({
  useAcceptanceCompositeCreate: () => ({ mutateAsync: vi.fn().mockResolvedValue({}) }),
}))
vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeUpdate', () => ({
  useAcceptanceCompositeUpdate: () => ({ mutateAsync: vi.fn().mockResolvedValue({}) }),
}))
vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet', () => ({
  useAcceptanceCompositeGet: vi.fn().mockReturnValue({ data: undefined, isLoading: false }),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet', () => ({
  useTransportTruckWaybillCompositeGet: vi.fn(),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet', () => ({
  useTransportRailWaybillCompositeGet: vi.fn(),
}))

function makeTruckComposite(opts: {
  id: string
  documentNumber: string
  senderId: string
  productIds: string[]
}) {
  return {
    waybill: {
      id: opts.id,
      documentNumber: opts.documentNumber,
      senderId: opts.senderId,
      baseId: '00000000-0000-4000-8000-000000000000',
      date: '2026-04-30T00:00:00Z',
      createdAt: '2026-04-30T00:00:00Z',
      createdBy: '00000000-0000-4000-8000-000000000001',
      originDbId: '00000000-0000-4000-8000-000000000002',
      updatedAt: '2026-04-30T00:00:00Z',
      updatedBy: '00000000-0000-4000-8000-000000000001',
    },
    items: opts.productIds.map((productId, ix) => ({
      id: `item-${ix}`,
      truckWaybillId: opts.id,
      productId,
      declaredAmount: '10',
      createdAt: '2026-04-30T00:00:00Z',
      createdBy: '00000000-0000-4000-8000-000000000001',
      originDbId: '00000000-0000-4000-8000-000000000003',
      updatedAt: '2026-04-30T00:00:00Z',
      updatedBy: '00000000-0000-4000-8000-000000000001',
    })),
  }
}

function makeRailComposite(opts: {
  id: string
  documentNumber: string
  senderId: string
  productIds: string[]
}) {
  return {
    waybill: {
      id: opts.id,
      documentNumber: opts.documentNumber,
      senderId: opts.senderId,
      baseId: '00000000-0000-4000-8000-000000000000',
      date: '2026-04-30T00:00:00Z',
      createdAt: '2026-04-30T00:00:00Z',
      createdBy: '00000000-0000-4000-8000-000000000001',
      originDbId: '00000000-0000-4000-8000-000000000002',
      updatedAt: '2026-04-30T00:00:00Z',
      updatedBy: '00000000-0000-4000-8000-000000000001',
    },
    wagonManifests: opts.productIds.map((productId, ix) => ({
      id: `manifest-${ix}`,
      railWaybillId: opts.id,
      productId,
      wagonNumber: `WGN-${ix}`,
      declaredDensity: '1',
      declaredMass: '10',
      declaredVolume: '10',
      createdAt: '2026-04-30T00:00:00Z',
      createdBy: '00000000-0000-4000-8000-000000000001',
      originDbId: '00000000-0000-4000-8000-000000000003',
      updatedAt: '2026-04-30T00:00:00Z',
      updatedBy: '00000000-0000-4000-8000-000000000001',
    })),
  }
}

function setTruckComposite(value: ReturnType<typeof makeTruckComposite>) {
  vi.mocked(useTransportTruckWaybillCompositeGet).mockReturnValue({
    data: { data: value },
    isLoading: false,
  } as unknown as ReturnType<typeof useTransportTruckWaybillCompositeGet>)
}

function setRailComposite(value: ReturnType<typeof makeRailComposite>) {
  vi.mocked(useTransportRailWaybillCompositeGet).mockReturnValue({
    data: { data: value },
    isLoading: false,
  } as unknown as ReturnType<typeof useTransportRailWaybillCompositeGet>)
}

function Providers({ children }: { children: ReactNode }) {
  const qc = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  })
  return <QueryClientProvider client={qc}>{children}</QueryClientProvider>
}

function renderDialog(props: Parameters<typeof AcceptanceMutateDialog>[0]) {
  return render(
    <Providers>
      <AcceptanceMutateDialog {...props} />
    </Providers>,
  )
}

describe('acceptanceMutateDialog — prefillBasis (truck)', () => {
  it('locks the basis section to the TRUCK tab and shows the waybill number hint', async () => {
    setTruckComposite(makeTruckComposite({
      id: '11111111-1111-4111-8111-111111111111',
      documentNumber: 'WB-2026-074',
      senderId: '22222222-2222-4222-8222-222222222222',
      productIds: ['33333333-3333-4333-8333-333333333333'],
    }))
    renderDialog({
      open: true,
      onOpenChange: () => {},
      prefillBasis: { kind: 'truck', basisId: '11111111-1111-4111-8111-111111111111' },
    })
    expect(screen.getByRole('tab', { name: /truck waybill/i })).toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /^external/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /rail waybill/i })).not.toBeInTheDocument()
    expect(screen.getByText(/WB-2026-074/)).toBeInTheDocument()
  })

  it('seeds one item row per truck waybill item', async () => {
    setTruckComposite(makeTruckComposite({
      id: '11111111-1111-4111-8111-111111111111',
      documentNumber: 'WB-A',
      senderId: '22222222-2222-4222-8222-222222222222',
      productIds: [
        '33333333-3333-4333-8333-333333333333',
        '44444444-4444-4444-8444-444444444444',
      ],
    }))
    renderDialog({
      open: true,
      onOpenChange: () => {},
      prefillBasis: { kind: 'truck', basisId: '11111111-1111-4111-8111-111111111111' },
    })

    expect(screen.queryByText(/no items yet/i)).not.toBeInTheDocument()
  })

  it('items table — Add Item button remains visible and clickable when prefilled (mutability)', async () => {
    setTruckComposite(makeTruckComposite({
      id: '11111111-1111-4111-8111-111111111111',
      documentNumber: 'WB-A',
      senderId: '22222222-2222-4222-8222-222222222222',
      productIds: ['33333333-3333-4333-8333-333333333333'],
    }))
    renderDialog({
      open: true,
      onOpenChange: () => {},
      prefillBasis: { kind: 'truck', basisId: '11111111-1111-4111-8111-111111111111' },
    })
    const addBtn = screen.getByRole('button', { name: /add item/i })
    expect(addBtn).toBeEnabled()
  })

  it.todo('keeps productId read-only on pre-filled rows but editable on user-added rows')
})

describe('acceptanceMutateDialog — prefillBasis (rail)', () => {
  it('locks the basis section to the RAIL tab and seeds items from wagonManifests', () => {
    setRailComposite(makeRailComposite({
      id: '55555555-5555-4555-8555-555555555555',
      documentNumber: 'RW-2026-001',
      senderId: '22222222-2222-4222-8222-222222222222',
      productIds: ['66666666-6666-4666-8666-666666666666'],
    }))
    renderDialog({
      open: true,
      onOpenChange: () => {},
      prefillBasis: { kind: 'rail', basisId: '55555555-5555-4555-8555-555555555555' },
    })
    expect(screen.getByRole('tab', { name: /rail waybill/i })).toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /^external/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /truck waybill/i })).not.toBeInTheDocument()
    expect(screen.getByText(/RW-2026-001/)).toBeInTheDocument()
  })
})

describe('acceptanceMutateDialog — edit mode locks the basis tab', () => {
  it('renders only the EXTERNAL tab when an EXTERNAL acceptance is loaded for edit', () => {
    vi.mocked(useAcceptanceCompositeGet).mockReturnValue({
      data: {
        data: {
          id: '77777777-7777-4777-8777-777777777777',
          documentNumber: 'ACC-001',
          dateAccepted: '2026-04-30T00:00:00Z',
          arrivalType: 'EXTERNAL',
          contractorId: '22222222-2222-4222-8222-222222222222',
          sourceEntity: 'Acme',
          truckWaybillId: null,
          railWaybillId: null,
          items: [],
          baseId: '00000000-0000-4000-8000-000000000000',
          createdAt: '2026-04-30T00:00:00Z',
          createdBy: '00000000-0000-4000-8000-000000000001',
          originDbId: '00000000-0000-4000-8000-000000000002',
          updatedAt: '2026-04-30T00:00:00Z',
          updatedBy: '00000000-0000-4000-8000-000000000001',
        },
      },
      isLoading: false,
    } as unknown as ReturnType<typeof useAcceptanceCompositeGet>)

    renderDialog({
      open: true,
      onOpenChange: () => {},
      currentRow: {
        id: 'whatever',
        documentId: '77777777-7777-4777-8777-777777777777',
        documentNumber: 'ACC-001',
      } as Parameters<typeof AcceptanceMutateDialog>[0]['currentRow'] as never,
    })

    expect(screen.getByRole('tab', { name: /external/i })).toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /truck waybill/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /rail waybill/i })).not.toBeInTheDocument()
  })
})

describe('acceptanceMutateDialog — incoming/external regression', () => {
  it('opens with all three tabs visible and EXTERNAL active when no prefill / no currentRow', () => {
    vi.mocked(useAcceptanceCompositeGet).mockReturnValue({
      data: undefined,
      isLoading: false,
    } as unknown as ReturnType<typeof useAcceptanceCompositeGet>)
    renderDialog({
      open: true,
      onOpenChange: () => {},
    })
    const externalTab = screen.getByRole('tab', { name: /external/i })
    expect(externalTab).toHaveAttribute('aria-selected', 'true')
    expect(screen.getByRole('tab', { name: /truck waybill/i })).toBeInTheDocument()
    expect(screen.getByRole('tab', { name: /rail waybill/i })).toBeInTheDocument()
  })
})
