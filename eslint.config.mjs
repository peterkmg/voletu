import antfu from '@antfu/eslint-config'

export default antfu(
  {
    ignores: ['packages/frontend/src/routeTree.gen.ts'],

    formatters: true,

    stylistic: {
      indent: 2,
      quotes: 'single',
    },

    rules: {
      'toml/padding-line-between-pairs': 'off',
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
      'packages/frontend/src/context/**/*.tsx',
      'packages/frontend/src/features/**/*-provider.tsx',
    ],
    rules: { 'react-refresh/only-export-components': 'off' },
  },
)
