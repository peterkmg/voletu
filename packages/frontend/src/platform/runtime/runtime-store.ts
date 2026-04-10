import { create } from 'zustand'

export type BootstrapStatus = 'idle' | 'booting' | 'ready' | 'error'
export type HealthStatus = 'idle' | 'hydrated' | 'unavailable'

interface RuntimeStore {
  bootstrapStatus: BootstrapStatus
  bootstrapError: string | null
  healthStatus: HealthStatus
  startBootstrap: () => void
  finishBootstrap: () => void
  failBootstrap: (message: string) => void
  markHealthHydrated: () => void
  markHealthUnavailable: () => void
  resetBootstrap: () => void
}

export const useRuntimeStore = create<RuntimeStore>()(set => ({
  bootstrapStatus: 'idle',
  bootstrapError: null,
  healthStatus: 'idle',

  startBootstrap: () => {
    set({ bootstrapStatus: 'booting', bootstrapError: null })
  },

  finishBootstrap: () => {
    set({ bootstrapStatus: 'ready', bootstrapError: null })
  },

  failBootstrap: (message: string) => {
    set({ bootstrapStatus: 'error', bootstrapError: message })
  },

  markHealthHydrated: () => {
    set({ healthStatus: 'hydrated' })
  },

  markHealthUnavailable: () => {
    set({ healthStatus: 'unavailable' })
  },

  resetBootstrap: () => {
    set({ bootstrapStatus: 'idle', bootstrapError: null, healthStatus: 'idle' })
  },
}))
