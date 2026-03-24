/**
 * Extract a human-readable message from an unknown caught value.
 *
 * @param err - The value caught in a try/catch block
 * @param fallback - Fallback message when err is not an Error instance
 * @returns The error message string
 */
export function extractErrorMessage(
  err: unknown,
  fallback = 'An unexpected error occurred',
): string {
  if (err instanceof Error)
    return err.message
  if (typeof err === 'string')
    return err
  return fallback
}
