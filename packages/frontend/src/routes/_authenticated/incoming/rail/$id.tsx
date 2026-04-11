import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { RailReceiptDetail } from '~/views/incoming/rail-receipt'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/incoming/rail/$id')({
  component: RailReceiptDetail,
})
