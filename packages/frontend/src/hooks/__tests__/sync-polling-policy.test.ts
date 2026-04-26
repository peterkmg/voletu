import {
  getCurrentSyncPollInterval,
  resolveSyncPollInterval,
  SYNC_POLL_INTERVAL_ACTIVE_MS,
  SYNC_POLL_INTERVAL_IDLE_MS,
} from '../sync-polling-policy'

describe('sync polling policy', () => {
  it('returns active interval for visible state', () => {
    expect(resolveSyncPollInterval('visible')).toBe(SYNC_POLL_INTERVAL_ACTIVE_MS)
  })

  it.each(['hidden', 'prerender'] as const)(
    'returns idle interval for %s state',
    (state) => {
      expect(resolveSyncPollInterval(state)).toBe(SYNC_POLL_INTERVAL_IDLE_MS)
    },
  )

  it('returns idle interval when document is unavailable', () => {
    expect(getCurrentSyncPollInterval(null)).toBe(SYNC_POLL_INTERVAL_IDLE_MS)
  })
})
