import antfu from '@antfu/eslint-config'

export default antfu(
  {
    ignores: ['packages/frontend/src/routeTree.gen.ts', 'packages/frontend/src/generated/**'],

    formatters: true,

    stylistic: {
      indent: 2,
      quotes: 'single',
    },

    rules: {
      'toml/padding-line-between-pairs': 'off',
      // Legitimate pattern: syncing derived state (pagination, breakpoints, debounce)
      'react-hooks-extra/no-direct-set-state-in-use-effect': 'off',
      // Deliberate pattern: _setX destructuring when a custom setter wraps raw useState
      'react-naming-convention/use-state': 'off',
    },

    typescript: true,
    react: true,
  },
  {
    // TanStack Router requires exporting Route (non-component) alongside components
    files: ['packages/frontend/src/routes/**/*.tsx'],
    rules: { 'react-refresh/only-export-components': 'off' },
  },
  {
    // shadcn/ui components export variant utilities alongside components
    // Providers export both a component and a hook from the same file
    files: [
      'packages/frontend/src/components/ui/*.tsx',
      'packages/frontend/src/components/**/*-context.tsx',
      'packages/frontend/src/context/**/*.tsx',
      'packages/frontend/src/features/**/*-provider.tsx',
    ],
    rules: { 'react-refresh/only-export-components': 'off' },
  },
)
