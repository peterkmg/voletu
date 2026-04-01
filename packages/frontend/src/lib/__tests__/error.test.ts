import { extractErrorMessage } from '~/lib/error'

describe('extractErrorMessage()', () => {
  it('extracts message from an Error instance', () => {
    const err = new Error('Something went wrong')
    expect(extractErrorMessage(err)).toBe('Something went wrong')
  })

  it('extracts message from a TypeError', () => {
    const err = new TypeError('Type mismatch')
    expect(extractErrorMessage(err)).toBe('Type mismatch')
  })

  it('returns the string directly when err is a string', () => {
    expect(extractErrorMessage('plain error text')).toBe('plain error text')
  })

  it('returns an empty string when err is an empty string', () => {
    expect(extractErrorMessage('')).toBe('')
  })

  it('returns default fallback for unknown types', () => {
    expect(extractErrorMessage(42)).toBe('An unexpected error occurred')
  })

  it('returns default fallback for null', () => {
    expect(extractErrorMessage(null)).toBe('An unexpected error occurred')
  })

  it('returns default fallback for undefined', () => {
    expect(extractErrorMessage(undefined)).toBe('An unexpected error occurred')
  })

  it('returns default fallback for an object', () => {
    expect(extractErrorMessage({ code: 500 })).toBe('An unexpected error occurred')
  })

  it('returns default fallback for a boolean', () => {
    expect(extractErrorMessage(false)).toBe('An unexpected error occurred')
  })

  it('uses custom fallback message', () => {
    expect(extractErrorMessage(123, 'Custom fallback')).toBe('Custom fallback')
  })

  it('uses custom fallback for null', () => {
    expect(extractErrorMessage(null, 'Nothing here')).toBe('Nothing here')
  })

  it('prefers Error.message over custom fallback', () => {
    const err = new Error('Real message')
    expect(extractErrorMessage(err, 'Fallback')).toBe('Real message')
  })

  it('prefers string error over custom fallback', () => {
    expect(extractErrorMessage('Direct string', 'Fallback')).toBe('Direct string')
  })
})
