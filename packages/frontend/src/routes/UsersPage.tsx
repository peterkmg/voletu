import type { CreateUserRequest, UserResponse } from '../api/client'
import { useEffect, useState } from 'react'

import { useNavigate } from 'react-router-dom'
import {
  api,

  getDesktopInitStatus,
  getEnvInfo,
  isTauri,
  resetDesktopInitialization,

} from '../api/client'

const EMPTY_FORM: CreateUserRequest = {
  username: '',
  password: '',
  fullname: '',
  role_name: 'admin',
}

export function UsersPage() {
  const navigate = useNavigate()
  const envInfo = getEnvInfo()

  const [checking, setChecking] = useState(true)
  const [users, setUsers] = useState<UserResponse[]>([])
  const [form, setForm] = useState<CreateUserRequest>(EMPTY_FORM)
  const [error, setError] = useState<string>('')
  const [info, setInfo] = useState<string>('')

  useEffect(() => {
    async function checkStatus() {
      try {
        const status = await getDesktopInitStatus()
        if (!status.initialized) {
          navigate('/setup', { replace: true })
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

  function setField<K extends keyof CreateUserRequest>(
    key: K,
    value: CreateUserRequest[K],
  ) {
    setForm(prev => ({ ...prev, [key]: value }))
  }

  async function loadUsers() {
    try {
      setError('')
      const loaded = await api.listUsers()
      setUsers(loaded)
      setInfo(`Loaded ${loaded.length} user(s)`)
    }
    catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    }
  }

  async function createUser(e: React.FormEvent) {
    e.preventDefault()
    try {
      setError('')
      const user = await api.createUser({
        ...form,
        fullname: form.fullname?.trim() || undefined,
      })
      setUsers(prev => [...prev, user])
      setForm(EMPTY_FORM)
      setInfo(`User '${user.username}' created (${user.id})`)
    }
    catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    }
  }

  async function deleteUser(id: string, username: string) {
    if (!confirm(`Delete user '${username}'?`))
      return
    try {
      setError('')
      await api.deleteUser(id)
      setUsers(prev => prev.filter(u => u.id !== id))
      setInfo(`User '${username}' deleted`)
    }
    catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    }
  }

  async function resetDesktopInit() {
    if (!isTauri())
      return
    if (!confirm('Reset desktop initialization and remove stored secrets?'))
      return

    try {
      setError('')
      await resetDesktopInitialization()
      setUsers([])
      setInfo('Desktop initialization has been reset.')
      navigate('/setup', { replace: true })
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
      <h1>Voletu – User Management</h1>

      <div
        style={{
          marginBottom: '1.5rem',
          padding: '0.75rem 1rem',
          background: '#222',
          borderRadius: '6px',
          fontSize: '0.85rem',
        }}
      >
        <strong>Mode:</strong>
        {' '}
        {envInfo.mode}
        {' '}
        {isTauri() ? '🖥️' : '🌐'}
&nbsp;&nbsp;
        <strong>API:</strong>
        {' '}
        {envInfo.apiUrl}
        {isTauri() && (
          <button onClick={resetDesktopInit} style={{ marginLeft: '1rem' }}>
            Reset Initialization
          </button>
        )}
      </div>

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

      <section style={{ marginBottom: '2rem' }}>
        <h2>Create User</h2>
        <form onSubmit={createUser}>
          <div className="row" style={{ flexWrap: 'wrap', gap: '0.5rem' }}>
            <input
              value={form.username}
              onChange={e => setField('username', e.target.value)}
              placeholder="Username *"
              required
              minLength={3}
            />
            <input
              type="password"
              value={form.password}
              onChange={e => setField('password', e.target.value)}
              placeholder="Password *"
              required
              minLength={6}
            />
            <input
              value={form.fullname ?? ''}
              onChange={e => setField('fullname', e.target.value)}
              placeholder="Full name (optional)"
            />
            <input
              value={form.role_name}
              onChange={e => setField('role_name', e.target.value)}
              placeholder="Role name *"
              required
            />
            <button type="submit">Create</button>
          </div>
        </form>
      </section>

      <section>
        <h2>
          Users
          {' '}
          <button onClick={loadUsers} style={{ fontSize: '0.8rem' }}>
            Refresh
          </button>
        </h2>

        {users.length === 0
          ? (
              <p style={{ color: '#6b7280' }}>No users loaded yet – click Refresh.</p>
            )
          : (
              <table
                style={{ width: '100%', borderCollapse: 'collapse', textAlign: 'left' }}
              >
                <thead>
                  <tr style={{ borderBottom: '1px solid #374151' }}>
                    <th style={{ padding: '0.4rem 0.6rem' }}>Username</th>
                    <th style={{ padding: '0.4rem 0.6rem' }}>Full name</th>
                    <th style={{ padding: '0.4rem 0.6rem' }}>Role</th>
                    <th style={{ padding: '0.4rem 0.6rem' }}>ID</th>
                    <th style={{ padding: '0.4rem 0.6rem' }}></th>
                  </tr>
                </thead>
                <tbody>
                  {users.map(u => (
                    <tr key={u.id} style={{ borderBottom: '1px solid #1f2937' }}>
                      <td style={{ padding: '0.4rem 0.6rem' }}>{u.username}</td>
                      <td style={{ padding: '0.4rem 0.6rem', color: '#9ca3af' }}>
                        {u.fullname ?? '–'}
                      </td>
                      <td style={{ padding: '0.4rem 0.6rem' }}>{u.role}</td>
                      <td
                        style={{
                          padding: '0.4rem 0.6rem',
                          fontFamily: 'monospace',
                          fontSize: '0.75rem',
                          color: '#6b7280',
                        }}
                      >
                        {u.id}
                      </td>
                      <td style={{ padding: '0.4rem 0.6rem' }}>
                        <button
                          onClick={() => deleteUser(u.id, u.username)}
                          style={{
                            background: '#7f1d1d',
                            color: '#fca5a5',
                            border: 'none',
                            padding: '0.2rem 0.6rem',
                            borderRadius: '4px',
                            cursor: 'pointer',
                          }}
                        >
                          Delete
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
      </section>
    </main>
  )
}
