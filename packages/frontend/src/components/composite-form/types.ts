import type { ComponentType, ReactNode } from 'react'
import type { ControllerFieldState, ControllerRenderProps, FieldValues, Path } from 'react-hook-form'

export type CompositeMode = 'create' | 'edit'

/**
 * Props passed to a `HeaderFieldSpec.component`. The outer renderer
 * (`<DocHeaderSection>` / `<DocItemRowDrawer>`) creates a single `<FormField>`
 * per spec and forwards the Controller render-prop pieces directly. Field
 * components MUST NOT create their own `<FormField>` / `<FormItem>` /
 * `<FormMessage>` — the outer renderer already provides them, and nesting
 * them here causes duplicate validation messages and labels.
 */
export interface HeaderFieldComponentProps<TForm extends FieldValues = FieldValues> {
  field: ControllerRenderProps<TForm, Path<TForm>>
  fieldState: ControllerFieldState
  placeholder?: string
  disabled?: boolean
}

/** Specification for one header field rendered by `<DocHeaderSection>`. */
export interface HeaderFieldSpec<TForm extends FieldValues = FieldValues> {
  /** Form field name (RHF dot path). */
  name: Path<TForm>
  /** i18n key for the visible label. */
  labelKey: string
  /** i18n key for placeholder (optional). */
  placeholderKey?: string
  /**
   * Component that renders the bare input. Receives the Controller
   * render-prop pieces (`field`, `fieldState`) plus optional `placeholder` /
   * `disabled`. See `HeaderFieldComponentProps` for the contract.
   */
  component: ComponentType<HeaderFieldComponentProps<TForm>>
  /** Whether to render the required-field asterisk and apply `required` semantics. */
  required?: boolean
  /** Optional grid span (1 = single column, 2 = full row in a 2-col grid). Default 1. */
  colSpan?: 1 | 2
}

/** Specification for one column in the read-only summary view of `<DocItemsTable>`. */
export interface ColumnSpec<TItem = unknown> {
  /** Field name in the row item (used for keying and display). */
  key: keyof TItem & string
  /** i18n key for the column header. */
  labelKey: string
  /** Custom renderer for the cell value. Default: stringify the value. */
  render?: (value: unknown, row: TItem) => ReactNode
  /**
   * CSS Grid track size for this column when consumed by the grid-based `<Table>`.
   * Examples: `'minmax(0, 1fr)'` (default), `'120px'`, `'minmax(80px, 200px)'`, `'auto'`.
   * Defaults to `minmax(0, 1fr)` (equal flex share).
   */
  gridWidth?: string
  /**
   * Decorative Tailwind class applied to the cell (e.g. `w-32`, `w-1/4`).
   *
   * NOTE: Does NOT control column width — the bespoke shadcn `<Table>` is a CSS Grid
   * wrapper whose track sizes come from `gridTemplate` (and thus `gridWidth`).
   * This class only styles cell contents (e.g. text wrapping, padding).
   */
  widthClass?: string
  /** Tailwind alignment, default `text-start`. */
  alignClass?: string
}

/** Specification for one field in the row drawer form. Same shape as HeaderFieldSpec. */
export type RowFieldSpec<TItem extends FieldValues = FieldValues> = HeaderFieldSpec<TItem>

/** A single error returned by the server validation response. Mirrors backend `ValidationIssue`. */
export interface ServerValidationIssue {
  field: string // RHF dot+index path, e.g. "items.2.accepted_amount"
  code: string // stable code, used as i18n key under `forms.validation.<code>`
  message?: string // server-side default rendering (fallback)
}

export interface ServerValidationError {
  /**
   * Backend's `ApiError::ValidationFields` response shape:
   *   { success: false, error: { code: "VALIDATION_ERROR", message: "...", issues: [...] } }
   * The thrown error from a Kubb-generated mutation may surface this nested under `.error.issues`
   * or already unwrapped — we handle both. The relevant field is the `issues` array.
   */
  errors: ServerValidationIssue[]
}

/** Density modifier for `<DocItemsTable>`. `compact` is used for nested inner tables. */
export type TableDensity = 'default' | 'compact'

/** Generic mutation function shape — accepts the form data, returns the saved entity. */
export type CompositeMutationFn<TForm extends FieldValues, TResponse> = (
  data: TForm,
) => Promise<TResponse>

/** Generic onSuccess callback the dialog invokes after a successful mutation. */
export type CompositeSuccessHandler<TResponse> = (saved: TResponse) => void

/**
 * Type guard: distinguishes a server-validation error from other errors.
 *
 * The Kubb client may throw the raw `ApiError::ValidationFields` envelope
 * `{ success: false, error: { code, message, issues } }`. This guard accepts
 * either that shape or a pre-extracted `{ errors: [...] }` shape.
 */
export function isServerValidationError(error: unknown): error is ServerValidationError {
  if (typeof error !== 'object' || error === null)
    return false
  // Direct shape: { errors: [...] }
  if ('errors' in error && Array.isArray((error as ServerValidationError).errors)) {
    return true
  }
  // Envelope shape: { error: { issues: [...] } }
  const envelope = error as { error?: { issues?: unknown } }
  return Array.isArray(envelope.error?.issues)
}

/** Extract issues from either the direct or envelope shape returned by `isServerValidationError`. */
export function extractValidationIssues(error: ServerValidationError | { error?: { issues?: ServerValidationIssue[] } }): ServerValidationIssue[] {
  if ('errors' in error && Array.isArray(error.errors))
    return error.errors
  const envelope = error as { error?: { issues?: ServerValidationIssue[] } }
  return envelope.error?.issues ?? []
}
