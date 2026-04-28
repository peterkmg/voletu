import { readdir, readFile } from 'node:fs/promises'
import { dirname, join, relative, sep } from 'node:path'
import { fileURLToPath } from 'node:url'
import ts from 'typescript'

const dataTableDir = dirname(dirname(fileURLToPath(import.meta.url)))
const auditedAttributes = new Set(['aria-label', 'title', 'placeholder'])
const forbiddenUiStrings = new Set([
  'Select all',
  'Select row',
  'Min',
  'Max',
  'Min value',
  'Max value',
  'Clear',
  'No results found.',
  'Clear filters',
  'Clear filter',
  'Open menu',
])

const forbiddenTranslationPrefixes = [
  'common:table',
  'common:dataTable',
  'table.',
  'dataTable.',
]

interface Violation {
  file: string
  line: number
  column: number
  kind: string
  text: string
}

async function collectDataTableTsxFiles(directory: string): Promise<string[]> {
  const entries = await readdir(directory, { withFileTypes: true })
  const files = await Promise.all(entries.map(async (entry) => {
    const path = join(directory, entry.name)

    if (entry.isDirectory()) {
      if (entry.name === '__tests__')
        return []

      return collectDataTableTsxFiles(path)
    }

    if (!entry.isFile() || !entry.name.endsWith('.tsx') || entry.name.endsWith('.test.tsx'))
      return []

    return [path]
  }))

  return files.flat()
}

function normalizeText(value: string) {
  return value.replace(/\s+/g, ' ').trim()
}

function locationFor(sourceFile: ts.SourceFile, node: ts.Node) {
  const position = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile))

  return {
    line: position.line + 1,
    column: position.character + 1,
  }
}

function violationFor(sourceFile: ts.SourceFile, file: string, node: ts.Node, kind: string, text: string): Violation {
  return {
    file: relative(dataTableDir, file).split(sep).join('/'),
    ...locationFor(sourceFile, node),
    kind,
    text,
  }
}

function collectViolations(file: string, source: string) {
  const sourceFile = ts.createSourceFile(file, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX)
  const violations: Violation[] = []

  function checkUiString(node: ts.Node, kind: string, value: string) {
    const text = normalizeText(value)

    if (forbiddenUiStrings.has(text))
      violations.push(violationFor(sourceFile, file, node, kind, text))
  }

  function checkTranslationLookup(node: ts.CallExpression) {
    if (!ts.isIdentifier(node.expression) || node.expression.text !== 't')
      return

    const [keyArg] = node.arguments
    if (!keyArg)
      return

    const checkedKeys = translationKeyCandidates(keyArg)
    for (const key of checkedKeys) {
      if (forbiddenTranslationPrefixes.some(prefix => key.startsWith(prefix))) {
        violations.push(violationFor(sourceFile, file, keyArg, 'translation key', key))
        return
      }
    }
  }

  function visit(node: ts.Node) {
    if (ts.isJsxText(node)) {
      checkUiString(node, 'jsx text', node.getText(sourceFile))
    }
    else if (ts.isJsxAttribute(node) && ts.isIdentifier(node.name) && auditedAttributes.has(node.name.text)) {
      const initializer = node.initializer

      if (initializer && ts.isStringLiteral(initializer))
        checkUiString(initializer, node.name.text, initializer.text)

      if (initializer && ts.isJsxExpression(initializer) && initializer.expression && ts.isStringLiteral(initializer.expression))
        checkUiString(initializer.expression, node.name.text, initializer.expression.text)
    }
    else if (ts.isCallExpression(node)) {
      checkTranslationLookup(node)
    }

    ts.forEachChild(node, visit)
  }

  visit(sourceFile)

  return violations
}

function translationKeyCandidates(node: ts.Expression) {
  if (ts.isStringLiteralLike(node))
    return [node.text]

  if (ts.isNoSubstitutionTemplateLiteral(node))
    return [node.text]

  if (ts.isTemplateExpression(node)) {
    return [
      node.head.text,
      ...node.templateSpans.map(span => span.literal.text),
    ].filter(Boolean)
  }

  return []
}

describe('data-table i18n', () => {
  it('keeps generic table UI strings in the tables namespace', async () => {
    const files = await collectDataTableTsxFiles(dataTableDir)
    const violations = (await Promise.all(files.map(async file =>
      collectViolations(file, await readFile(file, 'utf8')),
    ))).flat()

    if (violations.length > 0) {
      throw new Error(`Found ${violations.length} data-table i18n violation(s):\n${
        violations
          .map(violation => `${violation.file}:${violation.line}:${violation.column} ${violation.kind} "${violation.text}"`)
          .join('\n')
      }`)
    }
  })
})
