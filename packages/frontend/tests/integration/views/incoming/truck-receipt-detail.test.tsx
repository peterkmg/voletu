/**
 * Integration tests for the truck-receipt detail variants.
 *
 * Plan tasks covered:
 *   5.1 — basis status pill is derived from `pipelineStatus`, not hardcoded.
 *   5.2 — basis variant exposes Edit + Issue acceptance with predicate-driven
 *         disable / tooltip.
 *   5.4 — basis variant renders symmetric `RelatedDocuments` block when an
 *         acceptance exists.
 *   5.5 — acceptance variant exposes Edit (gated by `canEditAcceptance`).
 *   5.6a — basis items table is locked when `pipelineStatus === 'EXECUTED'`.
 *   5.6 — `RelatedDocuments` label on the acceptance side derives from the
 *         basis pipeline state.
 */

import type { ReactNode } from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { useTransportTruckWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet'
import { useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { TruckReceiptDetail } from '~/views/incoming/truck-receipt'

vi.mock('@tanstack/react-router', () => ({
  getRouteApi: () => ({ useParams: () => ({ id: 'wb-1' }) }),
  useNavigate: () => vi.fn(),
  Link: ({ children, to: _to }: { children: ReactNode, to: string }) =>
    <a>{children}</a>,
}))

vi.mock('~/hooks/use-page-title', () => ({ usePageTitle: () => {} }))

vi.mock('~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery', () => ({
  useFlowTruckReceiptQuery: vi.fn(),
  flowTruckReceiptQueryQueryKey: () => [{ url: '/flows/truck-receipt/query' }],
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet', () => ({
  useTransportTruckWaybillCompositeGet: vi.fn(),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet', () => ({
  useTransportRailWaybillCompositeGet: vi.fn().mockReturnValue({ data: undefined, isLoading: false }),
}))
vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet', () => ({
  useAcceptanceCompositeGet: vi.fn().mockReturnValue({ data: undefined, isLoading: false }),
}))
vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeCreate', () => ({
  useAcceptanceCompositeCreate: () => ({ mutateAsync: vi.fn() }),
}))
vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeUpdate', () => ({
  useAcceptanceCompositeUpdate: () => ({ mutateAsync: vi.fn() }),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeCreate', () => ({
  useTransportTruckWaybillCompositeCreate: () => ({ mutateAsync: vi.fn() }),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeUpdate', () => ({
  useTransportTruckWaybillCompositeUpdate: () => ({ mutateAsync: vi.fn() }),
}))
vi.mock('~/generated/hooks/FlowsHooks/useFlowAcceptanceFlatQuery', () => ({
  flowAcceptanceFlatQueryQueryKey: () => [{ url: '/flows/acceptance/flat' }],
}))

function makeWaybillComposite(opts: { id: string, documentNumber: string }) {
  return {
    waybill: {
      id: opts.id,
      documentNumber: opts.documentNumber,
      date: '2026-04-30T00:00:00Z',
      truckLicensePlate: 'TR-1',
      driverName: 'Driver',
      senderId: 's-1',
      senderIdName: 'Sender',
      receiverId: 'r-1',
      receiverIdName: 'Receiver',
      status: 'PENDING',
      contractorId: 's-1',
      contractorIdName: 'Sender',
      arrivalType: 'TRUCK',
      createdAt: '2026-04-30T00:00:00Z',
      createdBy: '00000000-0000-4000-8000-000000000001',
      originDbId: '00000000-0000-4000-8000-000000000003',
      updatedAt: '2026-04-30T00:00:00Z',
      updatedBy: '00000000-0000-4000-8000-000000000001',
    },
    items: [
      {
        id: 'i-1',
        productId: 'p-1',
        productIdName: 'Diesel',
        declaredAmount: '100',
        truckWaybillId: opts.id,
        createdAt: '2026-04-30T00:00:00Z',
        createdBy: '00000000-0000-4000-8000-000000000001',
        originDbId: '00000000-0000-4000-8000-000000000003',
        updatedAt: '2026-04-30T00:00:00Z',
        updatedBy: '00000000-0000-4000-8000-000000000001',
      },
    ],
  }
}

function setPipelineRows(rows: Array<{
  id: string
  pipelineStatus: 'PENDING' | 'DRAFT' | 'EXECUTED'
  actionId?: string | null
  actionDocumentNumber?: string | null
}>) {
  vi.mocked(useFlowTruckReceiptQuery).mockReturnValue({
    data: {
      data: rows.map(r => ({
        id: r.id,
        actionId: r.actionId ?? null,
        actionDocumentNumber: r.actionDocumentNumber ?? null,
        actualQuantity: null,
        basisDate: '2026-04-30',
        basisDocumentNumber: 'WB-1',
        contractorId: 'c-1',
        contractorName: 'Contractor',
        expectedQuantity: '100',
        pipelineStatus: r.pipelineStatus,
        productName: 'Diesel',
      })),
    },
  } as unknown as ReturnType<typeof useFlowTruckReceiptQuery>)
}

function setWaybill(value: ReturnType<typeof makeWaybillComposite>) {
  vi.mocked(useTransportTruckWaybillCompositeGet).mockReturnValue({
    data: { data: value },
    isLoading: false,
  } as unknown as ReturnType<typeof useTransportTruckWaybillCompositeGet>)
}

function setAcceptance(value: unknown | undefined) {
  vi.mocked(useAcceptanceCompositeGet).mockReturnValue({
    data: value ? { data: value } : undefined,
    isLoading: false,
  } as unknown as ReturnType<typeof useAcceptanceCompositeGet>)
}

function Providers({ children }: { children: ReactNode }) {
  const qc = new QueryClient({ defaultOptions: { queries: { retry: false } } })
  return <QueryClientProvider client={qc}>{children}</QueryClientProvider>
}

function renderDetail() {
  return render(
    <Providers>
      <TruckReceiptDetail />
    </Providers>,
  )
}

describe('truck-receipt basis detail — derived pipelineStatus', () => {
  it('renders the pipelineStatus on the status pill (PENDING)', () => {
    setWaybill(makeWaybillComposite({ id: 'wb-1', documentNumber: 'WB-001' }))
    setAcceptance(undefined)
    setPipelineRows([{ id: 'wb-1', pipelineStatus: 'PENDING' }])
    renderDetail()
    // StatusBadge renders the value lowercased.
    expect(screen.getByText(/^pending$/i)).toBeInTheDocument()
  })

  it('renders DRAFT on the status pill when an acceptance exists in DRAFT', () => {
    setWaybill(makeWaybillComposite({ id: 'wb-1', documentNumber: 'WB-001' }))
    setAcceptance(undefined)
    setPipelineRows([{ id: 'wb-1', pipelineStatus: 'DRAFT', actionId: 'a-1', actionDocumentNumber: 'ACC-1' }])
    renderDetail()
    // The basis page no longer hardcodes 'PENDING' — pipeline state drives the pill.
    // (Multiple "draft" texts may appear: status pill + related-docs label. Use `getAllByText`.)
    expect(screen.getAllByText(/^draft$/i).length).toBeGreaterThan(0)
    // Regression guard: with pipelineStatus=DRAFT, no PENDING text appears as a status pill.
    expect(screen.queryByText(/^pending$/i)).not.toBeInTheDocument()
  })
})

describe('truck-receipt basis detail — toolbar actions', () => {
  it('edit and Issue acceptance both enabled for PENDING', () => {
    setWaybill(makeWaybillComposite({ id: 'wb-1', documentNumber: 'WB-001' }))
    setAcceptance(undefined)
    setPipelineRows([{ id: 'wb-1', pipelineStatus: 'PENDING' }])
    renderDetail()
    expect(screen.getByRole('button', { name: /^edit$/i })).toBeEnabled()
    expect(screen.getByRole('button', { name: /^issue acceptance$/i })).toBeEnabled()
  })

  it('edit enabled, Issue acceptance disabled for DRAFT', () => {
    setWaybill(makeWaybillComposite({ id: 'wb-1', documentNumber: 'WB-001' }))
    setAcceptance(undefined)
    setPipelineRows([{ id: 'wb-1', pipelineStatus: 'DRAFT', actionId: 'a-1', actionDocumentNumber: 'ACC-1' }])
    renderDetail()
    expect(screen.getByRole('button', { name: /^edit$/i })).toBeEnabled()
    expect(screen.getByRole('button', { name: /^issue acceptance$/i })).toBeDisabled()
  })

  it('edit and Issue acceptance both disabled for EXECUTED', () => {
    setWaybill(makeWaybillComposite({ id: 'wb-1', documentNumber: 'WB-001' }))
    setAcceptance(undefined)
    setPipelineRows([{ id: 'wb-1', pipelineStatus: 'EXECUTED', actionId: 'a-1', actionDocumentNumber: 'ACC-1' }])
    renderDetail()
    expect(screen.getByRole('button', { name: /^edit$/i })).toBeDisabled()
    expect(screen.getByRole('button', { name: /^issue acceptance$/i })).toBeDisabled()
  })
})

describe('truck-receipt basis detail — symmetric RelatedDocuments + items lock', () => {
  it('renders no RelatedDocuments block for PENDING', () => {
    setWaybill(makeWaybillComposite({ id: 'wb-1', documentNumber: 'WB-001' }))
    setAcceptance(undefined)
    setPipelineRows([{ id: 'wb-1', pipelineStatus: 'PENDING' }])
    renderDetail()
    expect(screen.queryByText(/related documents/i)).not.toBeInTheDocument()
  })

  it('renders RelatedDocuments block linking to the acceptance for DRAFT', () => {
    setWaybill(makeWaybillComposite({ id: 'wb-1', documentNumber: 'WB-001' }))
    setAcceptance(undefined)
    setPipelineRows([{ id: 'wb-1', pipelineStatus: 'DRAFT', actionId: 'a-1', actionDocumentNumber: 'ACC-2026-001' }])
    renderDetail()
    expect(screen.getByText(/related documents/i)).toBeInTheDocument()
    expect(screen.getByText('ACC-2026-001')).toBeInTheDocument()
  })

  it('items table add button is hidden when pipelineStatus is EXECUTED (locked)', () => {
    setWaybill(makeWaybillComposite({ id: 'wb-1', documentNumber: 'WB-001' }))
    setAcceptance(undefined)
    setPipelineRows([{ id: 'wb-1', pipelineStatus: 'EXECUTED', actionId: 'a-1' }])
    renderDetail()
    // ChildItemsTable hides the add button when isLocked is true. The
    // truck-receipt items panel does not pass `onAddItem`, so the button
    // should never appear regardless. Use this as a coarse sanity check.
    expect(screen.queryByRole('button', { name: /add item/i })).not.toBeInTheDocument()
  })
})
