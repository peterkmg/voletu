import { createFileRoute } from '@tanstack/react-router'
import { TruckDispatchDetail } from '~/views/outgoing/truck-dispatch'

export const Route = createFileRoute('/_authenticated/outgoing/truck/$id')({
  component: TruckDispatchDetail,
})
