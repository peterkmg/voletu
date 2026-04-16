import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { useNodeStore } from '~/stores/node-store'
import { ChangeCentralUrlDialog } from '../components/change-central-url-dialog'

interface ApiConfig { method: string, url: string, data?: unknown }
const apiCalls: ApiConfig[] = []
let nextError: { message: string } | null = null

vi.mock('~/api/client', () => ({
  client: vi.fn(async (config: ApiConfig) => {
    apiCalls.push(config)
    if (nextError) {
      const err = nextError
      nextError = null
      throw err
    }
    return { data: { success: true, data: {} } }
  }),
}))

function renderDialog(onOpenChange: (open: boolean) => void = () => {}) {
  const qc = new QueryClient({ defaultOptions: { queries: { retry: false }, mutations: { retry: false } } })
  return render(
    <QueryClientProvider client={qc}>
      <ChangeCentralUrlDialog open={true} onOpenChange={onOpenChange} />
    </QueryClientProvider>,
  )
}

beforeEach(() => {
  apiCalls.length = 0
  nextError = null
  useNodeStore.getState().reset()
  useNodeStore.getState().setStatus({
    nodeType: 'PERIPHERAL',
    isInitialized: true,
    centralApiUrl: 'https://old.example.com',
  })
})

afterEach(() => {
  vi.clearAllMocks()
})

describe('changeCentralUrlDialog', () => {
  it('renders with the current central URL pre-filled', () => {
    renderDialog()
    const input = screen.getByRole('textbox', { name: /central api url/i }) as HTMLInputElement
    expect(input.value).toBe('https://old.example.com')
  })

  it('rejects a malformed URL without hitting the API', async () => {
    const user = userEvent.setup()
    renderDialog()
    const input = screen.getByRole('textbox', { name: /central api url/i })
    await user.clear(input)
    await user.type(input, 'not-a-url')
    await user.click(screen.getByRole('button', { name: /^apply$/i }))

    // Give react-hook-form a chance to run validation. The API must not be hit.
    await waitFor(() => expect(apiCalls.length).toBe(0))
    // aria-invalid is set on the input when validation fails.
    expect(input).toHaveAttribute('aria-invalid', 'true')
  })

  it('rejects URLs without http/https scheme', async () => {
    const user = userEvent.setup()
    renderDialog()
    const input = screen.getByRole('textbox', { name: /central api url/i })
    await user.clear(input)
    await user.type(input, 'ftp://central.example.com')
    await user.click(screen.getByRole('button', { name: /^apply$/i }))

    await screen.findByText(/must start with http/i)
    expect(apiCalls.length).toBe(0)
  })

  it('submits a PATCH and closes the dialog on success', async () => {
    const user = userEvent.setup()
    const onOpenChange = vi.fn()
    renderDialog(onOpenChange)

    const input = screen.getByRole('textbox', { name: /central api url/i })
    await user.clear(input)
    await user.type(input, 'https://new.example.com')
    await user.click(screen.getByRole('button', { name: /^apply$/i }))

    await waitFor(() => expect(apiCalls.length).toBe(1))
    const call = apiCalls[0]!
    expect(call.method).toBe('PATCH')
    expect(call.url).toBe('/node/central-api-url')
    expect(call.data).toEqual({ url: 'https://new.example.com' })

    await waitFor(() => expect(onOpenChange).toHaveBeenCalledWith(false))
  })

  it('trims whitespace from the submitted URL', async () => {
    const user = userEvent.setup()
    renderDialog()
    const input = screen.getByRole('textbox', { name: /central api url/i })
    await user.clear(input)
    await user.type(input, '  https://trimmed.example.com  ')
    await user.click(screen.getByRole('button', { name: /^apply$/i }))

    await waitFor(() => expect(apiCalls.length).toBe(1))
    expect((apiCalls[0]!.data as { url: string }).url).toBe('https://trimmed.example.com')
  })

  it('keeps the dialog open on API error', async () => {
    const user = userEvent.setup()
    const onOpenChange = vi.fn()
    renderDialog(onOpenChange)

    nextError = { message: 'Central API at https://x is unreachable' }

    const input = screen.getByRole('textbox', { name: /central api url/i })
    await user.clear(input)
    await user.type(input, 'https://x.example.com')
    await user.click(screen.getByRole('button', { name: /^apply$/i }))

    await waitFor(() => expect(apiCalls.length).toBe(1))
    // Dialog must NOT be closed on error.
    expect(onOpenChange).not.toHaveBeenCalledWith(false)
  })

  it('cancel closes the dialog without calling the API', async () => {
    const user = userEvent.setup()
    const onOpenChange = vi.fn()
    renderDialog(onOpenChange)

    await user.click(screen.getByRole('button', { name: /cancel/i }))

    expect(apiCalls.length).toBe(0)
    expect(onOpenChange).toHaveBeenCalledWith(false)
  })
})
