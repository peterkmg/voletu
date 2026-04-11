import { createFileRoute } from '@tanstack/react-router'
import { PhysicalTransferDetail } from '~/views/internal/physical-transfer'

export const Route = createFileRoute('/_authenticated/internal/physical-transfer/$id')({
  component: PhysicalTransferDetail,
})
