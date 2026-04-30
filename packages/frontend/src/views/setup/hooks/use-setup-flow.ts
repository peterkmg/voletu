import type { SaveLocalConfigPayload } from '~/tauri/commands'
import { useState } from 'react'
import { extractErrorMessage } from '~/lib/error'
import { setApiBaseUrl } from '~/platform/runtime/api-base-url'
import {
  applyHealthSnapshot,
  fetchHealth,
  waitForApiHealthy,
} from '~/platform/runtime/health'
import { useRuntimeStore } from '~/platform/runtime/runtime-store'
import { useStartupStore } from '~/stores/startup-store'
import {
  saveLocalConfig,
  saveRemoteConfig,
  startLocalApi,
} from '~/tauri/commands'

export function useSetupFlow() {
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const applyStartupState = useStartupStore(s => s.applyStartupState)

  const submitRemoteConfig = async (remoteApiUrl: string) => {
    setIsSubmitting(true)
    setError(null)

    try {
      const health = await fetchHealth({ baseUrl: remoteApiUrl })
      const state = await saveRemoteConfig({ remoteApiUrl })

      applyHealthSnapshot(health)
      useRuntimeStore.getState().markHealthHydrated()
      setApiBaseUrl(state.apiBaseUrl ?? remoteApiUrl)
      applyStartupState(state)
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
      const health = await waitForApiHealthy(baseUrl)

      applyHealthSnapshot(health)
      useRuntimeStore.getState().markHealthHydrated()
      setApiBaseUrl(baseUrl)
      applyStartupState(state)
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
