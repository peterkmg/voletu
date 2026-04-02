import { createFileRoute } from '@tanstack/react-router'
import { ReconciliationDetail } from '~/features/internal/reconciliation'

export const Route = createFileRoute('/_authenticated/internal/reconciliation/$id')({
  component: ReconciliationDetail,
})
