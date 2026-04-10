import { extractErrorMessage } from '~/lib/error'
import { useAuthStore } from '~/stores/auth-store'
import { useStartupStore } from '~/stores/startup-store'
import { setApiBaseUrl } from './api-base-url'
import { applyHealthSnapshot, fetchHealth } from './health'
import { useRuntimeStore } from './runtime-store'

let inflightBootstrap: Promise<void> | null = null

async function runBootstrap(): Promise<void> {
  await useStartupStore.getState().refresh()

  const { startupState } = useStartupStore.getState()
  if (startupState?.apiBaseUrl) {
    setApiBaseUrl(startupState.apiBaseUrl)
  }

  const authStore = useAuthStore.getState()
  if (authStore.status === 'unknown') {
    await authStore.boot()
  }

  if (startupState?.apiBaseUrl && !startupState.needsSetup) {
    try {
      const health = await fetchHealth()
      applyHealthSnapshot(health)
      useRuntimeStore.getState().markHealthHydrated()
    }
    catch {
      useRuntimeStore.getState().markHealthUnavailable()
      // Health hydration is best-effort during bootstrap because the API may be
      // restarting while setup/auth state is still otherwise usable.
    }
  }
}

export function resetRuntimeBootstrap(): void {
  inflightBootstrap = null
  useRuntimeStore.getState().resetBootstrap()
}

export async function ensureBootstrapped(): Promise<void> {
  const runtime = useRuntimeStore.getState()

  if (runtime.bootstrapStatus === 'ready') {
    return
  }

  if (inflightBootstrap) {
    return inflightBootstrap
  }

  runtime.startBootstrap()

  inflightBootstrap = runBootstrap()
    .then(() => {
      useRuntimeStore.getState().finishBootstrap()
    })
    .catch((error) => {
      useRuntimeStore.getState().failBootstrap(
        extractErrorMessage(error, 'Failed to bootstrap runtime'),
      )
      throw error
    })
    .finally(() => {
      inflightBootstrap = null
    })

  return inflightBootstrap
}
