import { useTranslation } from 'react-i18next'
import { Card, CardContent, CardHeader, CardTitle } from '~/components/ui/card'
import { usePageTitle } from '~/hooks/use-page-title'
import { DocumentForm } from './document-form'
import { DocumentHeader } from './document-header'

interface DocumentDetailConfig {
  title: string
  entityLabel: string
  backTo: string
  executeFn?: (id: string) => Promise<unknown>
  revertFn?: (id: string) => Promise<unknown>
  queryKey?: readonly unknown[]
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
  const isLocked = document.status === 'EXECUTED'
  usePageTitle(`${config.title} ${document.documentNumber}`)

  return (
    <div data-layout="fixed" className="flex h-full flex-col">
      {/* Sticky header */}
      <div className="sticky top-0 z-10 rounded-t-lg border-b bg-background px-4 py-3">
        <div className="mx-auto max-w-5xl">
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
        </div>
      </div>

      {/* Scrollable content */}
      <div className="flex-1 overflow-y-auto">
        <div className="mx-auto max-w-5xl space-y-6 p-4 pb-8">
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
      </div>
    </div>
  )
}
