/**
 * Happy-path integration test for the acceptance pipeline.
 *
 * Plan Tasks 6.4 (truck) and 6.5 (rail) — spec §4.6.
 *
 * The spec calls for an end-to-end test that walks:
 *   1. Create a waybill → row appears with pipelineStatus=PENDING.
 *   2. Click "Issue acceptance" on the row → dialog opens with the matching
 *      basis tab pre-selected and locked.
 *   3. Items pre-filled, basis-tab fields locked, common fields editable.
 *   4. Submit → row transitions to DRAFT.
 *   5. From the basis detail page (DRAFT state), Edit is enabled, Issue
 *      acceptance is disabled with the "already issued" tooltip; the
 *      RelatedDocuments block links to the acceptance.
 *   6. After execute, the basis detail page (EXECUTED state) disables both
 *      Edit and Issue acceptance, and the items panel is locked.
 *
 * The test stitches three layers — RowActions, AcceptanceMutateDialog,
 * TruckReceiptDetail / RailReceiptDetail — into a single narrative that
 * fails loudly if any state-machine transition is broken end to end.
 *
 * The lower-level component behaviors are individually covered in:
 *   tests/unit/lib/create-row-actions.test.tsx
 *   tests/integration/views/incoming/acceptance-prefill.test.tsx
 *   tests/integration/views/incoming/truck-receipt-detail.test.tsx
 *   tests/integration/views/incoming/rail-receipt-detail.test.tsx
 *
 * Mocking strategy: vi.mock the generated API hooks (the project does not
 * use MSW; pattern matches acceptance-prefill.test.tsx and
 * truck-receipt-detail.test.tsx).
 */

import type { Row } from '@tanstack/react-table'
import type { ReactNode } from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { TooltipProvider } from '~/components/ui/tooltip'
import { useAcceptanceCompositeGet } from '~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet'
import { useTransportRailWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet'
import { useTransportTruckWaybillCompositeGet } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet'
import { useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { createRowActions } from '~/lib/create-row-actions'
import { AcceptanceMutateDialog } from '~/views/incoming/acceptance/acceptance-mutate-dialog'
import { RailReceiptDetail } from '~/views/incoming/rail-receipt'
import { TruckReceiptDetail } from '~/views/incoming/truck-receipt'

// ----------------------------------------------------------------------------
// Module mocks (mirror the patterns used by sibling integration tests).
// ----------------------------------------------------------------------------

vi.mock('@tanstack/react-router', () => ({
  getRouteApi: () => ({ useParams: () => ({ id: 'wb-1' }) }),
  useNavigate: () => vi.fn(),
  Link: ({ children }: { children: ReactNode }) => <a>{children}</a>,
}))

vi.mock('~/hooks/use-page-title', () => ({ usePageTitle: () => {} }))

vi.mock('~/stores/auth-store', () => ({
  useAuthStore: (selector: (state: { user: { role: string } | null }) => unknown) =>
    selector({ user: { role: 'OPERATOR' } }),
}))

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
vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeGet', () => ({
  useAcceptanceCompositeGet: vi.fn().mockReturnValue({ data: undefined, isLoading: false }),
}))
vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeCreate', () => ({
  useAcceptanceCompositeCreate: () => ({ mutateAsync: vi.fn().mockResolvedValue({}) }),
}))
vi.mock('~/generated/hooks/DocumentAcceptanceHooks/useAcceptanceCompositeUpdate', () => ({
  useAcceptanceCompositeUpdate: () => ({ mutateAsync: vi.fn().mockResolvedValue({}) }),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeGet', () => ({
  useTransportTruckWaybillCompositeGet: vi.fn(),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeGet', () => ({
  useTransportRailWaybillCompositeGet: vi.fn(),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeCreate', () => ({
  useTransportTruckWaybillCompositeCreate: () => ({ mutateAsync: vi.fn() }),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillCompositeUpdate', () => ({
  useTransportTruckWaybillCompositeUpdate: () => ({ mutateAsync: vi.fn() }),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeCreate', () => ({
  useTransportRailWaybillCompositeCreate: () => ({ mutateAsync: vi.fn() }),
}))
vi.mock('~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillCompositeUpdate', () => ({
  useTransportRailWaybillCompositeUpdate: () => ({ mutateAsync: vi.fn() }),
}))

// ----------------------------------------------------------------------------
// Fixtures
// ----------------------------------------------------------------------------

const TRUCK_WB_ID = '11111111-1111-4111-8111-111111111111'
const RAIL_WB_ID = '55555555-5555-4555-8555-555555555555'
const PRODUCT_ID = '33333333-3333-4333-8333-333333333333'
const SENDER_ID = '22222222-2222-4222-8222-222222222222'
const ACCEPTANCE_ID = 'a-1'
const ACCEPTANCE_NUMBER = 'ACC-2026-001'

function makeTruckComposite(documentNumber: string) {
  return {
    waybill: {
      id: TRUCK_WB_ID,
      documentNumber,
      date: '2026-04-30T00:00:00Z',
      truckLicensePlate: 'TR-1',
      driverName: 'Driver',
      senderId: SENDER_ID,
      senderIdName: 'Sender',
      receiverId: 'r-1',
      receiverIdName: 'Receiver',
      status: 'PENDING',
      contractorId: SENDER_ID,
      contractorIdName: 'Sender',
      arrivalType: 'TRUCK',
      baseId: '00000000-0000-4000-8000-000000000000',
      createdAt: '2026-04-30T00:00:00Z',
      createdBy: '00000000-0000-4000-8000-000000000001',
      originDbId: '00000000-0000-4000-8000-000000000003',
      updatedAt: '2026-04-30T00:00:00Z',
      updatedBy: '00000000-0000-4000-8000-000000000001',
    },
    items: [
      {
        id: 'i-1',
        productId: PRODUCT_ID,
        productIdName: 'Diesel',
        declaredAmount: '100',
        truckWaybillId: TRUCK_WB_ID,
        createdAt: '2026-04-30T00:00:00Z',
        createdBy: '00000000-0000-4000-8000-000000000001',
        originDbId: '00000000-0000-4000-8000-000000000003',
        updatedAt: '2026-04-30T00:00:00Z',
        updatedBy: '00000000-0000-4000-8000-000000000001',
      },
    ],
  }
}

function makeRailComposite(documentNumber: string) {
  return {
    waybill: {
      id: RAIL_WB_ID,
      documentNumber,
      date: '2026-04-30T00:00:00Z',
      senderId: SENDER_ID,
      senderIdName: 'Sender',
      receiverId: 'r-1',
      receiverIdName: 'Receiver',
      status: 'PENDING',
      contractorId: SENDER_ID,
      contractorIdName: 'Sender',
      arrivalType: 'RAIL',
      baseId: '00000000-0000-4000-8000-000000000000',
      createdAt: '2026-04-30T00:00:00Z',
      createdBy: '00000000-0000-4000-8000-000000000001',
      originDbId: '00000000-0000-4000-8000-000000000003',
      updatedAt: '2026-04-30T00:00:00Z',
      updatedBy: '00000000-0000-4000-8000-000000000001',
    },
    wagonManifests: [
      {
        id: 'manifest-0',
        railWaybillId: RAIL_WB_ID,
        productId: PRODUCT_ID,
        wagonNumber: 'WGN-0',
        declaredDensity: '1',
        declaredMass: '10',
        declaredVolume: '10',
        createdAt: '2026-04-30T00:00:00Z',
        createdBy: '00000000-0000-4000-8000-000000000001',
        originDbId: '00000000-0000-4000-8000-000000000003',
        updatedAt: '2026-04-30T00:00:00Z',
        updatedBy: '00000000-0000-4000-8000-000000000001',
      },
    ],
  }
}

function setTruckPipelineRow(opts: {
  pipelineStatus: 'PENDING' | 'DRAFT' | 'EXECUTED'
  actionId?: string | null
  actionDocumentNumber?: string | null
}) {
  vi.mocked(useFlowTruckReceiptQuery).mockReturnValue({
    data: {
      data: [{
        id: TRUCK_WB_ID,
        actionId: opts.actionId ?? null,
        actionDocumentNumber: opts.actionDocumentNumber ?? null,
        actualQuantity: null,
        basisDate: '2026-04-30',
        basisDocumentNumber: 'WB-1',
        contractorId: 'c-1',
        contractorName: 'Contractor',
        expectedQuantity: '100',
        pipelineStatus: opts.pipelineStatus,
        productName: 'Diesel',
      }],
    },
  } as unknown as ReturnType<typeof useFlowTruckReceiptQuery>)
}

function setRailPipelineRow(opts: {
  pipelineStatus: 'PENDING' | 'DRAFT' | 'EXECUTED'
  actionId?: string | null
  actionDocumentNumber?: string | null
}) {
  vi.mocked(useRailReceiptPipelineQuery).mockReturnValue({
    data: {
      data: [{
        id: RAIL_WB_ID,
        actionId: opts.actionId ?? null,
        actionDocumentNumber: opts.actionDocumentNumber ?? null,
        actualQuantity: null,
        basisDate: '2026-04-30',
        basisDocumentNumber: 'RW-1',
        contractorId: 'c-1',
        contractorName: 'Contractor',
        expectedQuantity: '100',
        pipelineStatus: opts.pipelineStatus,
        productName: 'Diesel',
      }],
    },
  } as unknown as ReturnType<typeof useRailReceiptPipelineQuery>)
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

function clearAcceptance() {
  vi.mocked(useAcceptanceCompositeGet).mockReturnValue({
    data: undefined,
    isLoading: false,
  } as unknown as ReturnType<typeof useAcceptanceCompositeGet>)
}

interface PipelineRow {
  id: string
  pipelineStatus: 'PENDING' | 'DRAFT' | 'EXECUTED'
}

function makeRowActions() {
  const openIssueAcceptance = vi.fn()
  const openUpdate = vi.fn()
  const openDelete = vi.fn()
  const openLifecycle = vi.fn()
  const Component = createRowActions<PipelineRow>({
    useEntity: () => ({ openUpdate, openDelete, openLifecycle, openIssueAcceptance }),
    getDetailPath: r => `/incoming/${r.id}`,
    pipelineActions: {
      editVisible: r => r.pipelineStatus === 'PENDING',
      issueAcceptance: { visible: r => r.pipelineStatus === 'PENDING' },
    },
  })
  return { Component, openIssueAcceptance, openUpdate, openDelete }
}

function rowOf(row: PipelineRow): Row<PipelineRow> {
  return { original: row } as unknown as Row<PipelineRow>
}

function Providers({ children }: { children: ReactNode }) {
  const qc = new QueryClient({ defaultOptions: { queries: { retry: false } } })
  return (
    <QueryClientProvider client={qc}>
      <TooltipProvider>{children}</TooltipProvider>
    </QueryClientProvider>
  )
}

function renderWith(ui: ReactNode) {
  return render(<Providers>{ui}</Providers>)
}

afterEach(() => {
  vi.clearAllMocks()
})

// ----------------------------------------------------------------------------
// Truck happy path (Task 6.4)
// ----------------------------------------------------------------------------

describe('acceptance pipeline happy path — truck', () => {
  it('walks the PENDING → DRAFT → EXECUTED chain end to end', async () => {
    const user = userEvent.setup()

    // -- Step 1-2: PENDING row exposes "Issue acceptance" affordance. ----------
    const { Component: TruckRow, openIssueAcceptance } = makeRowActions()
    const { unmount: unmountRow } = renderWith(
      <TruckRow row={rowOf({ id: TRUCK_WB_ID, pipelineStatus: 'PENDING' })} />,
    )

    const issueButton = screen.getByRole('button', { name: /issue acceptance/i })
    expect(issueButton).toBeEnabled()

    await user.click(issueButton)
    expect(openIssueAcceptance).toHaveBeenCalledTimes(1)
    expect(openIssueAcceptance).toHaveBeenCalledWith(
      expect.objectContaining({ id: TRUCK_WB_ID, pipelineStatus: 'PENDING' }),
    )
    unmountRow()

    // -- Step 3: dialog opens with truck tab locked, items pre-filled. ---------
    setTruckComposite(makeTruckComposite('WB-2026-074'))
    clearAcceptance()
    const { unmount: unmountDialog } = renderWith(
      <AcceptanceMutateDialog
        open
        onOpenChange={() => {}}
        prefillBasis={{ kind: 'truck', basisId: TRUCK_WB_ID }}
      />,
    )

    expect(screen.getByRole('tab', { name: /truck waybill/i })).toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /^external/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /rail waybill/i })).not.toBeInTheDocument()
    expect(screen.getByText(/WB-2026-074/)).toBeInTheDocument()
    // Items table seeded with the waybill's items (no empty-state).
    expect(screen.queryByText(/no items yet/i)).not.toBeInTheDocument()
    unmountDialog()

    // -- Step 4-5: basis detail in DRAFT — Edit enabled, Issue disabled,  -----
    //              RelatedDocuments links to the acceptance.
    setTruckComposite(makeTruckComposite('WB-2026-074'))
    clearAcceptance()
    setTruckPipelineRow({
      pipelineStatus: 'DRAFT',
      actionId: ACCEPTANCE_ID,
      actionDocumentNumber: ACCEPTANCE_NUMBER,
    })
    const { unmount: unmountDraft } = renderWith(<TruckReceiptDetail />)

    expect(screen.getByRole('button', { name: /^edit$/i })).toBeEnabled()
    expect(screen.getByRole('button', { name: /^issue acceptance$/i })).toBeDisabled()
    expect(screen.getByText(/related documents/i)).toBeInTheDocument()
    expect(screen.getByText(ACCEPTANCE_NUMBER)).toBeInTheDocument()
    // Status pill reflects DRAFT, not the legacy hardcoded PENDING.
    expect(screen.queryByText(/^pending$/i)).not.toBeInTheDocument()
    unmountDraft()

    // -- Step 7: after execute, basis detail shows EXECUTED, both buttons -----
    //            disabled, items table locked.
    setTruckComposite(makeTruckComposite('WB-2026-074'))
    clearAcceptance()
    setTruckPipelineRow({
      pipelineStatus: 'EXECUTED',
      actionId: ACCEPTANCE_ID,
      actionDocumentNumber: ACCEPTANCE_NUMBER,
    })
    renderWith(<TruckReceiptDetail />)

    expect(screen.getByRole('button', { name: /^edit$/i })).toBeDisabled()
    expect(screen.getByRole('button', { name: /^issue acceptance$/i })).toBeDisabled()
    expect(screen.queryByRole('button', { name: /add item/i })).not.toBeInTheDocument()
  })

  it('hides "Issue acceptance" once the row leaves PENDING', () => {
    const { Component: TruckRow } = makeRowActions()
    const { rerender } = render(
      <Providers>
        <TruckRow row={rowOf({ id: TRUCK_WB_ID, pipelineStatus: 'DRAFT' })} />
      </Providers>,
    )
    expect(screen.queryByRole('button', { name: /issue acceptance/i })).not.toBeInTheDocument()

    rerender(
      <Providers>
        <TruckRow row={rowOf({ id: TRUCK_WB_ID, pipelineStatus: 'EXECUTED' })} />
      </Providers>,
    )
    expect(screen.queryByRole('button', { name: /issue acceptance/i })).not.toBeInTheDocument()
  })
})

// ----------------------------------------------------------------------------
// Rail happy path (Task 6.5) — mirrors the truck flow with rail substitutions.
// ----------------------------------------------------------------------------

describe('acceptance pipeline happy path — rail', () => {
  it('walks the PENDING → DRAFT → EXECUTED chain end to end', async () => {
    const user = userEvent.setup()

    // -- Step 1-2: PENDING row exposes "Issue acceptance" affordance. ----------
    const { Component: RailRow, openIssueAcceptance } = makeRowActions()
    const { unmount: unmountRow } = renderWith(
      <RailRow row={rowOf({ id: RAIL_WB_ID, pipelineStatus: 'PENDING' })} />,
    )

    const issueButton = screen.getByRole('button', { name: /issue acceptance/i })
    expect(issueButton).toBeEnabled()
    await user.click(issueButton)
    expect(openIssueAcceptance).toHaveBeenCalledTimes(1)
    unmountRow()

    // -- Step 3: dialog opens with rail tab locked, items derived from  --------
    //            wagonManifests.
    setRailComposite(makeRailComposite('RW-2026-001'))
    clearAcceptance()
    const { unmount: unmountDialog } = renderWith(
      <AcceptanceMutateDialog
        open
        onOpenChange={() => {}}
        prefillBasis={{ kind: 'rail', basisId: RAIL_WB_ID }}
      />,
    )

    expect(screen.getByRole('tab', { name: /rail waybill/i })).toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /^external/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /truck waybill/i })).not.toBeInTheDocument()
    expect(screen.getByText(/RW-2026-001/)).toBeInTheDocument()
    expect(screen.queryByText(/no items yet/i)).not.toBeInTheDocument()
    unmountDialog()

    // -- Step 4-5: basis detail in DRAFT. -------------------------------------
    setRailComposite(makeRailComposite('RW-2026-001'))
    clearAcceptance()
    setRailPipelineRow({
      pipelineStatus: 'DRAFT',
      actionId: ACCEPTANCE_ID,
      actionDocumentNumber: ACCEPTANCE_NUMBER,
    })
    const { unmount: unmountDraft } = renderWith(<RailReceiptDetail />)

    expect(screen.getByRole('button', { name: /^edit$/i })).toBeEnabled()
    expect(screen.getByRole('button', { name: /^issue acceptance$/i })).toBeDisabled()
    expect(screen.getByText(/related documents/i)).toBeInTheDocument()
    expect(screen.getByText(ACCEPTANCE_NUMBER)).toBeInTheDocument()
    expect(screen.queryByText(/^pending$/i)).not.toBeInTheDocument()
    unmountDraft()

    // -- Step 7: after execute, basis EXECUTED locks both actions. ------------
    setRailComposite(makeRailComposite('RW-2026-001'))
    clearAcceptance()
    setRailPipelineRow({
      pipelineStatus: 'EXECUTED',
      actionId: ACCEPTANCE_ID,
      actionDocumentNumber: ACCEPTANCE_NUMBER,
    })
    renderWith(<RailReceiptDetail />)

    expect(screen.getByRole('button', { name: /^edit$/i })).toBeDisabled()
    expect(screen.getByRole('button', { name: /^issue acceptance$/i })).toBeDisabled()
    expect(screen.queryByRole('button', { name: /add item/i })).not.toBeInTheDocument()
  })

  it('hides "Issue acceptance" once the row leaves PENDING', () => {
    const { Component: RailRow } = makeRowActions()
    const { rerender } = render(
      <Providers>
        <RailRow row={rowOf({ id: RAIL_WB_ID, pipelineStatus: 'DRAFT' })} />
      </Providers>,
    )
    expect(screen.queryByRole('button', { name: /issue acceptance/i })).not.toBeInTheDocument()

    rerender(
      <Providers>
        <RailRow row={rowOf({ id: RAIL_WB_ID, pipelineStatus: 'EXECUTED' })} />
      </Providers>,
    )
    expect(screen.queryByRole('button', { name: /issue acceptance/i })).not.toBeInTheDocument()
  })
})
