import type { ActionDescriptor } from './document-header'
import { useTranslation } from 'react-i18next'
import { Card, CardContent, CardHeader, CardTitle } from '~/components/ui/card'
import { isDocEditable } from '~/hooks/use-doc-editable'
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

  actions?: ActionDescriptor[]
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
  const { t } = useTranslation(['documents'])
  const isLocked = !isDocEditable(document)
  usePageTitle(`${config.title} ${document.documentNumber}`)

  return (
    <div data-layout="fixed" data-slot="document-detail-page" className="flex h-full flex-col print:block print:h-auto">
      {/* Sticky header */}
      <div className="sticky top-0 z-10 rounded-t-lg border-b bg-background px-4 py-3 print:hidden">
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
            actions={config.actions}
          />
        </div>
      </div>

      {/* Scrollable content */}
      <div className="flex-1 overflow-y-auto print:overflow-visible">
        <div className="mx-auto max-w-5xl space-y-6 p-4 pb-8 print:max-w-none print:p-0">
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
                {t('documents:document.information')}
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
                  {t('documents:document.executionMetadata')}
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
