import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { TruckReceiptDetail } from '~/views/incoming/truck-receipt'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/incoming/truck/$id')({
  component: TruckReceiptDetail,
})
