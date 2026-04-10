import { TRAILING_SLASHES } from '~/lib/utils'

interface RuntimeGlobals {
  __VOLETU_API_BASE_URL__?: string
}

export function normalizeApiBaseUrl(value: string): string {
  return value.replace(TRAILING_SLASHES, '')
}

export function getApiBaseUrl(): string {
  return normalizeApiBaseUrl(
    (globalThis as RuntimeGlobals).__VOLETU_API_BASE_URL__
    ?? import.meta.env.VITE_API_BASE_URL
    ?? 'http://127.0.0.1:3000',
  )
}

export function setApiBaseUrl(baseUrl: string): void {
  ;(globalThis as RuntimeGlobals).__VOLETU_API_BASE_URL__
    = normalizeApiBaseUrl(baseUrl)
}
