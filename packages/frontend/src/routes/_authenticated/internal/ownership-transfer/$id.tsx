import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { OwnershipTransferDetail } from '~/views/internal/ownership-transfer'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/internal/ownership-transfer/$id')({
  component: OwnershipTransferDetail,
})
