import { cn, getPageNumbers } from '~/lib/utils'

describe('cn()', () => {
  it('merges class names', () => {
    expect(cn('foo', 'bar')).toBe('foo bar')
  })

  it('handles conditional classes', () => {
    expect(cn('base', false && 'hidden', 'end')).toBe('base end')
  })

  it('resolves Tailwind conflicts (last wins)', () => {
    expect(cn('p-4', 'p-2')).toBe('p-2')
  })

  it('resolves complex Tailwind conflicts', () => {
    expect(cn('text-red-500', 'text-blue-700')).toBe('text-blue-700')
  })

  it('handles empty inputs', () => {
    expect(cn()).toBe('')
  })

  it('handles undefined and null values', () => {
    expect(cn('a', undefined, null, 'b')).toBe('a b')
  })

  it('handles array inputs', () => {
    expect(cn(['foo', 'bar'])).toBe('foo bar')
  })

  it('handles object inputs', () => {
    expect(cn({ active: true, disabled: false })).toBe('active')
  })
})

describe('getPageNumbers()', () => {
  it('returns all pages when totalPages <= 7', () => {
    expect(getPageNumbers(1, 5)).toEqual([1, 2, 3, 4, 5])
  })

  it('returns all pages for exactly 7 pages', () => {
    expect(getPageNumbers(4, 7)).toEqual([1, 2, 3, 4, 5, 6, 7])
  })

  it('returns [1, 2, 3, 4, ..., last] when on page 1 with many pages', () => {
    const result = getPageNumbers(1, 20)
    expect(result[0]).toBe(1)
    expect(result).toContain(2)
    expect(result[result.length - 1]).toBe(20)
    expect(result).toContain('...')
  })

  it('returns [1, ..., pages around current, ..., last] when in the middle', () => {
    const result = getPageNumbers(10, 20)
    expect(result[0]).toBe(1)
    expect(result[result.length - 1]).toBe(20)
    expect(result).toContain(9)
    expect(result).toContain(10)
    expect(result).toContain(11)
    // Should have ellipsis on both sides
    const ellipsisCount = result.filter(p => p === '...').length
    expect(ellipsisCount).toBe(2)
  })

  it('returns [1, ..., last-3, last-2, last-1, last] when on last page', () => {
    const result = getPageNumbers(20, 20)
    expect(result[0]).toBe(1)
    expect(result[result.length - 1]).toBe(20)
    expect(result).toContain(19)
  })

  it('does not show leading ellipsis when on page 3', () => {
    const result = getPageNumbers(3, 20)
    expect(result[0]).toBe(1)
    expect(result[1]).toBe(2) // No ellipsis between 1 and 2
  })

  it('does not show trailing ellipsis when on second-to-last page', () => {
    const result = getPageNumbers(19, 20)
    const lastEllipsisIdx = result.lastIndexOf('...')
    // The trailing ellipsis should not appear because current (19) >= totalPages - 2 (18)
    // If there is an ellipsis, it should be the leading one, not trailing
    expect(lastEllipsisIdx).toBeLessThanOrEqual(1)
    expect(result[result.length - 1]).toBe(20)
  })

  it('returns single page for totalPages = 1', () => {
    expect(getPageNumbers(1, 1)).toEqual([1])
  })

  it('handles page 4 boundary (leading ellipsis appears)', () => {
    const result = getPageNumbers(4, 10)
    expect(result).toContain('...')
    expect(result[0]).toBe(1)
  })

  it('always includes first and last page for large sets', () => {
    const result = getPageNumbers(50, 100)
    expect(result[0]).toBe(1)
    expect(result[result.length - 1]).toBe(100)
  })
})
