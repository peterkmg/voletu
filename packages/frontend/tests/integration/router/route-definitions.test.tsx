import { describe, expect, it } from 'vitest'
import { defineDetailRoute } from '~/router/define-detail-route'
import { defineListRoute } from '~/router/define-list-route'
import { defineViewRoute } from '~/router/define-view-route'
import {
  createEnabledListSearchSchema,
  paginatedListSearchSchema,
} from '~/router/search-schemas'

function TestListView() {
  return <div>list view</div>
}

function TestDetailView() {
  return <div>detail view</div>
}

describe('route definition helpers', () => {
  it('builds a list route from a shared search schema and component', () => {
    const Route = defineListRoute('/_authenticated/catalog/bases/')({
      validateSearch: paginatedListSearchSchema,
      component: TestListView,
    })

    expect(Route.options.validateSearch).toBe(paginatedListSearchSchema)
    expect(Route.options.component).toBe(TestListView)
  })

  it('supports create-enabled list schemas without re-declaring route glue', () => {
    const Route = defineListRoute('/_authenticated/incoming/external/')({
      validateSearch: createEnabledListSearchSchema,
      component: TestListView,
    })

    expect(Route.options.validateSearch).toBe(createEnabledListSearchSchema)
  })

  it('builds detail routes from the shared detail helper', () => {
    const Route = defineDetailRoute('/_authenticated/incoming/external/$id')({
      component: TestDetailView,
    })

    expect(Route.options.component).toBe(TestDetailView)
    expect(Route.options.validateSearch).toBeUndefined()
  })

  it('builds standalone view routes without repeating createFileRoute wiring', () => {
    const Route = defineViewRoute('/_authenticated/settings/')({
      component: TestDetailView,
    })

    expect(Route.options.component).toBe(TestDetailView)
  })
})
