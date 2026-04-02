import { createFileRoute } from '@tanstack/react-router'
import { RailReceiptDetail } from '~/features/incoming/rail-receipt'

export const Route = createFileRoute('/_authenticated/incoming/rail/$id')({
  component: RailReceiptDetail,
})
