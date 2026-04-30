import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it } from 'vitest'
import { useNodeStore } from '~/stores/node-store'
import { SyncReadinessCard } from '~/views/system/sync/components/sync-readiness-card'

function renderCard() {
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

  describe('bootstrap/peripheral pre-setup branch', () => {
    it('shows setup checklist (not skeleton) when peripheral is initialized but bases are not loaded', () => {
      useNodeStore.getState().setStatus({
        nodeType: 'PERIPHERAL',
        isInitialized: true,
        assignedBaseIds: [],
      })

      renderCard()

      expect(screen.getByText(/sync configuration/i)).toBeInTheDocument()
      expect(screen.getByText(/node initialized/i)).toBeInTheDocument()
      expect(screen.getByText(/bases assigned/i)).toBeInTheDocument()
      expect(screen.getByRole('button', { name: /assign bases/i })).toBeInTheDocument()
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
      useNodeStore.getState().setStatus({ workerState: 'OnlineIdle' })
      useNodeStore.getState().setStatus({ workerState: 'Offline' })

      renderCard()
      const centralConnectedItem = screen.getByText(/central connection verified/i)

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

      expect(screen.getAllByText(/^online$/i).length).toBeGreaterThan(0)
    })

    it('does NOT show the checklist labels', () => {
      renderCard()
      expect(screen.queryByText(/node initialized/i)).toBeNull()
    })

    it('hides the expanded content by default', () => {
      renderCard()

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

      expect(await screen.findByText(/change central api url/i)).toBeInTheDocument()
    })

    it('opens the base-assignment dialog when "Manage…" is clicked', async () => {
      const user = userEvent.setup()
      renderCard()
      await user.click(screen.getByTestId('sync-readiness-trigger'))
      const manageBtn = await screen.findByRole('button', { name: /manage/i })
      await user.click(manageBtn)

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
