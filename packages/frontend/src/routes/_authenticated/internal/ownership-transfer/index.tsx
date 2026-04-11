import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { OwnershipTransferPage } from '~/views/internal/ownership-transfer'

export const Route = defineListRoute(createFileRoute, '/_authenticated/internal/ownership-transfer/')({
  validateSearch: createEnabledListSearchSchema,
  component: OwnershipTransferPage,
})
