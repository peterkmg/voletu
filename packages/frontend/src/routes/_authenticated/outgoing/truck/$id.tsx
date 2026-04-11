import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { TruckDispatchDetail } from '~/views/outgoing/truck-dispatch'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/outgoing/truck/$id')({
  component: TruckDispatchDetail,
})
