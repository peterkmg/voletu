import { toDateTimeLocalValue } from '~/lib/datetime-local'

function localDateTimeString(date: Date): string {
  const year = String(date.getFullYear())
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  const hour = String(date.getHours()).padStart(2, '0')
  const minute = String(date.getMinutes()).padStart(2, '0')
  return `${year}-${month}-${day}T${hour}:${minute}`
}

describe('toDateTimeLocalValue()', () => {
  it('converts an ISO timestamp with timezone into datetime-local value', () => {
    const iso = '2026-04-20T05:30:00.000Z'
    const expected = localDateTimeString(new Date(iso))
    expect(toDateTimeLocalValue(iso)).toBe(expected)
  })

  it('keeps an already valid datetime-local value unchanged', () => {
    expect(toDateTimeLocalValue('2026-04-20T05:30')).toBe('2026-04-20T05:30')
  })

  it('drops seconds from datetime-local-with-seconds values', () => {
    expect(toDateTimeLocalValue('2026-04-20T05:30:45')).toBe('2026-04-20T05:30')
  })

  it('returns an empty string for nullish values', () => {
    expect(toDateTimeLocalValue(null)).toBe('')
    expect(toDateTimeLocalValue(undefined)).toBe('')
  })
})
