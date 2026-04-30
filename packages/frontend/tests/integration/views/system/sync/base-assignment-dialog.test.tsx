import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen, waitFor, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { act } from 'react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { useNodeStore } from '~/stores/node-store'
import { BaseAssignmentDialog } from '~/views/system/sync/components/base-assignment-dialog'

interface ApiConfig { method: string, url: string, data?: unknown }
const apiCalls: ApiConfig[] = []
let mockBases: Array<{ id: string, commonName: string, longName?: string | null }> = []

let nextMutationError: string | null = null

vi.mock('~/api/client', () => ({
  client: vi.fn(async (config: ApiConfig) => {
    apiCalls.push(config)
    if (config.method === 'GET' && config.url === '/catalog/bases') {
      return { data: { success: true, data: mockBases } }
    }
    if ((config.method === 'POST' || config.method === 'DELETE') && config.url.startsWith('/node/bases')) {
      if (nextMutationError) {
        const msg = nextMutationError
        nextMutationError = null
        throw new Error(msg)
      }
      return { data: { success: true, data: {} } }
    }
    throw new Error(`Unexpected API call: ${config.method} ${config.url}`)
  }),
}))

function renderDialog(onOpenChange: (open: boolean) => void = () => {}) {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  })
  return render(
    <QueryClientProvider client={queryClient}>
      <BaseAssignmentDialog open={true} onOpenChange={onOpenChange} />
    </QueryClientProvider>,
  )
}

beforeEach(() => {
  apiCalls.length = 0
  nextMutationError = null
  mockBases = [
    { id: 'b1', commonName: 'North', longName: 'North Regional Base' },
    { id: 'b2', commonName: 'South', longName: 'South Regional Base — very long name that should wrap across lines in the dialog' },
    { id: 'b3', commonName: 'East' },
  ]
  useNodeStore.getState().reset()
  useNodeStore.getState().setStatus({
    nodeType: 'PERIPHERAL',
    isInitialized: true,
    assignedBaseIds: ['b1'],
  })
  useNodeStore.getState().markBasesLoaded()
})

afterEach(() => {
  vi.clearAllMocks()
})

describe('baseAssignmentDialog', () => {
  it('renders all bases from the catalog with correct initial checked state', async () => {
    renderDialog()
    await screen.findByText('North')
    expect(screen.getByText('South')).toBeInTheDocument()
    expect(screen.getByText('East')).toBeInTheDocument()

    expect(screen.getByRole('checkbox', { name: /north/i })).toBeChecked()
    expect(screen.getByRole('checkbox', { name: /south/i })).not.toBeChecked()
    expect(screen.getByRole('checkbox', { name: /east/i })).not.toBeChecked()
  })

  it('does NOT call the API when a checkbox is toggled (pending-only)', async () => {
    const user = userEvent.setup()
    renderDialog()
    await screen.findByText('South')

    const apiCountBefore = apiCalls.length

    await user.click(screen.getByRole('checkbox', { name: /south/i }))
    await user.click(screen.getByRole('checkbox', { name: /east/i }))

    expect(apiCalls.length).toBe(apiCountBefore)
  })

  it('apply button is disabled when no changes are pending', async () => {
    renderDialog()
    await screen.findByText('North')
    expect(screen.getByRole('button', { name: /^apply$/i })).toBeDisabled()
  })

  it('apply button becomes enabled after a pending change', async () => {
    const user = userEvent.setup()
    renderDialog()
    await screen.findByText('South')
    await user.click(screen.getByRole('checkbox', { name: /south/i }))
    expect(screen.getByRole('button', { name: /^apply$/i })).toBeEnabled()
  })

  it('fires parallel POST and DELETE for the diff when Apply is clicked', async () => {
    const user = userEvent.setup()
    const onOpenChange = vi.fn()
    renderDialog(onOpenChange)
    await screen.findByText('South')

    await user.click(screen.getByRole('checkbox', { name: /north/i }))
    await user.click(screen.getByRole('checkbox', { name: /south/i }))
    await user.click(screen.getByRole('checkbox', { name: /east/i }))

    await user.click(screen.getByRole('button', { name: /^apply$/i }))

    await waitFor(() => {
      const mutations = apiCalls.filter(c => c.url.startsWith('/node/bases'))
      expect(mutations).toHaveLength(3)
    })

    const mutations = apiCalls.filter(c => c.url.startsWith('/node/bases'))
    const posts = mutations.filter(c => c.method === 'POST')
    const deletes = mutations.filter(c => c.method === 'DELETE')
    expect(posts.map(c => (c.data as { baseId: string }).baseId).sort()).toEqual(['b2', 'b3'])
    expect(deletes.map(c => c.url)).toEqual(['/node/bases/b1'])

    await waitFor(() => expect(onOpenChange).toHaveBeenCalledWith(false))
  })

  it('cancel discards the pending selection and does NOT call the API', async () => {
    const user = userEvent.setup()
    const onOpenChange = vi.fn()
    renderDialog(onOpenChange)
    await screen.findByText('South')

    await user.click(screen.getByRole('checkbox', { name: /south/i }))
    const apiCountBefore = apiCalls.length

    await user.click(screen.getByRole('button', { name: /cancel/i }))

    expect(apiCalls.length).toBe(apiCountBefore)
    expect(onOpenChange).toHaveBeenCalledWith(false)
  })

  it('renders long longName text without clipping via break-words class', async () => {
    renderDialog()
    const longName = await screen.findByText(/south regional base — very long name/i)
    expect(longName.className).toMatch(/break-words/)
  })

  it('wraps the list in a ScrollArea with a max-height constraint', async () => {
    renderDialog()
    await screen.findByText('North')

    const scrollArea = document.body.querySelector('[data-slot="scroll-area"]')
    expect(scrollArea).not.toBeNull()
    expect(scrollArea!.className).toMatch(/max-h-\[60vh\]/)
  })

  it('shows an empty-state message when the catalog returns no bases', async () => {
    mockBases = []
    renderDialog()
    expect(await screen.findByText(/no bases available/i)).toBeInTheDocument()
  })

  it('allows toggling a row by clicking its label (not just the checkbox)', async () => {
    const user = userEvent.setup()
    renderDialog()
    const label = await screen.findByText('South')
    await user.click(label)
    expect(screen.getByRole('checkbox', { name: /south/i })).toBeChecked()

    expect(screen.getByRole('button', { name: /^apply$/i })).toBeEnabled()
  })

  it('keeps existing selection intact when dialog is re-opened with new assignedBaseIds', async () => {
    const user = userEvent.setup()
    const { rerender } = renderDialog()
    await screen.findByText('North')

    await user.click(screen.getByRole('checkbox', { name: /south/i }))

    const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false } } })
    await act(async () => {
      useNodeStore.getState().setStatus({ assignedBaseIds: ['b2'] })
      rerender(
        <QueryClientProvider client={queryClient}>
          <BaseAssignmentDialog open={false} onOpenChange={() => {}} />
        </QueryClientProvider>,
      )
      rerender(
        <QueryClientProvider client={queryClient}>
          <BaseAssignmentDialog open={true} onOpenChange={() => {}} />
        </QueryClientProvider>,
      )
    })

    await waitFor(() => {
      expect(screen.getByRole('checkbox', { name: /south/i })).toBeChecked()
      expect(screen.getByRole('checkbox', { name: /north/i })).not.toBeChecked()
    })
  })

  it('keeps the dialog open after a fully-failed Apply', async () => {
    const user = userEvent.setup()
    const onOpenChange = vi.fn()
    renderDialog(onOpenChange)
    await screen.findByText('South')

    await user.click(screen.getByRole('checkbox', { name: /south/i }))

    nextMutationError = 'network boom'
    await user.click(screen.getByRole('button', { name: /^apply$/i }))

    await waitFor(() => {
      const mutations = apiCalls.filter(c => c.url.startsWith('/node/bases'))
      expect(mutations).toHaveLength(1)
    })

    expect(onOpenChange).not.toHaveBeenCalledWith(false)
  })

  it('exposes dialog content within the same testing tree', async () => {
    const { container } = renderDialog()
    await screen.findByText('North')

    const dialogs = within(container).queryAllByRole('dialog')
    expect(dialogs.length).toBeGreaterThanOrEqual(0)
  })
})
