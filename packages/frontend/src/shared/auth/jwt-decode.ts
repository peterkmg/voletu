/**
 * Decode a JWT's payload and extract the `exp` (expiration) claim.
 * Does NOT verify the signature — this is a client-side convenience
 * for checking expiry before making a request. The server always
 * verifies the full token via auth middleware.
 *
 * Returns the `exp` timestamp (seconds since epoch), or null if
 * the token is malformed or has no `exp` claim.
 */
export function decodeJwtExp(token: string): number | null {
  try {
    const parts = token.split('.')
    if (parts.length !== 3)
      return null

    // JWT payload is base64url-encoded. Replace URL-safe chars and decode.
    const payload = parts[1]!
      .replace(/-/g, '+')
      .replace(/_/g, '/')
    const padded = payload + '='.repeat((4 - payload.length % 4) % 4)
    const decoded = JSON.parse(atob(padded))

    return typeof decoded.exp === 'number' ? decoded.exp : null
  }
  catch {
    return null
  }
}

/**
 * Returns true if the given JWT token will expire within `thresholdSeconds`
 * from now (or is already expired).
 */
export function isTokenExpiringSoon(token: string, thresholdSeconds = 300): boolean {
  const exp = decodeJwtExp(token)
  if (exp === null)
    return true // treat unreadable tokens as expired
  return exp - Math.floor(Date.now() / 1000) < thresholdSeconds
}
