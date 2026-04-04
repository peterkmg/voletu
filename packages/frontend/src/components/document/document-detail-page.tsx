import { DocumentHeader } from './document-header'
import { DocumentForm } from './document-form'
import { BasisLink } from './basis-link'

interface DocumentDetailConfig {
  title: string
  entityLabel: string
  backTo: string
  executeFn: (id: string) => Promise<unknown>
  revertFn: (id: string) => Promise<unknown>
  queryKey: readonly unknown[]
  statusColorMap?: Record<string, string>
  basis?: {
    label: string
  }
}

interface DocumentDetailPageProps {
  config: DocumentDetailConfig
  document: {
    id: string
    documentNumber: string
    status: string
    [key: string]: unknown
  }
  subtitle?: string
  basisDocument?: {
    documentNumber: string
    details: { label: string; value: string }[]
    navigateTo: string
  }
  formContent: React.ReactNode
  itemsContent?: React.ReactNode
  metadataContent?: React.ReactNode
}

export type { DocumentDetailConfig }

export function DocumentDetailPage({
  config,
  document,
  subtitle,
  basisDocument,
  formContent,
  itemsContent,
  metadataContent,
}: DocumentDetailPageProps) {
  const isLocked = document.status === 'POSTED'

  return (
    <div className="mx-auto max-w-4xl space-y-6 p-4">
      <DocumentHeader
        title={config.title}
        documentNumber={document.documentNumber}
        subtitle={subtitle}
        status={document.status}
        statusColorMap={config.statusColorMap}
        backTo={config.backTo}
        executeFn={config.executeFn}
        revertFn={config.revertFn}
        queryKey={config.queryKey}
        entityLabel={config.entityLabel}
        documentId={document.id}
      />

      {basisDocument && config.basis && (
        <BasisLink
          label={config.basis.label}
          documentNumber={basisDocument.documentNumber}
          details={basisDocument.details}
          navigateTo={basisDocument.navigateTo}
        />
      )}

      <div>
        <h3 className="mb-3 text-sm font-medium uppercase tracking-wider text-muted-foreground">
          Document Details
        </h3>
        <DocumentForm status={document.status}>
          {formContent}
        </DocumentForm>
      </div>

      {itemsContent}

      {isLocked && metadataContent && (
        <div>
          <h3 className="mb-3 text-sm font-medium uppercase tracking-wider text-muted-foreground">
            Execution Metadata
          </h3>
          {metadataContent}
        </div>
      )}
    </div>
  )
}
