import { createFileRoute } from '@tanstack/react-router'
import { OwnershipTransferDetail } from '~/views/internal/ownership-transfer'

export const Route = createFileRoute('/_authenticated/internal/ownership-transfer/$id')({
  component: OwnershipTransferDetail,
})
