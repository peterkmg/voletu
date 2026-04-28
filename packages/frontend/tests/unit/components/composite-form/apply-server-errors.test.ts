import { renderHook } from '@testing-library/react'
import { useForm } from 'react-hook-form'
import { describe, expect, it, vi } from 'vitest'
import { applyServerErrors } from '~/components/composite-form/apply-server-errors'

describe('applyServerErrors', () => {
  it('maps each direct-shape server error to form.setError using the dot+index path', () => {
    const { result } = renderHook(() =>
      useForm<{ items: { qty: number }[] }>({
        defaultValues: { items: [{ qty: 0 }, { qty: 0 }, { qty: 0 }] },
      }),
    )

    const setErrorSpy = vi.spyOn(result.current, 'setError')

    applyServerErrors(result.current, {
      errors: [
        { field: 'items.0.qty', code: 'too_small' },
        { field: 'items.2.qty', code: 'too_small' },
      ],
    })

    expect(setErrorSpy).toHaveBeenCalledWith(
      'items.0.qty',
      expect.objectContaining({ type: 'server', message: 'forms.validation.too_small' }),
    )
    expect(setErrorSpy).toHaveBeenCalledWith(
      'items.2.qty',
      expect.objectContaining({ type: 'server', message: 'forms.validation.too_small' }),
    )
  })

  it('also accepts the envelope shape { error: { issues: [...] } }', () => {
    const { result } = renderHook(() =>
      useForm<{ document_number: string }>({ defaultValues: { document_number: '' } }),
    )

    const setErrorSpy = vi.spyOn(result.current, 'setError')

    applyServerErrors(result.current, {
      error: {
        issues: [{ field: 'document_number', code: 'min_length' }],
      },
    })

    expect(setErrorSpy).toHaveBeenCalledWith(
      'document_number',
      expect.objectContaining({ type: 'server', message: 'forms.validation.min_length' }),
    )
  })

  it('focuses the first failing field', () => {
    const { result } = renderHook(() =>
      useForm<{ items: { qty: number }[] }>({ defaultValues: { items: [{ qty: 0 }] } }),
    )
    const setFocusSpy = vi.spyOn(result.current, 'setFocus')

    applyServerErrors(result.current, {
      errors: [{ field: 'items.0.qty', code: 'too_small' }],
    })

    expect(setFocusSpy).toHaveBeenCalledWith('items.0.qty', { shouldSelect: true })
  })

  it('falls back to the server message when no code is provided', () => {
    const { result } = renderHook(() => useForm<{ x: string }>({ defaultValues: { x: '' } }))
    const setErrorSpy = vi.spyOn(result.current, 'setError')

    applyServerErrors(result.current, {
      errors: [{ field: 'x', code: '', message: 'Custom server message' }],
    })

    expect(setErrorSpy).toHaveBeenCalledWith(
      'x',
      expect.objectContaining({ type: 'server', message: 'Custom server message' }),
    )
  })

  it('returns global errors (no field path) without calling setError', () => {
    const { result } = renderHook(() => useForm<{ x: string }>({ defaultValues: { x: '' } }))
    const setErrorSpy = vi.spyOn(result.current, 'setError')

    const globals = applyServerErrors(result.current, {
      errors: [{ field: '', code: 'composite_invalid', message: 'Global rule failed' }],
    })

    expect(setErrorSpy).not.toHaveBeenCalled()
    expect(globals).toEqual([
      { field: '', code: 'composite_invalid', message: 'Global rule failed' },
    ])
  })

  it('returns empty array when there are no validation issues', () => {
    const { result } = renderHook(() => useForm<{ x: string }>({ defaultValues: { x: '' } }))
    const globals = applyServerErrors(result.current, { errors: [] })
    expect(globals).toEqual([])
  })
})
