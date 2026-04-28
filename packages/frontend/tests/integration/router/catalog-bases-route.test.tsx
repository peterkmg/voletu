import { describe, expect, it } from 'vitest'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { Route } from '~/routes/_authenticated/catalog/bases'

describe('catalog bases route', () => {
  it('reuses the shared paginated list schema', () => {
    expect(Route.options.validateSearch).toBe(paginatedListSearchSchema)
  })
})
