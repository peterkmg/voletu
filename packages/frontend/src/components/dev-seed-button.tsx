import { useState } from 'react'
import { toast } from 'sonner'
import { useStartupStore } from '~/stores/startup-store'

export function DevSeedButton() {
  const [loading, setLoading] = useState(false)

  const handleSeed = async () => {
    const apiBaseUrl = useStartupStore.getState().startupState?.apiBaseUrl
    if (!apiBaseUrl) {
      toast.error('API not configured — complete setup first')
      return
    }

    setLoading(true)
    try {
      const token = localStorage.getItem('accessToken')
      const response = await fetch(`${apiBaseUrl}/dev/seed`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Idempotency-Key': crypto.randomUUID(),
          ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
      })

      const body = await response.json() as { success: boolean, data?: Record<string, number>, error?: { message?: string } }

      if (!body.success) {
        throw new Error(body.error?.message ?? 'Seed failed')
      }

      const counts = body.data ?? {}
      const summary = Object.entries(counts)
        .map(([k, v]) => `${k}: ${v}`)
        .join(', ')
      toast.success(`Seeded — ${summary}`)
    }
    catch (err) {
      toast.error(err instanceof Error ? err.message : 'Seed failed')
    }
    finally {
      setLoading(false)
    }
  }

  return (
    <button
      type="button"
      onClick={handleSeed}
      disabled={loading}
      className="fixed bottom-10 left-1/2 -translate-x-1/2 z-[9999] rounded bg-orange-500 px-3 py-1 text-xs font-medium text-white shadow-md disabled:opacity-50 hover:bg-orange-600 transition-colors"
    >
      {loading ? 'Seeding…' : '🌱 Seed DB'}
    </button>
  )
}
