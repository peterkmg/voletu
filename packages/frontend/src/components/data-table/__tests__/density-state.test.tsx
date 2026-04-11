import { act, renderHook } from '@testing-library/react'

describe('table density state', () => {
  beforeEach(() => {
    localStorage.removeItem('voletu.table-density')
    vi.resetModules()
  })

  it('reads the persisted density value', async () => {
    localStorage.setItem('voletu.table-density', 'compact')
    const { useTableDensity } = await import('~/components/data-table/density-state')

    const { result } = renderHook(() => useTableDensity())

    expect(result.current.density).toBe('compact')
  })

  it('shares updates across consumers without relying on synthetic storage events', async () => {
    const { useTableDensity } = await import('~/components/data-table/density-state')

    const first = renderHook(() => useTableDensity())
    const second = renderHook(() => useTableDensity())

    act(() => {
      first.result.current.setDensity('comfortable')
    })

    expect(first.result.current.density).toBe('comfortable')
    expect(second.result.current.density).toBe('comfortable')
    expect(localStorage.getItem('voletu.table-density')).toBe('comfortable')
  })
})
