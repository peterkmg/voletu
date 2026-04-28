import { readdir, readFile } from 'node:fs/promises'
import { dirname, join, relative, sep } from 'node:path'
import { fileURLToPath } from 'node:url'
import ts from 'typescript'
import { describe, expect, it } from 'vitest'

const frontendSrc = dirname(dirname(dirname(fileURLToPath(import.meta.url))))
const auditedAttributes = new Set(['aria-label', 'title', 'placeholder'])
const auditedToastMethods = new Set(['error', 'success', 'info', 'warning', 'message'])
const allowedExact = new Set([
  'API',
  'ID',
  'JWT',
  'URL',
  'UUID',
  'MySQL',
  'PostgreSQL',
  'SQLite',
  'Voletu',
  'voletu',
])

interface Violation {
  file: string
  line: number
  column: number
  kind: string
  text: string
}

async function collectTsxFiles(directory: string): Promise<string[]> {
  const entries = await readdir(directory, { withFileTypes: true })
  const files = await Promise.all(entries.map(async (entry) => {
    const path = join(directory, entry.name)

    if (entry.isDirectory()) {
      if (entry.name === '__tests__')
        return []

      return collectTsxFiles(path)
    }

    if (!entry.isFile() || !entry.name.endsWith('.tsx') || entry.name.endsWith('.test.tsx') || entry.name === 'routeTree.gen.tsx')
      return []

    return [path]
  }))

  return files.flat()
}

function normalizeText(value: string) {
  return value.replace(/\s+/g, ' ').trim()
}

function isPlaceholderExample(text: string) {
  if (/^https?:\/\/[\w.-]+(?::\d+)?(?:\/[\w./~:%?#[\]{}=-]*)?$/i.test(text))
    return true

  if (/^(?:mailto|tel):\S+$/i.test(text))
    return true

  if (/^(?:\.{1,2}|~)?\/[\w./~:-]+$/.test(text))
    return true

  if (/^[A-Z]:\\[\w .\\-]+$/i.test(text))
    return true

  if (/^[a-z0-9][a-z0-9.-]+\.[a-z]{2,}(?::\d+)?(?:\/[\w./~:-]*)?$/i.test(text))
    return true

  if (/^[\w.-]+\.(?:db|sqlite|sqlite3)$/i.test(text))
    return true

  return false
}

function isAllowedText(value: string, kind: string) {
  const text = normalizeText(value)

  if (!text || !/\p{L}/u.test(text))
    return true

  if (/^&[a-z]+;$/i.test(text))
    return true

  if (allowedExact.has(text))
    return true

  if (/^(?:(?:Ctrl|Cmd|Alt|Shift|Meta|Option)\+)+(?:[A-Z0-9,./;'[\]\\`=\-]|Enter|Escape|Esc|Tab|Space|Backspace|Delete|Arrow(?:Up|Down|Left|Right))$|^F(?:[1-9]|1[0-2])$/.test(text))
    return true

  if (kind === 'placeholder' && isPlaceholderExample(text))
    return true

  return false
}

function locationFor(sourceFile: ts.SourceFile, node: ts.Node) {
  const position = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile))

  return {
    line: position.line + 1,
    column: position.character + 1,
  }
}

function collectViolations(file: string, source: string) {
  const sourceFile = ts.createSourceFile(file, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX)
  const violations: Violation[] = []

  function addViolation(node: ts.Node, kind: string, value: string) {
    const text = normalizeText(value)

    if (isAllowedText(text, kind))
      return

    violations.push({
      file: relative(frontendSrc, file).split(sep).join('/'),
      ...locationFor(sourceFile, node),
      kind,
      text,
    })
  }

  function visit(node: ts.Node) {
    if (ts.isJsxText(node)) {
      addViolation(node, 'jsx text', node.getText(sourceFile))
    }
    else if (ts.isJsxAttribute(node) && ts.isIdentifier(node.name) && auditedAttributes.has(node.name.text)) {
      const attributeName = node.name.text
      const initializer = node.initializer

      if (initializer && ts.isStringLiteral(initializer))
        addViolation(initializer, attributeName, initializer.text)

      if (initializer && ts.isJsxExpression(initializer) && initializer.expression && ts.isStringLiteral(initializer.expression))
        addViolation(initializer.expression, attributeName, initializer.expression.text)
    }
    else if (ts.isCallExpression(node)) {
      const expression = node.expression

      if (
        ts.isPropertyAccessExpression(expression)
        && ts.isIdentifier(expression.expression)
        && expression.expression.text === 'toast'
        && auditedToastMethods.has(expression.name.text)
      ) {
        const [message] = node.arguments
        if (message && ts.isStringLiteralLike(message))
          addViolation(message, 'toast message', message.text)
        if (message && ts.isTemplateExpression(message))
          addViolation(message, 'toast message', message.getText(sourceFile))
      }

      if (ts.isIdentifier(expression) && expression.text === 'extractErrorMessage') {
        const fallback = node.arguments[1]
        if (fallback && ts.isStringLiteralLike(fallback))
          addViolation(fallback, 'error fallback', fallback.text)
      }
    }

    ts.forEachChild(node, visit)
  }

  visit(sourceFile)

  return violations
}

describe('hardcoded UI strings', () => {
  it('does not allow example-ish or lowercase literals in user-facing labels', () => {
    expect(collectViolations(join(frontendSrc, 'fixture.tsx'), '<div>Demo mode</div>')).toHaveLength(1)
    expect(collectViolations(join(frontendSrc, 'fixture.tsx'), '<button title="Test connection" />')).toHaveLength(1)
    expect(collectViolations(join(frontendSrc, 'fixture.tsx'), '<button aria-label="save" />')).toHaveLength(1)
  })

  it('allows narrow literal placeholder examples', () => {
    expect(collectViolations(join(frontendSrc, 'fixture.tsx'), '<input placeholder="https://central.example.com" />')).toHaveLength(0)
    expect(collectViolations(join(frontendSrc, 'fixture.tsx'), '<input placeholder="voletu.db" />')).toHaveLength(0)
  })

  it('keeps TSX user-facing literals behind i18n', async () => {
    const files = await collectTsxFiles(frontendSrc)
    const violations = (await Promise.all(files.map(async file =>
      collectViolations(file, await readFile(file, 'utf8')),
    ))).flat()

    if (violations.length > 0) {
      const formatted = violations
        .slice(0, 100)
        .map(violation => `${violation.file}:${violation.line}:${violation.column} ${violation.kind} "${violation.text}"`)
        .join('\n')
      const suffix = violations.length > 100 ? `\n...and ${violations.length - 100} more` : ''

      throw new Error(`Found ${violations.length} hardcoded UI string(s):\n${formatted}${suffix}`)
    }
  })
})
