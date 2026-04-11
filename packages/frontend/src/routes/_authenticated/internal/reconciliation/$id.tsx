import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { ReconciliationDetail } from '~/views/internal/reconciliation'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/internal/reconciliation/$id')({
  component: ReconciliationDetail,
})
