import { createFileRoute } from '@tanstack/react-router'
import { ReconciliationDetail } from '~/views/internal/reconciliation'

export const Route = createFileRoute('/_authenticated/internal/reconciliation/$id')({
  component: ReconciliationDetail,
})
