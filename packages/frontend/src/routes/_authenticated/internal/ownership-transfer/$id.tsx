import { createFileRoute } from '@tanstack/react-router'
import { OwnershipTransferDetail } from '~/features/internal/ownership-transfer'

export const Route = createFileRoute('/_authenticated/internal/ownership-transfer/$id')({
  component: OwnershipTransferDetail,
})
