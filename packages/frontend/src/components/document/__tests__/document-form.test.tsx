import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { DocumentForm, useDocumentFormLock } from '../document-form'

function LockStatus() {
  const { isLocked } = useDocumentFormLock()
  return <span data-testid="locked">{String(isLocked)}</span>
}

describe('DocumentForm', () => {
  it('renders children as editable when draft', () => {
    const { container } = render(
      <DocumentForm status="draft">
        <input data-testid="field" />
      </DocumentForm>,
    )
    const fieldset = container.querySelector('fieldset')
    expect(fieldset).not.toBeDisabled()
  })

  it('renders children as disabled when posted', () => {
    const { container } = render(
      <DocumentForm status="posted">
        <input data-testid="field" />
      </DocumentForm>,
    )
    const fieldset = container.querySelector('fieldset')
    expect(fieldset).toBeDisabled()
  })

  it('renders children as disabled when executed', () => {
    const { container } = render(
      <DocumentForm status="executed">
        <input data-testid="field" />
      </DocumentForm>,
    )
    const fieldset = container.querySelector('fieldset')
    expect(fieldset).toBeDisabled()
  })

  it('provides isLocked=false via context when draft', () => {
    render(
      <DocumentForm status="draft">
        <LockStatus />
      </DocumentForm>,
    )
    expect(screen.getByTestId('locked')).toHaveTextContent('false')
  })

  it('provides isLocked=true via context when posted', () => {
    render(
      <DocumentForm status="posted">
        <LockStatus />
      </DocumentForm>,
    )
    expect(screen.getByTestId('locked')).toHaveTextContent('true')
  })
})
