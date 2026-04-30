import { afterEach, describe, expect, it, vi } from 'vitest'
import {
  createTauriSaveAdapter,
  sanitizeExportFilename,
  saveExportFileInBrowser,
} from '~/lib/files'

describe('export file saving', () => {
  afterEach(() => {
    vi.restoreAllMocks()
    document.body.innerHTML = ''
  })

  it('sanitizes filenames before they are handed to a platform save adapter', () => {
    expect(sanitizeExportFilename(' ledger/report:April?.csv ')).toBe('ledger-report-April-.csv')
    expect(sanitizeExportFilename('   ')).toBe('export')
  })

  it('downloads text payloads in the browser and cleans up the temporary link', async () => {
    const objectUrl = 'blob:voletu-export'
    const createObjectUrl = vi.spyOn(URL, 'createObjectURL').mockReturnValue(objectUrl)
    const revokeObjectUrl = vi.spyOn(URL, 'revokeObjectURL').mockImplementation(() => {})
    const click = vi.spyOn(HTMLAnchorElement.prototype, 'click').mockImplementation(() => {})

    const result = await saveExportFileInBrowser({
      suggestedName: 'report.csv',
      mimeType: 'text/csv;charset=utf-8',
      contents: 'name,amount\nAlpha,12',
    })

    expect(result).toEqual({ status: 'saved', target: 'browser' })
    expect(createObjectUrl).toHaveBeenCalledTimes(1)
    expect(createObjectUrl.mock.calls[0]?.[0]).toBeInstanceOf(Blob)
    expect(click).toHaveBeenCalledTimes(1)
    expect(revokeObjectUrl).toHaveBeenCalledWith(objectUrl)
    expect(document.body.querySelectorAll('a')).toHaveLength(0)
  })

  it('saves through the Tauri dialog and filesystem adapters', async () => {
    const save = vi.fn().mockResolvedValue('C:\\Users\\pk\\Downloads\\report.csv')
    const writeFile = vi.fn().mockResolvedValue(undefined)
    const saveWithTauri = createTauriSaveAdapter({ save, writeFile })

    const result = await saveWithTauri({
      suggestedName: 'report.csv',
      mimeType: 'text/csv;charset=utf-8',
      contents: 'name,amount\nAlpha,12',
    })

    expect(result).toEqual({ status: 'saved', target: 'tauri' })
    expect(save).toHaveBeenCalledWith({
      defaultPath: 'report.csv',
      filters: [{ name: 'CSV', extensions: ['csv'] }],
    })
    expect(writeFile).toHaveBeenCalledWith(
      'C:\\Users\\pk\\Downloads\\report.csv',
      new TextEncoder().encode('name,amount\nAlpha,12'),
    )
  })

  it('reports cancellation when the Tauri save dialog returns no path', async () => {
    const save = vi.fn().mockResolvedValue(null)
    const writeFile = vi.fn().mockResolvedValue(undefined)
    const saveWithTauri = createTauriSaveAdapter({ save, writeFile })

    const result = await saveWithTauri({
      suggestedName: 'report.csv',
      mimeType: 'text/csv;charset=utf-8',
      contents: 'name,amount\nAlpha,12',
    })

    expect(result).toEqual({ status: 'cancelled', target: 'tauri' })
    expect(writeFile).not.toHaveBeenCalled()
  })
})
