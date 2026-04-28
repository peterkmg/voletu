import type { RenderOptions } from '@testing-library/react'
import type { ReactElement } from 'react'
import { render } from '@testing-library/react'

export function renderWithProviders(ui: ReactElement, options?: Omit<RenderOptions, 'wrapper'>) {
  return render(ui, options)
}

export { render } from '@testing-library/react'
export { default as userEvent } from '@testing-library/user-event'
