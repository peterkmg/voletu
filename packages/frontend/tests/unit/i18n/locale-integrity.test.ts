import { readdir, readFile } from 'node:fs/promises'
import { dirname, join, relative, sep } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

type LocaleTree = Record<string, unknown>

const testDirectory = dirname(fileURLToPath(import.meta.url))
const frontendSrc = join(testDirectory, '../../../src')
const localeModules = import.meta.glob('../../../src/i18n/locales/*/*.json', {
  eager: true,
  import: 'default',
}) as Record<string, LocaleTree>

interface TranslationReference {
  file: string
  key: string
}

const translationKeyPropertyNames = [
  'descriptionKey',
  'emptyStateKey',
  'labelKey',
  'placeholderKey',
  'sectionTitleKey',
  'titleKey',
]
const pluralSuffixes = ['zero', 'one', 'two', 'few', 'many', 'other']

async function collectSourceFiles(directory: string): Promise<string[]> {
  const entries = await readdir(directory, { withFileTypes: true })
  const files = await Promise.all(entries.map(async (entry) => {
    const path = join(directory, entry.name)

    if (entry.isDirectory()) {
      if (entry.name === '__tests__' || entry.name === 'locales')
        return []

      return collectSourceFiles(path)
    }

    if (!entry.isFile() || !/\.(?:ts|tsx)$/.test(entry.name) || entry.name === 'routeTree.gen.ts')
      return []

    return [path]
  }))

  return files.flat()
}

function collectLocales() {
  const locales: Record<string, Record<string, LocaleTree>> = {}

  for (const [path, value] of Object.entries(localeModules)) {
    const match = path.match(/\/locales\/([^/]+)\/([^/]+)\.json$/)
    const locale = match?.[1]
    const namespace = match?.[2]

    if (!locale || !namespace)
      throw new Error(`Unexpected locale path: ${path}`)

    locales[locale] ??= {}
    locales[locale][namespace] = value
  }

  return locales
}

function flattenKeys(value: unknown, prefix = ''): string[] {
  if (!value || typeof value !== 'object' || Array.isArray(value))
    return prefix ? [prefix] : []

  return Object.entries(value as Record<string, unknown>).flatMap(([key, child]) =>
    flattenKeys(child, prefix ? `${prefix}.${key}` : key),
  )
}

function difference(left: string[], right: string[]) {
  const rightSet = new Set(right)
  return left.filter(value => !rightSet.has(value))
}

function getPath(value: LocaleTree, keyPath: string) {
  return keyPath.split('.').reduce<unknown>((current, segment) => {
    if (!current || typeof current !== 'object' || Array.isArray(current))
      return undefined

    return (current as LocaleTree)[segment]
  }, value)
}

function hasTranslationKey(tree: LocaleTree, keyPath: string) {
  if (getPath(tree, keyPath) !== undefined)
    return true

  const variants = pluralSuffixes.filter(suffix => getPath(tree, `${keyPath}_${suffix}`) !== undefined)

  return variants.includes('one') && variants.includes('other')
}

function collectDefaultNamespaces(source: string) {
  const namespaces = new Set<string>()
  const pattern = /\buseTranslation\(\s*(?:['"]([a-z0-9-]+)['"]|\[\s*['"]([a-z0-9-]+)['"])/gi
  for (const match of source.matchAll(pattern)) {
    const namespace = match[1] ?? match[2]

    if (namespace)
      namespaces.add(namespace)
  }

  return namespaces
}

function addTranslationReference(references: Set<string>, key: string, defaultNamespaces: Set<string>) {
  if (key.includes(':')) {
    references.add(key)
    return
  }

  for (const namespace of defaultNamespaces) {
    references.add(`${namespace}:${key}`)
  }
}

function collectTranslationReferences(file: string, source: string): TranslationReference[] {
  const references = new Set<string>()
  const defaultNamespaces = collectDefaultNamespaces(source)
  const keyPropertyPattern = translationKeyPropertyNames.join('|')
  const patterns = [
    /\b(?:t|i18n\.t)\(\s*['"]([a-z0-9-]+:[^'"`]+)['"]/gi,
    /\b(?:t|i18n\.t)\(\s*['"]([^:'"`]+)['"]/gi,
    new RegExp(`\\b(?:entityLabel|label|title|${keyPropertyPattern}):\\s*['"]([^'"]+)['"]`, 'gi'),
    new RegExp(`\\b(?:${keyPropertyPattern})=["']([^"']+)["']`, 'gi'),
  ]

  for (const pattern of patterns) {
    for (const match of source.matchAll(pattern)) {
      const key = match[1]

      if (key)
        addTranslationReference(references, key, defaultNamespaces)
    }
  }

  return [...references].map(key => ({
    file: relative(frontendSrc, file).split(sep).join('/'),
    key,
  }))
}

describe('locale integrity', () => {
  it('keeps the supported locale list to English and Russian', () => {
    const locales = collectLocales()

    expect(Object.keys(locales).sort()).toEqual(['en', 'ru'])
  })

  it('keeps namespace files and key paths in sync across locales', () => {
    const locales = collectLocales()
    const [referenceLocale, comparisonLocale] = ['en', 'ru']
    const referenceTrees = locales[referenceLocale]
    const comparisonTrees = locales[comparisonLocale]

    if (!referenceTrees)
      throw new Error(`Missing reference locale: ${referenceLocale}`)

    if (!comparisonTrees)
      throw new Error(`Missing comparison locale: ${comparisonLocale}`)

    const referenceNamespaces = Object.keys(referenceTrees).sort()
    const comparisonNamespaces = Object.keys(comparisonTrees).sort()

    expect(comparisonNamespaces).toEqual(referenceNamespaces)

    for (const namespace of referenceNamespaces) {
      const referenceTree = referenceTrees[namespace]
      const comparisonTree = comparisonTrees[namespace]

      if (!referenceTree)
        throw new Error(`Missing namespace ${referenceLocale}/${namespace}`)

      if (!comparisonTree)
        throw new Error(`Missing namespace ${comparisonLocale}/${namespace}`)

      const referenceKeys = flattenKeys(referenceTree).sort()
      const comparisonKeys = flattenKeys(comparisonTree).sort()

      expect(comparisonKeys, [
        `${comparisonLocale}/${namespace} does not match ${referenceLocale}/${namespace}`,
        `Missing in ${comparisonLocale}: ${difference(referenceKeys, comparisonKeys).join(', ') || 'none'}`,
        `Extra in ${comparisonLocale}: ${difference(comparisonKeys, referenceKeys).join(', ') || 'none'}`,
      ].join('\n')).toEqual(referenceKeys)
    }
  })

  it('resolves literal translation keys used by source files', async () => {
    const locales = collectLocales()
    const sourceFiles = await collectSourceFiles(frontendSrc)
    const references = (await Promise.all(sourceFiles.map(async file =>
      collectTranslationReferences(file, await readFile(file, 'utf8')),
    ))).flat()
    const missing: string[] = []

    for (const { file, key } of references) {
      const [namespace, keyPath] = key.split(':')

      if (!namespace || !keyPath)
        continue

      for (const [locale, namespaces] of Object.entries(locales)) {
        const tree = namespaces[namespace]

        if (!tree || !hasTranslationKey(tree, keyPath))
          missing.push(`${file} references ${key}, missing in ${locale}`)
      }
    }

    expect(missing).toEqual([])
  })
})
