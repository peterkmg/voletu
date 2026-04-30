import type { ComponentType, ReactNode } from 'react'
import type { ControllerFieldState, ControllerRenderProps, FieldValues, Path } from 'react-hook-form'

export type CompositeMode = 'create' | 'edit'

export interface HeaderFieldComponentProps<TForm extends FieldValues = FieldValues> {
  field: ControllerRenderProps<TForm, Path<TForm>>
  fieldState: ControllerFieldState
  placeholder?: string
  disabled?: boolean
}

export interface HeaderFieldSpec<TForm extends FieldValues = FieldValues> {
  name: Path<TForm>
  labelKey: string
  placeholderKey?: string
  component: ComponentType<HeaderFieldComponentProps<TForm>>
  required?: boolean
  colSpan?: 1 | 2
}

export interface ColumnSpec<TItem = unknown> {
  key: keyof TItem & string
  labelKey: string
  render?: (value: unknown, row: TItem) => ReactNode
  gridWidth?: string
  widthClass?: string
  alignClass?: string
}

export type RowFieldSpec<TItem extends FieldValues = FieldValues> = HeaderFieldSpec<TItem>

export interface ServerValidationIssue {
  field: string
  code: string
  message?: string
}

export interface ServerValidationError {
  errors: ServerValidationIssue[]
}

export type TableDensity = 'default' | 'compact'

export type CompositeMutationFn<TForm extends FieldValues, TResponse> = (
  data: TForm,
) => Promise<TResponse>

export type CompositeSuccessHandler<TResponse> = (saved: TResponse) => void

export function isServerValidationError(error: unknown): error is ServerValidationError {
  if (typeof error !== 'object' || error === null)
    return false

  if ('errors' in error && Array.isArray((error as ServerValidationError).errors)) {
    return true
  }

  const envelope = error as { error?: { issues?: unknown } }

  return Array.isArray(envelope.error?.issues)
}

export function extractValidationIssues(error: ServerValidationError | { error?: { issues?: ServerValidationIssue[] } }): ServerValidationIssue[] {
  if ('errors' in error && Array.isArray(error.errors))
    return error.errors

  const envelope = error as { error?: { issues?: ServerValidationIssue[] } }

  return envelope.error?.issues ?? []
}
