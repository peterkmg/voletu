import { createFileRoute } from '@tanstack/react-router'
import { TruckReceiptDetail } from '~/views/incoming/truck-receipt'

export const Route = createFileRoute('/_authenticated/incoming/truck/$id')({
  component: TruckReceiptDetail,
})
