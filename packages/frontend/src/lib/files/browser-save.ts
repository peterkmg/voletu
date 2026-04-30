import type { ExportFile, SaveExportFileResult } from './export-file'
import { normalizeExportContents, sanitizeExportFilename } from './export-file'

export async function saveExportFileInBrowser(
  file: ExportFile,
): Promise<SaveExportFileResult> {
  const blob = new Blob([normalizeExportContents(file.contents)], {
    type: file.mimeType,
  })

  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')

  link.href = url
  link.download = sanitizeExportFilename(file.suggestedName)
  link.style.display = 'none'
  document.body.appendChild(link)

  try {
    link.click()
    return { status: 'saved', target: 'browser' }
  }
  finally {
    link.remove()
    URL.revokeObjectURL(url)
  }
}
