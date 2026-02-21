import { invoke } from '@tauri-apps/api/core'

// Port constants – must match hardcoded values in Rust configs
const DESKTOP_API_PORT = 3030
const SERVER_API_PORT = 3000

// ── Tauri detection ──────────────────────────────────────────────────────────

function isTauriRuntime(): boolean {
  if (typeof window === 'undefined')
    return false
  return (
    '__TAURI__' in window
    || '__TAURI_INTERNALS__' in window
    || navigator.userAgent.includes('Tauri')
  )
}

function getBaseUrl(): string {
  if (isTauriRuntime()) {
    return `http://127.0.0.1:${DESKTOP_API_PORT}`
  }
  return import.meta.env.VITE_API_URL ?? `http://localhost:${SERVER_API_PORT}`
}

export function getEnvInfo() {
  return {
    mode: isTauriRuntime() ? 'desktop' : 'web',
    apiUrl: getBaseUrl(),
    port: isTauriRuntime() ? DESKTOP_API_PORT : SERVER_API_PORT,
  }
}

export type SetupDatabaseType = 'sqlite' | 'postgres' | 'mysql'

export type StartupStage = 'setup' | 'superadmin' | 'login'

export interface DesktopStartupState {
  stage: StartupStage
  api_url: string
  reason?: string
}

export interface LoginRequest {
  username: string
  password: string
}

export interface LoginResponse {
  accessToken: string
  refreshToken: string
  user: UserResponse
}

export interface SuperadminRequest {
  username: string
  password: string
  fullname?: string
}

export async function resolveDesktopStartupState(): Promise<DesktopStartupState> {
  if (!isTauriRuntime()) {
    return {
      stage: 'login',
      api_url: getBaseUrl(),
    }
  }
  return invoke<DesktopStartupState>('resolve_startup_state')
}

export interface DesktopInitStatus {
  initialized: boolean
  api_url: string
}

export interface InitializeDesktopAppRequest {
  db_type: SetupDatabaseType
  sqlite_file?: string
  sqlite_password?: string
  host?: string
  port?: number
  database?: string
  username?: string
  password?: string
}

export async function getDesktopInitStatus(): Promise<DesktopInitStatus> {
  const state = await resolveDesktopStartupState()
  return {
    initialized: state.stage !== 'setup',
    api_url: state.api_url,
  }
}

export async function initializeDesktopApp(
  request: InitializeDesktopAppRequest,
): Promise<void> {
  if (!isTauriRuntime())
    return
  await invoke('initialize_app', { request })
}

export async function resetDesktopInitialization(): Promise<void> {
  if (!isTauriRuntime())
    return
  await invoke('reset_app_initialization')
}

// ── API response envelope ────────────────────────────────────────────────────

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: { code: string, message: string }
}

// ── Domain types (mirror Rust DTOs) ─────────────────────────────────────────

/** Matches `UserResponse` in `packages/core/src/dtos/user.rs` */
export interface UserResponse {
  id: string // UUID
  username: string
  fullname?: string
  role: string
}

/** Matches `CreateUserRequest` in `packages/core/src/dtos/user.rs` */
export interface CreateUserRequest {
  username: string
  password: string
  fullname?: string
  role_name: string
}

// ── HTTP client ──────────────────────────────────────────────────────────────

class HttpApiClient {
  private serverReady = false

  /** In Tauri mode, poll /users until the embedded server responds. */
  private async waitForServer(): Promise<void> {
    if (!isTauriRuntime() || this.serverReady)
      return

    const baseUrl = getBaseUrl()
    for (let i = 0; i < 10; i++) {
      try {
        const res = await fetch(`${baseUrl}/users`, { method: 'GET' })
        if (res.ok || res.status === 200) {
          this.serverReady = true
          console.log('✅ API server ready')
          return
        }
      }
      catch {
        console.log(`⏳ Waiting for API server… (attempt ${i + 1}/10)`)
      }
      await new Promise(r => setTimeout(r, 500))
    }
    throw new Error('API server failed to start after 10 attempts')
  }

  private async request<T>(
    endpoint: string,
    method = 'GET',
    body?: unknown,
  ): Promise<T> {
    await this.waitForServer()

    const res = await fetch(`${getBaseUrl()}${endpoint}`, {
      method,
      headers: { 'Content-Type': 'application/json' },
      ...(body !== undefined ? { body: JSON.stringify(body) } : {}),
    })

    const envelope: ApiResponse<T> = await res.json()

    if (!envelope.success || !res.ok) {
      throw new Error(
        envelope.error?.message ?? `HTTP ${res.status}: ${res.statusText}`,
      )
    }

    // DELETE returns success:true with data: null / undefined – handle that
    return envelope.data as T
  }

  // ── User endpoints ─────────────────────────────────────────────────────

  listUsers(): Promise<UserResponse[]> {
    return this.request<UserResponse[]>('/users')
  }

  createUser(data: CreateUserRequest): Promise<UserResponse> {
    return this.request<UserResponse>('/users', 'POST', data)
  }

  deleteUser(id: string): Promise<void> {
    return this.request<void>(`/users/${id}`, 'DELETE')
  }

  login(request: LoginRequest): Promise<LoginResponse> {
    return this.request<LoginResponse>('/auth/login', 'POST', request)
  }

  createSuperadmin(request: SuperadminRequest): Promise<UserResponse> {
    return this.request<UserResponse>('/users', 'POST', {
      ...request,
      role_name: 'superadmin',
    })
  }
}

export const api = new HttpApiClient()
export const isTauri = isTauriRuntime
