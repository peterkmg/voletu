import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { CargoFlowPage } from '~/views/cargo-flow'

export const Route = defineListRoute(createFileRoute, '/_authenticated/cargo-flow/')({
  validateSearch: createEnabledListSearchSchema,
  component: CargoFlowPage,
})
