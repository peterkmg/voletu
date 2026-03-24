function normalizeBaseUrl(value: string): string {
  return value.replace(/\/+$/, '')
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
