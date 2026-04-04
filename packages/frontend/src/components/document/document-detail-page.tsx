import { useTranslation } from 'react-i18next'
import { Card, CardContent, CardHeader, CardTitle } from '~/components/ui/card'
import { DocumentForm } from './document-form'
import { DocumentHeader } from './document-header'

interface DocumentDetailConfig {
  title: string
  entityLabel: string
  backTo: string
  executeFn: (id: string) => Promise<unknown>
  revertFn: (id: string) => Promise<unknown>
  queryKey: readonly unknown[]
  statusColorMap?: Record<string, string>
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
  relatedContent?: React.ReactNode
  formContent: React.ReactNode
  itemsContent?: React.ReactNode
  metadataContent?: React.ReactNode
}

export type { DocumentDetailConfig }

export function DocumentDetailPage({
  config,
  document,
  subtitle,
  relatedContent,
  formContent,
  itemsContent,
  metadataContent,
}: DocumentDetailPageProps) {
  const { t } = useTranslation(['common'])
  const isLocked = document.status === 'POSTED'

  return (
    <div className="mx-auto max-w-5xl space-y-6 p-4">
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

      <Card>
        <CardHeader>
          <CardTitle className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
            {t('common:document.information')}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <DocumentForm status={document.status}>
            {formContent}
          </DocumentForm>
        </CardContent>
      </Card>

      {relatedContent}

      {itemsContent}

      {isLocked && metadataContent && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
              {t('common:document.executionMetadata')}
            </CardTitle>
          </CardHeader>
          <CardContent>
            {metadataContent}
          </CardContent>
        </Card>
      )}
    </div>
  )
}
