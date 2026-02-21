import { LoaderCircle } from 'lucide-react'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'

import { resolveDesktopStartupState } from '../api/client'

export function RootRoute() {
  const navigate = useNavigate()
  const [error, setError] = useState<string>('')

  useEffect(() => {
    async function resolveRoute() {
      try {
        const state = await resolveDesktopStartupState()
        if (state.stage === 'setup') {
          navigate('/setup', { replace: true })
          return
        }
        if (state.stage === 'superadmin') {
          navigate('/superadmin', { replace: true })
          return
        }
        navigate('/login', { replace: true })
      }
      catch (e) {
        setError(e instanceof Error ? e.message : String(e))
      }
    }

    void resolveRoute()
  }, [navigate])

  return (
    <main className="container">
      <h1>Voletu</h1>
      {error
        ? (
            <p style={{ color: '#f87171' }}>{error}</p>
          )
        : (
            <div className="row" style={{ gap: '0.5rem', alignItems: 'center' }}>
              <LoaderCircle size={18} />
              <p>Preparing application...</p>
            </div>
          )}
    </main>
  )
}
