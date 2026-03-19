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

export async function client<TData, _TError = unknown, TVariables = unknown>(config: RequestConfig<TVariables>): Promise<ResponseConfig<TData>> {
  const token = localStorage.getItem('accessToken')
  const isMutating = MUTATING_METHODS.has(config.method.toUpperCase())

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
