import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import { DocumentDetailPage } from '~/components/document/document-detail-page'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}))

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}))

vi.mock('~/hooks/use-page-title', () => ({
  usePageTitle: vi.fn(),
}))

describe('documentDetailPage', () => {
  it('renders detail views without lifecycle controls when no document actions are provided', () => {
    render(
      <DocumentDetailPage
        config={{
          title: 'documents.title',
          entityLabel: 'documents.entity',
          backTo: '/documents',
        }}
        document={{
          id: 'doc-1',
          documentNumber: 'DOC-001',
          status: 'PENDING',
        }}
        subtitle="documents.subtitle"
        formContent={<div>form content</div>}
        itemsContent={<div>items content</div>}
      />,
    )

    expect(screen.getByText('documents.title')).toBeInTheDocument()
    expect(screen.getByText('DOC-001')).toBeInTheDocument()
    expect(screen.getByText('documents.subtitle')).toBeInTheDocument()
    expect(screen.getByText('form content')).toBeInTheDocument()
    expect(screen.getByText('items content')).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'documents:lifecycle.execute' })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'documents:lifecycle.revert' })).not.toBeInTheDocument()
  })
})
