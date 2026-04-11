import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { createEnabledListSearchSchema } from '~/router/search-schemas'
import { BlendingPage } from '~/views/internal/blending'

export const Route = defineListRoute(createFileRoute, '/_authenticated/internal/blending/')({
  validateSearch: createEnabledListSearchSchema,
  component: BlendingPage,
})
