import { createFileRoute } from '@tanstack/react-router'
import { TruckReceiptDetail } from '~/features/incoming/truck-receipt'

export const Route = createFileRoute('/_authenticated/incoming/truck/$id')({
  component: TruckReceiptDetail,
})
