const amountFormatter = new Intl.NumberFormat('fr-FR', {
  minimumFractionDigits: 0,
  maximumFractionDigits: 3,
})

export function formatAmount(value: unknown): string {
  if (value === null || value === undefined || value === '')
    return ''
  const n = typeof value === 'number' ? value : Number(value)
  if (Number.isNaN(n))
    return String(value)
  return amountFormatter.format(n)
}
