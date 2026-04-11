import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { DirectDispatchDetail } from '~/views/outgoing/direct-dispatch'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/outgoing/direct/$id')({
  component: DirectDispatchDetail,
})
