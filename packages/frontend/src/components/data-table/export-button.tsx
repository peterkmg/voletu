import type { Table } from '@tanstack/react-table'
import { Download } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { Button } from '~/components/ui/button'
import { saveExportFile } from '~/lib/files'
import { buildCsvExportFile } from './csv-export'

interface ExportButtonProps<TData> {
  table: Table<TData>
  filename?: string
}

export function ExportButton<TData>({
  table,
  filename = 'export',
}: ExportButtonProps<TData>) {
  const { t } = useTranslation('tables')

  const handleExport = async () => {
    try {
      await saveExportFile(buildCsvExportFile(table, { filename }))
    }
    catch {
      toast.error(t('tables:exportFailed'))
    }
  }

  return (
    <Button
      variant="outline"
      size="sm"
      type="button"
      className="h-8"
      onClick={handleExport}
    >
      <Download className="size-4" />
      {t('tables:export')}
    </Button>
  )
}
