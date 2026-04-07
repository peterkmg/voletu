import { useState } from 'react'

/**
 * Returns a stable idempotency key (UUID v4) for the lifetime of the component.
 *
 * A new key is generated each time the component mounts (e.g., dialog opens).
 * The key persists across re-renders within the same mount lifecycle, so
 * double-clicks and retries reuse the same key.
 *
 * Usage:
 *   const idempotencyKey = useIdempotencyKey()
 *   await someCreate(data, { headers: { 'Idempotency-Key': idempotencyKey } })
 */
export function useIdempotencyKey(): string {
  const [key] = useState(() => crypto.randomUUID())
  return key
}
