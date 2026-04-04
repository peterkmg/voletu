import { Link } from '@tanstack/react-router'
import { ChevronRight } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Card, CardContent, CardHeader, CardTitle } from '~/components/ui/card'
import { StatusBadge } from '~/components/ui/status-badge'
import { cn } from '~/lib/utils'

export interface RelatedDocument {
  type: 'basis' | 'reference'
  label: string
  documentNumber: string
  status?: string
  statusColorMap?: Record<string, string>
  details?: { label: string, value: string }[]
  to: string
}

interface RelatedDocumentsProps {
  documents: RelatedDocument[]
}

export function RelatedDocuments({ documents }: RelatedDocumentsProps) {
  const { t } = useTranslation(['common'])

  if (documents.length === 0)
    return null

  const basis = documents.filter(d => d.type === 'basis')
  const references = documents.filter(d => d.type === 'reference')

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
          {t('common:document.relatedDocuments')}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-2">
        {basis.map(doc => (
          <RelatedDocumentItem key={doc.documentNumber} doc={doc} borderColor="border-l-blue-500" labelColor="text-blue-500" />
        ))}
        {references.map(doc => (
          <RelatedDocumentItem key={doc.documentNumber} doc={doc} borderColor="border-l-violet-500" labelColor="text-violet-500" />
        ))}
      </CardContent>
    </Card>
  )
}

function RelatedDocumentItem({
  doc,
  borderColor,
  labelColor,
}: {
  doc: RelatedDocument
  borderColor: string
  labelColor: string
}) {
  return (
    <Link to={doc.to} className="block">
      <div
        className={cn(
          'flex items-center gap-3 rounded-lg border-l-4 bg-muted/50 p-3 transition-colors hover:bg-muted',
          borderColor,
        )}
      >
        <div className="flex-1 space-y-1">
          <div className={cn('text-xs font-semibold uppercase tracking-wider', labelColor)}>
            {doc.type === 'basis' ? 'Basis' : 'Referenced by'}
            :
            {doc.label}
          </div>
          <div className="flex items-center gap-2">
            <span className="font-semibold">{doc.documentNumber}</span>
            {doc.status && doc.statusColorMap && (
              <StatusBadge value={doc.status} colorMap={doc.statusColorMap} />
            )}
          </div>
          {doc.details && doc.details.length > 0 && (
            <div className="flex gap-4 text-xs text-muted-foreground">
              {doc.details.map(d => (
                <span key={d.label}>
                  {d.label}
                  :
                  {' '}
                  {d.value}
                </span>
              ))}
            </div>
          )}
        </div>
        <ChevronRight className="size-4 text-muted-foreground" />
      </div>
    </Link>
  )
}
