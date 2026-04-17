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
      // Redundant with the rule above — both flag setState-in-effect for the same patterns
      'react/set-state-in-effect': 'off',
      // Deliberate pattern: _setX destructuring when a custom setter wraps raw useState
      'react/use-state': 'off',
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
    // Composite-form per-doc *-form-config.tsx files co-locate schemas, types,
    // columns, and small field components by design (single source of truth
    // per document)
    files: [
      'packages/frontend/src/components/ui/*.tsx',
      'packages/frontend/src/components/**/*-context.tsx',
      'packages/frontend/src/context/**/*.tsx',
      'packages/frontend/src/features/**/*-provider.tsx',
      'packages/frontend/src/components/data-table/density.tsx',
      'packages/frontend/src/components/document/document-form.tsx',
      'packages/frontend/src/views/**/*-form-config.tsx',
    ],
    rules: { 'react-refresh/only-export-components': 'off' },
  },
  {
    // Factory files intentionally define components/hooks inside creator functions
    files: ['packages/frontend/src/lib/create-*.tsx'],
    rules: { 'react/component-hook-factories': 'off' },
  },
  {
    // Test infrastructure: vi.mock factories define hooks inside callbacks,
    // test-utils re-exports non-component helpers alongside wrappers
    files: [
      'packages/frontend/src/**/__tests__/**',
      'packages/frontend/src/**/*.test.*',
      'packages/frontend/src/test-utils.tsx',
    ],
    rules: {
      'react/component-hook-factories': 'off',
      'react/no-unnecessary-use-prefix': 'off',
      'react-refresh/only-export-components': 'off',
    },
  },
)
