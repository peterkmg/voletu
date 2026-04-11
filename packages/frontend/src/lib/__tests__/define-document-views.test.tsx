import type { Row } from '@tanstack/react-table'
import { render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import {
  defineDocumentViews,
  defineResolvedDetailView,
} from '~/lib/define-document-views'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}))

vi.mock('@tanstack/react-router', () => ({
  useNavigate: () => vi.fn(),
}))

vi.mock('~/stores/auth-store', () => ({
  useAuthStore: (selector: (state: { user: { role: string } }) => unknown) =>
    selector({ user: { role: 'ADMIN' } }),
}))

vi.mock('~/components/layout/header', () => ({
  Header: () => <div data-testid="header" />,
}))

vi.mock('~/components/layout/main', () => ({
  Main: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="main">{children}</div>
  ),
}))

vi.mock('~/hooks/use-page-title', () => ({
  usePageTitle: vi.fn(),
}))

vi.mock('~/components/data-table', () => ({
  RowActions: ({ actions }: { actions: Array<{ label: string, onClick: () => void }> }) => (
    <div>
      {actions.map(action => (
        <button key={action.label} type="button" onClick={action.onClick}>
          {action.label}
        </button>
      ))}
    </div>
  ),
}))

vi.mock('~/components/document', async () => {
  const actual = await vi.importActual<object>('~/components/document')
  return {
    ...actual,
    DocumentDetailPage: ({
      config,
      document,
      subtitle,
      relatedContent,
      formContent,
      itemsContent,
      metadataContent,
    }: {
      config: { title: string, entityLabel: string, backTo: string }
      document: { id: string, documentNumber: string, status: string }
      subtitle?: string
      relatedContent?: React.ReactNode
      formContent: React.ReactNode
      itemsContent?: React.ReactNode
      metadataContent?: React.ReactNode
    }) => (
      <section data-testid="detail-page">
        <div data-testid="detail-title">{config.title}</div>
        <div data-testid="detail-entity">{config.entityLabel}</div>
        <div data-testid="detail-back-to">{config.backTo}</div>
        <div data-testid="detail-document">{document.documentNumber}</div>
        <div data-testid="detail-status">{document.status}</div>
        {subtitle && <div data-testid="detail-subtitle">{subtitle}</div>}
        {relatedContent}
        {formContent}
        {itemsContent}
        {metadataContent}
      </section>
    ),
  }
})

interface TestRow {
  id: string
  documentId: string
  name: string
}

interface TestDetailData {
  document: {
    id: string
    documentNumber: string
    status: string
  }
  formLabel: string
  relatedLabel: string
  itemCount: number
  executedAt?: string
}

const testRow: TestRow = {
  id: 'row-1',
  documentId: 'doc-1',
  name: 'Alpha',
}

describe('defineDocumentViews', () => {
  it('builds a standard document detail page from one shared config', () => {
    const documentViewDefinition = defineDocumentViews<TestRow, TestDetailData>({
      displayName: 'TestDocuments',
      useText: () => ({
        title: 'documents.title',
        entityLabel: 'documents.entity',
        subtitle: 'documents.subtitle',
      }),
      useQuery: () => ({
        data: { data: [testRow] },
        isLoading: false,
      }),
      Table: ({
        data,
        actions,
        RowActions,
      }: {
        data: TestRow[]
        actions?: React.ReactNode
        RowActions: React.ComponentType<{ row: Row<TestRow> }>
      }) => (
        <div>
          <div data-testid="row-name">{data[0]?.name}</div>
          {actions}
          <RowActions row={{ original: data[0] } as Row<TestRow>} />
        </div>
      ),
      MutateDialog: () => null,
      rowActions: {
        getDetailPath: row => `/documents/${row.documentId}`,
      },
      documentActions: {
        enableRowLifecycle: true,
        executeFn: vi.fn(async () => undefined),
        revertFn: vi.fn(async () => undefined),
        queryKey: () => ['documents'],
      },
      detail: {
        useDetailId: () => 'doc-1',
        useDetailQuery: () => ({
          data: {
            data: {
              document: {
                id: 'doc-1',
                documentNumber: 'DOC-001',
                status: 'EXECUTED',
              },
              formLabel: 'Form content',
              relatedLabel: 'Related content',
              itemCount: 3,
              executedAt: '2026-04-11T10:00:00Z',
            },
          },
          isLoading: false,
        }),
        backTo: '/documents',
        getDocument: data => data.document,
        renderRelatedContent: ({ data }) => <div>{data.relatedLabel}</div>,
        renderFormContent: ({ data }) => <div>{data.formLabel}</div>,
        renderItemsContent: ({ data, isLocked }) => (
          <div>{`${isLocked ? 'locked' : 'open'}:${data.itemCount}`}</div>
        ),
        renderMetadataContent: ({ data }) => <div>{data.executedAt}</div>,
      },
    })

    render(<documentViewDefinition.DetailView />)

    expect(screen.getByTestId('detail-title')).toHaveTextContent('documents.title')
    expect(screen.getByTestId('detail-entity')).toHaveTextContent('documents.entity')
    expect(screen.getByTestId('detail-back-to')).toHaveTextContent('/documents')
    expect(screen.getByTestId('detail-document')).toHaveTextContent('DOC-001')
    expect(screen.getByTestId('detail-subtitle')).toHaveTextContent('documents.subtitle')
    expect(screen.getByText('Related content')).toBeInTheDocument()
    expect(screen.getByText('Form content')).toBeInTheDocument()
    expect(screen.getByText('locked:3')).toBeInTheDocument()
    expect(screen.getByText('2026-04-11T10:00:00Z')).toBeInTheDocument()
  })
})

describe('defineResolvedDetailView', () => {
  it('renders the first matching document variant and falls back cleanly when nothing resolves', () => {
    const MatchingDetail = defineResolvedDetailView({
      useDetailId: () => 'doc-1',
      getNotFoundMessage: () => 'Document not found',
      useVariants: () => [
        {
          isLoading: false,
        },
        {
          isLoading: false,
          content: <div>pending-detail</div>,
        },
      ],
    })

    const MissingDetail = defineResolvedDetailView({
      useDetailId: () => 'doc-2',
      getNotFoundMessage: () => 'Document not found',
      useVariants: () => [
        {
          isLoading: false,
        },
      ],
    })

    const { rerender } = render(<MatchingDetail />)
    expect(screen.getByText('pending-detail')).toBeInTheDocument()

    rerender(<MissingDetail />)
    expect(screen.getByText('Document not found')).toBeInTheDocument()
  })
})
