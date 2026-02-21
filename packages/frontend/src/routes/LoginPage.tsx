import { useState } from 'react'

import { api } from '../api/client'

export function LoginPage() {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [error, setError] = useState('')
  const [info, setInfo] = useState('')

  async function login(e: React.FormEvent) {
    e.preventDefault()

    try {
      setError('')
      const response = await api.login({ username, password })
      setInfo(`Signed in as ${response.user.username} (${response.user.role})`)
    }
    catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    }
  }

  return (
    <main className="container">
      <h1>Voletu Login</h1>

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

      <section style={{ maxWidth: '520px', margin: '0 auto', width: '100%' }}>
        <form onSubmit={login}>
          <div className="row" style={{ gap: '0.6rem', flexWrap: 'wrap' }}>
            <input
              value={username}
              onChange={e => setUsername(e.target.value)}
              placeholder="Username"
              minLength={3}
              required
            />
            <input
              type="password"
              value={password}
              onChange={e => setPassword(e.target.value)}
              placeholder="Password"
              minLength={6}
              required
            />
            <button type="submit">Login</button>
          </div>
        </form>
      </section>
    </main>
  )
}
