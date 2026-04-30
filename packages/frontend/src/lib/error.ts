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
