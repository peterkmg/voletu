import type { FieldPath, FieldValues, UseFormReturn } from 'react-hook-form'
import type { ServerValidationError, ServerValidationIssue } from './types'
import { extractValidationIssues } from './types'

export function applyServerErrors<TForm extends FieldValues>(
  form: UseFormReturn<TForm>,
  error: ServerValidationError | { error?: { issues?: ServerValidationIssue[] } },
): ServerValidationIssue[] {
  const issues = extractValidationIssues(error)
  const globals: ServerValidationIssue[] = []

  let firstFieldPath: FieldPath<TForm> | null = null

  for (const issue of issues) {
    if (!issue.field) {
      globals.push(issue)
      continue
    }

    const message = issue.code
      ? `forms.validation.${issue.code}`
      : (issue.message ?? 'forms.validation.unknown')

    form.setError(issue.field as FieldPath<TForm>, { type: 'server', message })

    if (firstFieldPath === null) {
      firstFieldPath = issue.field as FieldPath<TForm>
    }
  }

  if (firstFieldPath) {
    form.setFocus(firstFieldPath, { shouldSelect: true })
  }

  return globals
}
