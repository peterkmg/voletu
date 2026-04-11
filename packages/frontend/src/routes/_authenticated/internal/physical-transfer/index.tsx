import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { PhysicalTransferPage } from '~/views/internal/physical-transfer'

export const Route = defineListRoute(createFileRoute, '/_authenticated/internal/physical-transfer/')({
  validateSearch: createEnabledListSearchSchema,
  component: PhysicalTransferPage,
})
