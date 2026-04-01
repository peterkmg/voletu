import { invoke } from '@tauri-apps/api/core'

const DEFAULT_IPC_TIMEOUT = 15_000

/** Wraps Tauri invoke with a timeout to prevent indefinite hangs */
async function invokeWithTimeout<T>(
  cmd: string,
  args?: Record<string, unknown>,
  timeout = DEFAULT_IPC_TIMEOUT,
): Promise<T> {
  return Promise.race([
    invoke<T>(cmd, args),
    new Promise<never>((_, reject) =>
      setTimeout(() => reject(new Error(`IPC '${cmd}' timed out after ${timeout}ms`)), timeout),
    ),
  ])
}

export type AppMode = 'remote' | 'local'

export interface StartupState {
  needsSetup: boolean
  mode: AppMode | null
  apiBaseUrl: string | null
  isDebugBuild: boolean
}

export interface SaveRemoteConfigPayload {
  remoteApiUrl: string
}

export type DbType = 'sqlite' | 'postgres' | 'mysql'

export interface SaveLocalConfigPayload {
  dbType: DbType
  sqliteFile?: string
  host?: string
  port?: number
  database?: string
  username?: string
  dbPassword: string
  jwtExpirationSeconds: number
  jwtRefreshExpirationSeconds: number
  logFilter?: string
  logFile?: string
}

export async function getStartupState(): Promise<StartupState> {
  return invokeWithTimeout<StartupState>('get_startup_state')
}

export async function saveRemoteConfig(
  payload: SaveRemoteConfigPayload,
): Promise<StartupState> {
  return invokeWithTimeout<StartupState>('save_remote_config', { req: payload })
}

export async function saveLocalConfig(
  payload: SaveLocalConfigPayload,
): Promise<StartupState> {
  return invokeWithTimeout<StartupState>('save_local_config', { req: payload })
}

export async function startLocalApi(): Promise<StartupState> {
  return invokeWithTimeout<StartupState>('start_local_api')
}

export async function resetConfigAndMode(): Promise<StartupState> {
  return invokeWithTimeout<StartupState>('reset_config_and_mode')
}
