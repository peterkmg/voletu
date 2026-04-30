import type { NodeType, WorkerState } from '~/stores/node-store'
import { client } from '~/api/client'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'
import { getApiBaseUrl } from './api-base-url'

interface ApiEnvelope<TData> {
  success: boolean
  data?: TData
  error?: { message?: string } | null
}

interface HealthPayload {
  status: string
  isInitialized: boolean
  nodeType: string | null
  nodeName: string | null
}

interface NodeStatusPayload {
  isInitialized: boolean
  nodeType: string | null
  nodeName: string | null
  workerState: string | null
  lastSyncAt: string | null
  centralApiUrl: string | null
  assignedBaseIds?: string[]
}

export interface HealthSnapshot {
  status: string
  isInitialized: boolean
  nodeType: NodeType | null
  nodeName: string | null
}

export interface NodeStatusSnapshot {
  isInitialized: boolean
  nodeType: NodeType | null
  nodeName: string | null
  workerState: WorkerState | null
  lastSyncAt: string | null
  centralApiUrl: string | null
  assignedBaseIds: string[]
}

function toNodeType(value: string | null): NodeType | null {
  return value === 'CENTRAL' || value === 'PERIPHERAL' ? value : null
}

function toWorkerState(value: string | null): WorkerState | null {
  switch (value) {
    case 'Sleeping':
    case 'Offline':
    case 'OnlineIdle':
    case 'Syncing':
    case 'Backoff':
      return value
    default:
      return null
  }
}

function delay(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

async function readEnvelope<TData>(
  response: Response,
  fallbackMessage: string,
): Promise<TData> {
  if (!response.ok) {
    throw new Error(`${fallbackMessage}: ${response.status}`)
  }

  const envelope = await response.json() as ApiEnvelope<TData>
  if (!envelope.success || envelope.data === undefined) {
    throw new Error(envelope.error?.message ?? fallbackMessage)
  }

  return envelope.data
}

export async function fetchHealth(
  options?: { baseUrl?: string, signal?: AbortSignal },
): Promise<HealthSnapshot> {
  const response = await fetch(`${options?.baseUrl ?? getApiBaseUrl()}/health`, {
    signal: options?.signal,
  })

  const payload = await readEnvelope<HealthPayload>(response, 'Health check failed')

  return {
    status: payload.status,
    isInitialized: payload.isInitialized,
    nodeType: toNodeType(payload.nodeType),
    nodeName: payload.nodeName,
  }
}

export async function checkHealth(baseUrl: string): Promise<boolean> {
  try {
    await fetchHealth({ baseUrl })
    return true
  }
  catch {
    return false
  }
}

export function applyHealthSnapshot(health: HealthSnapshot): void {
  useNodeStore.getState().setStatus({
    isInitialized: health.isInitialized,
    nodeType: health.nodeType,
    nodeName: health.nodeName,
  })
}

export async function fetchNodeStatus(
  options?: { accessToken?: string | null, baseUrl?: string, signal?: AbortSignal },
): Promise<NodeStatusSnapshot> {
  if (options?.accessToken === undefined && options?.baseUrl === undefined) {
    const response = await client<ApiEnvelope<NodeStatusPayload>>({
      method: 'GET',
      url: '/node/status',
      signal: options?.signal,
    })

    return toNodeStatusSnapshot(response.data.data!)
  }

  const accessToken = options?.accessToken ?? useAuthStore.getState().accessToken
  if (!accessToken) {
    throw new Error('Missing access token')
  }

  const response = await fetch(`${options?.baseUrl ?? getApiBaseUrl()}/node/status`, {
    signal: options?.signal,
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  })
  const payload = await readEnvelope<NodeStatusPayload>(
    response,
    'Node status request failed',
  )

  return toNodeStatusSnapshot(payload)
}

function toNodeStatusSnapshot(payload: NodeStatusPayload): NodeStatusSnapshot {
  return {
    isInitialized: payload.isInitialized,
    nodeType: toNodeType(payload.nodeType),
    nodeName: payload.nodeName,
    workerState: toWorkerState(payload.workerState),
    lastSyncAt: payload.lastSyncAt,
    centralApiUrl: payload.centralApiUrl,
    assignedBaseIds: payload.assignedBaseIds ?? [],
  }
}

export function applyNodeStatusSnapshot(status: NodeStatusSnapshot): void {
  useNodeStore.getState().setStatus(status)
}

export async function waitForApiHealthy(
  baseUrl: string,
  options?: { maxAttempts?: number, intervalMs?: number },
): Promise<HealthSnapshot> {
  const maxAttempts = options?.maxAttempts ?? 30
  const intervalMs = options?.intervalMs ?? 500

  for (let attempt = 0; attempt < maxAttempts; attempt++) {
    try {
      return await fetchHealth({ baseUrl })
    }
    catch {
      if (attempt === maxAttempts - 1) {
        break
      }
      await delay(intervalMs)
    }
  }

  throw new Error('API did not become healthy in time')
}
