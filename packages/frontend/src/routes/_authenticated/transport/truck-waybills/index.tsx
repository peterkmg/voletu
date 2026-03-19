import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { TruckWaybills } from '~/features/transport/truck-waybills'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute(
  '/_authenticated/transport/truck-waybills/',
)({
  validateSearch: searchSchema,
  component: TruckWaybills,
})
