/**
 * Integration tests for the rail-receipt detail variants. Mirror of
 * truck-receipt-detail.test.tsx with rail substitutions.
 *
 * Plan tasks covered:
 *   5.1 / 5.3 / 5.4 / 5.5 / 5.6a / 5.6 (rail twin).
 */

import type { ReactNode } from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'

import { useTransportRailWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet'
import { useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { RailReceiptDetail } from '~/views/incoming/rail-receipt'

vi.mock('@tanstack/react-router', () => ({
  getRouteApi: () => ({ useParams: () => ({ id: 'rwb-1' }) }),
  useNavigate: () => vi.fn(),
  Link: ({ children }: { children: ReactNode }) => <a>{children}</a>,
}))

vi.mock('~/hooks/use-page-title', () => ({ usePageTitle: () => {} }))

vi.mock('~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery', () => ({
  useRailReceiptPipelineQuery: vi.fn(),
  railReceiptPipelineQueryQueryKey: () => [{ url: '/flows/rail-receipt/query' }],
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet', () => ({
  useTransportRailWaybillCompositeGet: vi.fn(),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet', () => ({
  useTransportTruckWaybillCompositeGet: vi.fn().mockReturnValue({ data: undefined, isLoading: false }),
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
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeCreate', () => ({
  useTransportRailWaybillCompositeCreate: () => ({ mutateAsync: vi.fn() }),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeUpdate', () => ({
  useTransportRailWaybillCompositeUpdate: () => ({ mutateAsync: vi.fn() }),
}))
vi.mock('~/generated/hooks/FlowsHooks/useFlowAcceptanceFlatQuery', () => ({
  flowAcceptanceFlatQueryQueryKey: () => [{ url: '/flows/acceptance/flat' }],
}))
vi.mock('~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery', () => ({
  useFlowTruckReceiptQuery: vi.fn().mockReturnValue({ data: { data: [] } }),
  flowTruckReceiptQueryQueryKey: () => [{ url: '/flows/truck-receipt/query' }],
}))

function makeRailWaybillComposite(opts: { id: string, documentNumber: string }) {
  return {
    waybill: {
      id: opts.id,
      documentNumber: opts.documentNumber,
      date: '2026-04-30T00:00:00Z',
      senderId: 's-1',
      senderIdName: 'Sender',
      receiverId: 'r-1',
      receiverIdName: 'Receiver',
      status: 'PENDING',
      contractorId: 's-1',
      contractorIdName: 'Sender',
      arrivalType: 'RAIL',
      createdAt: '2026-04-30T00:00:00Z',
      createdBy: '00000000-0000-4000-8000-000000000001',
      originDbId: '00000000-0000-4000-8000-000000000003',
      updatedAt: '2026-04-30T00:00:00Z',
      updatedBy: '00000000-0000-4000-8000-000000000001',
    },
    wagonManifests: [
      {
        id: 'wm-1',
        wagonNumber: 'W-100',
        productId: 'p-1',
        productIdName: 'Diesel',
        declaredMass: '50',
        railWaybillId: opts.id,
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
  vi.mocked(useRailReceiptPipelineQuery).mockReturnValue({
    data: {
      data: rows.map(r => ({
        id: r.id,
        actionId: r.actionId ?? null,
        actionDocumentNumber: r.actionDocumentNumber ?? null,
        actualQuantity: null,
        basisDate: '2026-04-30',
        basisDocumentNumber: 'RWB-1',
        contractorId: 'c-1',
        contractorName: 'Contractor',
        expectedQuantity: '50',
        pipelineStatus: r.pipelineStatus,
        productName: 'Diesel',
      })),
    },
  } as unknown as ReturnType<typeof useRailReceiptPipelineQuery>)
}

function setWaybill(value: ReturnType<typeof makeRailWaybillComposite>) {
  vi.mocked(useTransportRailWaybillCompositeGet).mockReturnValue({
    data: { data: value },
    isLoading: false,
  } as unknown as ReturnType<typeof useTransportRailWaybillCompositeGet>)
}

function Providers({ children }: { children: ReactNode }) {
  const qc = new QueryClient({ defaultOptions: { queries: { retry: false } } })
  return <QueryClientProvider client={qc}>{children}</QueryClientProvider>
}

function renderDetail() {
  return render(
    <Providers>
      <RailReceiptDetail />
    </Providers>,
  )
}

describe('rail-receipt basis detail — toolbar actions per pipeline state', () => {
  it.each([
    ['PENDING', { editEnabled: true, issueEnabled: true }],
    ['DRAFT', { editEnabled: true, issueEnabled: false }],
    ['EXECUTED', { editEnabled: false, issueEnabled: false }],
  ] as const)('pipelineStatus=%s → edit=%s issue=%s', (status, { editEnabled, issueEnabled }) => {
    setWaybill(makeRailWaybillComposite({ id: 'rwb-1', documentNumber: 'RWB-001' }))
    setPipelineRows([{
      id: 'rwb-1',
      pipelineStatus: status,
      actionId: status === 'PENDING' ? null : 'a-1',
      actionDocumentNumber: status === 'PENDING' ? null : 'ACC-RW-1',
    }])
    renderDetail()
    const editBtn = screen.getByRole('button', { name: /^edit$/i })
    const issueBtn = screen.getByRole('button', { name: /^issue acceptance$/i })
    if (editEnabled)
      expect(editBtn).toBeEnabled()
    else expect(editBtn).toBeDisabled()
    if (issueEnabled)
      expect(issueBtn).toBeEnabled()
    else expect(issueBtn).toBeDisabled()
  })
})

describe('rail-receipt basis detail — RelatedDocuments + status pill', () => {
  it('renders RelatedDocuments linking to acceptance for DRAFT', () => {
    setWaybill(makeRailWaybillComposite({ id: 'rwb-1', documentNumber: 'RWB-001' }))
    setPipelineRows([{ id: 'rwb-1', pipelineStatus: 'DRAFT', actionId: 'a-1', actionDocumentNumber: 'ACC-RW-2026-001' }])
    renderDetail()
    expect(screen.getByText(/related documents/i)).toBeInTheDocument()
    expect(screen.getByText('ACC-RW-2026-001')).toBeInTheDocument()
  })

  it('does not render the legacy hardcoded PENDING when pipelineStatus is EXECUTED', () => {
    setWaybill(makeRailWaybillComposite({ id: 'rwb-1', documentNumber: 'RWB-001' }))
    setPipelineRows([{ id: 'rwb-1', pipelineStatus: 'EXECUTED', actionId: 'a-1' }])
    renderDetail()
    expect(screen.queryByText(/^pending$/i)).not.toBeInTheDocument()
    expect(screen.getAllByText(/^executed$/i).length).toBeGreaterThan(0)
  })
})
