import process from 'node:process'
import { defineConfig } from '@kubb/core'
import { pluginClient } from '@kubb/plugin-client'
import { pluginOas } from '@kubb/plugin-oas'
import { pluginReactQuery } from '@kubb/plugin-react-query'
import { pluginTs } from '@kubb/plugin-ts'
import { pluginZod } from '@kubb/plugin-zod'

const openApiTarget
  = process.env.OPENAPI_URL ?? 'http://127.0.0.1:3000/api-docs/openapi.json'


function tagName(group: string, suffix: string): string {
  return group.replace(/[^a-z0-9]/gi, '') + suffix
}

export default defineConfig({
  input: { path: openApiTarget },
  output: { path: './src/generated', clean: true },
  plugins: [
    pluginOas(),
    pluginTs({
      output: { path: './types' },
      group: { type: 'tag', name: ({ group }) => tagName(group, 'Types') },
    }),
    pluginClient({
      output: { path: './client' },
      group: { type: 'tag', name: ({ group }) => tagName(group, 'Client') },
      importPath: '~/api/client',
    }),
    pluginZod({
      output: { path: './zod' },
      group: { type: 'tag', name: ({ group }) => tagName(group, 'Zod') },
      typed: true,
    }),
    pluginReactQuery({
      output: { path: './hooks' },
      group: { type: 'tag', name: ({ group }) => tagName(group, 'Hooks') },
      client: {
        dataReturnType: 'data',
        importPath: '~/api/client',
      },
      query: { methods: ['get'] },
      mutation: { methods: ['post', 'put', 'patch', 'delete'] },
    }),
  ],
})
