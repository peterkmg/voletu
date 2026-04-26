export const SYNC_POLL_INTERVAL_ACTIVE_MS = 2_500
export const SYNC_POLL_INTERVAL_IDLE_MS = 20_000

type VisibilityStateLike = 'visible' | 'hidden' | 'prerender'

export function resolveSyncPollInterval(visibilityState: VisibilityStateLike): number {
  return visibilityState === 'visible'
    ? SYNC_POLL_INTERVAL_ACTIVE_MS
    : SYNC_POLL_INTERVAL_IDLE_MS
}

export function getCurrentSyncPollInterval(
  doc: Pick<Document, 'visibilityState'> | null = typeof document === 'undefined' ? null : document,
): number {
  if (!doc)
    return SYNC_POLL_INTERVAL_IDLE_MS
  return resolveSyncPollInterval(doc.visibilityState as VisibilityStateLike)
}
