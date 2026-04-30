import type { ExportFile, SaveExportFileResult } from './export-file'
import { fileExtension, normalizeExportBytes, sanitizeExportFilename } from './export-file'

type SaveDialog = (options: {
  defaultPath: string
  filters?: Array<{ name: string, extensions: string[] }>
}) => Promise<string | null>

type WriteFile = (path: string, contents: Uint8Array) => Promise<void>

interface TauriSaveDependencies {
  save: SaveDialog
  writeFile: WriteFile
}

interface TauriDialogModule {
  save: SaveDialog
}

interface TauriFsModule {
  writeFile: WriteFile
}

export function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

export function createTauriSaveAdapter({ save, writeFile }: TauriSaveDependencies) {
  return async function saveExportFileWithDependencies(
    file: ExportFile,
  ): Promise<SaveExportFileResult> {
    const suggestedName = sanitizeExportFilename(file.suggestedName)
    const path = await save({
      defaultPath: suggestedName,
      filters: exportFileFilters(suggestedName, file.mimeType),
    })

    if (!path)
      return { status: 'cancelled', target: 'tauri' }

    await writeFile(path, normalizeExportBytes(file.contents))
    return { status: 'saved', target: 'tauri' }
  }
}

export async function saveExportFileWithTauri(
  file: ExportFile,
): Promise<SaveExportFileResult> {
  const [{ save }, { writeFile }] = await Promise.all([
    import('@tauri-apps/plugin-dialog') as Promise<TauriDialogModule>,
    import('@tauri-apps/plugin-fs') as Promise<TauriFsModule>,
  ])

  return createTauriSaveAdapter({ save, writeFile })(file)
}

function exportFileFilters(
  suggestedName: string,
  mimeType: string,
): Array<{ name: string, extensions: string[] }> | undefined {
  const extension = fileExtension(suggestedName)
  if (!extension)
    return undefined

  return [{ name: filterName(extension, mimeType), extensions: [extension] }]
}

function filterName(extension: string, mimeType: string): string {
  if (extension === 'csv' || mimeType.includes('csv'))
    return 'CSV'

  if (extension === 'xlsx')
    return 'Excel'

  if (extension === 'png')
    return 'PNG'

  if (extension === 'svg')
    return 'SVG'

  return extension.toUpperCase()
}
