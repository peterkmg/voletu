import { useEffect, useState } from 'react'

const DEVTOOLS_KEY = 'voletu.devtools'

export function useDevToolsVisible() {
  const [visible, setVisible] = useState(
    () => localStorage.getItem(DEVTOOLS_KEY) === 'true',
  )

  useEffect(() => {
    function onStorage(e: StorageEvent) {
      if (e.key === DEVTOOLS_KEY) {
        setVisible(e.newValue === 'true')
      }
    }
    window.addEventListener('storage', onStorage)
    return () => window.removeEventListener('storage', onStorage)
  }, [])

  return visible
}

export function toggleDevTools() {
  const next = localStorage.getItem(DEVTOOLS_KEY) !== 'true'
  localStorage.setItem(DEVTOOLS_KEY, String(next))
  window.dispatchEvent(new StorageEvent('storage', {
    key: DEVTOOLS_KEY,
    newValue: String(next),
  }))
  return next
}
