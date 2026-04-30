import { formatDate, formatDateTime, truncateId, zeroPad } from '~/lib/formatters'

describe('formatDate()', () => {
  it('formats an ISO date string as YYYY-MM-DD', () => {
    expect(formatDate('2026-03-15T10:30:00.000Z')).toMatch(/^\d{4}-\d{2}-\d{2}$/)
  })

  it('formats a specific date correctly', () => {
    const result = formatDate('2026-01-05T00:00:00.000Z')

    expect(result).toMatch(/^\d{4}-\d{2}-\d{2}$/)
  })

  it('pads single-digit months and days', () => {
    const result = formatDate('2026-01-01T12:00:00.000Z')
    expect(result).toMatch(/^\d{4}-0\d-0\d$/)
  })

  it('handles end-of-year dates', () => {
    const result = formatDate('2025-12-31T12:00:00.000Z')
    expect(result).toMatch(/^2025-12-31$/)
  })
})

describe('formatDateTime()', () => {
  it('includes both date and time parts', () => {
    const result = formatDateTime('2026-03-15T10:30:00.000Z')
    expect(result).toMatch(/^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$/)
  })

  it('pads hours and minutes', () => {
    const result = formatDateTime('2026-06-15T04:05:00.000Z')
    expect(result).toMatch(/^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$/)
  })

  it('handles midnight', () => {
    const result = formatDateTime('2026-01-01T00:00:00.000Z')
    expect(result).toMatch(/^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$/)
  })
})

describe('zeroPad()', () => {
  it('pads a single digit to width 3', () => {
    expect(zeroPad(2, 3)).toBe('002')
  })

  it('pads a number to width 2', () => {
    expect(zeroPad(5, 2)).toBe('05')
  })

  it('does not truncate if number exceeds width', () => {
    expect(zeroPad(1234, 2)).toBe('1234')
  })

  it('handles zero', () => {
    expect(zeroPad(0, 4)).toBe('0000')
  })

  it('handles width of 1', () => {
    expect(zeroPad(7, 1)).toBe('7')
  })

  it('pads to large width', () => {
    expect(zeroPad(1, 8)).toBe('00000001')
  })
})

describe('truncateId()', () => {
  it('truncates a UUID to 8 characters with ellipsis', () => {
    const uuid = 'a1b2c3d4-e5f6-7890-abcd-ef1234567890'
    expect(truncateId(uuid)).toBe('a1b2c3d4\u2026')
  })

  it('returns short strings unchanged', () => {
    expect(truncateId('abcd')).toBe('abcd')
  })

  it('returns exactly 8-character strings unchanged', () => {
    expect(truncateId('12345678')).toBe('12345678')
  })

  it('truncates 9-character strings', () => {
    expect(truncateId('123456789')).toBe('12345678\u2026')
  })

  it('handles empty string', () => {
    expect(truncateId('')).toBe('')
  })
})
