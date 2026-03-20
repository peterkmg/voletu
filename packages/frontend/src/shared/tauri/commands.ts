import { invoke } from '@tauri-apps/api/core'

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
  return invoke<StartupState>('get_startup_state')
}

export async function saveRemoteConfig(
  payload: SaveRemoteConfigPayload,
): Promise<StartupState> {
  return invoke<StartupState>('save_remote_config', { req: payload })
}

export async function saveLocalConfig(
  payload: SaveLocalConfigPayload,
): Promise<StartupState> {
  return invoke<StartupState>('save_local_config', { req: payload })
}

export async function startLocalApi(): Promise<StartupState> {
  return invoke<StartupState>('start_local_api')
}

export async function resetConfigAndMode(): Promise<StartupState> {
  return invoke<StartupState>('reset_config_and_mode')
}
