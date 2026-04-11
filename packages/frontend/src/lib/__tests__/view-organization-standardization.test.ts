import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const packageRoot = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  '../../..',
)

const standardizedCrudViewFiles = [
  'src/views/catalog/bases.tsx',
  'src/views/catalog/companies.tsx',
  'src/views/catalog/ports.tsx',
  'src/views/catalog/product-groups.tsx',
  'src/views/catalog/product-types.tsx',
  'src/views/catalog/products.tsx',
  'src/views/catalog/storages.tsx',
  'src/views/catalog/warehouses.tsx',
  'src/views/incoming/external-acceptance.tsx',
  'src/views/incoming/rail-receipt.tsx',
  'src/views/incoming/truck-receipt.tsx',
  'src/views/internal/blending.tsx',
  'src/views/internal/ownership-transfer.tsx',
  'src/views/internal/physical-transfer.tsx',
  'src/views/internal/reconciliation.tsx',
  'src/views/outgoing/bunkering.tsx',
  'src/views/outgoing/direct-dispatch.tsx',
  'src/views/outgoing/truck-dispatch.tsx',
  'src/views/system/users.tsx',
] as const

const documentViewFiles = [
  'src/views/incoming/external-acceptance.tsx',
  'src/views/internal/blending.tsx',
  'src/views/internal/ownership-transfer.tsx',
  'src/views/internal/physical-transfer.tsx',
  'src/views/internal/reconciliation.tsx',
  'src/views/outgoing/bunkering.tsx',
  'src/views/outgoing/direct-dispatch.tsx',
  'src/views/outgoing/truck-dispatch.tsx',
] as const

const resolvedDetailFiles = [
  'src/views/incoming/rail-receipt.tsx',
  'src/views/incoming/truck-receipt.tsx',
] as const

const legacyCrudAssemblyMarkers = [
  'createEntityProvider(',
  'createEntityDialogs(',
  'createPrimaryButtons(',
  'createRowActions(',
  'createDeleteDialog(',
] as const

describe('view organization standardization', () => {
  it.each(standardizedCrudViewFiles)(
    '%s is assembled through shared view definitions without legacy wiring',
    (viewPath) => {
      const content = readFileSync(path.join(packageRoot, viewPath), 'utf8')

      expect(
        content.includes('defineCrudViews')
        || content.includes('defineDocumentViews'),
      ).toBe(true)

      for (const marker of legacyCrudAssemblyMarkers) {
        expect(content).not.toContain(marker)
      }
    },
  )

  it.each(documentViewFiles)(
    '%s uses defineDocumentViews for document-family standardization',
    (viewPath) => {
      const content = readFileSync(path.join(packageRoot, viewPath), 'utf8')
      expect(content).toContain('defineDocumentViews')
    },
  )

  it.each(resolvedDetailFiles)(
    '%s uses defineResolvedDetailView for pipeline detail resolution',
    (viewPath) => {
      const content = readFileSync(path.join(packageRoot, viewPath), 'utf8')
      expect(content).toContain('defineResolvedDetailView')
    },
  )
})
