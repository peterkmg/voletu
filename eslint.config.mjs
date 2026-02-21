import antfu from '@antfu/eslint-config'

export default antfu({
  formatters: true,

  stylistic: {
    indent: 2,
    quotes: 'single',
  },

  rules: {
    'toml/padding-line-between-pairs': "off",
  },

  typescript: true,
  react: true,
})
