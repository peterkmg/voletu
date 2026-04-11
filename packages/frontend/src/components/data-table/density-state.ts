import { useSyncExternalStore } from 'react'

export type TableDensity = 'compact' | 'normal' | 'comfortable'

const STORAGE_KEY = 'voletu.table-density'
const DEFAULT_DENSITY: TableDensity = 'normal'
const listeners = new Set<() => void>()

function normalizeTableDensity(value: unknown): TableDensity {
  switch (value) {
    case 'compact':
    case 'normal':
    case 'comfortable':
      return value
    default:
      return DEFAULT_DENSITY
  }
}

function getDensitySnapshot(): TableDensity {
  if (typeof window === 'undefined') {
    return DEFAULT_DENSITY
  }

  return normalizeTableDensity(window.localStorage.getItem(STORAGE_KEY))
}

function notifyDensityListeners() {
  listeners.forEach(listener => listener())
}

function subscribeToDensity(listener: () => void): () => void {
  listeners.add(listener)

  const onStorage = (event: StorageEvent) => {
    if (event.key === STORAGE_KEY) {
      listener()
    }
  }

  window.addEventListener('storage', onStorage)

  return () => {
    listeners.delete(listener)
    window.removeEventListener('storage', onStorage)
  }
}

export function setTableDensityPreference(density: TableDensity): void {
  if (typeof window === 'undefined') {
    return
  }

  window.localStorage.setItem(STORAGE_KEY, normalizeTableDensity(density))
  notifyDensityListeners()
}

export function useTableDensity() {
  const density = useSyncExternalStore(
    subscribeToDensity,
    getDensitySnapshot,
    () => DEFAULT_DENSITY,
  )

  return {
    density,
    setDensity: setTableDensityPreference,
  }
}
