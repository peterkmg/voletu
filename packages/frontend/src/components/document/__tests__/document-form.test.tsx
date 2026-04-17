import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { DocumentForm, useDocumentFormLock } from '../document-form'

function LockStatus() {
  const { isLocked } = useDocumentFormLock()
  return <span data-testid="locked">{String(isLocked)}</span>
}

describe('documentForm', () => {
  it('renders children as editable when DRAFT', () => {
    const { container } = render(
      <DocumentForm status="DRAFT">
        <input data-testid="field" />
      </DocumentForm>,
    )
    const fieldset = container.querySelector('fieldset')
    expect(fieldset).not.toBeDisabled()
  })

  it('renders children as disabled when EXECUTED', () => {
    const { container } = render(
      <DocumentForm status="EXECUTED">
        <input data-testid="field" />
      </DocumentForm>,
    )
    const fieldset = container.querySelector('fieldset')
    expect(fieldset).toBeDisabled()
  })

  it('locks for unknown status (safe default)', () => {
    const { container } = render(
      <DocumentForm status="SOME_OTHER">
        <input data-testid="field" />
      </DocumentForm>,
    )
    const fieldset = container.querySelector('fieldset')
    expect(fieldset).toBeDisabled()
  })

  it('provides isLocked=false via context when DRAFT', () => {
    render(
      <DocumentForm status="DRAFT">
        <LockStatus />
      </DocumentForm>,
    )
    expect(screen.getByTestId('locked')).toHaveTextContent('false')
  })

  it('provides isLocked=true via context when EXECUTED', () => {
    render(
      <DocumentForm status="EXECUTED">
        <LockStatus />
      </DocumentForm>,
    )
    expect(screen.getByTestId('locked')).toHaveTextContent('true')
  })
})
