import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'
import { isTokenExpiringSoon } from '~/shared/auth/jwt-decode'
import { refreshLock } from '~/shared/auth/refresh'

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

function normalizeBaseUrl(value: string): string {
  return value.replace(/\/+$/, '')
}

const API_BASE_URL = normalizeBaseUrl(
  (globalThis as { __VOLETU_API_BASE_URL__?: string }).__VOLETU_API_BASE_URL__
  ?? 'http://127.0.0.1:3000',
)

interface ApiEnvelope {
  success: boolean
  error?: { message?: string } | null
}

/**
 * Kubb's custom client. Called for every generated API request.
 *
 * Returns the full JSON response body as ResponseConfig.data — which matches the
 * generated OpenAPI types (e.g. ApiResponseVecBaseResponse includes success/data/error).
 * Throws before returning when success is false, so callers never see error envelopes.
 *
 * Feature code accesses the inner data via: query.data?.data ?? []
 */
const MUTATING_METHODS = new Set(['POST', 'PUT', 'PATCH', 'DELETE'])

const PROACTIVE_REFRESH_THRESHOLD_SECONDS = 300 // 5 minutes

/**
 * Get the current access token from zustand, optionally refreshing
 * if it is expiring soon.
 */
async function getValidToken(): Promise<string | null> {
  const { accessToken, refreshToken } = useAuthStore.getState().auth
  if (!accessToken) return null

  if (isTokenExpiringSoon(accessToken, PROACTIVE_REFRESH_THRESHOLD_SECONDS) && refreshToken) {
    try {
      const newSession = await refreshLock.acquire(refreshToken)
      useAuthStore.getState().auth.setSession(newSession)
      return newSession.accessToken
    }
    catch {
      return accessToken
    }
  }

  return accessToken
}

/**
 * Attempt a silent token refresh and return the new access token.
 * Returns null if refresh is not possible.
 */
async function attemptSilentRefresh(): Promise<string | null> {
  const { refreshToken } = useAuthStore.getState().auth
  if (!refreshToken) return null

  try {
    const newSession = await refreshLock.acquire(refreshToken)
    useAuthStore.getState().auth.setSession(newSession)
    return newSession.accessToken
  }
  catch {
    useAuthStore.getState().auth.reset()
    return null
  }
}

export async function client<TData, _TError = unknown, TVariables = unknown>(
  config: RequestConfig<TVariables>,
  _isRetry = false,
): Promise<ResponseConfig<TData>> {
  const isMutating = MUTATING_METHODS.has(config.method.toUpperCase())
  const token = await getValidToken()

  const response = await fetch(`${API_BASE_URL}${config.url}`, {
    method: config.method.toUpperCase(),
    body: config.data instanceof FormData
      ? config.data
      : config.data !== undefined
        ? JSON.stringify(config.data)
        : undefined,
    signal: config.signal,
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(isMutating ? { 'Idempotency-Key': crypto.randomUUID() } : {}),
      ...(config.headers ?? {}),
    },
  })

  // 401 Interceptor: check response.status directly (not string matching).
  // On 401, attempt silent refresh and replay the original request once.
  if (response.status === 401 && !_isRetry) {
    const newToken = await attemptSilentRefresh()
    if (newToken) {
      // Replay the original request with the new token.
      // For mutations: the replay generates a new auto-UUID idempotency key.
      // This is safe because the backend's idempotency middleware removes keys
      // from cache on non-2xx responses (the original 401 was non-2xx).
      // If the caller provided a stable key via config.headers, it is preserved
      // on replay since config is passed through unchanged.
      return client<TData, _TError, TVariables>(config, true)
    }
    // Refresh failed — fall through to throw below.
  }

  // 403 NODE_NOT_INITIALIZED interceptor: update the node store so the
  // banner/UI reflects the uninitialized state immediately.
  if (response.status === 403) {
    const cloned = response.clone()
    try {
      const body = await cloned.json() as { error?: { code?: string } }
      if (body?.error?.code === 'NODE_NOT_INITIALIZED') {
        useNodeStore.getState().setStatus({ isInitialized: false })
      }
    }
    catch { /* ignore parse failures, fall through to generic error */ }
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
