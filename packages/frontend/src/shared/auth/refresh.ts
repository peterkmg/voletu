import type { AuthSession } from './session'

type RefreshFn = (refreshToken: string) => Promise<AuthSession>

export interface RefreshLock {
  acquire: (refreshToken: string) => Promise<AuthSession>
}

export function createRefreshLock(refreshFn: RefreshFn): RefreshLock {
  let inflight: Promise<AuthSession> | null = null

  return {
    acquire(refreshToken: string): Promise<AuthSession> {
      if (!inflight) {
        inflight = refreshFn(refreshToken).finally(() => {
          inflight = null
        })
      }
      return inflight
    },
  }
}

import type { LoginResponse } from '~/generated/types/LoginResponse'
import { toAuthSession } from './session'

/** Resolve the API base URL from the same global as kubb-client, without importing kubb-client. */
function getApiBaseUrl(): string {
  return ((globalThis as { __VOLETU_API_BASE_URL__?: string }).__VOLETU_API_BASE_URL__ ?? 'http://127.0.0.1:3000').replace(/\/+$/, '')
}

async function callRefreshEndpoint(refreshToken: string): Promise<AuthSession> {
  const response = await fetch(`${getApiBaseUrl()}/auth/refresh`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Idempotency-Key': crypto.randomUUID(),
    },
    body: JSON.stringify({ refreshToken }),
  })

  if (!response.ok) {
    throw new Error(`Refresh failed with status ${response.status}`)
  }

  const envelope = await response.json() as { success: boolean, data?: LoginResponse, error?: { message?: string } }
  if (!envelope.success || !envelope.data) {
    throw new Error(envelope.error?.message ?? 'Refresh failed')
  }

  return toAuthSession(envelope.data)
}

export const refreshLock = createRefreshLock(callRefreshEndpoint)
