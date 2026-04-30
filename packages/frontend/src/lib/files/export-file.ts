export interface ExportFile {
  suggestedName: string
  mimeType: string
  contents: string | Uint8Array
}

export interface SaveExportFileResult {
  status: 'saved' | 'cancelled'
  target: 'browser' | 'tauri'
}

export function sanitizeExportFilename(filename: string): string {
  const sanitized = filename
    .trim()
    .replace(/[<>:"/\\|?*\x00-\x1F]/g, '-')
    .replace(/\s+/g, ' ')

  return sanitized || 'export'
}

export function normalizeExportContents(contents: ExportFile['contents']): BlobPart {
  if (typeof contents === 'string')
    return contents
  return new Uint8Array(contents).buffer
}

export function normalizeExportBytes(contents: ExportFile['contents']): Uint8Array {
  if (typeof contents === 'string')
    return new TextEncoder().encode(contents)
  return contents
}

export function fileExtension(filename: string): string | undefined {
  const segments = filename.split(/[\\/]/)
  const lastSegment = segments[segments.length - 1] ?? filename
  const index = lastSegment.lastIndexOf('.')
  if (index <= 0 || index === lastSegment.length - 1)
    return undefined
  return lastSegment.slice(index + 1).toLowerCase()
}

export async function saveExportFile(file: ExportFile): Promise<SaveExportFileResult> {
  const { isTauriRuntime, saveExportFileWithTauri } = await import('./tauri-save')
  if (isTauriRuntime())
    return saveExportFileWithTauri(file)

  const { saveExportFileInBrowser } = await import('./browser-save')
  return saveExportFileInBrowser(file)
}
