export function formatDate(iso: string): string {
  const d = new Date(iso)
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  return `${y}-${m}-${day}`
}

export function formatDateTime(iso: string): string {
  const d = new Date(iso)
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  const h = String(d.getHours()).padStart(2, '0')
  const min = String(d.getMinutes()).padStart(2, '0')
  return `${y}-${m}-${day} ${h}:${min}`
}

export function zeroPad(n: number, width: number): string {
  return String(n).padStart(width, '0')
}

export function truncateId(id: string): string {
  return id.length > 8 ? `${id.slice(0, 8)}…` : id
}

export function formatAmount(value: number | string | null | undefined, unit?: string): string {
  if (value == null || value === '')
    return '\u2014'

  const num = typeof value === 'string' ? Number.parseFloat(value) : value
  if (Number.isNaN(num))
    return '\u2014'

  const formatted = num.toLocaleString('fr-FR', {
    minimumFractionDigits: 3,
    maximumFractionDigits: 3,
  })

  return unit ? `${formatted} ${unit}` : formatted
}
