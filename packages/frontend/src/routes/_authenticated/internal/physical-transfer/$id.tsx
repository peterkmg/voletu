import { createFileRoute } from '@tanstack/react-router'
import { PhysicalTransferDetail } from '~/features/internal/physical-transfer'

export const Route = createFileRoute('/_authenticated/internal/physical-transfer/$id')({
  component: PhysicalTransferDetail,
})
