/**
 * Integration tests for `<AcceptanceBasisSection>`.
 *
 * Coverage corresponds to plan tasks 3.1, 3.2, 3.3:
 *   3.1 — tab visibility per locked state, tab-switch updates `arrivalType`.
 *   3.2 — items-clear confirm dialog: shows when items > 0, cancel preserves,
 *         confirm clears and switches.
 *   3.3 — waybill picker is filtered to PENDING rows only and writes the
 *         selected id into form state.
 *
 * The flow query hooks are mocked at the module level so tests can drive
 * the picker's data deterministically.
 */

import type { ReactNode } from 'react'
import type { ArrivalType } from '~/generated/types/ArrivalType'
import type { AcceptanceCreate } from '~/views/incoming/acceptance/acceptance-form-config'
import { render, screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { FormProvider, useForm } from 'react-hook-form'
import { describe, expect, it, vi } from 'vitest'

import { useFlowTruckReceiptQuery } from '~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery'
import { useRailReceiptPipelineQuery } from '~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery'
import { AcceptanceBasisSection } from '~/views/incoming/acceptance/acceptance-basis-section'
import {

  emptyAcceptanceCreate,
} from '~/views/incoming/acceptance/acceptance-form-config'

vi.mock('~/generated/hooks/FlowsHooks/useFlowTruckReceiptQuery', () => ({
  useFlowTruckReceiptQuery: vi.fn(),
}))

vi.mock('~/generated/hooks/FlowsHooks/useRailReceiptPipelineQuery', () => ({
  useRailReceiptPipelineQuery: vi.fn(),
}))

function setTruckRows(rows: unknown[]) {
  vi.mocked(useFlowTruckReceiptQuery).mockReturnValue({
    data: { data: rows },
    isLoading: false,
  } as unknown as ReturnType<typeof useFlowTruckReceiptQuery>)
}

function setRailRows(rows: unknown[]) {
  vi.mocked(useRailReceiptPipelineQuery).mockReturnValue({
    data: { data: rows },
    isLoading: false,
  } as unknown as ReturnType<typeof useRailReceiptPipelineQuery>)
}

function makeRow(
  overrides: Partial<{
    id: string
    basisDocumentNumber: string
    contractorName: string
    pipelineStatus: 'PENDING' | 'DRAFT' | 'EXECUTED'
    contractorId: string
    basisDate: string
  }> = {},
) {
  return {
    id: overrides.id ?? '11111111-1111-4111-8111-111111111111',
    basisDocumentNumber: overrides.basisDocumentNumber ?? 'WB-001',
    contractorName: overrides.contractorName ?? 'Acme',
    contractorId: overrides.contractorId ?? '22222222-2222-4222-8222-222222222222',
    basisDate: overrides.basisDate ?? '2026-04-30T00:00:00Z',
    pipelineStatus: overrides.pipelineStatus ?? 'PENDING',
  }
}

interface HarnessProps {
  defaultValues?: Partial<AcceptanceCreate>
  locked?: boolean
  lockedHintNumber?: string
  onForm?: (form: ReturnType<typeof useForm<AcceptanceCreate>>) => void
  children?: ReactNode
}

function Harness({
  defaultValues,
  locked,
  lockedHintNumber,
  onForm,
  children,
}: HarnessProps) {
  const form = useForm<AcceptanceCreate>({
    defaultValues: { ...emptyAcceptanceCreate, ...defaultValues } as AcceptanceCreate,
  })
  onForm?.(form)
  return (
    <FormProvider {...form}>
      <AcceptanceBasisSection
        mode="create"
        locked={locked}
        lockedHintNumber={lockedHintNumber}
      />
      {children}
    </FormProvider>
  )
}

function renderHarness(props: HarnessProps = {}) {
  let formApi: ReturnType<typeof useForm<AcceptanceCreate>> | null = null
  const captured = (api: ReturnType<typeof useForm<AcceptanceCreate>>) => {
    formApi = api
  }
  const utils = render(<Harness {...props} onForm={captured} />)
  return {
    ...utils,
    form: () => {
      if (!formApi)
        throw new Error('form ref not captured')
      return formApi
    },
  }
}

describe('acceptanceBasisSection — unlocked tab switching', () => {
  it('renders all three tabs with EXTERNAL active by default', () => {
    setTruckRows([])
    setRailRows([])
    renderHarness()
    const externalTab = screen.getByRole('tab', { name: /external/i })
    expect(externalTab).toHaveAttribute('aria-selected', 'true')
    expect(screen.getByRole('tab', { name: /truck waybill/i })).toBeInTheDocument()
    expect(screen.getByRole('tab', { name: /rail waybill/i })).toBeInTheDocument()
  })

  it('switching to TRUCK tab updates arrivalType to TRUCK', async () => {
    setTruckRows([])
    setRailRows([])
    const { form } = renderHarness()
    await userEvent.click(screen.getByRole('tab', { name: /truck waybill/i }))
    expect(form().getValues('arrivalType')).toBe<ArrivalType>('TRUCK')
  })

  it('switching to RAIL tab updates arrivalType to RAIL', async () => {
    setTruckRows([])
    setRailRows([])
    const { form } = renderHarness()
    await userEvent.click(screen.getByRole('tab', { name: /rail waybill/i }))
    expect(form().getValues('arrivalType')).toBe<ArrivalType>('RAIL')
  })

  it('switching from EXTERNAL clears sourceEntity', async () => {
    setTruckRows([])
    setRailRows([])
    const { form } = renderHarness({
      defaultValues: { arrivalType: 'EXTERNAL', sourceEntity: 'Acme' },
    })
    await userEvent.click(screen.getByRole('tab', { name: /truck waybill/i }))
    expect(form().getValues('sourceEntity')).toBeNull()
  })

  it('switching from TRUCK clears truckWaybillId', async () => {
    setTruckRows([])
    setRailRows([])
    const { form } = renderHarness({
      defaultValues: {
        arrivalType: 'TRUCK',
        truckWaybillId: '33333333-3333-4333-8333-333333333333',
      },
    })
    await userEvent.click(screen.getByRole('tab', { name: /external/i }))
    expect(form().getValues('truckWaybillId')).toBeNull()
  })
})

describe('acceptanceBasisSection — locked mode', () => {
  it('hides inactive tabs when locked', () => {
    setTruckRows([])
    setRailRows([])
    renderHarness({
      defaultValues: {
        arrivalType: 'TRUCK',
        truckWaybillId: '44444444-4444-4444-8444-444444444444',
      },
      locked: true,
    })
    expect(screen.queryByRole('tab', { name: /external/i })).not.toBeInTheDocument()
    expect(screen.queryByRole('tab', { name: /rail waybill/i })).not.toBeInTheDocument()
    expect(screen.getByRole('tab', { name: /truck waybill/i })).toBeInTheDocument()
  })

  it('renders a lock glyph on the active locked tab', () => {
    setTruckRows([])
    setRailRows([])
    renderHarness({
      defaultValues: {
        arrivalType: 'TRUCK',
        truckWaybillId: '44444444-4444-4444-8444-444444444444',
      },
      locked: true,
    })
    const tab = screen.getByRole('tab', { name: /truck waybill/i })
    expect(tab.querySelector('[data-slot="basis-tab-lock"]')).not.toBeNull()
  })

  it('renders the locked-hint number when provided', () => {
    setTruckRows([])
    setRailRows([])
    renderHarness({
      defaultValues: {
        arrivalType: 'TRUCK',
        truckWaybillId: '44444444-4444-4444-8444-444444444444',
      },
      locked: true,
      lockedHintNumber: 'WB-2026-074',
    })
    expect(screen.getByText(/WB-2026-074/)).toBeInTheDocument()
  })
})

describe('acceptanceBasisSection — tab switch with non-empty items', () => {
  const oneItem = [
    {
      productId: '55555555-5555-4555-8555-555555555555',
      storageId: '66666666-6666-4666-8666-666666666666',
      acceptedAmount: '5',
    },
  ]

  it('shows a confirm dialog when items array is non-empty', async () => {
    setTruckRows([])
    setRailRows([])
    renderHarness({
      defaultValues: {
        arrivalType: 'EXTERNAL',
        items: oneItem,
      },
    })
    await userEvent.click(screen.getByRole('tab', { name: /truck waybill/i }))
    expect(screen.getByRole('alertdialog')).toBeInTheDocument()
  })

  it('cancel preserves items and previous tab', async () => {
    setTruckRows([])
    setRailRows([])
    const { form } = renderHarness({
      defaultValues: {
        arrivalType: 'EXTERNAL',
        items: oneItem,
      },
    })
    await userEvent.click(screen.getByRole('tab', { name: /truck waybill/i }))
    const dialog = screen.getByRole('alertdialog')
    await userEvent.click(within(dialog).getByRole('button', { name: /cancel/i }))
    expect(form().getValues('arrivalType')).toBe<ArrivalType>('EXTERNAL')
    expect(form().getValues('items')).toHaveLength(1)
  })

  it('confirm clears items and switches tab', async () => {
    setTruckRows([])
    setRailRows([])
    const { form } = renderHarness({
      defaultValues: {
        arrivalType: 'EXTERNAL',
        items: oneItem,
      },
    })
    await userEvent.click(screen.getByRole('tab', { name: /truck waybill/i }))
    const dialog = screen.getByRole('alertdialog')
    await userEvent.click(within(dialog).getByRole('button', { name: /continue/i }))
    expect(form().getValues('arrivalType')).toBe<ArrivalType>('TRUCK')
    expect(form().getValues('items')).toHaveLength(0)
  })
})

describe('acceptanceBasisSection — waybill picker (truck tab)', () => {
  it('lists only PENDING truck waybills', async () => {
    setTruckRows([
      makeRow({ id: 'a', basisDocumentNumber: 'WB-A', pipelineStatus: 'PENDING' }),
      makeRow({ id: 'b', basisDocumentNumber: 'WB-B', pipelineStatus: 'DRAFT' }),
      makeRow({ id: 'c', basisDocumentNumber: 'WB-C', pipelineStatus: 'EXECUTED' }),
    ])
    setRailRows([])
    renderHarness({ defaultValues: { arrivalType: 'TRUCK' } })
    // Open the select
    await userEvent.click(screen.getByRole('combobox'))
    // Only PENDING row appears as an option
    expect(screen.getByRole('option', { name: /WB-A/ })).toBeInTheDocument()
    expect(screen.queryByRole('option', { name: /WB-B/ })).not.toBeInTheDocument()
    expect(screen.queryByRole('option', { name: /WB-C/ })).not.toBeInTheDocument()
  })

  it('selecting a waybill writes truckWaybillId', async () => {
    const id = '77777777-7777-4777-8777-777777777777'
    setTruckRows([
      makeRow({ id, basisDocumentNumber: 'WB-Z', pipelineStatus: 'PENDING' }),
    ])
    setRailRows([])
    const { form } = renderHarness({ defaultValues: { arrivalType: 'TRUCK' } })
    await userEvent.click(screen.getByRole('combobox'))
    await userEvent.click(screen.getByRole('option', { name: /WB-Z/ }))
    expect(form().getValues('truckWaybillId')).toBe(id)
  })

  it('lists no options when no PENDING rows are available', async () => {
    // The picker delegates to the project-standard `EntityPickerInput`. The
    // PENDING filter excludes DRAFT/EXECUTED rows; with only a DRAFT row in
    // the dataset, the combobox opens with zero matching options.
    //
    // We assert by absence of any waybill-row option rather than by the
    // empty-state text — `cmdk`'s `<CommandEmpty>` is only rendered after a
    // search query is typed, so an opened-but-empty combobox renders no
    // visible empty-state by design.
    setTruckRows([
      makeRow({ id: 'd', basisDocumentNumber: 'WB-Drafted', pipelineStatus: 'DRAFT' }),
    ])
    setRailRows([])
    renderHarness({ defaultValues: { arrivalType: 'TRUCK' } })
    await userEvent.click(screen.getByRole('combobox'))
    expect(screen.queryByRole('option', { name: /WB-Drafted/ })).not.toBeInTheDocument()
  })
})

describe('acceptanceBasisSection — waybill picker (rail tab)', () => {
  it('lists only PENDING rail waybills', async () => {
    setTruckRows([])
    setRailRows([
      makeRow({ id: 'r1', basisDocumentNumber: 'RW-1', pipelineStatus: 'PENDING' }),
      makeRow({ id: 'r2', basisDocumentNumber: 'RW-2', pipelineStatus: 'EXECUTED' }),
    ])
    renderHarness({ defaultValues: { arrivalType: 'RAIL' } })
    await userEvent.click(screen.getByRole('combobox'))
    expect(screen.getByRole('option', { name: /RW-1/ })).toBeInTheDocument()
    expect(screen.queryByRole('option', { name: /RW-2/ })).not.toBeInTheDocument()
  })
})
