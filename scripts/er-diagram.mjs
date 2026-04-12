#!/usr/bin/env node
/**
 * Generates a Mermaid ER diagram from the Voletu database schema.
 *
 * 1. Compiles and runs the dump_schema example with -C link-dead-code
 *    to produce an unencrypted SQLite file with the full schema.
 * 2. Runs sea-orm-cli to generate the ER diagram from that file.
 * 3. Copies the .mermaid output to docs/ and cleans up.
 *
 * Prerequisites:
 *   cargo install sea-orm-cli --version 2.0.0-rc.38
 */

import { execSync } from 'node:child_process'
import { cpSync, existsSync, mkdirSync, rmSync } from 'node:fs'
import { resolve } from 'node:path'
import process from 'node:process'

const root = resolve(import.meta.dirname, '..')
const schemaDb = resolve(root, 'target/schema.db')
const erTmp = resolve(root, 'target/er-tmp')
const docsDir = resolve(root, 'docs')
const output = resolve(docsDir, 'entities.mermaid')

function run(cmd, env = {}) {
  console.log(`> ${cmd}`)
  execSync(cmd, {
    cwd: root,
    stdio: 'inherit',
    env: { ...process.env, ...env },
  })
}

// 1. Compile with -C link-dead-code so the linker keeps entity-registry
//    submissions, then run to produce target/schema.db.
run('cargo run -p voletu-core --example dump_schema')

// 2. Generate the ER diagram from the plain SQLite file.
//    Use relative paths — Windows absolute paths (D:\...) break sqlite:// URLs.
run(
  `sea-orm-cli generate entity --er-diagram -u sqlite://target/schema.db -o target/er-tmp`,
)

// 3. Copy the diagram to docs/ and clean up.
if (!existsSync(docsDir))
  mkdirSync(docsDir, { recursive: true })
cpSync(resolve(erTmp, 'entities.mermaid'), output)
rmSync(erTmp, { recursive: true })
rmSync(schemaDb)

console.log(`ER diagram written to ${output}`)
