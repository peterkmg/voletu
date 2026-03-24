import type { StartupState as TauriStartupState } from '~/shared/tauri/commands'
import { create } from 'zustand'
import { extractErrorMessage } from '~/lib/error'
import { getStartupState } from '~/shared/tauri/commands'

interface StartupStoreState {
  startupState: TauriStartupState | null
  isLoading: boolean
  error: string | null
  refresh: () => Promise<void>
  applyStartupState: (next: TauriStartupState) => void
}

export const useStartupStore = create<StartupStoreState>()(set => ({
  startupState: null,
  isLoading: false,
  error: null,

  refresh: async () => {
    set({ isLoading: true, error: null })
    try {
      const state = await getStartupState()
      set({ startupState: state, isLoading: false })
    }
    catch (err) {
      set({
        isLoading: false,
        error: extractErrorMessage(err, 'Failed to get startup state'),
      })
    }
  },

  applyStartupState: (next: TauriStartupState) => {
    set({ startupState: next })
  },
}))
