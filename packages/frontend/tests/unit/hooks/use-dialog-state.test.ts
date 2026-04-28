import { act, renderHook } from '@testing-library/react'
import useDialogState from '~/hooks/use-dialog-state'

describe('useDialogState', () => {
  describe('initial state', () => {
    it('defaults to null when no argument is provided', () => {
      const { result } = renderHook(() => useDialogState())
      expect(result.current[0]).toBeNull()
    })

    it('accepts an initial string value', () => {
      const { result } = renderHook(() => useDialogState('edit'))
      expect(result.current[0]).toBe('edit')
    })

    it('accepts an initial boolean value', () => {
      const { result } = renderHook(() => useDialogState(true))
      expect(result.current[0]).toBe(true)
    })
  })

  describe('toggle behavior (string)', () => {
    it('opens when setting a value from null', () => {
      const { result } = renderHook(() => useDialogState<string>())

      act(() => {
        result.current[1]('create')
      })

      expect(result.current[0]).toBe('create')
    })

    it('closes (sets to null) when setting the same value again', () => {
      const { result } = renderHook(() => useDialogState<string>())

      act(() => result.current[1]('create'))
      act(() => result.current[1]('create'))

      expect(result.current[0]).toBeNull()
    })

    it('switches to a different value without toggling off', () => {
      const { result } = renderHook(() => useDialogState<string>())

      act(() => result.current[1]('create'))
      act(() => result.current[1]('edit'))

      expect(result.current[0]).toBe('edit')
    })
  })

  describe('toggle behavior (boolean)', () => {
    it('toggles true to null', () => {
      const { result } = renderHook(() => useDialogState(true))

      act(() => result.current[1](true))

      expect(result.current[0]).toBeNull()
    })

    it('sets true from null', () => {
      const { result } = renderHook(() => useDialogState<boolean>())

      act(() => result.current[1](true))

      expect(result.current[0]).toBe(true)
    })
  })

  describe('explicit close', () => {
    it('can close by setting null directly', () => {
      const { result } = renderHook(() => useDialogState<string>())

      act(() => result.current[1]('open'))
      act(() => result.current[1](null))

      expect(result.current[0]).toBeNull()
    })
  })
})
