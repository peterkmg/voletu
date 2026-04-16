import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it } from 'vitest'
import { useNodeStore } from '~/stores/node-store'
import { SyncReadinessCard } from '../components/sync-readiness-card'

function renderCard() {
  // A plain QueryClient is needed because BaseAssignmentDialog uses useQuery.
  // The dialog only runs its queries when opened, so the provider is enough.
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } })
  return render(
    <QueryClientProvider client={client}>
      <SyncReadinessCard />
    </QueryClientProvider>,
  )
}

beforeEach(() => {
  useNodeStore.getState().reset()
})

describe('syncReadinessCard', () => {
  it('renders nothing for a CENTRAL node', () => {
    useNodeStore.getState().setStatus({
      nodeType: 'CENTRAL',
      isInitialized: true,
    })
    const { container } = renderCard()
    expect(container.firstChild).toBeNull()
  })

  describe('setupLoading branch (peripheral, bases not yet loaded)', () => {
    it('shows the skeleton and does NOT render the checklist', () => {
      useNodeStore.getState().setStatus({
        nodeType: 'PERIPHERAL',
        isInitialized: true,
      })
      // basesLoaded intentionally left false.
      renderCard()
      // Skeletons are not addressable by role; check that the checklist step
      // labels are absent to confirm we are in the loading branch.
      expect(screen.queryByText(/node initialized/i)).toBeNull()
      expect(screen.queryByText(/central connection verified/i)).toBeNull()
      // And the page title ("Sync Configuration") is still shown.
      expect(screen.getByText(/sync configuration/i)).toBeInTheDocument()
    })
  })

  describe('setupIncomplete branch (peripheral, no bases)', () => {
    beforeEach(() => {
      useNodeStore.getState().setStatus({
        nodeType: 'PERIPHERAL',
        isInitialized: true,
        assignedBaseIds: [],
      })
      useNodeStore.getState().markBasesLoaded()
    })

    it('shows the checklist with the "Assign bases" action', () => {
      renderCard()
      expect(screen.getByText(/node initialized/i)).toBeInTheDocument()
      expect(screen.getByText(/central connection verified/i)).toBeInTheDocument()
      expect(screen.getByText(/bases assigned/i)).toBeInTheDocument()
      expect(screen.getByRole('button', { name: /assign bases/i })).toBeInTheDocument()
    })

    it('keeps the central-connection step checked after a transient Offline, once verified', () => {
      // Observe a reachable state first, then flip to Offline.
      useNodeStore.getState().setStatus({ workerState: 'OnlineIdle' })
      useNodeStore.getState().setStatus({ workerState: 'Offline' })

      renderCard()
      const centralConnectedItem = screen.getByText(/central connection verified/i)
      // The "done" state is conveyed via the line-through class on the <span>.
      expect(centralConnectedItem.className).toMatch(/line-through/)
    })

    it('leaves the central-connection step unchecked if reachable state was never observed', () => {
      useNodeStore.getState().setStatus({ workerState: 'Backoff' })
      renderCard()
      const centralConnectedItem = screen.getByText(/central connection verified/i)
      expect(centralConnectedItem.className).not.toMatch(/line-through/)
    })
  })

  describe('setupComplete branch (collapsed configured card)', () => {
    beforeEach(() => {
      useNodeStore.getState().setStatus({
        nodeType: 'PERIPHERAL',
        isInitialized: true,
        assignedBaseIds: ['base-1', 'base-2'],
        workerState: 'OnlineIdle',
        centralApiUrl: 'https://central.example.com',
      })
      useNodeStore.getState().markBasesLoaded()
    })

    it('renders the "Node configured" title', () => {
      renderCard()
      expect(screen.getByText(/node configured/i)).toBeInTheDocument()
    })

    it('renders the runtime status subtitle (Online)', () => {
      renderCard()
      // "Online" appears in the collapsed header.
      expect(screen.getAllByText(/^online$/i).length).toBeGreaterThan(0)
    })

    it('does NOT show the checklist labels', () => {
      renderCard()
      expect(screen.queryByText(/node initialized/i)).toBeNull()
    })

    it('hides the expanded content by default', () => {
      renderCard()
      // Central API URL row is behind the collapsible; "break-all" text not
      // rendered unless expanded.
      expect(screen.queryByText('https://central.example.com')).toBeNull()
    })

    it('reveals the central URL and bases row when expanded', async () => {
      const user = userEvent.setup()
      renderCard()
      await user.click(screen.getByTestId('sync-readiness-trigger'))
      expect(await screen.findByText('https://central.example.com')).toBeInTheDocument()
      expect(screen.getByText(/2 bases assigned/i)).toBeInTheDocument()
    })

    it('opens the ChangeCentralUrlDialog when "Change…" is clicked', async () => {
      const user = userEvent.setup()
      renderCard()
      await user.click(screen.getByTestId('sync-readiness-trigger'))
      const changeBtn = await screen.findByRole('button', { name: /change/i })
      expect(changeBtn).toBeEnabled()
      await user.click(changeBtn)
      // Dialog title from i18n (sync.centralUrl.title).
      expect(await screen.findByText(/change central api url/i)).toBeInTheDocument()
    })

    it('opens the base-assignment dialog when "Manage…" is clicked', async () => {
      const user = userEvent.setup()
      renderCard()
      await user.click(screen.getByTestId('sync-readiness-trigger'))
      const manageBtn = await screen.findByRole('button', { name: /manage/i })
      await user.click(manageBtn)
      // The dialog renders its own title once open.
      expect(await screen.findByText(/base assignment/i)).toBeInTheDocument()
    })

    it('falls back to "Not set" if centralApiUrl is null', async () => {
      useNodeStore.getState().setStatus({ centralApiUrl: null })
      const user = userEvent.setup()
      renderCard()
      await user.click(screen.getByTestId('sync-readiness-trigger'))
      expect(await screen.findByText(/not set/i)).toBeInTheDocument()
    })

    it('shows "Offline" subtitle when setup complete but worker is Offline', () => {
      useNodeStore.getState().setStatus({ workerState: 'Offline' })
      renderCard()
      expect(screen.getByText(/node configured/i)).toBeInTheDocument()
      expect(screen.getAllByText(/^offline$/i).length).toBeGreaterThan(0)
    })
  })
})
