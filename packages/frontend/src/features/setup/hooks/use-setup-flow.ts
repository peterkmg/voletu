import type { SaveLocalConfigPayload } from '~/shared/tauri/commands'
import { useState } from 'react'
import { setApiBaseUrl } from '~/shared/api/base-url'
import {
  saveLocalConfig,
  saveRemoteConfig,
  startLocalApi,
} from '~/shared/tauri/commands'
import { useStartupStore } from '~/stores/startup-store'

async function checkHealth(baseUrl: string): Promise<boolean> {
  try {
    const res = await fetch(`${baseUrl.replace(/\/+$/, '')}/health`)
    return res.ok
  }
  catch {
    return false
  }
}

function delay(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

async function waitForApiHealthy(baseUrl: string): Promise<void> {
  const maxAttempts = 30
  const interval = 500

  for (let i = 0; i < maxAttempts; i++) {
    const healthy = await checkHealth(baseUrl)
    if (healthy)
      return
    await delay(interval)
  }

  throw new Error('API did not become healthy in time')
}

export function useSetupFlow() {
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const applyStartupState = useStartupStore(s => s.applyStartupState)

  const submitRemoteConfig = async (remoteApiUrl: string) => {
    setIsSubmitting(true)
    setError(null)

    try {
      setApiBaseUrl(remoteApiUrl)

      const healthy = await checkHealth(remoteApiUrl)
      if (!healthy) {
        throw new Error('Cannot connect to remote API')
      }

      const state = await saveRemoteConfig({ remoteApiUrl })
      applyStartupState(state)
    }
    catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to save remote config'
      setError(message)
      throw err
    }
    finally {
      setIsSubmitting(false)
    }
  }

  const submitLocalConfig = async (payload: SaveLocalConfigPayload) => {
    setIsSubmitting(true)
    setError(null)

    try {
      await saveLocalConfig(payload)
      const state = await startLocalApi()

      const baseUrl = state.apiBaseUrl!
      setApiBaseUrl(baseUrl)

      await waitForApiHealthy(baseUrl)
      applyStartupState(state)
    }
    catch (err) {
      const message = err instanceof Error ? err.message : typeof err === 'string' ? err : 'Failed to start local API'
      setError(message)
      throw err
    }
    finally {
      setIsSubmitting(false)
    }
  }

  return { isSubmitting, error, submitRemoteConfig, submitLocalConfig }
}

export { checkHealth }
