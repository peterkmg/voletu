import { renderHook } from '@testing-library/react'
import { useIdempotencyKey } from '~/hooks/use-idempotency-key'

const UUID_REGEX =
  /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i

describe('useIdempotencyKey', () => {
  it('returns a UUID v4 string', () => {
    const { result } = renderHook(() => useIdempotencyKey())
    expect(result.current).toMatch(UUID_REGEX)
  })

  it('returns the same value across re-renders', () => {
    const { result, rerender } = renderHook(() => useIdempotencyKey())

    const first = result.current
    rerender()
    const second = result.current

    expect(second).toBe(first)
  })

  it('returns a new value on a fresh mount', () => {
    const { result: first, unmount } = renderHook(() => useIdempotencyKey())
    const key1 = first.current
    unmount()

    const { result: second } = renderHook(() => useIdempotencyKey())
    const key2 = second.current

    expect(key2).toMatch(UUID_REGEX)
    expect(key2).not.toBe(key1)
  })

  it('each independent mount produces a unique key', () => {
    const keys = new Set<string>()

    for (let i = 0; i < 10; i++) {
      const { result, unmount } = renderHook(() => useIdempotencyKey())
      keys.add(result.current)
      unmount()
    }

    expect(keys.size).toBe(10)
  })
})
