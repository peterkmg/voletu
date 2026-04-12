#!/usr/bin/env node
/**
 * Unified coverage report for the Voletu monorepo.
 *
 * Collects coverage from:
 *   - Frontend (Vitest + @vitest/coverage-v8)  ->  lcov
 *   - Rust workspace (cargo-llvm-cov)          ->  lcov
 *
 * Merges both into a single HTML report at coverage/html/index.html
 * and produces a canonical coverage/lcov.info for CI uploads
 * (Codecov, Coveralls, SonarCloud, etc.).
 *
 * Prerequisites (one-time install):
 *   rustup component add llvm-tools-preview
 *   cargo install cargo-llvm-cov
 *   cargo install grcov
 */

import { execSync } from 'node:child_process'
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from 'node:fs'
import { dirname, isAbsolute, join, resolve } from 'node:path'
import process from 'node:process'
import { fileURLToPath } from 'node:url'

const ROOT_DIR = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const COVERAGE_DIR = join(ROOT_DIR, 'coverage')

const c = {
  bold: '\x1B[1m',
  green: '\x1B[32m',
  red: '\x1B[31m',
  dim: '\x1B[2m',
  reset: '\x1B[0m',
}

function step(msg) {
  return console.log(`\n${c.bold}${c.green}>${c.reset} ${c.bold}${msg}${c.reset}`)
}

function fail(msg) {
  console.error(`\n${c.red}x ${msg}${c.reset}`)
  process.exit(1)
}

function run(cmd) {
  try {
    execSync(cmd, { cwd: ROOT_DIR, stdio: 'inherit' })
  }
  catch {
    fail(`Command failed: ${cmd}`)
  }
}

function checkTool(probe, hint) {
  try {
    execSync(probe, { stdio: 'ignore' })
  }
  catch {
    fail(`Required tool missing: \`${probe}\`\n   Install with: ${hint}`)
  }
}

/**
 * Normalize every `SF:` (source-file) entry in an lcov.info string.
 *
 * Converts each source path to an absolute, forward-slash form so
 * grcov can resolve files regardless of which collector wrote them:
 *   - Vitest emits paths relative to its working directory
 *     (e.g. `src/main.tsx` rooted at packages/frontend)
 *   - cargo-llvm-cov emits absolute Windows paths with backslashes
 *   - grcov (Linux-first) is happiest with one consistent style
 */
function normalizeLcov(lcov, baseDir) {
  return lcov
    .split('\n')
    .map((line) => {
      if (!line.startsWith('SF:'))
        return line
      let p = line.slice(3)
      if (!isAbsolute(p))
        p = resolve(baseDir, p)
      return `SF:${p.replace(/\\/g, '/')}`
    })
    .join('\n')
}

// ── Prerequisites ────────────────────────────────────────────
checkTool('cargo llvm-cov --version', 'cargo install cargo-llvm-cov')
checkTool('grcov --version', 'cargo install grcov')

// ── Clean previous output ────────────────────────────────────
step('Cleaning previous coverage output')
rmSync(COVERAGE_DIR, { recursive: true, force: true })
mkdirSync(COVERAGE_DIR, { recursive: true })

// ── Frontend coverage (Vitest, v8 provider) ──────────────────
step('Collecting frontend coverage (Vitest)')
run('pnpm --filter @voletu/frontend test:coverage')

const frontendLcov = join(
  ROOT_DIR,
  'packages',
  'frontend',
  'coverage',
  'lcov.info',
)
if (!existsSync(frontendLcov)) {
  fail(`Frontend lcov.info not found at: ${frontendLcov}`)
}
copyFileSync(frontendLcov, join(COVERAGE_DIR, 'frontend-lcov.info'))

// ── Rust coverage (cargo-llvm-cov) ───────────────────────────
step('Collecting Rust coverage (cargo-llvm-cov)')
const rustLcov = join(COVERAGE_DIR, 'rust-lcov.info')
run(
  `cargo llvm-cov --workspace --exclude voletu-core-macros --lcov --output-path "${rustLcov}"`,
)

// ── Merge ────────────────────────────────────────────────────
// lcov files for non-overlapping source trees can be safely
// concatenated, but every `SF:` path must first be rewritten to
// an absolute forward-slash form. Vitest writes paths relative
// to packages/frontend; cargo-llvm-cov writes absolute Windows
// paths with backslashes. Without normalization, grcov silently
// drops the frontend entries because it cannot find them under
// --source-dir.
step('Merging lcov data')
const FRONTEND_BASE = join(ROOT_DIR, 'packages', 'frontend')
const frontendNormalized = normalizeLcov(
  readFileSync(join(COVERAGE_DIR, 'frontend-lcov.info'), 'utf8'),
  FRONTEND_BASE,
)
const rustNormalized = normalizeLcov(
  readFileSync(rustLcov, 'utf8'),
  ROOT_DIR,
)
writeFileSync(
  join(COVERAGE_DIR, 'lcov.info'),
  [frontendNormalized, rustNormalized].join('\n'),
)

// ── HTML report (grcov) ──────────────────────────────────────
// Pass --source-dir as a forward-slash path for consistency
// with the normalized SF: entries above. grcov uses this dir
// to read source files when embedding them in the HTML report.
step('Generating unified HTML report (grcov)')
const htmlOut = join(COVERAGE_DIR, 'html')
const sourceDir = ROOT_DIR.replace(/\\/g, '/')
run(
  `grcov "${join(COVERAGE_DIR, 'lcov.info')}" --source-dir "${sourceDir}" --output-type html --branch --output-path "${htmlOut}"`,
)

console.log('')
console.log(`${c.bold}${c.green}Coverage report ready${c.reset}`)
console.log(`  ${c.dim}HTML:${c.reset}  ${c.bold}coverage/html/index.html${c.reset}`)
console.log(`  ${c.dim}lcov:${c.reset}  ${c.bold}coverage/lcov.info${c.reset}`)
