import { isTokenExpiringSoon } from '~/auth/session'
import { TRAILING_SLASHES } from '~/lib/utils'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'

export interface RequestConfig<TData = unknown> {
  url?: string
  method: 'GET' | 'PUT' | 'PATCH' | 'POST' | 'DELETE'
  params?: object
  data?: TData | FormData
  responseType?: 'arraybuffer' | 'blob' | 'document' | 'json' | 'text' | 'stream'
  signal?: AbortSignal
  headers?: HeadersInit
}

export interface ResponseConfig<TData = unknown> {
  data: TData
  status: number
  statusText: string
}

export type ResponseErrorConfig<TError = unknown> = TError

// ---------------------------------------------------------------------------
// Base URL
// ---------------------------------------------------------------------------

function normalizeBaseUrl(value: string): string {
  return value.replace(TRAILING_SLASHES, '')
}

export function getBaseUrl(): string {
  return normalizeBaseUrl(
    (globalThis as { __VOLETU_API_BASE_URL__?: string }).__VOLETU_API_BASE_URL__
    ?? 'http://127.0.0.1:3000',
  )
}

export function setApiBaseUrl(baseUrl: string): void {
  ;(globalThis as { __VOLETU_API_BASE_URL__?: string }).__VOLETU_API_BASE_URL__
    = normalizeBaseUrl(baseUrl)
}

// ---------------------------------------------------------------------------
// API client
// ---------------------------------------------------------------------------

interface ApiEnvelope {
  success: boolean
  error?: { message?: string } | null
}

const MUTATING_METHODS = new Set(['POST', 'PUT', 'PATCH', 'DELETE'])
const PROACTIVE_REFRESH_SECONDS = 300 // 5 minutes before expiry

/**
 * Kubb HTTP client. Every generated API call goes through this function.
 *
 * Auth flow:
 * 1. If access token is near expiry → proactive refresh (transparent)
 * 2. Attach token → send request
 * 3. If 401 → refresh → replay (transparent to caller)
 * 4. If refresh fails → logout → throw
 */
export async function client<TData, _TError = unknown, TVariables = unknown>(
  config: RequestConfig<TVariables>,
): Promise<ResponseConfig<TData>> {
  const store = useAuthStore.getState()

  // Proactive refresh: if token is valid but nearing expiry, refresh before sending
  if (store.status === 'valid' && store.accessToken
    && isTokenExpiringSoon(store.accessToken, PROACTIVE_REFRESH_SECONDS)) {
    await store.onUnauthorized()
  }

  const { accessToken } = useAuthStore.getState()
  const isMutating = MUTATING_METHODS.has(config.method.toUpperCase())

  // Serialize params as query string
  let url = `${getBaseUrl()}${config.url}`
  if (config.params && typeof config.params === 'object') {
    const searchParams = new URLSearchParams()
    for (const [key, value] of Object.entries(config.params)) {
      if (value !== undefined && value !== null)
        searchParams.set(key, String(value))
    }
    const qs = searchParams.toString()
    if (qs)
      url += `${url.includes('?') ? '&' : '?'}${qs}`
  }

  const response = await fetch(url, {
    method: config.method.toUpperCase(),
    body: config.data instanceof FormData
      ? config.data
      : config.data !== undefined
        ? JSON.stringify(config.data)
        : undefined,
    signal: config.signal,
    headers: {
      'Content-Type': 'application/json',
      ...(accessToken ? { Authorization: `Bearer ${accessToken}` } : {}),
      ...(isMutating ? { 'Idempotency-Key': crypto.randomUUID() } : {}),
      ...(config.headers ?? {}),
    },
  })

  // 401 → refresh → replay
  if (response.status === 401) {
    const refreshed = await useAuthStore.getState().onUnauthorized()
    if (refreshed)
      return client(config)
    throw new Error('Session expired')
  }

  // 403 NODE_NOT_INITIALIZED → update node store
  if (response.status === 403) {
    const cloned = response.clone()
    try {
      const body = await cloned.json() as { error?: { code?: string } }
      if (body?.error?.code === 'NODE_NOT_INITIALIZED') {
        useNodeStore.getState().setStatus({ isInitialized: false })
      }
    }
    catch { /* ignore parse failures */ }
  }

  if (!response.ok) {
    const text = await response.text()
    throw new Error(text || `Request failed with status ${response.status}`)
  }

  if (response.status === 204) {
    return { data: undefined as TData, status: 204, statusText: 'No Content' }
  }

  const envelope = await response.json() as TData & ApiEnvelope
  if (!envelope.success) {
    throw new Error(
      (envelope as ApiEnvelope).error?.message ?? 'Request failed',
    )
  }

  return {
    data: envelope,
    status: response.status,
    statusText: response.statusText,
  }
}

export type Client = typeof client
export default client
