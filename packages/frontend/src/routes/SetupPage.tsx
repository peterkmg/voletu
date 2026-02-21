import type { SetupDatabaseType } from '~/api/client'
import { useEffect, useState } from 'react'

import { useNavigate } from 'react-router-dom'
import {
  initializeDesktopApp,
  resolveDesktopStartupState,

} from '~/api/client'

interface SetupFormState {
  db_type: SetupDatabaseType
  sqlite_file: string
  sqlite_password: string
  host: string
  port: string
  database: string
  username: string
  password: string
}

const EMPTY_SETUP_FORM: SetupFormState = {
  db_type: 'sqlite',
  sqlite_file: 'voletu.db',
  sqlite_password: '',
  host: '',
  port: '',
  database: '',
  username: '',
  password: '',
}

export function SetupPage() {
  const navigate = useNavigate()
  const [checking, setChecking] = useState(true)
  const [error, setError] = useState('')
  const [info, setInfo] = useState('')
  const [setupForm, setSetupForm] = useState<SetupFormState>(EMPTY_SETUP_FORM)

  useEffect(() => {
    async function checkStatus() {
      try {
        const state = await resolveDesktopStartupState()
        if (state.stage === 'login') {
          navigate('/login', { replace: true })
          return
        }
        if (state.stage === 'superadmin') {
          navigate('/superadmin', { replace: true })
        }
      }
      catch (e) {
        setError(e instanceof Error ? e.message : String(e))
      }
      finally {
        setChecking(false)
      }
    }

    void checkStatus()
  }, [navigate])

  function setSetupField<K extends keyof SetupFormState>(
    key: K,
    value: SetupFormState[K],
  ) {
    setSetupForm(prev => ({ ...prev, [key]: value }))
  }

  async function initializeDesktop(e: React.FormEvent) {
    e.preventDefault()

    try {
      setError('')
      setInfo('')

      if (setupForm.db_type === 'sqlite') {
        await initializeDesktopApp({
          db_type: 'sqlite',
          sqlite_file: setupForm.sqlite_file,
          sqlite_password: setupForm.sqlite_password,
        })
      }
      else {
        await initializeDesktopApp({
          db_type: setupForm.db_type,
          host: setupForm.host,
          port: Number(setupForm.port),
          database: setupForm.database,
          username: setupForm.username,
          password: setupForm.password,
        })
      }

      setInfo('Desktop application initialized successfully.')

      const state = await resolveDesktopStartupState()
      if (state.stage === 'superadmin') {
        navigate('/superadmin', { replace: true })
      }
      else {
        navigate('/login', { replace: true })
      }
    }
    catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    }
  }

  if (checking) {
    return (
      <main className="container">
        <h1>Voletu</h1>
        <p>Preparing application...</p>
      </main>
    )
  }

  return (
    <main className="container">
      <h1>Voletu – Initial Setup</h1>

      {error && (
        <div
          style={{
            color: '#f87171',
            marginBottom: '1rem',
            padding: '0.5rem 1rem',
            background: '#450a0a',
            borderRadius: '6px',
          }}
        >
          ❌
          {' '}
          {error}
        </div>
      )}

      {info && !error && (
        <div
          style={{
            color: '#86efac',
            marginBottom: '1rem',
            padding: '0.5rem 1rem',
            background: '#052e16',
            borderRadius: '6px',
          }}
        >
          ✅
          {' '}
          {info}
        </div>
      )}

      <section style={{ maxWidth: '900px', margin: '0 auto', width: '100%' }}>
        <form onSubmit={initializeDesktop}>
          <div className="row" style={{ gap: '0.6rem', flexWrap: 'wrap' }}>
            <select
              value={setupForm.db_type}
              onChange={e => setSetupField('db_type', e.target.value as SetupDatabaseType)}
            >
              <option value="sqlite">SQLite (SQLCipher)</option>
              <option value="postgres">PostgreSQL</option>
              <option value="mysql">MySQL</option>
            </select>

            {setupForm.db_type === 'sqlite'
              ? (
                  <>
                    <input
                      value={setupForm.sqlite_file}
                      onChange={e => setSetupField('sqlite_file', e.target.value)}
                      placeholder="SQLite file path"
                      required
                    />
                    <input
                      type="password"
                      value={setupForm.sqlite_password}
                      onChange={e => setSetupField('sqlite_password', e.target.value)}
                      placeholder="SQLCipher password"
                      required
                    />
                  </>
                )
              : (
                  <>
                    <input
                      value={setupForm.host}
                      onChange={e => setSetupField('host', e.target.value)}
                      placeholder="Database host"
                      required
                    />
                    <input
                      type="number"
                      value={setupForm.port}
                      onChange={e => setSetupField('port', e.target.value)}
                      placeholder="Port"
                      required
                    />
                    <input
                      value={setupForm.database}
                      onChange={e => setSetupField('database', e.target.value)}
                      placeholder="Database name"
                      required
                    />
                    <input
                      value={setupForm.username}
                      onChange={e => setSetupField('username', e.target.value)}
                      placeholder="Database username"
                      required
                    />
                    <input
                      type="password"
                      value={setupForm.password}
                      onChange={e => setSetupField('password', e.target.value)}
                      placeholder="Database password"
                      required
                    />
                  </>
                )}

            <button type="submit">Initialize App</button>
          </div>
        </form>

        <p style={{ color: '#6b7280', marginTop: '0.75rem', textAlign: 'left' }}>
          JWT master secret is generated automatically and stored securely.
        </p>
      </section>
    </main>
  )
}
