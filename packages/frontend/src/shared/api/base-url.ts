function normalizeBaseUrl(value: string): string {
  return value.replace(/\/+$/, '')
}

export function setApiBaseUrl(baseUrl: string): void {
  ;(globalThis as { __VOLETU_API_BASE_URL__?: string }).__VOLETU_API_BASE_URL__
    = normalizeBaseUrl(baseUrl)
}
