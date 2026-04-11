import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { PhysicalTransferDetail } from '~/views/internal/physical-transfer'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/internal/physical-transfer/$id')({
  component: PhysicalTransferDetail,
})
