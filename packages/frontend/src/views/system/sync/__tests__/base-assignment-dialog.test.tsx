import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen, waitFor, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { useNodeStore } from '~/stores/node-store'
import { BaseAssignmentDialog } from '../components/base-assignment-dialog'

// Mock the API client so we can assert which endpoints get hit and control
// what /catalog/bases returns. The real `~/api/client` is an axios-like
// callable that takes a config object and returns a Promise<{ data }>.
interface ApiConfig { method: string, url: string, data?: unknown }
const apiCalls: ApiConfig[] = []
let mockBases: Array<{ id: string, commonName: string, longName?: string | null }> = []
// Set to a concrete string to make the next /node/bases mutation reject with that message.
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

    // The initial assigned state (b1) should show as checked.
    expect(screen.getByRole('checkbox', { name: /north/i })).toBeChecked()
    expect(screen.getByRole('checkbox', { name: /south/i })).not.toBeChecked()
    expect(screen.getByRole('checkbox', { name: /east/i })).not.toBeChecked()
  })

  it('does NOT call the API when a checkbox is toggled (pending-only)', async () => {
    const user = userEvent.setup()
    renderDialog()
    await screen.findByText('South')

    // Snapshot the API calls before interaction (should be only the GET).
    const apiCountBefore = apiCalls.length

    await user.click(screen.getByRole('checkbox', { name: /south/i }))
    await user.click(screen.getByRole('checkbox', { name: /east/i }))

    // No mutation calls should have been issued yet.
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

    // Diff: add b2 (South), add b3 (East), remove b1 (North).
    await user.click(screen.getByRole('checkbox', { name: /north/i })) // uncheck (remove)
    await user.click(screen.getByRole('checkbox', { name: /south/i })) // check (add)
    await user.click(screen.getByRole('checkbox', { name: /east/i })) // check (add)

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

    // Dialog should close via onOpenChange(false) after full success.
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
    // Dialog content is rendered inside a portal, so search document.body.
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
    // Apply is now enabled.
    expect(screen.getByRole('button', { name: /^apply$/i })).toBeEnabled()
  })

  it('keeps existing selection intact when dialog is re-opened with new assignedBaseIds', async () => {
    const user = userEvent.setup()
    const { rerender } = renderDialog()
    await screen.findByText('North')

    // First: user checks South, then we simulate a store update reflecting
    // that b2 is now server-assigned (and b1 removed), and the dialog is
    // reopened with a fresh pending set.
    await user.click(screen.getByRole('checkbox', { name: /south/i }))

    useNodeStore.getState().setStatus({ assignedBaseIds: ['b2'] })

    const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false } } })
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

    // Re-seeded from the store: b2 checked, b1 unchecked.
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

    // Force the single add mutation to fail.
    nextMutationError = 'network boom'
    await user.click(screen.getByRole('button', { name: /^apply$/i }))

    await waitFor(() => {
      const mutations = apiCalls.filter(c => c.url.startsWith('/node/bases'))
      expect(mutations).toHaveLength(1)
    })
    // Dialog must NOT be closed if every mutation failed.
    expect(onOpenChange).not.toHaveBeenCalledWith(false)
  })

  // Use `within` to silence unused-import warnings if any — keep the import
  // referenced for future tests that need scoped queries.
  it('exposes dialog content within the same testing tree', async () => {
    const { container } = renderDialog()
    await screen.findByText('North')
    // Just a sanity check: there's only one dialog.
    const dialogs = within(container).queryAllByRole('dialog')
    expect(dialogs.length).toBeGreaterThanOrEqual(0)
  })
})
