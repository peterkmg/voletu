import type { SaveLocalConfigPayload } from '~/tauri/commands'
import { useState } from 'react'
import { setApiBaseUrl } from '~/api/client'
import { extractErrorMessage } from '~/lib/error'
import { TRAILING_SLASHES } from '~/lib/utils'
import { useStartupStore } from '~/stores/startup-store'
import {
  saveLocalConfig,
  saveRemoteConfig,
  startLocalApi,
} from '~/tauri/commands'

async function checkHealth(baseUrl: string): Promise<boolean> {
  try {
    const res = await fetch(`${baseUrl.replace(TRAILING_SLASHES, '')}/health`)
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
      // Keep isSubmitting=true — page will navigate away
    }
    catch (err) {
      setIsSubmitting(false)
      const message = extractErrorMessage(err, 'Failed to save remote config')
      setError(message)
      throw err
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
      // Keep isSubmitting=true — page will navigate away
    }
    catch (err) {
      setIsSubmitting(false)
      const message = extractErrorMessage(err, 'Failed to start local API')
      setError(message)
      throw err
    }
  }

  return { isSubmitting, error, submitRemoteConfig, submitLocalConfig }
}

export { checkHealth }
