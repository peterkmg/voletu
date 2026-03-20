/**
 * Format an ISO date string as YYYY-MM-DD.
 */
export function formatDate(iso: string): string {
  const d = new Date(iso)
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  return `${y}-${m}-${day}`
}

/**
 * Format an ISO date string as YYYY-MM-DD HH:mm.
 */
export function formatDateTime(iso: string): string {
  const d = new Date(iso)
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  const h = String(d.getHours()).padStart(2, '0')
  const min = String(d.getMinutes()).padStart(2, '0')
  return `${y}-${m}-${day} ${h}:${min}`
}

/**
 * Zero-pad a number to a given width.
 * e.g. zeroPad(2, 3) = "002"
 */
export function zeroPad(n: number, width: number): string {
  return String(n).padStart(width, '0')
}

/**
 * Truncate a UUID for display, showing first 8 characters.
 */
export function truncateId(id: string): string {
  return id.length > 8 ? `${id.slice(0, 8)}…` : id
}
